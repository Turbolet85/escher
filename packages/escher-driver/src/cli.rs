//! The command line: every driver command as one run of a binary, answered with one line of
//! JSON on stdout and a status that says how it ended.

use std::ffi::OsString;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command as Process, ExitCode, Stdio};
use std::time::Duration;

use crate::client::{self, Answer};
use crate::command::{ArgValue, Call, SessionCommand, validate, validate_session};
use crate::error::SessionError;
use crate::host::{IDLE_EXPIRY, serve};
use crate::json;
use crate::refusal::{Cause, Refusal};
use crate::schema::{self, ArgKind, ArgSpec, Level, VERBS, VerbSpec};
use crate::session::Session;

/// The status of a command that ran.
const ACCEPTED: u8 = 0;
/// The status of a command that was refused.
const REFUSED: u8 = 1;
/// The status of a line that is no command.
const USAGE: u8 = 2;
/// The status of a command a session edge stopped.
const SESSION_ERROR: u8 = 3;

/// The option that names the session's state directory, on every command.
const SESSION_OPTION: &str = "--session";

/// The first word of the host role.
const HOST_ROLE: &str = "serve";

/// How long `start` waits for the host it spawns to answer.
const START_BOUND: Duration = Duration::from_secs(30);

/// Runs one command line and returns the process status.
///
/// `args` are the arguments after the program's name, `apps` the names of the apps the binary
/// boots, and `boot` builds the session of the app a name names: the binary's own boot, as
/// this crate names no app.
///
/// The first word is a verb of [`VERBS`] or `serve`. A verb's arguments are flags spelt as
/// the schema spells them — `--id`, `--text`, `--key`, `--ms`, each followed by its value, and
/// a bare `--shift` — except that `start` takes its app as one plain word; `--session <dir>`
/// names the session's state directory on every command. `start` spawns the running binary
/// again as `serve <app> --session <dir>`, its streams closed, and answers once that host
/// does; `status` reaches the session and `stop` ends it; every other verb is sent to the
/// session as a call ([`call`](crate::call)). `serve` is the host role: it boots the app and
/// serves it in the foreground until it is stopped or has been idle for [`IDLE_EXPIRY`].
///
/// A command ends one of four ways, and its status says which:
///
/// - `0`, accepted: the answer on stdout as one line of JSON. A step that ran and did not go
///   quiet is accepted, and its `settled` reads false. The host role writes nothing.
/// - `1`, refused: the refusal on stdout as one line of JSON. A word that is no verb and an
///   argument the verb's schema does not admit are refusals. A call is checked before the
///   session is asked for, so a refused call reaches no session.
/// - `2`, usage: one fixed line on stderr and nothing on stdout — no command, an argument
///   that is not text, a `--session` that is absent or given twice, an app the binary does
///   not boot, a `serve` line of another shape.
/// - `3`, session error: the error on stdout as one line of JSON, and its fixed message on
///   stderr.
///
/// Nothing a caller typed is written back in any ending. This function installs no log
/// subscriber and reads no environment variable.
pub fn command_line(
    args: impl IntoIterator<Item = OsString>,
    apps: &[&str],
    boot: impl FnOnce(&str) -> Result<Session, SessionError>,
) -> ExitCode {
    let ending = match text_arguments(args) {
        Some(args) => run(read(&args, apps), boot),
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
    Serve { app: String, state_dir: PathBuf },
    /// A session-level command.
    Session {
        command: SessionCommand,
        state_dir: PathBuf,
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
            [app, option, state_dir]
                if option == SESSION_OPTION && apps.contains(&app.as_str()) =>
            {
                Line::Serve {
                    app: app.clone(),
                    state_dir: PathBuf::from(state_dir),
                }
            }
            _ => Line::Usage,
        };
    }
    let Some(spec) = schema::verb(word) else {
        return Line::Refused(Refusal::new(Cause::UnknownVerb));
    };
    let (call, state_dirs) = read_arguments(spec, rest);
    let checked = match spec.level {
        Level::Instance => validate(&call).map(|_| None),
        Level::Session => validate_session(&call).map(Some),
    };
    let command = match checked {
        Ok(command) => command,
        Err(refusal) => return Line::Refused(refusal),
    };
    let [Some(state_dir)] = state_dirs.as_slice() else {
        return Line::Usage;
    };
    let state_dir = PathBuf::from(state_dir);
    match command {
        None => Line::Instance { call, state_dir },
        Some(SessionCommand::Start { app }) if !apps.contains(&app.as_str()) => Line::Usage,
        Some(command) => Line::Session { command, state_dir },
    }
}

