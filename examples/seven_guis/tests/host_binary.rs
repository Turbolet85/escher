//! The `escher-session` binary boots a lean stand task, serves it as a session, answers, and
//! stops leaving nothing behind, writing to stderr only; an unknown task is refused before
//! anything boots.

#![cfg(not(target_arch = "wasm32"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

/// A state directory of this file's own, its name short enough for a socket address.
fn state_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(name)
}

/// Removes what a killed earlier run may have left: the socket file and the empty directory.
fn clear(state_dir: &Path) {
    let _ = fs::remove_file(state_dir.join("session.sock"));
    let _ = fs::remove_dir(state_dir);
}

/// Kills and reaps the host when a failing check still holds it.
#[cfg(unix)]
struct Host(std::process::Child);

#[cfg(unix)]
impl Drop for Host {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[cfg(unix)]
#[test]
fn the_binary_serves_and_stops() {
    use std::io::Read;
    use std::thread;
    use std::time::{Duration, Instant};

    use escher_driver::{attach, start, stop};

    let dir = state_dir("hb-serve");
    clear(&dir);
    let mut command = Command::new(BINARY);
    command
        .arg("counter")
        .arg(&dir)
        .env("RUST_LOG", "info")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let started = start(&dir, command, Duration::from_secs(60)).expect("the binary starts");
    let mut host = Host(started.child);
    assert_eq!(started.hello.pid, host.0.id(), "the binary itself answers");
    assert_eq!(started.hello.label, "counter");

    let hello = attach(&dir).expect("the session answers again");
    assert_eq!(hello.pid, host.0.id(), "the same process answers");
    assert_eq!(hello.label, "counter");

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
    assert!(stdout.is_empty(), "the binary writes nothing to stdout");
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
fn an_unknown_task_is_refused_before_anything_boots() {
    let dir = state_dir("hb-refuse");
    clear(&dir);

    let output = Command::new(BINARY)
        .arg("no-such-task")
        .arg(&dir)
        .stdin(Stdio::null())
        .output()
        .expect("the binary runs");

    assert_eq!(output.status.code(), Some(2), "a usage error exits 2");
    assert!(output.stdout.is_empty(), "nothing is written to stdout");
    assert!(
        !output.stderr.is_empty(),
        "a usage line is written to stderr"
    );
    assert!(!dir.exists(), "no state directory is created");
}
