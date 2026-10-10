//! A session's lifecycle over its socket, driven from a second process: `attach` and `stop`
//! with no session, `start`, a second `attach`, a call that runs and a call that is refused, a
//! request outside the protocol, `start` while one is up, `stop` and what it leaves, a second
//! `stop`, and a host that died without `stop`. And the idle expiry: a host with no request for
//! its expiry ends as `stop` does. Each host is this test binary re-run on one ignored child,
//! serving a counter session booted through the stand. Unix only: the lifecycle crosses a
//! Unix-domain socket.

#![cfg(unix)]

use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use escher_driver::{IDLE_EXPIRY, Session, SessionError, attach, call, serve, start, stop};
use seven_guis::stand::LeanTask;

mod common;
mod session_common;
use session_common::{Host, READY, SOCKET_FILE, clear, click, host_command, snapshot, state_dir};

const STATE_DIR: &str = "ss-life";
const CHILD: &str = "session_host";

const IDLE_STATE_DIR: &str = "ss-idle";
const IDLE_CHILD: &str = "session_host_with_a_short_expiry";

/// The expiry the idle child serves with.
const SHORT_EXPIRY: Duration = Duration::from_secs(1);

/// How long the idle check waits for a host whose expiry is [`SHORT_EXPIRY`] to end.
const EXPIRED: Duration = Duration::from_secs(20);

/// An id no element of the counter carries.
const NOBODY: &str = "ss-life-names-no-element";