/// The call the words after a verb spell, and the value of each `--session` among them:
/// `None` for one with no word after it.
///
/// Nothing is judged here. A word the verb's schema has no place for becomes an argument with
/// no name, a flag with no word after it one with no value of its kind, and a flag given twice
/// two arguments, so the schema's own check says what is wrong, in its own order.
fn read_arguments(spec: &VerbSpec, words: &[String]) -> (Call, Vec<Option<String>>) {
    // A session-level verb takes its arguments as plain words, an instance-level one as flags.
    let none: &[ArgSpec] = &[];
    let (flags, plain) = match spec.level {
        Level::Instance => (spec.args, none),
        Level::Session => (none, spec.args),
    };
    let mut plain = plain.iter();
    let mut args = Vec::new();
    let mut state_dirs = Vec::new();
    let mut words = words.iter();
    while let Some(word) = words.next() {
        if word == SESSION_OPTION {
            state_dirs.push(words.next().cloned());
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
                    Some(value) if flag.kind == ArgKind::Milliseconds => value
                        .parse()
                        .map_or_else(|_| ArgValue::Text(value.clone()), ArgValue::Number),
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
    (call, state_dirs)
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
            Ending::Served => Written {
                stdout: None,
                stderr: None,
                status: ACCEPTED,
            },
        }
    }
}

