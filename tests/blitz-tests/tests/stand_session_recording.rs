//! A driver session keeps itself as a replayable check, owner-only and holding each answer as
//! the call got it but a shot's pixels, and the check replays. In both layout modes. *On each
//! lean task:* a session recording under the test target's temp dir runs a script of a click, a
//! `type`, a `press`, an `advance` and a `scroll` where the task has the control, a call refused
//! for an id no screen reads, a `snapshot` and a `screenshot`; its check reads back with one step
//! per call, each step's acceptance and digest those of the answer the call got and its `answer`
//! the line the driver wrote for it — a shot's without its image, so no `png` key and none of the
//! shot's first 64 base64 characters occur in the check — and `Check::replay` on a freshly
//! booted session holds with every step run. The check file reads `0600` in a `0700` directory.
//! *On a password fixture:* a sentinel typed into the password input is kept as the fixed
//! stand-in, flagged masked — the sentinel's bytes occur nowhere, and a sentinel of another
//! length leaves the same file — and the check still holds on replay. *A recording directory open
//! to its group* is refused with nothing written. A failure message carries the layout mode, the
//! task and a step index, never an id, a name or what a screen reads. Unix only: a check's modes
//! are unix modes.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use escher_driver::{
    ArgValue, Call, Check, MASKED_STAND_IN, Outcome, Refusal, SessionError, answer_digest,
};
use seven_guis::stand::LeanTask;

mod fixtures;
mod session_common;
use session_common::{
    advance, click, hold, kept, mode, press, screenshot, scroll, shot_prefix, slug, snapshot,
    state_dir, type_into,
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
            type_into("flight-type", "return"),
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

/// The line a check keeps for `answer`: the line the driver wrote, a shot's without its image.
fn kept_line(answer: &Result<Outcome, Refusal>) -> String {
    match answer {
        Ok(Outcome::Shot { width, height, .. }) => {
            format!(r#"{{"width":{width},"height":{height}}}"#)
        }
        Ok(outcome) => outcome.to_json(),
        Err(refusal) => refusal.to_json(),
    }
}

#[test]
fn a_recorded_session_keeps_each_answer_but_a_shots_image_and_replays() {
    for incremental in [false, true] {
        let mode_name = if incremental { "incremental" } else { "full" };
        for task in LeanTask::ALL {
            let task_name = slug(task);
            let dir = fresh(&format!("rec-{task_name}-{mode_name}"));
            let (session, _ticks) = hold(task, incremental);
            let mut session = session.recording_into(&dir).expect("the recording starts");
            let calls = script(task);
            let mut answers = Vec::new();
            let mut lines = Vec::new();
            for call in &calls {
                let answer = session.run(call);
                answers.push((answer.is_ok(), answer_digest(&answer)));
                lines.push(kept_line(&answer));
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

            let text = String::from_utf8(bytes).expect("the check is text");
            // The opening `[` and record come first; each step's record ends with its answer.
            for (index, (record, line)) in text.lines().skip(2).zip(&lines).enumerate() {
                let record = record.strip_suffix(',').unwrap_or(record);
                assert!(
                    record.ends_with(&format!(r#","answer":{line}}}"#)),
                    "{mode_name} {task_name}: step {index} keeps the line its call got"
                );
            }
            let prefix = shot_prefix(&mut session);
            assert!(
                !text.contains(r#""png""#) && !text.contains(&prefix),
                "{mode_name} {task_name}: the check holds a shot's image"
            );

            let (mut fresh_session, _ticks) = hold(task, incremental);
            let replay = check.replay(&mut fresh_session);
            let changed_at = replay.change().map(|change| change.step());
            assert!(
                replay.holds() && replay.ran() == calls.len(),
                "{mode_name} {task_name}: the check holds on replay, its first change at step \
                 {changed_at:?}"
            );

            assert!(
                mode(&dir) == 0o700,
                "{mode_name} {task_name}: the directory is owner-only"
            );
            assert!(
                mode(&dir.join("check-1.json")) == 0o600,
                "{mode_name} {task_name}: the check is owner-only"
            );

            let _ = fs::remove_dir_all(&dir);
        }
    }
}

/// Records `start`, a `type` of `secret` into the password input and a `snapshot` into `dir`:
/// the answers' acceptances and digests.
fn record_typed(dir: &Path, secret: &str, incremental: bool) -> Vec<(bool, u64)> {
    let mut session = fixtures::hold("masked", fixtures::password_named, incremental)
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

        let replay = check.replay(&mut fixtures::hold(
            "masked",
            fixtures::password_named,
            incremental,
        ));
        assert!(
            replay.holds() && replay.ran() == answers.len(),
            "{mode_name}: the masked check holds on replay"
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
