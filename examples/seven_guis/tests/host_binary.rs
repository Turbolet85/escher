//! The `escher-session` binary in its host role boots a lean stand task, serves it as a session,
//! answers, and stops leaving nothing behind, writing nothing to stdout; a host that cannot
//! serve ends as a session error; and a line that is no command is refused with a usage line
//! before anything boots.

#![cfg(not(target_arch = "wasm32"))]

mod common;

use std::process::{Command, Stdio};

#[cfg(unix)]
use common::Host;
use common::{clear, client, state_dir, text};

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

#[cfg(unix)]
#[test]
fn the_binary_serves_and_stops() {
    use std::io::Read;
    use std::thread;
    use std::time::{Duration, Instant};

    use escher_driver::{IDLE_EXPIRY, SessionError, attach, start, stop};

    let dir = state_dir("hb-serve");
    clear(&dir);
    let mut command = Command::new(BINARY);
    command
        .args(["serve", "counter", "--session"])
        .arg(&dir)
        .env("RUST_LOG", "info")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let started = start(&dir, command, Duration::from_secs(60)).expect("the binary starts");
    let mut host = Host(started.child);
    assert_eq!(started.hello.pid, host.0.id(), "the binary itself answers");
    assert_eq!(started.hello.label, "counter");
    assert_eq!(started.hello.idle_expiry_s, IDLE_EXPIRY.as_secs());
    assert_eq!(IDLE_EXPIRY.as_secs(), 1800);

    let hello = attach(&dir).expect("the session answers again");
    assert_eq!(hello.pid, host.0.id(), "the same process answers");
    assert_eq!(hello.label, "counter");

    // A second host on the same directory ends as a session error: status 3, the error on
    // its stdout and the error's message on its stderr.
    let second = client(BINARY, &["serve", "counter", "--session", text(&dir)]);
    assert_eq!(second.status, Some(3), "a host that cannot serve exits 3");
    assert!(
        second.line() == SessionError::AlreadyRunning.to_json(),
        "its stdout is the error for a running session"
    );
    assert!(
        String::from_utf8_lossy(&second.stderr).contains(&SessionError::AlreadyRunning.to_string()),
        "its stderr carries the error's message"
    );
    assert_eq!(
        attach(&dir).map(|hello| hello.pid),
        Ok(host.0.id()),
        "the first host still answers"
    );

    stop(&dir).expect("the session stops");
    assert!(!dir.exists(), "stop leaves no state directory");

    let deadline = Instant::now() + Duration::from_secs(10);
    let status = loop {
        if let Some(status) = host.0.try_wait().expect("the host can be waited on") {
            break status;
        }
        assert!(Instant::now() < deadline, "the host exits after stop");
        thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(status.code(), Some(0), "the host exits 0 after stop");

    let (mut stdout, mut stderr) = (String::new(), String::new());
    host.0
        .stdout
        .take()
        .expect("stdout is piped")
        .read_to_string(&mut stdout)
        .expect("stdout is text");
    host.0
        .stderr
        .take()
        .expect("stderr is piped")
        .read_to_string(&mut stderr)
        .expect("stderr is text");
    assert!(stdout.is_empty(), "the host writes nothing to stdout");
    assert!(
        stderr.contains("service.name=seven_guis"),
        "its stderr lines carry the service name"
    );
    assert!(
        stderr.contains("service.version="),
        "its stderr lines carry the service version"
    );
}

#[test]
fn a_line_that_is_no_command_is_refused_before_anything_boots() {
    let dir = state_dir("hb-refuse");
    clear(&dir);
    let at = text(&dir);

    let rows: [&[&str]; 9] = [
        &[],
        &["serve", "no-such-task", "--session", at],
        &["start", "no-such-task", "--session", at],
        // The two-argument form the host took before it had a command line.
        &["counter", at],
        &["serve", "counter", at],
        &["serve", "counter"],
        &["start", "counter"],
        &["snapshot"],
        &["snapshot", "--session", at, "--session", at],
    ];
    for (row, args) in rows.into_iter().enumerate() {
        let ran = client(BINARY, args);
        // The old two-argument form names no verb: it is a refusal, like any unknown word.
        let (status, on_stdout) = if row == 3 { (1, true) } else { (2, false) };
        assert_eq!(ran.status, Some(status), "row {row}: the status");
        assert!(
            ran.stdout.is_empty() != on_stdout,
            "row {row}: stdout holds {} bytes",
            ran.stdout.len()
        );
        if !on_stdout {
            let stderr = String::from_utf8_lossy(&ran.stderr);
            assert!(
                stderr.starts_with("usage: ") && stderr.lines().count() == 1,
                "row {row}: one usage line is written to stderr"
            );
            assert!(
                [
                    "counter",
                    "flight-booker",
                    "timer",
                    "crud",
                    "snapshot",
                    "serve"
                ]
                .iter()
                .all(|word| stderr.contains(word)),
                "row {row}: the usage line names the commands and the tasks"
            );
            assert!(
                !stderr.contains("no-such-task") && !stderr.contains(at),
                "row {row}: the usage line holds nothing the caller typed"
            );
        }
        assert!(!dir.exists(), "row {row}: no state directory is created");
    }
}
