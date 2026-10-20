//! The command line: every driver command as one run of a binary, answered with one line of
//! JSON on stdout and a status that says how it ended.

use std::ffi::OsString;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command as Process, ExitCode, Stdio};
use std::time::Duration;

use crate::client::{self, Answer};
use crate::command::{ArgValue, Call, SessionCommand, validate, validate_session};
use crate::describe::describe;
use crate::error::SessionError;
use crate::host::{IDLE_EXPIRY, serve};
use crate::json;
use crate::mcp::serve_mcp;
use crate::recording;
use crate::refusal::{Cause, Refusal};
use crate::schema::{self, ArgKind, ArgSpec, EXIT_STATUSES, Level, VERBS, VerbSpec};
use crate::session::Session;

/// The status of a command that ran.
const ACCEPTED: u8 = EXIT_STATUSES[0].status;
/// The status of a command that was refused.
const REFUSED: u8 = EXIT_STATUSES[1].status;
/// The status of a line that is no command.
const USAGE: u8 = EXIT_STATUSES[2].status;
/// The status of a command a session edge stopped.
const SESSION_ERROR: u8 = EXIT_STATUSES[3].status;

/// The option that names the session's state directory, on every command.
const SESSION_OPTION: &str = "--session";

/// The option that names a recording directory, on `start`, the host role and the MCP role.
const RECORD_OPTION: &str = "--record";

/// The first word of the host role.
const HOST_ROLE: &str = "serve";

/// The word of the MCP role.
const MCP_ROLE: &str = "mcp";

/// How long `start` waits for the host it spawns to answer.
const START_BOUND: Duration = Duration::from_secs(30);

/// Runs one command line and returns the process status.
///
/// `args` are the arguments after the program's name, `apps` the names of the apps the binary
/// boots, and `boot` builds the session of the app a name names: the binary's own boot, as
/// this crate names no app.
///
/// The first word is a verb of [`VERBS`], `serve` or `mcp`. A verb's arguments are flags spelt as
/// the schema spells them — `--id`, `--text`, `--key`, `--ms`, each followed by its value, and
/// a bare `--shift` — except that `start` takes its app as one plain word; `--session <dir>`
/// names the session's state directory on every command. `start` spawns the running binary
/// again as `serve <app> --session <dir>`, its streams closed, and answers once that host
/// does; given `--record <dir>`, it first creates that recording directory owner-only or
/// refuses it when it is open to its group or to others, then passes it on to the host, which
/// keeps its session as a replayable check there ([`Session::recording_into`]). `status`
/// reaches the session and `stop` ends it; `help` answers the tool's description
/// of itself ([`describe`](crate::describe)) in this process, reaching no session — it takes no
/// argument, and at most one `--session`, which it does not read; every other verb is sent to
/// the session as a call ([`call`](crate::call)). `serve` is the host role: it boots the app and
/// serves it in the foreground until it is stopped or has been idle for [`IDLE_EXPIRY`], and
/// given `--record <dir>` after its state directory records its session there. `mcp`, alone on
/// its line or with `--record <dir>`, is the MCP role: it speaks MCP on stdin and stdout
/// ([`serve_mcp`](crate::serve_mcp)), holding the apps it starts in its own process, until stdin
/// ends, and given a recording directory checks it before it serves and records each session it
/// starts there, one check from `start` to `stop`. Stdout is the protocol's: the role writes
/// nothing else there, ends `0` when stdin ends, and `3` with the error's fixed message on
/// stderr when a read or a write fails or its recording directory is refused. A `--record`
/// anywhere else, given twice or with no directory after it, is a usage error.
///
/// A command ends one of four ways, the rows of [`EXIT_STATUSES`], and its status says which:
///
/// - `0`, accepted: the answer on stdout as one line of JSON. A step that ran and did not go
///   quiet is accepted, and its `settled` reads false. The host role writes nothing.
/// - `1`, refused: the refusal on stdout as one line of JSON. A word that is no verb and an
///   argument the verb's schema does not admit are refusals. A call is checked before the
///   session is asked for, so a refused call reaches no session.
/// - `2`, usage: one fixed line on stderr and nothing on stdout — no command, an argument
///   that is not text, a `--session` that is absent or given twice, an app the binary does
///   not boot, a `serve` or an `mcp` line of another shape.
/// - `3`, session error: the error on stdout as one line of JSON, and its fixed message on
///   stderr.
///
/// Nothing a caller typed is written back in any ending. This function installs no log
/// subscriber and reads no environment variable.
pub fn command_line(
    args: impl IntoIterator<Item = OsString>,
    apps: &[&str],
    boot: impl FnMut(&str) -> Result<Session, SessionError>,
) -> ExitCode {
    let ending = match text_arguments(args) {
        Some(args) => run(read(&args, apps), apps, boot),
        None => Ending::Usage,
    };
    let written = ending.written(apps);
    write(&written);
    ExitCode::from(written.status)
}

