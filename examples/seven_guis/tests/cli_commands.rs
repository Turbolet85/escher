//! Every driver command through the real `escher-session` binary: `start`, `status`, `stop` and
//! each of the six verbs answer one line of uncoloured JSON on stdout and end with a status
//! that tells accepted (`0`) from refused (`1`) and from a session error (`3`). The snapshot's
//! line is the library's own form of the screen booted in process, inside its byte budget. Each
//! check starts and stops its own session; a failure names a kind, a status or a count, never
//! an id, a name or a value. Unix only: a session crosses a Unix-domain socket.

#![cfg(unix)]

mod common;

use common::{Ran, Started, cause, clear, client, keys, number, state_dir, text};
use escher_driver::{Outcome, SessionError, VERBS, verb};
use seven_guis::stand::{self, LeanTask};

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

/// The four tasks the binary boots, by the slug `start` takes.
const TASKS: [(&str, LeanTask); 4] = [
    ("counter", LeanTask::Counter),
    ("flight-booker", LeanTask::FlightBooker),
    ("timer", LeanTask::Timer),
    ("crud", LeanTask::Crud),
];

/// An id no element of the stand carries.
const NOBODY: &str = "cli-names-no-element";

/// The fields the table states for `name`, without the one present only when a step did not
/// go quiet.
fn fields(name: &str) -> Vec<String> {
    verb(name)
        .expect("the table lists the verb")
        .fields
        .iter()
        .filter(|field| field.always)
        .map(|field| field.name.to_string())
        .collect()
}

/// A session started through the binary, and the commands run against it.
struct Session {
    started: Started,
}

impl Session {
    #[track_caller]
    fn start(name: &str, task: &str) -> Session {
        let dir = state_dir(name);
        clear(&dir);
        let ran = client(BINARY, &["start", task, "--session", text(&dir)]);
        assert_eq!(ran.status, Some(0), "start ends accepted");
        let pid = number(ran.line(), "pid").expect("start answers its host's process id");
        Session {
            started: Started {
                dir,
                pid: Some(pid),
            },
        }
    }

    fn run(&self, args: &[&str]) -> Ran {
        let mut line = args.to_vec();
        line.extend(["--session", text(&self.started.dir)]);
        client(BINARY, &line)
    }

    /// The answer of a command that must end accepted, with nothing on its stderr.
    #[track_caller]
    fn accepted(&self, args: &[&str]) -> String {
        let ran = self.run(args);
        assert_eq!(ran.status, Some(0), "the command ends accepted");
        assert!(
            ran.stderr.is_empty(),
            "an accepted command wrote {} bytes to stderr",
            ran.stderr.len()
        );
        ran.line().to_string()
    }

    /// The cause of a command that must end refused.
    #[track_caller]
    fn refused(&self, args: &[&str]) -> String {
        let ran = self.run(args);
        assert_eq!(ran.status, Some(1), "the command ends refused");
        cause(ran.line())
            .expect("a refused command answers its refusal")
            .to_string()
    }

