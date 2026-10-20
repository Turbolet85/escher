//! Drives the command line's argv reader against the verb table: each line read by its rules in
//! order, the usage errors, each ending's streams and status, the usage line, and the `mcp` role
//! and `--record` readers. A child module of `cli`, so it reads the reader's private items and its
//! tests keep their `cli::tests` paths.

use blitz_test_harness::Busy;

use super::*;
use crate::refusal::Fault;

const APPS: [&str; 2] = ["counter", "flight-booker"];
const DIR: &str = "state";
const RECORD: &str = "kept";

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
        record: None,
    }
}

fn recorded_start(app: &str) -> Line {
    Line::Session {
        command: SessionCommand::Start {
            app: app.to_string(),
        },
        state_dir: PathBuf::from(DIR),
        record: Some(PathBuf::from(RECORD)),
    }
}

fn served(record: Option<&str>) -> Line {
    Line::Serve {
        app: "counter".to_string(),
        state_dir: PathBuf::from(DIR),
        record: record.map(PathBuf::from),
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
        // The host role, in its one shape, with or without its recording directory.
        (&["serve", "counter", "--session", DIR], served(None)),
        (
            &["serve", "counter", "--session", DIR, "--record", RECORD],
            served(Some(RECORD)),
        ),
        (
            &["serve", "counter", "--session", DIR, "--record"],
            Line::Usage,
        ),
        (
            &[
                "serve",
                "counter",
                "--session",
                DIR,
                "--record",
                RECORD,
                "--record",
                RECORD,
            ],
            Line::Usage,
        ),
        (
            &["serve", "counter", "--record", RECORD, "--session", DIR],
            Line::Usage,
        ),
        (
            &["serve", "counter", "--session", DIR, "--more", RECORD],
            Line::Usage,
        ),
        (&["serve"], Line::Usage),
        (&["serve", "counter"], Line::Usage),
        (&["serve", "counter", DIR], Line::Usage),
        (&["serve", "counter", "--session"], Line::Usage),
        (&["serve", "--session", DIR, "counter"], Line::Usage),
        (&["serve", "counter", "--session", DIR, "more"], Line::Usage),
        (&["serve", "timer", "--session", DIR], Line::Usage),
        // The MCP role, alone on its line or with its recording directory.
        (&["mcp"], Line::Mcp { record: None }),
        (
            &["mcp", "--record", RECORD],
            Line::Mcp {
                record: Some(PathBuf::from(RECORD)),
            },
        ),
        (&["mcp", "x"], Line::Usage),
        (&["mcp", "--session", DIR], Line::Usage),
        (&["mcp", "--record"], Line::Usage),
        (
            &["mcp", "--record", RECORD, "--record", RECORD],
            Line::Usage,
        ),
        (&["mcp", "--record", RECORD, "x"], Line::Usage),
        // A first word the table does not hold.
        (&["capture", "--session", DIR], unknown()),
        (&["Click", "--id", "save", "--session", DIR], unknown()),
        (&["", "--session", DIR], unknown()),
        (&["--session", DIR, "snapshot"], unknown()),
        (&["capture"], unknown()),
        // Each instance verb, its arguments as flags in any order.
        (&["snapshot", "--session", DIR], instance("snapshot", &[])),
        (
            &["screenshot", "--session", DIR],
            instance("screenshot", &[]),
        ),
        (
            &["screenshot", "--session", DIR, "--id", "crud-list"],
            instance("screenshot", &[("id", text("crud-list"))]),
        ),
        (&["lint", "--session", DIR], instance("lint", &[])),
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
        // `start` takes a recording directory, at most once and with its value.
        (
            &["start", "counter", "--session", DIR, "--record", RECORD],
            recorded_start("counter"),
        ),
        (
            &[
                "start",
                "--record",
                RECORD,
                "flight-booker",
                "--session",
                DIR,
            ],
            recorded_start("flight-booker"),
        ),
        (
            &["start", "counter", "--session", DIR, "--record"],
            Line::Usage,
        ),
        (
            &[
                "start",
                "counter",
                "--session",
                DIR,
                "--record",
                RECORD,
                "--record",
                RECORD,
            ],
            Line::Usage,
        ),
        (
            &["start", "timer", "--session", DIR, "--record", RECORD],
            Line::Usage,
        ),
        // Anywhere else a recording directory is a usage error.
        (
            &["snapshot", "--session", DIR, "--record", RECORD],
            Line::Usage,
        ),
        (
            &[
                "click",
                "--id",
                "save",
                "--record",
                RECORD,
                "--session",
                DIR,
            ],
            Line::Usage,
        ),
        (
            &["status", "--session", DIR, "--record", RECORD],
            Line::Usage,
        ),
        (&["stop", "--record", RECORD, "--session", DIR], Line::Usage),
        (&["help", "--record", RECORD], Line::Usage),
        (&["help", "--session", DIR, "--record"], Line::Usage),
        // The word after a flag is its value, `--record` included.
        (
            &[
                "type",
                "--id",
                "who",
                "--text",
                "--record",
                "--session",
                DIR,
            ],
            instance("type", &[("id", text("who")), ("text", text("--record"))]),
        ),
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
            &["lint", "--id", "save", "--session", DIR],
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
        // `help` takes no argument, and at most one `--session`, which it does not read.
        (&["help"], Line::Help),
        (&["help", "--session", DIR], Line::Help),
        (&["help", "x"], malformed(Fault::Unnamed)),
        (&["help", "--verb", "click"], malformed(Fault::Unnamed)),
        (&["help", "x", "--session", DIR], malformed(Fault::Unnamed)),
        (&["help", "--session"], Line::Usage),
        (&["help", "--session", DIR, "--session", DIR], Line::Usage),
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
    assert_eq!(rows.len(), 96);
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
        (Ending::Spoken(Ok(())), None, None, 0),
        (
            Ending::Spoken(Err(SessionError::Io(io::ErrorKind::BrokenPipe))),
            None,
            Some(SessionError::Io(io::ErrorKind::BrokenPipe).to_string()),
            3,
        ),
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
        failed(SessionError::RecordDirNotPrivate),
        failed(SessionError::RecordingFailed),
    ];
    assert_eq!(rows.len(), 21);
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
             than the session carries; the call ran and is not rolled back\",\"remedy\":\"the \
             call ran and is not rolled back: take a snapshot to read the screen as it \
             stands\"}}"
        )
    );
    assert_eq!((ACCEPTED, REFUSED, USAGE, SESSION_ERROR), (0, 1, 2, 3));
    let endings = [ACCEPTED, REFUSED, USAGE, SESSION_ERROR];
    assert_eq!(EXIT_STATUSES.map(|ending| ending.status), endings);
    let names = ["accepted", "refused", "usage", "session-error"];
    assert_eq!(EXIT_STATUSES.map(|ending| ending.name), names);
}

#[test]
fn the_usage_line_is_built_from_the_table_and_the_apps() {
    assert_eq!(
        usage(&APPS),
        "usage: <command> --session <dir>; commands: snapshot | click --id <id> | type --id \
         <id> --text <text> | press --key <key> [--shift] | advance --ms <milliseconds> | \
         scroll --id <id> | screenshot [--id <id>] | lint | start <app> [--record <dir>] | status \
         | stop | help; host role: serve <app> --session <dir> [--record <dir>]; mcp role: mcp \
         [--record <dir>]; apps: counter | flight-booker"
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
    assert_eq!(run(Line::Usage, &APPS, never), Ending::Usage);
    // `help` reaches no session and boots nothing.
    assert_eq!(run(Line::Help, &APPS, never), Ending::Accepted(describe()));
    let refusal = Refusal::malformed(Fault::Missing("id"));
    assert_eq!(
        run(Line::Refused(refusal), &APPS, never),
        Ending::Refused(refusal.to_json())
    );
    // The host role ends as a session error when its boot does.
    let serve = served(None);
    assert_eq!(
        run(serve, &APPS, |_| Err(SessionError::InvalidLabel)),
        Ending::Failed(SessionError::InvalidLabel)
    );
}