/// The usage line: every verb with its arguments as the table states them, the host role and
/// the apps.
fn usage(apps: &[&str]) -> String {
    let mut line = format!("usage: <command> {SESSION_OPTION} <dir>; commands:");
    for (index, verb) in VERBS.iter().enumerate() {
        line.push_str(if index == 0 { " " } else { " | " });
        line.push_str(verb.name);
        for argument in verb.args {
            let (name, kind) = (argument.name, argument.kind.name());
            let spelt = match (verb.level, argument.kind, argument.required) {
                (Level::Session, _, _) => format!(" <{name}>"),
                (Level::Instance, ArgKind::Flag, _) => format!(" [--{name}]"),
                (Level::Instance, _, true) => format!(" --{name} <{kind}>"),
                (Level::Instance, _, false) => format!(" [--{name} <{kind}>]"),
            };
            line.push_str(&spelt);
        }
    }
    line.push_str(&format!(
        "; host role: {HOST_ROLE} <app> {SESSION_OPTION} <dir>; apps: {}",
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

fn run(line: Line, boot: impl FnOnce(&str) -> Result<Session, SessionError>) -> Ending {
    let answered = |answer: Result<String, SessionError>| match answer {
        Ok(answer) => Ending::Accepted(answer),
        Err(error) => Ending::Failed(error),
    };
    match line {
        Line::Usage => Ending::Usage,
        Line::Refused(refusal) => Ending::Refused(refusal.to_json()),
        Line::Serve { app, state_dir } => {
            match boot(&app).and_then(|session| serve(&state_dir, session, IDLE_EXPIRY)) {
                Ok(()) => Ending::Served,
                Err(error) => Ending::Failed(error),
            }
        }
        Line::Session { command, state_dir } => answered(match command {
            SessionCommand::Start { app } => start(&app, &state_dir),
            SessionCommand::Status => client::attach(&state_dir).map(|hello| json::status(&hello)),
            SessionCommand::Stop => client::stop(&state_dir).map(|()| json::stopped()),
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

/// Starts a session on `app`: the running binary, spawned again in the host role with its
/// streams closed, and waited for until it answers.
fn start(app: &str, state_dir: &Path) -> Result<String, SessionError> {
    let binary = std::env::current_exe().map_err(|error| SessionError::Io(error.kind()))?;
    let mut host = Process::new(binary);
    host.args([HOST_ROLE, app, SESSION_OPTION])
        .arg(state_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // The session outlives this command: the host's handle is dropped, neither killed nor
    // reaped.
    let started = client::start(state_dir, host, START_BOUND)?;
    Ok(json::started(&started.hello))
}

#[cfg(test)]
mod tests {
    use blitz_test_harness::Busy;

    use super::*;
    use crate::refusal::Fault;

    const APPS: [&str; 2] = ["counter", "flight-booker"];
    const DIR: &str = "state";

    fn line(words: &[&str]) -> Line {
        let args: Vec<String> = words.iter().map(|word| word.to_string()).collect();
        read(&args, &APPS)
    }

    fn text(value: &str) -> ArgValue {
        ArgValue::Text(value.to_string())
    }

    fn instance(verb: &str, args: &[(&str, ArgValue)]) -> Line {
        Line::Instance {
            call: Call {
                verb: verb.to_string(),
                args: args
                    .iter()
                    .map(|(name, value)| (name.to_string(), value.clone()))
                    .collect(),
            },
            state_dir: PathBuf::from(DIR),
        }
    }

    fn session(command: SessionCommand) -> Line {
        Line::Session {
            command,
            state_dir: PathBuf::from(DIR),
        }
    }

    fn malformed(fault: Fault) -> Line {
        Line::Refused(Refusal::malformed(fault))
    }

    #[test]
    fn a_line_is_read_by_its_rules_in_order() {
        let unknown = || Line::Refused(Refusal::new(Cause::UnknownVerb));
        let rows: Vec<(&[&str], Line)> = vec![
            // No first word.
            (&[], Line::Usage),
            // The host role, in its one shape.
            (
                &["serve", "counter", "--session", DIR],
                Line::Serve {
                    app: "counter".to_string(),
                    state_dir: PathBuf::from(DIR),
                },
            ),
            (&["serve"], Line::Usage),
            (&["serve", "counter"], Line::Usage),
            (&["serve", "counter", DIR], Line::Usage),
            (&["serve", "counter", "--session"], Line::Usage),
            (&["serve", "--session", DIR, "counter"], Line::Usage),
            (&["serve", "counter", "--session", DIR, "more"], Line::Usage),
            (&["serve", "timer", "--session", DIR], Line::Usage),
            // A first word the table does not hold.
            (&["screenshot", "--session", DIR], unknown()),
            (&["Click", "--id", "save", "--session", DIR], unknown()),
            (&["", "--session", DIR], unknown()),
            (&["--session", DIR, "snapshot"], unknown()),
            (&["screenshot"], unknown()),
            // Each instance verb, its arguments as flags in any order.
            (&["snapshot", "--session", DIR], instance("snapshot", &[])),
            (
                &["click", "--id", "save", "--session", DIR],
                instance("click", &[("id", text("save"))]),
            ),
            (
                &["click", "--session", DIR, "--id", "form//input[2]: a b"],
                instance("click", &[("id", text("form//input[2]: a b"))]),
            ),
            (
                &["type", "--text", "Ada\n", "--id", "who", "--session", DIR],
                instance("type", &[("text", text("Ada\n")), ("id", text("who"))]),
            ),
            (
                &["type", "--id", "who", "--text", "", "--session", DIR],
                instance("type", &[("id", text("who")), ("text", text(""))]),
            ),
            // The word after a flag is its value, whatever it is.
            (
                &[
                    "type",
                    "--id",
                    "--text",
                    "--text",
                    "--session",
                    "--session",
                    DIR,
                ],
                instance(
                    "type",
                    &[("id", text("--text")), ("text", text("--session"))],
                ),
            ),
            (
                &["press", "--key", "tab", "--session", DIR],
                instance("press", &[("key", text("tab"))]),
            ),
            (
                &["press", "--shift", "--key", "tab", "--session", DIR],
                instance(
                    "press",
                    &[("shift", ArgValue::Flag(true)), ("key", text("tab"))],
                ),
            ),
            (
                &["advance", "--ms", "250", "--session", DIR],
                instance("advance", &[("ms", ArgValue::Number(250))]),
            ),
            (
                &["scroll", "--id", "row-14", "--session", DIR],
                instance("scroll", &[("id", text("row-14"))]),
            ),
            // Each session verb; `start` takes its app as one plain word.
            (
                &["start", "counter", "--session", DIR],
                session(SessionCommand::Start {
                    app: "counter".to_string(),
                }),
            ),
            (
                &["start", "--session", DIR, "flight-booker"],
                session(SessionCommand::Start {
                    app: "flight-booker".to_string(),
                }),
            ),
            (
                &["status", "--session", DIR],
                session(SessionCommand::Status),
            ),
            (&["stop", "--session", DIR], session(SessionCommand::Stop)),
            // A flag the row does not name, a stray word, a flag given twice.
            (
                &["click", "--id", "save", "--now", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["snapshot", "--id", "save", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["click", "save", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["press", "--key", "tab", "--shift", "now", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["start", "--app", "counter", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["start", "counter", "timer", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["status", "now", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["stop", "--id", "--session", DIR],
                malformed(Fault::Unnamed),
            ),
            (
                &["click", "--id", "save", "--id", "other", "--session", DIR],
                malformed(Fault::Repeated("id")),
            ),
            (
                &[
                    "press",
                    "--key",
                    "tab",
                    "--shift",
                    "--shift",
                    "--session",
                    DIR,
                ],
                malformed(Fault::Repeated("shift")),
            ),
            // A value that is missing or not of its kind, or outside its bound.
            (
                &["click", "--session", DIR, "--id"],
                malformed(Fault::WrongKind("id")),
            ),
            (
                &["advance", "--session", DIR, "--ms"],
                malformed(Fault::WrongKind("ms")),
            ),
            (
                &["advance", "--ms", "soon", "--session", DIR],
                malformed(Fault::WrongKind("ms")),
            ),
            (
                &["advance", "--ms", "1.5", "--session", DIR],
                malformed(Fault::WrongKind("ms")),
            ),
            (
                &["advance", "--ms", "0", "--session", DIR],
                malformed(Fault::OutOfBound("ms")),
            ),
            (
                &["advance", "--ms", "-5", "--session", DIR],
                malformed(Fault::OutOfBound("ms")),
            ),
            (
                &["press", "--key", "f1", "--session", DIR],
                malformed(Fault::OutOfBound("key")),
            ),
            (
                &["click", "--id", "", "--session", DIR],
                malformed(Fault::OutOfBound("id")),
            ),
            (
                &["start", "Counter", "--session", DIR],
                malformed(Fault::OutOfBound("app")),
            ),
            // An argument the row requires and the line lacks.
            (
                &["click", "--session", DIR],
                malformed(Fault::Missing("id")),
            ),
            (
                &["type", "--id", "who", "--session", DIR],
                malformed(Fault::Missing("text")),
            ),
            (
                &["start", "--session", DIR],
                malformed(Fault::Missing("app")),
            ),
            // The call is checked before the session is asked for: a refused call with no
            // address, or with two, is still the refusal.
            (&["click"], malformed(Fault::Missing("id"))),
            (&["start"], malformed(Fault::Missing("app"))),
            (
                &["click", "--session", DIR, "--session", DIR],
                malformed(Fault::Missing("id")),
            ),
            (&["click", "--session"], malformed(Fault::Missing("id"))),
            // `--session` exactly once, with its value.
            (&["snapshot"], Line::Usage),
            (&["click", "--id", "save"], Line::Usage),
            (&["snapshot", "--session"], Line::Usage),
            (
                &["snapshot", "--session", DIR, "--session", DIR],
                Line::Usage,
            ),
            (&["status"], Line::Usage),
            (&["start", "counter"], Line::Usage),
            // `start` on an app the binary does not boot.
            (&["start", "timer", "--session", DIR], Line::Usage),
        ];
        assert_eq!(rows.len(), 61);
        for (row, (words, read)) in rows.into_iter().enumerate() {
            assert!(line(words) == read, "row {row}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn an_argument_that_is_not_text_is_a_usage_error() {
        use std::os::unix::ffi::OsStringExt;

        let text = |words: &[&str]| words.iter().map(OsString::from).collect::<Vec<_>>();
        assert_eq!(
            text_arguments(text(&["snapshot", "--session", DIR])),
            Some(vec![
                "snapshot".to_string(),
                "--session".to_string(),
                DIR.to_string()
            ])
        );
        assert_eq!(text_arguments(Vec::new()), Some(Vec::new()));
        let mut not_text = text(&["click", "--id"]);
        not_text.push(OsString::from_vec(vec![0x66, 0xff]));
        assert_eq!(text_arguments(not_text), None);
    }

    #[test]
    fn each_ending_writes_its_streams_and_returns_its_status() {
        let usage_line = usage(&APPS);
        let failed = |error: SessionError| {
            (
                Ending::Failed(error),
                Some(error.to_json()),
                Some(error.to_string()),
                3,
            )
        };
        let rows = [
            (
                Ending::Accepted(r#"{"text":""}"#.to_string()),
                Some(r#"{"text":""}"#.to_string()),
                None,
                0,
            ),
            (
                Ending::Refused(Refusal::new(Cause::Covered).to_json()),
                Some(Refusal::new(Cause::Covered).to_json()),
                None,
                1,
            ),
            (Ending::Usage, None, Some(usage_line.clone()), 2),
            (Ending::Served, None, None, 0),
            failed(SessionError::AlreadyRunning),
            failed(SessionError::NoSession),
            failed(SessionError::Dead),
            failed(SessionError::HostExited(Some(101))),
            failed(SessionError::Timeout),
            failed(SessionError::Protocol),
            failed(SessionError::InvalidLabel),
            failed(SessionError::StateDirTooLong),
            failed(SessionError::StateDirNotPrivate),
            failed(SessionError::Unsupported),
            failed(SessionError::Io(io::ErrorKind::PermissionDenied)),
            failed(SessionError::NotSettled(Busy::Render)),
            failed(SessionError::AnswerTooLarge),
        ];
        assert_eq!(rows.len(), 17);
        for (row, (ending, stdout, stderr, status)) in rows.into_iter().enumerate() {
            let written = ending.written(&APPS);
            let expected = Written {
                stdout,
                stderr,
                status,
            };
            assert!(written == expected, "row {row}");
            for stream in [&written.stdout, &written.stderr].into_iter().flatten() {
                assert!(
                    !stream.contains('\n') && !stream.contains('\u{1b}'),
                    "row {row}"
                );
            }
        }
        // An answer over the socket's bound ends its command as a session error.
        let too_large = Ending::Failed(SessionError::AnswerTooLarge).written(&APPS);
        assert_eq!(too_large.status, SESSION_ERROR);
        assert_eq!(
            too_large.stdout.as_deref(),
            Some(
                "{\"error\":{\"kind\":\"answer-too-large\",\"message\":\"the answer is larger \
                 than the session carries; the call ran and is not rolled back\"}}"
            )
        );
        assert_eq!((ACCEPTED, REFUSED, USAGE, SESSION_ERROR), (0, 1, 2, 3));
    }

    #[test]
    fn the_usage_line_is_built_from_the_table_and_the_apps() {
        assert_eq!(
            usage(&APPS),
            "usage: <command> --session <dir>; commands: snapshot | click --id <id> | type --id \
             <id> --text <text> | press --key <key> [--shift] | advance --ms <milliseconds> | \
             scroll --id <id> | start <app> | status | stop; host role: serve <app> --session \
             <dir>; apps: counter | flight-booker"
        );
        for verb in VERBS {
            assert!(usage(&[]).contains(verb.name));
        }
    }

    #[test]
    fn a_line_that_is_no_command_or_is_refused_runs_nothing() {
        let never = |_: &str| -> Result<Session, SessionError> {
            panic!("the boot ran for a line that reaches no session")
        };
        assert_eq!(run(Line::Usage, never), Ending::Usage);
        let refusal = Refusal::malformed(Fault::Missing("id"));
        assert_eq!(
            run(Line::Refused(refusal), never),
            Ending::Refused(refusal.to_json())
        );
        // The host role ends as a session error when its boot does.
        let serve = Line::Serve {
            app: "counter".to_string(),
            state_dir: PathBuf::from(DIR),
        };
        assert_eq!(
            run(serve, |_| Err(SessionError::InvalidLabel)),
            Ending::Failed(SessionError::InvalidLabel)
        );
    }
}