/// The host command, its streams closed: the child's own test report is not this check's.
fn host(child: &str) -> Command {
    let mut command = host_command(child);
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

/// What the session's socket answers to one raw request line.
fn raw_exchange(dir: &Path, request: &[u8]) -> String {
    let mut stream = UnixStream::connect(dir.join(SOCKET_FILE)).expect("the socket answers");
    stream
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("a read bound is set");
    // The host may answer and close before a long line is written whole.
    let _ = stream.write_all(request);
    let mut reply = String::new();
    let _ = stream.read_to_string(&mut reply);
    reply
}

#[test]
fn start_attach_stop_and_each_edge() {
    let dir = state_dir(STATE_DIR);
    clear(&dir);

    assert_eq!(attach(&dir), Err(SessionError::NoSession), "no session");
    assert_eq!(stop(&dir), Err(SessionError::NoSession), "no session");
    assert_eq!(
        call(&dir, &snapshot()),
        Err(SessionError::NoSession),
        "no session"
    );

    let started = start(&dir, host(CHILD), READY).expect("a session starts");
    let first = started.hello;
    let mut host_one = Host::new(started.child);
    assert_eq!(first.pid, host_one.pid(), "the spawned process answers");
    assert_eq!(first.label, "counter");
    assert_eq!(first.served, 0, "the first answer of this host");
    assert_eq!(first.idle_expiry_s, 1800, "the host states its idle expiry");
    assert_eq!(IDLE_EXPIRY.as_secs(), 1800);
    assert_eq!(mode(&dir), 0o700, "the state directory is owner-only");
    assert_eq!(mode(&dir.join(SOCKET_FILE)), 0o600, "the socket is too");

    let second = attach(&dir).expect("the session answers again");
    assert_eq!(second.pid, first.pid, "the same process answers");
    assert!(second.served > first.served, "its served count rose");

    // A call crosses to the held instance and its answer comes back: one that runs, and one
    // the session refuses. A failure names no id and nothing a screen reads.
    let screen = call(&dir, &snapshot()).expect("the session answers a snapshot");
    assert!(
        screen.accepted && screen.json.starts_with("{\"text\":\""),
        "a hosted snapshot is accepted and answers the screen's text"
    );
    let clicked = call(&dir, &click("counter-increment")).expect("the session answers a click");
    assert!(
        clicked.accepted && clicked.json.starts_with("{\"settled\":true,"),
        "a hosted click is accepted and answers a settled step"
    );
    let changed = call(&dir, &snapshot()).expect("the session answers a snapshot");
    assert!(
        changed.accepted && changed.json != screen.json,
        "the click changed what the held instance shows"
    );
    let refused = call(&dir, &click(NOBODY)).expect("the session answers a refused call");
    assert!(
        !refused.accepted
            && refused
                .json
                .starts_with("{\"refused\":{\"cause\":\"not-found\","),
        "a hosted click on an id no screen reads is refused not-found"
    );

    // A request outside the wire's grammar, and one over its bound, are refused and run
    // nothing.
    let over_long = format!("call v2 click id=t:{}\n", "i".repeat(20_000));
    let outside: [&[u8]; 5] = [
        b"call v2 click id=x:counter-increment\n",
        b"call v2 click id=t:counter%2\n",
        b"click v2 id=t:counter-increment\n",
        b"snapshot\n",
        over_long.as_bytes(),
    ];
    for (row, request) in outside.into_iter().enumerate() {
        assert!(
            raw_exchange(&dir, request) == "refused malformed\n",
            "row {row}: a request outside the protocol is refused as malformed"
        );
    }
    assert_eq!(
        raw_exchange(&dir, b"call v1 snapshot\n"),
        "refused version\n",
        "a call in another version is refused by version"
    );
    assert!(
        call(&dir, &snapshot()).is_ok_and(|after| after.json == changed.json),
        "the refused requests changed nothing"
    );
    assert!(
        attach(&dir).is_ok_and(|hello| hello.pid == first.pid && hello.served > second.served),
        "the same host still answers, and counted what it answered"
    );

    assert!(
        matches!(
            start(&dir, host(CHILD), READY),
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

    let started = start(&dir, host(CHILD), READY).expect("a session starts again");
    let mut killed = Host::new(started.child);
    let killed_pid = killed.pid();
    killed.kill();
    assert!(
        dir.join(SOCKET_FILE).exists(),
        "a killed host leaves its socket file"
    );
    assert_eq!(attach(&dir), Err(SessionError::Dead), "a killed host");
    assert_eq!(stop(&dir), Err(SessionError::Dead), "a killed host");
    assert_eq!(
        call(&dir, &snapshot()),
        Err(SessionError::Dead),
        "a killed host"
    );

    let started = start(&dir, host(CHILD), READY).expect("a start over a dead session");
    let mut host_three = Host::new(started.child);
    assert_eq!(started.hello.pid, host_three.pid());
    assert_ne!(started.hello.pid, killed_pid, "a new process answers");
    assert_eq!(stop(&dir), Ok(()));
    assert!(!dir.exists(), "stop leaves no state directory");
    assert_eq!(host_three.exit_code(), Some(0));
}

#[test]
fn a_host_with_no_request_for_its_expiry_ends_as_stop_does() {
    let dir = state_dir(IDLE_STATE_DIR);
    clear(&dir);

    let started = start(&dir, host(IDLE_CHILD), READY).expect("a session starts");
    let mut idle = Host::new(started.child);
    assert_eq!(started.hello.pid, idle.pid(), "the spawned process answers");
    assert_eq!(
        started.hello.idle_expiry_s,
        SHORT_EXPIRY.as_secs(),
        "the host states the expiry it serves with"
    );

    // Nothing is asked of the session from here on: every answered request, an `attach`
    // included, starts the expiry again. The state directory is what is watched.
    let deadline = Instant::now() + EXPIRED;
    while dir.exists() {
        assert!(
            Instant::now() < deadline,
            "a host with no request ends within the bound"
        );
        thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(
        idle.exit_code(),
        Some(0),
        "the host exits 0 when its expiry runs out"
    );
    assert!(
        !dir.join(SOCKET_FILE).exists() && !dir.exists(),
        "the expiry leaves no socket file and no state directory"
    );
    assert_eq!(
        attach(&dir),
        Err(SessionError::NoSession),
        "no session answers after the expiry"
    );
}

#[test]
#[ignore = "spawned as the session host by start_attach_stop_and_each_edge"]
fn session_host() {
    let session = Session::start("counter", || common::boot(LeanTask::Counter, true))
        .expect("a counter session starts");
    serve(&state_dir(STATE_DIR), session, IDLE_EXPIRY)
        .expect("the host serves until it is stopped");
}

#[test]
#[ignore = "spawned as the session host by a_host_with_no_request_for_its_expiry_ends_as_stop_does"]
fn session_host_with_a_short_expiry() {
    let session = Session::start("counter", || common::boot(LeanTask::Counter, true))
        .expect("a counter session starts");
    serve(&state_dir(IDLE_STATE_DIR), session, SHORT_EXPIRY)
        .expect("the host serves until its expiry runs out");
}
