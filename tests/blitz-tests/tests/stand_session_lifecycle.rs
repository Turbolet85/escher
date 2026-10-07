//! A session's lifecycle over its socket, driven from a second process: `attach` and `stop`
//! with no session, `start`, a second `attach`, `start` while one is up, `stop` and what it
//! leaves, a second `stop`, and a host that died without `stop`. The host is this test binary
//! re-run on one ignored child, serving a counter session booted through the stand. Unix only:
//! the lifecycle crosses a Unix-domain socket.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};

use escher_driver::{Session, SessionError, attach, serve, start, stop};
use seven_guis::stand::LeanTask;

mod common;
mod session_common;
use session_common::{Host, READY, SOCKET_FILE, clear, host_command, state_dir};

const STATE_DIR: &str = "ss-life";
const CHILD: &str = "session_host";

/// The host command, its streams closed: the child's own test report is not this check's.
fn host() -> Command {
    let mut command = host_command(CHILD);
    command.stdout(Stdio::null()).stderr(Stdio::null());
    command
}

fn mode(path: &Path) -> u32 {
    fs::metadata(path)
        .expect("a path of a running session exists")
        .permissions()
        .mode()
        & 0o777
}

#[test]
fn start_attach_stop_and_each_edge() {
    let dir = state_dir(STATE_DIR);
    clear(&dir);

    assert_eq!(attach(&dir), Err(SessionError::NoSession), "no session");
    assert_eq!(stop(&dir), Err(SessionError::NoSession), "no session");

    let started = start(&dir, host(), READY).expect("a session starts");
    let first = started.hello;
    let mut host_one = Host::new(started.child);
    assert_eq!(first.pid, host_one.pid(), "the spawned process answers");
    assert_eq!(first.label, "counter");
    assert_eq!(first.served, 0, "the first answer of this host");
    assert_eq!(mode(&dir), 0o700, "the state directory is owner-only");
    assert_eq!(mode(&dir.join(SOCKET_FILE)), 0o600, "the socket is too");

    let second = attach(&dir).expect("the session answers again");
    assert_eq!(second.pid, first.pid, "the same process answers");
    assert!(second.served > first.served, "its served count rose");

    assert!(
        matches!(
            start(&dir, host(), READY),
            Err(SessionError::AlreadyRunning)
        ),
        "a start while a session is up"
    );
    assert_eq!(
        attach(&dir).map(|hello| hello.pid),
        Ok(first.pid),
        "the first session still answers"
    );

    assert_eq!(stop(&dir), Ok(()));
    assert!(!dir.exists(), "stop leaves no state directory");
    assert_eq!(host_one.exit_code(), Some(0), "the host exits 0 after stop");
    assert_eq!(stop(&dir), Err(SessionError::NoSession), "a second stop");

    let started = start(&dir, host(), READY).expect("a session starts again");
    let mut killed = Host::new(started.child);
    let killed_pid = killed.pid();
    killed.kill();
    assert!(
        dir.join(SOCKET_FILE).exists(),
        "a killed host leaves its socket file"
    );
    assert_eq!(attach(&dir), Err(SessionError::Dead), "a killed host");
    assert_eq!(stop(&dir), Err(SessionError::Dead), "a killed host");

    let started = start(&dir, host(), READY).expect("a start over a dead session");
    let mut host_three = Host::new(started.child);
    assert_eq!(started.hello.pid, host_three.pid());
    assert_ne!(started.hello.pid, killed_pid, "a new process answers");
    assert_eq!(stop(&dir), Ok(()));
    assert!(!dir.exists(), "stop leaves no state directory");
    assert_eq!(host_three.exit_code(), Some(0));
}

#[test]
#[ignore = "spawned as the session host by start_attach_stop_and_each_edge"]
fn session_host() {
    let session = Session::start("counter", || common::boot(LeanTask::Counter, true))
        .expect("a counter session starts");
    serve(&state_dir(STATE_DIR), session).expect("the host serves until it is stopped");
}
