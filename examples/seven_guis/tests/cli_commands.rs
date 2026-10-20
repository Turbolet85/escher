//! Every driver command through the real `escher-session` binary: `start`, `status`, `stop` and
//! each of the eight verbs answer one line of uncoloured JSON on stdout and end with a status
//! that tells accepted (`0`) from refused (`1`) and from a session error (`3`). The snapshot's
//! line is the library's own form of the screen booted in process, inside its byte budget; the
//! screenshot's `png` is the PNG of the screen booted in process, 800 × 600; a lint on a held
//! counter answers one line whose `findings` is empty. On a screen inside the budget,
//! `snapshot --part 1` answers that screen's text as part 1 of 1, a part past it the empty text,
//! and a part outside its bound or of another kind is refused `malformed`, its line the
//! library's own refusal.
//! `help` answers the library's own description of the tool, reaching no session. Every
//! refusal the binary is driven to — six causes, all but `covered` and `fonts-unavailable` — and
//! every session error
//! read here names the remedy the table states, and on Linux a state directory of 94 bytes, as
//! given, holds a session while one of 95 is refused with its bound. With its stdout a pipe no
//! one reads, a command drops its answer's write and ends with its ending's status, no panic
//! and nothing typed written back. Each check starts and stops
//! its own session; a failure names a kind, a status or a count, never an id, a name or a
//! value. Unix only: a session crosses a Unix-domain socket.

#![cfg(unix)]

mod common;

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use common::{
    RUN_BOUND, Ran, Started, cause, clear, client, drain, keys, number, state_dir, string_at, text,
};
use escher_driver::{CAUSES, Call, Fault, Outcome, Refusal, SessionError, VERBS, describe, verb};
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

/// The cause a refused command's line names, read once its remedy is checked against the one
/// the cause states.
#[track_caller]
fn remedied(line: &str) -> String {
    let named = cause(line).expect("a refused command answers its refusal");
    let stated = CAUSES
        .into_iter()
        .find(|cause| cause.name() == named)
        .expect("the refusal names a cause of the set");
    assert!(
        string_at(line, "remedy").as_deref() == Some(stated.remedy()),
        "the refusal names its cause's remedy"
    );
    named.to_string()
}

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
        remedied(ran.line())
    }

    #[track_caller]
    fn stop(self) {
        assert_eq!(self.accepted(&["stop"]), r#"{"stopped":true}"#);
        assert!(!self.started.dir.exists(), "stop leaves no state directory");
    }
}

#[test]
fn each_task_starts_answers_and_stops() {
    assert_eq!(VERBS.len(), 12);
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
fn snapshot_reads_a_part_by_number_and_a_part_outside_its_bound_is_refused() {
    let counter = Session::start("cc-part", "counter");
    let whole = counter.accepted(&["snapshot"]);
    assert!(
        keys(&whole) == ["text"],
        "a whole snapshot holds its text alone"
    );
    let first = counter.accepted(&["snapshot", "--part", "1"]);
    assert!(
        keys(&first) == ["text", "part", "parts"],
        "a part's line holds its three fields"
    );
    assert!(
        string_at(&first, "text") == string_at(&whole, "text"),
        "part 1 of a screen inside the budget is its whole text"
    );
    assert!(
        first.ends_with(",\"part\":1,\"parts\":1}"),
        "the screen reads as one part"
    );
    assert_eq!(
        counter.accepted(&["snapshot", "--part", "2"]),
        r#"{"text":"","part":2,"parts":1}"#
    );
    let rows: [(&str, Fault); 3] = [
        ("0", Fault::OutOfBound("part")),
        ("1001", Fault::OutOfBound("part")),
        ("x", Fault::WrongKind("part")),
    ];
    for (row, (part, fault)) in rows.into_iter().enumerate() {
        let ran = counter.run(&["snapshot", "--part", part]);
        assert_eq!(ran.status, Some(1), "row {row}: the command ends refused");
        assert!(remedied(ran.line()) == "malformed", "row {row}: its cause");
        assert!(
            ran.line() == Refusal::malformed(fault).to_json(),
            "row {row}: the line is the library's refusal, holding nothing typed"
        );
        assert!(ran.stderr.is_empty(), "row {row}: its stderr is empty");
    }
    assert!(
        counter.accepted(&["snapshot"]) == whole,
        "a plain snapshot is unchanged"
    );
    counter.stop();
}

/// Reads RFC 4648 base64 with padding: `None` for any other text.
fn decode_base64(text: &str) -> Option<Vec<u8>> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    if text.len() % 4 != 0 {
        return None;
    }
    let mut out = Vec::new();
    for chunk in text.as_bytes().chunks(4) {
        let padding = chunk.iter().rev().take_while(|byte| **byte == b'=').count();
        let mut n = 0u32;
        for byte in &chunk[..4 - padding] {
            n = n << 6 | ALPHABET.iter().position(|letter| letter == byte)? as u32;
        }
        n <<= 6 * padding as u32;
        out.extend_from_slice(&n.to_be_bytes()[1..4 - padding]);
    }
    Some(out)
}