    #[track_caller]
    fn stop(self) {
        assert_eq!(self.accepted(&["stop"]), r#"{"stopped":true}"#);
        assert!(!self.started.dir.exists(), "stop leaves no state directory");
    }
}

#[test]
fn each_task_starts_answers_and_stops() {
    assert_eq!(VERBS.len(), 9);
    for (index, (task, _)) in TASKS.into_iter().enumerate() {
        let dir = state_dir(&format!("cc-life-{index}"));
        clear(&dir);
        let at = text(&dir);

        let ran = client(BINARY, &["start", task, "--session", at]);
        assert_eq!(ran.status, Some(0), "task {index}: start ends accepted");
        assert!(
            ran.stderr.is_empty(),
            "task {index}: start's stderr is empty"
        );
        let started = ran.line().to_string();
        let pid = number(&started, "pid").expect("start answers a process id");
        let _guard = Started {
            dir: dir.clone(),
            pid: Some(pid),
        };
        assert!(
            keys(&started) == fields("start"),
            "task {index}: start's fields"
        );
        assert!(
            started.starts_with(&format!("{{\"label\":\"{task}\",")),
            "task {index}: start answers the task's label"
        );
        assert_eq!(number(&started, "idle_expiry_s"), Some(1800));

        let ran = client(BINARY, &["status", "--session", at]);
        assert_eq!(ran.status, Some(0), "task {index}: status ends accepted");
        assert!(
            ran.stderr.is_empty(),
            "task {index}: status's stderr is empty"
        );
        let status = ran.line();
        assert!(
            keys(status) == fields("status"),
            "task {index}: status's fields"
        );
        assert!(
            status.starts_with(&format!("{{\"label\":\"{task}\",")),
            "task {index}: status answers the task's label"
        );
        assert_eq!(number(status, "pid"), Some(pid), "the same host answers");
        assert_eq!(number(status, "idle_expiry_s"), Some(1800));

        let again = client(BINARY, &["start", task, "--session", at]);
        assert_eq!(
            again.status,
            Some(3),
            "task {index}: a start while the session is up is a session error"
        );
        assert!(
            again.line() == SessionError::AlreadyRunning.to_json(),
            "task {index}: the error is the one for a running session"
        );

        let ran = client(BINARY, &["stop", "--session", at]);
        assert_eq!(ran.status, Some(0), "task {index}: stop ends accepted");
        assert!(
            ran.stderr.is_empty(),
            "task {index}: stop's stderr is empty"
        );
        assert!(
            keys(ran.line()) == fields("stop"),
            "task {index}: stop's fields"
        );
        assert_eq!(ran.line(), r#"{"stopped":true}"#);
        assert!(
            !dir.exists(),
            "task {index}: stop leaves no state directory"
        );

        let after = client(BINARY, &["status", "--session", at]);
        assert_eq!(
            after.status,
            Some(3),
            "task {index}: a command after stop is a session error"
        );
    }
}

#[test]
fn each_verb_is_accepted_and_answers_its_fields() {
    let crud = Session::start("cc-verbs", "crud");
    let rows: [(&str, &[&str]); 6] = [
        ("snapshot", &["snapshot"]),
        ("click", &["click", "--id", "crud-create"]),
        (
            "type",
            &["type", "--id", "crud-name", "--text", "Ada Lovelace"],
        ),
        ("press", &["press", "--key", "tab"]),
        ("press", &["press", "--key", "tab", "--shift"]),
        ("scroll", &["scroll", "--id", "crud-create"]),
    ];
    for (row, (name, args)) in rows.into_iter().enumerate() {
        let answer = crud.accepted(args);
        assert!(
            keys(&answer) == fields(name),
            "row {row}: the verb's fields"
        );
        if name != "snapshot" {
            assert!(
                answer.starts_with("{\"settled\":true,"),
                "row {row}: the step went quiet"
            );
        }
    }
    let typed = crud.accepted(&["type", "--id", "crud-surname", "--text", "Byron \"B\"\n"]);
    assert!(
        typed.contains(r#""value":"Byron \"B\"""#),
        "the typed text is the control's value in the answer, a line break typing nothing"
    );
    let scrolled = crud.accepted(&["scroll", "--id", "crud-create"]);
    assert!(
        scrolled.ends_with(",\"in_view\":true}"),
        "a scroll says its target is in view"
    );
    crud.stop();

    let timer = Session::start("cc-verbs-t", "timer");
    let advanced = timer.accepted(&["advance", "--ms", "100"]);
    assert!(keys(&advanced) == fields("advance"), "advance's fields");
    assert_eq!(number(&advanced, "advanced_ms"), Some(100));
    timer.stop();
}

#[test]
fn a_snapshot_is_the_in_process_text_as_one_line_inside_its_budget() {
    for (index, (task, lean)) in TASKS.into_iter().enumerate() {
        // The host boots in the incremental layout mode; so does this.
        let screen = stand::boot(lean, stand::options(true)).doc.snapshot();
        let in_process = Outcome::Screen {
            text: screen.to_text(),
        }
        .to_json();
        assert!(
            screen.nodes().count() >= 10,
            "task {index}: the screen read in process holds its nodes"
        );

        let session = Session::start(&format!("cc-snap-{index}"), task);
        let answer = session.accepted(&["snapshot"]);
        assert!(
            answer == in_process,
            "task {index}: the command's line is the in-process screen's written form \
             ({} bytes against {})",
            answer.len(),
            in_process.len()
        );
        // The whole stdout line — the answer and its line break — is inside the budget.
        assert!(
            answer.len() < dioxus_native::SNAPSHOT_TEXT_BUDGET,
            "task {index}: the line is {} bytes",
            answer.len() + 1
        );
        assert_eq!(dioxus_native::SNAPSHOT_TEXT_BUDGET, 10_000);
        assert!(keys(&answer) == ["text"], "task {index}: one field");
        session.stop();
    }
}

#[test]
fn a_call_a_target_cannot_take_is_refused_with_its_cause() {
    let crud = Session::start("cc-refuse", "crud");
    assert_eq!(crud.refused(&["click", "--id", "crud-delete"]), "disabled");
    assert_eq!(crud.refused(&["click", "--id", NOBODY]), "not-found");
    assert_eq!(crud.refused(&["scroll", "--id", NOBODY]), "not-found");

    // Twelve rows more put the last one past the list's box: refused, scrolled to, clicked.
    for _ in 0..12 {
        crud.accepted(&["click", "--id", "crud-create"]);
    }
    assert_eq!(
        crud.refused(&["click", "--id", "crud-person-14"]),
        "off-screen"
    );
    let scrolled = crud.accepted(&["scroll", "--id", "crud-person-14"]);
    assert!(
        scrolled.ends_with(",\"in_view\":true}"),
        "the scroll says the row is in view"
    );
    crud.accepted(&["click", "--id", "crud-person-14"]);

    // The row the click selected is deleted: its id was read and names nothing now.
    crud.accepted(&["click", "--id", "crud-delete"]);
    assert_eq!(crud.refused(&["click", "--id", "crud-person-14"]), "stale");
    crud.stop();

    let flight = Session::start("cc-refuse-f", "flight-booker");
    assert_eq!(
        flight.refused(&["click", "--id", "flight-return-date"]),
        "disabled"
    );
    assert_eq!(
        flight.refused(&["type", "--id", "flight-return-date", "--text", "02.02.2026"]),
        "disabled"
    );
    flight.stop();

    let counter = Session::start("cc-refuse-c", "counter");
    assert_eq!(
        counter.refused(&["advance", "--ms", "100"]),
        "time-unavailable"
    );
    counter.stop();
}

#[test]
fn a_call_the_schema_refuses_reaches_no_session() {
    let dir = state_dir("cc-none");
    clear(&dir);
    let at = text(&dir);
    let rows: [(&[&str], &str); 6] = [
        (&["screenshot", "--session", at], "unknown-verb"),
        (&["screenshot"], "unknown-verb"),
        (&["click", "--session", at], "malformed"),
        (&["click"], "malformed"),
        (&["advance", "--ms", "soon", "--session", at], "malformed"),
        (&["press", "--key", "f1", "--session", at], "malformed"),
    ];
    for (row, (args, refused)) in rows.into_iter().enumerate() {
        let ran = client(BINARY, args);
        assert_eq!(ran.status, Some(1), "row {row}: the command ends refused");
        assert!(cause(ran.line()) == Some(refused), "row {row}: its cause");
        assert!(ran.stderr.is_empty(), "row {row}: its stderr is empty");
        assert!(!dir.exists(), "row {row}: no state directory is created");
    }
    let missing = client(BINARY, &["click", "--session", at]);
    assert!(
        missing
            .line()
            .ends_with(",\"fault\":\"the required argument `id` is missing\"}}"),
        "a malformed call's answer names the rule it broke"
    );
}

#[test]
fn a_command_with_no_session_is_a_session_error() {
    let dir = state_dir("cc-absent");
    clear(&dir);
    let at = text(&dir);
    let rows: [&[&str]; 4] = [
        &["snapshot", "--session", at],
        &["click", "--id", "counter-increment", "--session", at],
        &["status", "--session", at],
        &["stop", "--session", at],
    ];
    for (row, args) in rows.into_iter().enumerate() {
        let ran = client(BINARY, args);
        assert_eq!(
            ran.status,
            Some(3),
            "row {row}: the command is a session error"
        );
        assert!(
            ran.line() == SessionError::NoSession.to_json(),
            "row {row}: stdout is the error for no session"
        );
        assert!(
            ran.stderr == format!("{}\n", SessionError::NoSession).as_bytes(),
            "row {row}: stderr is the error's fixed message, {} bytes",
            ran.stderr.len()
        );
        assert!(!dir.exists(), "row {row}: no state directory is created");
    }
}

#[test]
fn a_line_that_is_no_command_is_a_usage_error() {
    let dir = state_dir("cc-usage");
    clear(&dir);
    let at = text(&dir);
    let rows: [&[&str]; 5] = [
        &[],
        &["snapshot"],
        &["click", "--id", "counter-increment"],
        &["status", "--session", at, "--session", at],
        &["start", "no-such-task", "--session", at],
    ];
    for (row, args) in rows.into_iter().enumerate() {
        let ran = client(BINARY, args);
        assert_eq!(ran.status, Some(2), "row {row}: the line is a usage error");
        assert!(
            ran.stdout.is_empty(),
            "row {row}: stdout holds {} bytes",
            ran.stdout.len()
        );
        let stderr = String::from_utf8_lossy(&ran.stderr);
        assert!(
            stderr.starts_with("usage: ") && stderr.lines().count() == 1,
            "row {row}: stderr is one usage line"
        );
        assert!(!dir.exists(), "row {row}: no state directory is created");
    }
}

#[test]
fn tab_on_a_fresh_boot_moves_focus_to_the_back_button_alone() {
    let counter = Session::start("cc-tab", "counter");
    let answer = counter.accepted(&["press", "--key", "tab"]);
    assert!(
        answer.starts_with(
            "{\"settled\":true,\"added\":[],\"removed\":[],\"changed\":[{\"id\":\"back-btn\","
        ),
        "the diff adds and removes nothing and its first changed node is the back button"
    );
    assert!(
        answer.matches("\"id\":").count() == 1 && answer.matches("\"focused\":true").count() == 1,
        "the diff names one node, and it reads focused"
    );
    counter.stop();
}
