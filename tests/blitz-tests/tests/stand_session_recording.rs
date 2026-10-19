//! A driver session keeps itself as a replayable check, owner-only and holding no screen
//! reading, and the check replays. In both layout modes. *On each lean task:* a session recording
//! under the test target's temp dir runs a script of a click, a `type`, a `press`, an `advance`
//! and a `scroll` where the task has the control, a call refused for an id no screen reads, a
//! `snapshot` and a `screenshot`; its check reads back with one step per call, each step's
//! acceptance and digest those of the answer the call got, and run again on a freshly booted
//! session every step answers alike. The check file reads `0600` in a `0700` directory, and no
//! accessible name, control value or snapshot-text line of the screens the session read occurs
//! in it but the text the script itself typed, which a check keeps as it crossed. *On a password
//! fixture:* a sentinel typed into the password input is kept as the fixed stand-in, flagged
//! masked — the sentinel's bytes occur nowhere, and a sentinel of another length leaves the same
//! file — and the replay's digests still match. *A recording directory open to its group* is
//! refused with nothing written. A failure message carries the layout mode, the task and a step
//! index, never an id, a name or what a screen reads. Unix only: a check's modes are unix modes.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use escher_driver::{ArgValue, Call, Check, MASKED_STAND_IN, Session, SessionError, answer_digest};
use seven_guis::stand::{self, LeanTask};

mod session_common;
use session_common::{
    advance, click, hold, kept, mode, press, replay, screenshot, scroll, slug, snapshot, state_dir,
    type_into,
};

/// An id no element of the stand or of the fixture carries.
const NOBODY: &str = "recording-names-no-element";

/// What the masked check types into the password input: synthetic, and no part of any id or
/// name.
const SENTINEL: &str = "Recorded-Sentinel-7mQv";

/// The script a session on `task` runs: each acting verb where the task has the control, a
/// refused call, a `snapshot` and a `screenshot`.
fn script(task: LeanTask) -> Vec<Call> {
    let mut calls = match task {
        LeanTask::Counter => vec![
            click("counter-increment"),
            press("tab", false),
            advance(100),
            scroll("counter-increment"),
        ],
        LeanTask::FlightBooker => vec![
            click("flight-return"),
            type_into("flight-start", "01.01.2030"),
            press("tab", false),
            advance(100),
            scroll("flight-book"),
        ],
        LeanTask::Timer => vec![
            click("timer-reset"),
            press("tab", false),
            press("arrow-right", false),
            advance(300),
            scroll("timer-reset"),
        ],
        LeanTask::Crud => vec![
            type_into("crud-name", "Ada"),
            type_into("crud-surname", "Lovelace"),
            click("crud-create"),
            press("tab", true),
            advance(100),
            scroll("crud-list"),
        ],
    };
    calls.extend([click(NOBODY), snapshot(), screenshot()]);
    calls
}

/// A fresh recording directory under the test target's temp dir: nothing is there.
fn fresh(name: &str) -> PathBuf {
    let dir = state_dir(name);
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The one check a recording directory holds.
#[track_caller]
fn the_check(dir: &Path) -> Vec<u8> {
    let names: Vec<_> = fs::read_dir(dir)
        .expect("the recording directory is listed")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert!(names == ["check-1.json"], "the directory holds one check");
    fs::read(dir.join("check-1.json")).expect("the check is read")
}

/// `text` as a JSON string writes it, without the quotes: how a reading would stand in a check.
fn escaped(text: &str) -> String {
    let mut out = String::new();
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if control < '\u{20}' => {
                out.push_str(&format!("\\u{:04x}", u32::from(control)))
            }
            other => out.push(other),
        }
    }
    out
}

