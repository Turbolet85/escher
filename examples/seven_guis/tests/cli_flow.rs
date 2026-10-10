//! A stand flow is scriptable in shell alone: the two scripts under `tests/flows/` drive the
//! real `escher-session` binary — start, read, act, check what came back, stop — under plain
//! `sh`, using no tool beyond the shell's own, and each completes with status 0 leaving no
//! session and no state directory. Unix only: a session crosses a Unix-domain socket.

#![cfg(unix)]

mod common;

use std::process::Command;

use common::{Started, clear, run, state_dir};
use escher_driver::{SessionError, attach};

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

/// Each flow with the state directory it is handed and the steps it holds.
const FLOWS: [(&str, &str, u32); 2] = [
    (
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/flows/counter.sh"),
        "cf-counter",
        8,
    ),
    (
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/flows/flight.sh"),
        "cf-flight",
        7,
    ),
];

#[test]
fn each_flow_completes_under_plain_sh_and_leaves_nothing() {
    assert_eq!(FLOWS.len(), 2);
    for (flow, (script, name, steps)) in FLOWS.into_iter().enumerate() {
        let dir = state_dir(name);
        clear(&dir);
        // A flow that fails stops its own session; this guard is for one that could not.
        let _guard = Started {
            dir: dir.clone(),
            pid: None,
        };

        let mut command = Command::new("sh");
        command
            .arg(script)
            .arg(BINARY)
            .arg(&dir)
            .env_remove("RUST_LOG");
        let ran = run(command);

        // A failing flow prints the number of its failing step and nothing else.
        let failed_step: Option<u32> = String::from_utf8_lossy(&ran.stdout)
            .trim()
            .strip_prefix("step ")
            .and_then(|step| step.parse().ok());
        assert!(
            ran.status == Some(0),
            "flow {flow}: status {:?}, failing step {failed_step:?} of {steps}",
            ran.status
        );
        assert!(
            ran.stdout.is_empty() && ran.stderr.is_empty(),
            "flow {flow}: a completed flow wrote {} bytes to stdout and {} to stderr",
            ran.stdout.len(),
            ran.stderr.len()
        );
        assert_eq!(
            attach(&dir),
            Err(SessionError::NoSession),
            "flow {flow}: no session answers afterwards"
        );
        assert!(!dir.exists(), "flow {flow}: no state directory remains");
    }
}

#[test]
fn a_flow_that_reads_something_else_says_which_step_and_stops_its_session() {
    // The counter's flow handed a binary that answers nothing as it expects — `true` — fails at
    // its first step: a flow's green is its steps reading as expected, not its having run.
    let dir = state_dir("cf-red");
    clear(&dir);
    let mut command = Command::new("sh");
    command.arg(FLOWS[0].0).arg("true").arg(&dir);
    let ran = run(command);
    assert_eq!(ran.status, Some(1), "a flow whose step reads wrong exits 1");
    assert_eq!(ran.stdout, b"step 1\n", "it prints the failing step");
    assert!(!dir.exists(), "it leaves no state directory");
}
