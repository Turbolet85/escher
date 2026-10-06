//! An event emitted after `escher_telemetry::init` reaches stderr with the service identity, and
//! never stdout.

use std::process::Command;

const SENTINEL: &str = "ESCHER-SENTINEL-7f3a";

#[test]
fn events_reach_stderr_and_never_stdout() {
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "child_emits", "--nocapture"])
        .env_remove("RUST_LOG")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "child failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stderr.contains(SENTINEL), "stderr:\n{stderr}");
    assert!(
        stderr.contains("service.name=blitz-tests"),
        "stderr:\n{stderr}"
    );
    assert!(stderr.contains("service.version="), "stderr:\n{stderr}");
    assert!(!stdout.contains(SENTINEL), "stdout:\n{stdout}");
}

#[test]
#[ignore = "spawned as a child process by events_reach_stderr_and_never_stdout"]
fn child_emits() {
    escher_telemetry::init(escher_telemetry::service_identity!()).unwrap();
    tracing::warn!(target: "escher_stand_probe", marker = SENTINEL, "child emitted");
}