/// The width and height a PNG's header states: `None` for bytes that do not open as a PNG.
fn png_size(bytes: &[u8]) -> Option<(u32, u32)> {
    let header = bytes.strip_prefix(b"\x89PNG\r\n\x1a\n")?;
    let ihdr = header.get(4..16)?.strip_prefix(b"IHDR")?;
    let width = u32::from_be_bytes(ihdr.get(0..4)?.try_into().ok()?);
    let height = u32::from_be_bytes(ihdr.get(4..8)?.try_into().ok()?);
    Some((width, height))
}

#[test]
fn a_screenshot_answers_one_line_whose_png_is_the_screen() {
    // The host boots in the incremental layout mode; so does this.
    let mut held =
        escher_driver::Session::start("crud", || stand::boot(LeanTask::Crud, stand::options(true)))
            .expect("the slug is a label");
    let call = Call {
        verb: "screenshot".to_string(),
        args: Vec::new(),
    };
    let Ok(Outcome::Shot { png, .. }) = held.run(&call) else {
        panic!("the screenshot runs in process");
    };

    let crud = Session::start("cc-shot", "crud");
    let answer = crud.accepted(&["screenshot"]);
    assert!(
        keys(&answer) == fields("screenshot"),
        "the screenshot's fields"
    );
    assert_eq!(number(&answer, "width"), Some(800));
    assert_eq!(number(&answer, "height"), Some(600));
    let shot = string_at(&answer, "png")
        .and_then(|text| decode_base64(&text))
        .expect("the png is base64");
    assert_eq!(png_size(&shot), Some((800, 600)), "the PNG is 800 × 600");
    assert!(
        shot == png,
        "the PNG is the screen's, as painted in process"
    );

    let element = crud.accepted(&["screenshot", "--id", "crud-list"]);
    assert_eq!(number(&element, "width"), Some(369));
    assert_eq!(number(&element, "height"), Some(420));

    let ran = crud.run(&["screenshot", "--id", NOBODY]);
    assert_eq!(ran.status, Some(1), "an unread id ends refused");
    assert_eq!(remedied(ran.line()), "not-found");
    assert!(
        !ran.line().contains(NOBODY) && ran.stderr.is_empty(),
        "the refusal writes back nothing the line named"
    );
    crud.stop();
}

#[test]
fn lint_answers_one_line_of_findings() {
    let counter = Session::start("cc-lint", "counter");
    let answer = counter.accepted(&["lint"]);
    assert!(keys(&answer) == fields("lint"), "the lint's fields");
    assert_eq!(
        answer, r#"{"findings":[]}"#,
        "the counter at boot reads no finding"
    );
    let ran = counter.run(&["lint", "--id", NOBODY]);
    assert_eq!(
        ran.status,
        Some(1),
        "a lint naming an argument ends refused"
    );
    assert_eq!(remedied(ran.line()), "malformed");
    assert!(
        !ran.line().contains(NOBODY) && ran.stderr.is_empty(),
        "the refusal writes back nothing the line named"
    );
    counter.stop();
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

    // A session with no time step of its own still moves its clock.
    let counter = Session::start("cc-refuse-c", "counter");
    let advanced = counter.accepted(&["advance", "--ms", "100"]);
    assert_eq!(number(&advanced, "advanced_ms"), Some(100));
    counter.stop();
}