/// The arguments as text, or `None` when one of them is not.
fn text_arguments(args: impl IntoIterator<Item = OsString>) -> Option<Vec<String>> {
    args.into_iter()
        .map(|argument| argument.into_string().ok())
        .collect()
}

/// What a command line asks for.
#[derive(Debug, PartialEq)]
enum Line {
    /// No command.
    Usage,
    /// A command the schema refuses.
    Refused(Refusal),
    /// The host role.
    Serve {
        app: String,
        state_dir: PathBuf,
        record: Option<PathBuf>,
    },
    /// The MCP role.
    Mcp { record: Option<PathBuf> },
    /// The tool's description of itself.
    Help,
    /// A session-level command; a recording directory on `start` alone.
    Session {
        command: SessionCommand,
        state_dir: PathBuf,
        record: Option<PathBuf>,
    },
    /// A call for the session's instance.
    Instance { call: Call, state_dir: PathBuf },
}

/// Reads a command line, each rule in its turn: the first word, then the call it spells,
/// checked against the schema, and only then the session's address.
fn read(args: &[String], apps: &[&str]) -> Line {
    let Some((word, rest)) = args.split_first() else {
        return Line::Usage;
    };
    if word == HOST_ROLE {
        return match rest {
            [app, option, state_dir, record @ ..]
                if option == SESSION_OPTION && apps.contains(&app.as_str()) =>
            {
                match record_option(record) {
                    Some(record) => Line::Serve {
                        app: app.clone(),
                        state_dir: PathBuf::from(state_dir),
                        record,
                    },
                    None => Line::Usage,
                }
            }
            _ => Line::Usage,
        };
    }
    if word == MCP_ROLE {
        return match record_option(rest) {
            Some(record) => Line::Mcp { record },
            None => Line::Usage,
        };
    }
    let Some(spec) = schema::verb(word) else {
        return Line::Refused(Refusal::new(Cause::UnknownVerb));
    };
    let (call, state_dirs, records) = read_arguments(spec, rest);
    let checked = match spec.level {
        Level::Instance => validate(&call).map(|_| None),
        Level::Session | Level::Tool => validate_session(&call).map(Some),
    };
    let command = match checked {
        Ok(command) => command,
        Err(refusal) => return Line::Refused(refusal),
    };
    if command == Some(SessionCommand::Help) {
        return match (state_dirs.as_slice(), records.as_slice()) {
            ([] | [Some(_)], []) => Line::Help,
            _ => Line::Usage,
        };
    }
    let [Some(state_dir)] = state_dirs.as_slice() else {
        return Line::Usage;
    };
    let state_dir = PathBuf::from(state_dir);
    let starts = matches!(command, Some(SessionCommand::Start { .. }));
    let record = match records.as_slice() {
        [] => None,
        [Some(record)] if starts => Some(PathBuf::from(record)),
        _ => return Line::Usage,
    };
    match command {
        None => Line::Instance { call, state_dir },
        Some(SessionCommand::Start { app }) if !apps.contains(&app.as_str()) => Line::Usage,
        Some(command) => Line::Session {
            command,
            state_dir,
            record,
        },
    }
}

/// The recording directory `words` name: `Some(None)` for no words, `Some(Some(dir))` for
/// `--record <dir>`, `None` for anything else.
fn record_option(words: &[String]) -> Option<Option<PathBuf>> {
    match words {
        [] => Some(None),
        [option, dir] if option == RECORD_OPTION => Some(Some(PathBuf::from(dir))),
        _ => None,
    }
}

