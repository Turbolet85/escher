//! A session's host writes nothing of the screen to its own streams or to its state directory:
//! a host holding a CRUD instance with a sentinel typed into it, started with `RUST_LOG=trace`,
//! answers a snapshot that holds the typed text, a stable id and an accessible name — and
//! writes none of the three to its stdout, its stderr or its state directory. What the screen
//! reads leaves the host only as the answer to a call. The host is this test binary re-run on
//! one ignored child, booted through the stand; it installs no log sink, as the session library
//! installs none. Unix only: the lifecycle crosses a Unix-domain socket.

#![cfg(unix)]

use std::fs;
use std::process::Stdio;

use escher_driver::{IDLE_EXPIRY, Session, attach, call, serve, start, stop};
use seven_guis::stand::LeanTask;

mod common;
mod session_common;
use session_common::{Host, READY, SOCKET_FILE, clear, host_command, snapshot, state_dir};

const STATE_DIR: &str = "ss-quiet";
const CHILD: &str = "session_host_with_typed_text";

/// The text the child types into the held instance: synthetic, and no part of any id or name.
const SENTINEL: &str = "Typed-Sentinel-93bdQx";

/// A stable id of the held instance.
const ID: &str = "crud-surname";

#[test]
fn a_host_writes_nothing_of_the_screen_to_its_streams_or_its_state_directory() {
    let fresh = common::boot(LeanTask::Crud, true).doc.snapshot();
    let row_name = fresh
        .get("crud-person-0")
        .map(|row| row.name.clone())
        .unwrap_or_default();
    assert!(!row_name.is_empty(), "a fixture row has an accessible name");
    assert!(
        fresh.get(ID).is_some(),
        "the id names a control of the task"
    );

    let dir = state_dir(STATE_DIR);
    clear(&dir);
    let mut command = host_command(CHILD);
    command
        .env("RUST_LOG", "trace")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let started = start(&dir, command, READY).expect("the session starts");
    let mut host = Host::new(started.child);
    assert_eq!(started.hello.label, "crud");
    assert_eq!(attach(&dir).map(|hello| hello.pid), Ok(host.pid()));

    // The host handles the screen: its answer to a snapshot holds all three.
    let screen = [
        ("the typed text", SENTINEL),
        ("a stable id", ID),
        ("an accessible name", row_name.as_str()),
    ];
    let answer = call(&dir, &snapshot()).expect("the session answers a snapshot");
    assert!(answer.accepted, "the hosted snapshot is accepted");
    for (kind, needle) in screen {
        assert!(
            answer.json.contains(needle),
            "{kind} is in the snapshot's answer"
        );
    }

    let listing: Vec<String> = fs::read_dir(&dir)
        .expect("the state directory is listed while the session is up")
        .map(|entry| {
            let entry = entry.expect("a state directory entry is read");
            entry.file_name().to_string_lossy().into_owned()
        })
        .collect();
    assert_eq!(
        listing,
        [SOCKET_FILE],
        "the state directory holds the socket"
    );
    assert_eq!(stop(&dir), Ok(()));
    assert_eq!(host.exit_code(), Some(0), "the host exits 0 after stop");

    let (stdout, stderr) = host.output();
    assert!(
        stdout.contains(CHILD),
        "the child's streams were captured: its test report names it"
    );
    let written = [
        ("stdout", stdout),
        ("stderr", stderr),
        ("the state directory", listing.join("\n")),
    ];
    for (place, bytes) in &written {
        for (kind, needle) in screen {
            assert!(!bytes.contains(needle), "{kind} occurs in {place}");
        }
    }
}

#[test]
#[ignore = "spawned as the session host by a_host_writes_nothing_of_the_screen_to_its_streams_or_its_state_directory"]
fn session_host_with_typed_text() {
    let mut session = Session::start("crud", || common::boot(LeanTask::Crud, true))
        .expect("a CRUD session starts");
    session.harness_mut().click("#crud-name");
    session.harness_mut().type_text(SENTINEL);
    assert!(
        common::editor_text(session.harness(), "crud-name") == SENTINEL,
        "the held instance holds the typed text"
    );
    serve(&state_dir(STATE_DIR), session, IDLE_EXPIRY)
        .expect("the host serves until it is stopped");
}
