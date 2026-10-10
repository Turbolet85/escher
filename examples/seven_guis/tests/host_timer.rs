//! The timer host: `escher-session serve timer` is the one host whose session carries a time
//! step, and a hosted `advance` moves its time by whole ticks — 250 ms asked moves 200 — with
//! the elapsed time the screen reads one tick pair on. No sleep and no clock read: time moves
//! by the command alone. Unix only: a session crosses a Unix-domain socket.

#![cfg(unix)]

mod common;

use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use common::{Host, clear, client, drain, number, state_dir, text};
use escher_driver::{attach, stop};

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");
const READY: Duration = Duration::from_secs(60);
const EXIT: Duration = Duration::from_secs(10);

#[test]
fn a_hosted_advance_moves_the_timer_by_whole_ticks() {
    let dir = state_dir("ht-timer");
    clear(&dir);
    let at = text(&dir);
    let mut host = Host(
        Command::new(BINARY)
            .args(["serve", "timer", "--session", at])
            .env_remove("RUST_LOG")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the binary spawns"),
    );
    let stdout = drain(host.0.stdout.take().expect("stdout is piped"));
    let stderr = drain(host.0.stderr.take().expect("stderr is piped"));

    let deadline = Instant::now() + READY;
    let hello = loop {
        if let Ok(hello) = attach(&dir) {
            break hello;
        }
        let exited = host.0.try_wait().expect("the host can be waited on");
        assert!(exited.is_none(), "the host exited before it answered");
        assert!(
            Instant::now() < deadline,
            "the host answers within the bound"
        );
        thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(hello.pid, host.0.id(), "the binary itself answers");
    assert_eq!(hello.label, "timer");

    let run = |args: &[&str]| {
        let mut line = args.to_vec();
        line.extend(["--session", at]);
        let ran = client(BINARY, &line);
        assert_eq!(ran.status, Some(0), "the command ends accepted");
        ran.line().to_string()
    };

    let before = run(&["snapshot"]);
    assert!(
        before.contains("Elapsed: 0.0s"),
        "the timer reads no elapsed time at boot"
    );

    // The stand's step delivers whole 100 ms ticks: 250 asked moves 200.
    let advanced = run(&["advance", "--ms", "250"]);
    assert!(
        advanced.starts_with("{\"settled\":true,"),
        "the step went quiet"
    );
    assert_eq!(number(&advanced, "advanced_ms"), Some(200));
    assert!(
        advanced.contains("\"id\":\"timer-elapsed\"") && advanced.contains("Elapsed: 0.2s"),
        "the diff names the elapsed time, two ticks on"
    );
    assert!(
        run(&["snapshot"]).contains("Elapsed: 0.2s"),
        "the screen reads the elapsed time two ticks on"
    );

    // Less than one tick moves nothing.
    let short = run(&["advance", "--ms", "99"]);
    assert_eq!(number(&short, "advanced_ms"), Some(0));
    assert!(
        run(&["snapshot"]).contains("Elapsed: 0.2s"),
        "the screen reads the same elapsed time"
    );

    stop(&dir).expect("the session stops");
    let deadline = Instant::now() + EXIT;
    let status = loop {
        if let Some(status) = host.0.try_wait().expect("the host can be waited on") {
            break status;
        }
        assert!(Instant::now() < deadline, "the host exits after stop");
        thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(status.code(), Some(0), "the host exits 0 after stop");
    let stdout = stdout
        .join()
        .expect("the stdout reader ends")
        .expect("stdout is read to its end");
    let stderr = stderr
        .join()
        .expect("the stderr reader ends")
        .expect("stderr is read to its end");
    assert!(
        stdout.is_empty(),
        "the host wrote {} bytes to stdout",
        stdout.len()
    );
    assert!(
        !String::from_utf8_lossy(&stderr).contains("Elapsed"),
        "nothing of the screen is on the host's stderr"
    );
    assert!(!dir.exists(), "stop leaves no state directory");
}