/// The call the words after a verb spell, and the value of each `--session` and each
/// `--record` among them: `None` for one with no word after it.
///
/// Nothing is judged here. A word the verb's schema has no place for becomes an argument with
/// no name, a flag with no word after it one with no value of its kind, and a flag given twice
/// two arguments, so the schema's own check says what is wrong, in its own order.
fn read_arguments(
    spec: &VerbSpec,
    words: &[String],
) -> (Call, Vec<Option<String>>, Vec<Option<String>>) {
    // A session-level verb takes its arguments as plain words, an instance-level one as flags.
    let none: &[ArgSpec] = &[];
    let (flags, plain) = match spec.level {
        Level::Instance => (spec.args, none),
        Level::Session | Level::Tool => (none, spec.args),
    };
    let mut plain = plain.iter();
    let mut args = Vec::new();
    let mut state_dirs = Vec::new();
    let mut records = Vec::new();
    let mut words = words.iter();
    while let Some(word) = words.next() {
        if word == SESSION_OPTION {
            state_dirs.push(words.next().cloned());
            continue;
        }
        if word == RECORD_OPTION {
            records.push(words.next().cloned());
            continue;
        }
        let named = word.strip_prefix("--");
        let flag = named.and_then(|name| flags.iter().find(|flag| flag.name == name));
        let argument = match (flag, named) {
            (Some(flag), _) if flag.kind == ArgKind::Flag => {
                (flag.name.to_string(), ArgValue::Flag(true))
            }
            (Some(flag), _) => {
                let value = match words.next() {
                    Some(value) if matches!(flag.kind, ArgKind::Milliseconds | ArgKind::Part) => {
                        value
                            .parse()
                            .map_or_else(|_| ArgValue::Text(value.clone()), ArgValue::Number)
                    }
                    Some(value) => ArgValue::Text(value.clone()),
                    None => ArgValue::Flag(true),
                };
                (flag.name.to_string(), value)
            }
            (None, None) => match plain.next() {
                Some(argument) => (argument.name.to_string(), ArgValue::Text(word.clone())),
                None => (String::new(), ArgValue::Text(word.clone())),
            },
            (None, Some(_)) => (String::new(), ArgValue::Text(word.clone())),
        };
        args.push(argument);
    }
    let call = Call {
        verb: spec.name.to_string(),
        args,
    };
    (call, state_dirs, records)
}

/// How a command ended.
#[derive(Debug, PartialEq)]
enum Ending {
    /// It ran: its answer.
    Accepted(String),
    /// It was refused: its refusal.
    Refused(String),
    /// The line was no command.
    Usage,
    /// A session edge stopped it.
    Failed(SessionError),
    /// The host role ended, after a stop or an expiry.
    Served,
    /// The MCP role ended: its input ended, or a read or a write failed.
    Spoken(Result<(), SessionError>),
}

/// What an ending writes and returns.
#[derive(Debug, PartialEq)]
struct Written {
    stdout: Option<String>,
    stderr: Option<String>,
    status: u8,
}

impl Ending {
    fn written(&self, apps: &[&str]) -> Written {
        match self {
            Ending::Accepted(answer) => Written {
                stdout: Some(answer.clone()),
                stderr: None,
                status: ACCEPTED,
            },
            Ending::Refused(refusal) => Written {
                stdout: Some(refusal.clone()),
                stderr: None,
                status: REFUSED,
            },
            Ending::Usage => Written {
                stdout: None,
                stderr: Some(usage(apps)),
                status: USAGE,
            },
            Ending::Failed(error) => Written {
                stdout: Some(error.to_json()),
                stderr: Some(error.to_string()),
                status: SESSION_ERROR,
            },
            Ending::Served | Ending::Spoken(Ok(())) => Written {
                stdout: None,
                stderr: None,
                status: ACCEPTED,
            },
            // Stdout belongs to the protocol.
            Ending::Spoken(Err(error)) => Written {
                stdout: None,
                stderr: Some(error.to_string()),
                status: SESSION_ERROR,
            },
        }
    }
}

/// The usage line: every verb with its arguments as the table states them, the host role, the
/// MCP role and the apps.
fn usage(apps: &[&str]) -> String {
    let mut line = format!("usage: <command> {SESSION_OPTION} <dir>; commands:");
    for (index, verb) in VERBS.iter().enumerate() {
        line.push_str(if index == 0 { " " } else { " | " });
        line.push_str(verb.name);
        for argument in verb.args {
            let (name, kind) = (argument.name, argument.kind.name());
            let spelt = match (verb.level, argument.kind, argument.required) {
                (Level::Session | Level::Tool, _, _) => format!(" <{name}>"),
                (Level::Instance, ArgKind::Flag, _) => format!(" [--{name}]"),
                (Level::Instance, _, true) => format!(" --{name} <{kind}>"),
                (Level::Instance, _, false) => format!(" [--{name} <{kind}>]"),
            };
            line.push_str(&spelt);
        }
        if verb.name == schema::START.name {
            line.push_str(&format!(" [{RECORD_OPTION} <dir>]"));
        }
    }
    line.push_str(&format!(
        "; host role: {HOST_ROLE} <app> {SESSION_OPTION} <dir> [{RECORD_OPTION} <dir>]; mcp role: \
         {MCP_ROLE} [{RECORD_OPTION} <dir>]; apps: {}",
        apps.join(" | ")
    ));
    line
}