#[test]
fn a_call_the_schema_refuses_reaches_no_session() {
    let dir = state_dir("cc-none");
    clear(&dir);
    let at = text(&dir);
    let rows: [(&[&str], &str); 6] = [
        (&["capture", "--session", at], "unknown-verb"),
        (&["capture"], "unknown-verb"),
        (&["click", "--session", at], "malformed"),
        (&["click"], "malformed"),
        (&["advance", "--ms", "soon", "--session", at], "malformed"),
        (&["press", "--key", "f1", "--session", at], "malformed"),
    ];
    for (row, (args, refused)) in rows.into_iter().enumerate() {
        let ran = client(BINARY, args);
        assert_eq!(ran.status, Some(1), "row {row}: the command ends refused");
        assert!(remedied(ran.line()) == refused, "row {row}: its cause");
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
            string_at(ran.line(), "remedy").as_deref() == Some(SessionError::NoSession.remedy()),
            "row {row}: the error names its remedy"
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
    let rows: [&[&str]; 9] = [
        &[],
        &["snapshot"],
        &["click", "--id", "counter-increment"],
        &["status", "--session", at, "--session", at],
        &["start", "no-such-task", "--session", at],
        // A recording directory anywhere but on `start` and the two roles, or with no value.
        &["snapshot", "--session", at, "--record", at],
        &["start", "counter", "--session", at, "--record"],
        &["mcp", "--record"],
        &["serve", "counter", "--session", at, "--record"],
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
        // The usage line names the recording directory where it is taken.
        for spelt in [
            " start <app> [--record <dir>] ",
            "; host role: serve <app> --session <dir> [--record <dir>]; ",
            "; mcp role: mcp [--record <dir>]; ",
        ] {
            assert!(
                stderr.contains(spelt),
                "row {row}: the usage line names --record"
            );
        }
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

#[test]
fn help_answers_the_tools_description_and_reaches_no_session() {
    let dir = state_dir("cc-help");
    clear(&dir);
    let rows: [&[&str]; 2] = [&["help"], &["help", "--session", text(&dir)]];
    for (row, args) in rows.into_iter().enumerate() {
        let ran = client(BINARY, args);
        assert_eq!(ran.status, Some(0), "row {row}: help ends accepted");
        assert!(
            ran.line() == describe(),
            "row {row}: stdout is the tool's description, {} bytes against {}",
            ran.line().len(),
            describe().len()
        );
        assert!(
            ran.stderr.is_empty(),
            "row {row}: stderr holds {} bytes",
            ran.stderr.len()
        );
        assert!(!dir.exists(), "row {row}: no state directory is created");
    }
    let named = client(BINARY, &["help", "--verb", "click"]);
    assert_eq!(named.status, Some(1), "an argument to help is refused");
    assert_eq!(remedied(named.line()), "malformed");
    assert!(
        !named.line().contains("--verb") && !named.line().contains("click"),
        "the refusal holds nothing the line named"
    );
}

/// Runs the binary as a client, `RUST_LOG` unset, with its stdout a pipe whose read end is
/// closed before the binary starts, for at most [`RUN_BOUND`]: its answer's write meets a closed
/// pipe. Nothing is read from its stdout.
fn with_stdout_closed(args: &[&str]) -> Ran {
    let (reader, writer) = std::io::pipe().expect("a pipe opens");
    drop(reader);
    let mut child = Command::new(BINARY)
        .args(args)
        .env_remove("RUST_LOG")
        .stdin(Stdio::null())
        .stdout(writer)
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary spawns");
    let stderr = drain(child.stderr.take().expect("stderr is piped"));
    let deadline = Instant::now() + RUN_BOUND;
    let status = loop {
        if let Some(status) = child.try_wait().expect("the binary can be waited on") {
            break status.code();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    Ran {
        status,
        stdout: Vec::new(),
        stderr: stderr
            .join()
            .expect("the stderr reader ends")
            .expect("stderr is read to its end"),
    }
}

#[test]
fn a_closed_stdout_drops_the_answer_and_keeps_the_ending() {
    let help = with_stdout_closed(&["help"]);
    assert_eq!(help.status, Some(0), "help still ends accepted");
    let stderr = String::from_utf8_lossy(&help.stderr);
    assert!(
        !stderr.contains("panicked") && !help.stderr.contains(&0x1b),
        "help's stderr holds no panic and no ESC byte, {} bytes",
        help.stderr.len()
    );

    let dir = state_dir("cc-closed");
    clear(&dir);
    let snapshot = with_stdout_closed(&["snapshot", "--session", text(&dir)]);
    assert_eq!(
        snapshot.status,
        Some(3),
        "a command with no session still ends a session error"
    );
    assert!(
        snapshot.stderr == format!("{}\n", SessionError::NoSession).as_bytes(),
        "stderr is the error's fixed message alone, {} bytes",
        snapshot.stderr.len()
    );
    assert!(!dir.exists(), "no state directory is created");
}

#[cfg(target_os = "linux")]
#[test]
fn a_state_directory_of_94_bytes_holds_a_session_and_one_of_95_is_refused_with_its_bound() {
    a_state_directory_at_the_bound_holds_a_session_and_one_byte_more_is_refused(94);
}

#[cfg(not(target_os = "linux"))]
#[test]
fn a_state_directory_of_90_bytes_holds_a_session_and_one_of_91_is_refused_with_its_bound() {
    a_state_directory_at_the_bound_holds_a_session_and_one_byte_more_is_refused(90);
}

/// A state directory given in `bound` bytes starts a session and stops it; one byte more is a
/// session error whose message states the bound and holds no path.
fn a_state_directory_at_the_bound_holds_a_session_and_one_byte_more_is_refused(bound: usize) {
    use std::path::{Path, PathBuf};
    use std::process::Command;

    let base = Path::new(env!("CARGO_TARGET_TMPDIR"));
    // A path as given: absolute when the target dir leaves room for a name, else relative to the
    // child's working directory, which counts its own bytes alone.
    let at = |length: usize| -> (PathBuf, String) {
        let prefix = format!("{}/", text(base));
        let given = if prefix.len() + 8 <= length {
            format!("{prefix}{}", "b".repeat(length - prefix.len()))
        } else {
            "b".repeat(length)
        };
        (base.join(&given), given)
    };
    let run = |args: &[&str]| {
        let mut command = Command::new(BINARY);
        command.args(args).current_dir(base).env_remove("RUST_LOG");
        common::run(command)
    };

    let (dir, given) = at(bound);
    assert_eq!(given.len(), bound);
    clear(&dir);
    let ran = run(&["start", "counter", "--session", &given]);
    assert_eq!(ran.status, Some(0), "a {bound}-byte state directory starts");
    let _started = Started {
        dir: dir.clone(),
        pid: number(ran.line(), "pid"),
    };
    let ran = run(&["stop", "--session", &given]);
    assert_eq!(ran.status, Some(0), "and stops");
    assert!(!dir.exists(), "stop leaves no state directory");

    let (dir, given) = at(bound + 1);
    assert_eq!(given.len(), bound + 1);
    clear(&dir);
    let ran = run(&["start", "counter", "--session", &given]);
    assert_eq!(
        ran.status,
        Some(3),
        "a {}-byte state directory is a session error",
        bound + 1
    );
    let error = SessionError::StateDirTooLong;
    assert!(
        ran.line() == error.to_json(),
        "stdout is the error for a state directory too long"
    );
    let message = string_at(ran.line(), "message").expect("the error holds its message");
    assert!(
        message.contains(&format!("at most {bound} bytes")),
        "the message names the bound"
    );
    assert!(!message.contains(&given), "the message holds no path");
    assert!(
        string_at(ran.line(), "remedy").as_deref() == Some(error.remedy()),
        "the error names its remedy"
    );
    assert!(
        ran.stderr == format!("{error}\n").as_bytes(),
        "stderr is the error's fixed message"
    );
    assert!(!dir.exists(), "no state directory is created");
}