/// The text arguments a script sent: what a check keeps as it crossed.
fn sent(calls: &[Call]) -> Vec<String> {
    calls
        .iter()
        .flat_map(|call| &call.args)
        .filter_map(|(_, value)| match value {
            ArgValue::Text(text) => Some(text.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn a_recorded_session_replays_and_keeps_no_screen_reading() {
    for incremental in [false, true] {
        let mode_name = if incremental { "incremental" } else { "full" };
        for task in LeanTask::ALL {
            let task_name = slug(task);
            let dir = fresh(&format!("rec-{task_name}-{mode_name}"));
            let (session, _ticks) = hold(task, incremental);
            let mut session = session.recording_into(&dir).expect("the recording starts");
            let calls = script(task);
            let mut answers = Vec::new();
            let mut names = Vec::new();
            let mut values = Vec::new();
            let mut lines = Vec::new();
            for call in &calls {
                let screen = session.harness().doc.snapshot();
                for node in screen.nodes() {
                    names.push(node.name.clone());
                    values.extend(node.state.value.clone());
                }
                lines.extend(screen.to_text().lines().map(str::to_string));
                let answer = session.run(call);
                answers.push((answer.is_ok(), answer_digest(&answer)));
                assert!(
                    !session.recording_failed(),
                    "{mode_name} {task_name}: the recording stands"
                );
            }
            assert!(
                answers.iter().filter(|(accepted, _)| !accepted).count() == 1,
                "{mode_name} {task_name}: the script's one refused call is its only one"
            );

            let bytes = the_check(&dir);
            let check = Check::from_bytes(&bytes).expect("the check reads back");
            assert!(
                check.app() == task_name,
                "{mode_name} {task_name}: the check names its app"
            );
            assert!(
                check.steps().len() == calls.len(),
                "{mode_name} {task_name}: one step per call"
            );
            let records = bytes.iter().filter(|byte| **byte == b'\n').count() - 2;
            assert!(
                records == calls.len() + 1,
                "{mode_name} {task_name}: the opening record and one per call"
            );
            for (index, (step, call)) in check.steps().iter().zip(&calls).enumerate() {
                assert!(
                    step.call().verb == call.verb && !step.masked(),
                    "{mode_name} {task_name}: step {index} is its call"
                );
            }
            assert!(
                kept(&check) == answers,
                "{mode_name} {task_name}: each step keeps its answer's acceptance and digest"
            );

            let (fresh_session, _ticks) = hold(task, incremental);
            let mut fresh_session = fresh_session;
            let replayed = replay(&mut fresh_session, &check);
            for (index, (again, recorded)) in replayed.iter().zip(&answers).enumerate() {
                assert!(
                    again == recorded,
                    "{mode_name} {task_name}: step {index} replays alike"
                );
            }

            assert!(
                mode(&dir) == 0o700,
                "{mode_name} {task_name}: the directory is owner-only"
            );
            assert!(
                mode(&dir.join("check-1.json")) == 0o600,
                "{mode_name} {task_name}: the check is owner-only"
            );

            let text = String::from_utf8(bytes).expect("the check is text");
            let typed = sent(&calls);
            let at_rest = |reading: &String| {
                !reading.is_empty()
                    && !typed.contains(reading)
                    && text.contains(&format!("\"{}\"", escaped(reading)))
            };
            assert!(
                !names.is_empty() && !lines.is_empty(),
                "{mode_name} {task_name}: the session read names and lines"
            );
            assert!(
                task == LeanTask::Counter || !values.is_empty(),
                "{mode_name} {task_name}: the session read control values"
            );
            assert!(
                names.iter().filter(|name| at_rest(name)).count() == 0,
                "{mode_name} {task_name}: an accessible name is at rest"
            );
            assert!(
                values.iter().filter(|value| at_rest(value)).count() == 0,
                "{mode_name} {task_name}: a control value is at rest"
            );
            assert!(
                lines
                    .iter()
                    .filter(|line| text.contains(&escaped(line.trim())))
                    .count()
                    == 0,
                "{mode_name} {task_name}: a snapshot-text line is at rest"
            );
            let _ = fs::remove_dir_all(&dir);
        }
    }
}

/// A password input, empty.
fn password_fixture() -> Element {
    rsx! {
        div { id: "fx-root",
            input { id: "fx-secret", r#type: "password", aria_label: "Secret" }
        }
    }
}

fn hold_fixture(incremental: bool) -> Session {
    Session::start("masked", || {
        Harness::from_vdom(
            VirtualDom::new(password_fixture),
            stand::options(incremental),
        )
    })
    .expect("the fixture's label is a label")
}

/// Records `start`, a `type` of `secret` into the password input and a `snapshot` into `dir`:
/// the answers' acceptances and digests.
fn record_typed(dir: &Path, secret: &str, incremental: bool) -> Vec<(bool, u64)> {
    let mut session = hold_fixture(incremental)
        .recording_into(dir)
        .expect("the recording starts");
    [type_into("fx-secret", secret), snapshot()]
        .iter()
        .map(|call| {
            let answer = session.run(call);
            (answer.is_ok(), answer_digest(&answer))
        })
        .collect()
}

#[test]
fn a_password_typed_in_a_recorded_session_is_kept_as_the_stand_in() {
    for incremental in [false, true] {
        let mode_name = if incremental { "incremental" } else { "full" };
        let dir = fresh(&format!("rec-masked-{mode_name}"));
        let answers = record_typed(&dir, SENTINEL, incremental);
        assert!(
            answers.iter().all(|(accepted, _)| *accepted),
            "{mode_name}: both calls ran"
        );
        let bytes = the_check(&dir);
        let text = String::from_utf8(bytes.clone()).expect("the check is text");
        assert!(
            text.matches(SENTINEL).count() == 0,
            "{mode_name}: the check holds the typed text"
        );
        let check = Check::from_bytes(&bytes).expect("the check reads back");
        let typed = &check.steps()[0];
        assert!(typed.masked(), "{mode_name}: the type is flagged masked");
        assert!(
            typed.call().args.contains(&(
                "text".to_string(),
                ArgValue::Text(MASKED_STAND_IN.to_string())
            )),
            "{mode_name}: the type keeps the stand-in"
        );
        assert!(
            kept(&check) == answers,
            "{mode_name}: each step keeps its answer"
        );

        // A secret of another length leaves the same file: no length is kept.
        let other = fresh(&format!("rec-masked-other-{mode_name}"));
        let longer = SENTINEL.repeat(3);
        record_typed(&other, &longer, incremental);
        assert!(
            the_check(&other) == bytes,
            "{mode_name}: a secret of another length leaves another file"
        );

        let mut replayed_session = hold_fixture(incremental);
        assert!(
            replay(&mut replayed_session, &check) == answers,
            "{mode_name}: the masked check replays alike"
        );
        for dir in [dir, other] {
            let _ = fs::remove_dir_all(&dir);
        }
    }
}

#[test]
fn a_recording_directory_open_to_its_group_is_refused_with_nothing_written() {
    let dir = fresh("rec-open");
    fs::create_dir(&dir).expect("the directory is created");
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o750)).expect("opened to the group");
    let (session, _ticks) = hold(LeanTask::Counter, true);
    assert!(
        matches!(
            session.recording_into(&dir),
            Err(SessionError::RecordDirNotPrivate)
        ),
        "the open directory is refused"
    );
    assert!(
        fs::read_dir(&dir).expect("listed").count() == 0,
        "nothing is written"
    );
    let _ = fs::remove_dir_all(&dir);
}