fn write(written: &Written) {
    // A reader that has gone away is not this command's failure: a write to a closed pipe is
    // dropped.
    if let Some(line) = &written.stdout {
        let mut stdout = io::stdout().lock();
        let _ = stdout
            .write_all(line.as_bytes())
            .and_then(|()| stdout.write_all(b"\n"))
            .and_then(|()| stdout.flush());
    }
    if let Some(line) = &written.stderr {
        let _ = writeln!(io::stderr().lock(), "{line}");
    }
}

fn run(
    line: Line,
    apps: &[&str],
    mut boot: impl FnMut(&str) -> Result<Session, SessionError>,
) -> Ending {
    let answered = |answer: Result<String, SessionError>| match answer {
        Ok(answer) => Ending::Accepted(answer),
        Err(error) => Ending::Failed(error),
    };
    match line {
        Line::Usage => Ending::Usage,
        Line::Refused(refusal) => Ending::Refused(refusal.to_json()),
        Line::Help => Ending::Accepted(describe()),
        Line::Serve {
            app,
            state_dir,
            record,
        } => match boot(&app)
            .and_then(|session| recording_into(session, record.as_deref()))
            .and_then(|session| serve(&state_dir, session, IDLE_EXPIRY))
        {
            Ok(()) => Ending::Served,
            Err(error) => Ending::Failed(error),
        },
        Line::Mcp { record: None } => Ending::Spoken(serve_mcp(
            io::stdin().lock(),
            io::stdout().lock(),
            apps,
            boot,
            IDLE_EXPIRY,
        )),
        Line::Mcp { record: Some(dir) } => {
            Ending::Spoken(recording::prepare_dir(&dir).and_then(|()| {
                serve_mcp(
                    io::stdin().lock(),
                    io::stdout().lock(),
                    apps,
                    |app| boot(app)?.recording_into(&dir),
                    IDLE_EXPIRY,
                )
            }))
        }
        Line::Session {
            command,
            state_dir,
            record,
        } => answered(match command {
            SessionCommand::Start { app } => start(&app, &state_dir, record.as_deref()),
            SessionCommand::Status => client::attach(&state_dir).map(|hello| json::status(&hello)),
            SessionCommand::Stop => client::stop(&state_dir).map(|()| json::stopped()),
            SessionCommand::Help => Ok(describe()),
        }),
        Line::Instance { call, state_dir } => match client::call(&state_dir, &call) {
            Ok(Answer {
                accepted: true,
                json,
            }) => Ending::Accepted(json),
            Ok(Answer { json, .. }) => Ending::Refused(json),
            Err(error) => Ending::Failed(error),
        },
    }
}

/// `session`, recording into `record` when one is named.
fn recording_into(session: Session, record: Option<&Path>) -> Result<Session, SessionError> {
    match record {
        Some(dir) => session.recording_into(dir),
        None => Ok(session),
    }
}

/// Starts a session on `app`: the running binary, spawned again in the host role with its
/// streams closed, and waited for until it answers. A recording directory is checked first, and
/// a refused one spawns nothing.
fn start(app: &str, state_dir: &Path, record: Option<&Path>) -> Result<String, SessionError> {
    if let Some(dir) = record {
        recording::prepare_dir(dir)?;
    }
    let binary = std::env::current_exe().map_err(|error| SessionError::Io(error.kind()))?;
    let mut host = Process::new(binary);
    host.args([HOST_ROLE, app, SESSION_OPTION]).arg(state_dir);
    if let Some(dir) = record {
        host.arg(RECORD_OPTION).arg(dir);
    }
    host.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // The session outlives this command: the host's handle is dropped, neither killed nor
    // reaped.
    let started = client::start(state_dir, host, START_BOUND)?;
    Ok(json::started(&started.hello))
}

#[cfg(test)]
#[path = "cli_tests.rs"]
mod tests;
