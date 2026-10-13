//! What the driver's verbs do to a range input: the stand's Timer holds one, its duration slider,
//! and through the driver it moves. In both layout modes a driver `click` on the slider's id
//! focuses it and sets its value by where the click lands — the centre of its bounds, the middle
//! of its track — and a `press` of the right and up arrows steps it up by its `step`, the left and
//! down arrows step it down, and Home and End take it to its minimum and maximum, whether it was
//! focused by that click or by two Tab presses. Each value change reaches the app's `oninput`, so
//! the display beside the slider follows, and each call returns settled with the diff of the
//! screen before and after it, the same in both modes. The reading before this — no verb moved
//! the slider — was measured at 2026-10-07-act-by-id. A failure message carries the layout mode,
//! a sequence letter and a step index, never an id or what a screen reads.

use accesskit::Role;
use dioxus_native_dom::{Reading, SnapshotDiff};
use escher_driver::{Call, Session};
use seven_guis::stand::LeanTask;

mod session_common;
use session_common::{act, click, focused, hold, press};

const SLIDER: &str = "timer-duration";
const DISPLAY: &str = "timer-duration-value";

/// What one step changes: the slider's focus alone, or its value and the display's text.
#[derive(Clone, Copy)]
enum Changes {
    Focus,
    Value,
}

/// What the slider and the display beside it read now: the slider's value and whether it reads
/// focused, and the display's name.
fn reading(session: &Session) -> Option<(String, bool, String)> {
    let snapshot = session.harness().doc.snapshot();
    let slider = snapshot.get(SLIDER)?;
    let display = snapshot.get(DISPLAY)?;
    Some((
        slider.state.value.clone()?,
        slider.state.focused,
        display.name.clone(),
    ))
}

/// The slider's role and name now.
fn slider_label(session: &Session) -> Option<(Role, String)> {
    let snapshot = session.harness().doc.snapshot();
    let slider = snapshot.get(SLIDER)?;
    Some((slider.role, slider.name.clone()))
}

/// Whether `diff` names exactly what `changes` states: the slider alone, reading only its focus
/// changed; or the slider and the display, the slider's value and the display's name among what
/// differs.
fn names(diff: &SnapshotDiff, changes: Changes) -> bool {
    if !diff.added.is_empty() || !diff.removed.is_empty() {
        return false;
    }
    let ids: Vec<&str> = diff.changed.iter().map(|node| &*node.id).collect();
    match changes {
        Changes::Focus => ids == [SLIDER] && diff.changed[0].differs == [Reading::Focused],
        Changes::Value => {
            ids == [SLIDER, DISPLAY]
                && diff.changed[0].differs.contains(&Reading::Value)
                && diff.changed[1].differs.contains(&Reading::Name)
        }
    }
}

/// One step of a sequence: the call, what it changes, and the slider's value and the display's
/// text after it.
type Step = (Call, Changes, &'static str, &'static str);

/// Runs each step through the driver and asserts it returns settled, returns the diff of this
/// file's own snapshots before and after it, names what the step states, and leaves the slider
/// focused at the value stated, the display reading the text stated. Returns the diffs.
#[track_caller]
fn run(mode: &str, session: &mut Session, steps: &[Step]) -> Vec<SnapshotDiff> {
    let mut diffs = Vec::new();
    for (index, (call, changes, value, display)) in steps.iter().enumerate() {
        let before = session.harness().doc.snapshot();
        let Some(acted) = act(session, call) else {
            panic!("{mode}: step {index} runs");
        };
        let after = session.harness().doc.snapshot();
        assert!(
            acted.settled && acted.busy.is_none(),
            "{mode}: step {index} returns settled"
        );
        assert!(
            acted.diff == before.diff(&after),
            "{mode}: step {index} returns the diff of the snapshots before and after it"
        );
        assert!(
            names(&acted.diff, *changes),
            "{mode}: step {index} names exactly the controls stated for it"
        );
        assert!(
            reading(session) == Some((value.to_string(), true, display.to_string())),
            "{mode}: step {index} leaves the slider focused at the value stated"
        );
        diffs.push(acted.diff);
    }
    diffs
}

/// Sequence A, from boot: a click, the six keys, a right arrow and a click.
fn sequence_a() -> [Step; 9] {
    [
        (click(SLIDER), Changes::Focus, "15", "15.0s"),
        (press("arrow-right", false), Changes::Value, "15.5", "15.5s"),
        (press("arrow-up", false), Changes::Value, "16", "16.0s"),
        (press("arrow-left", false), Changes::Value, "15.5", "15.5s"),
        (press("arrow-down", false), Changes::Value, "15", "15.0s"),
        (press("end", false), Changes::Value, "30", "30.0s"),
        (press("home", false), Changes::Value, "0", "0.0s"),
        (press("arrow-right", false), Changes::Value, "0.5", "0.5s"),
        (click(SLIDER), Changes::Value, "15", "15.0s"),
    ]
}

#[test]
fn the_driver_moves_a_range_input_by_click_and_key() {
    let mut by_mode: Vec<(Vec<SnapshotDiff>, Vec<SnapshotDiff>)> = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: sequence A");
        let (mut timer, _ticks) = hold(LeanTask::Timer, incremental);
        assert!(
            timer
                .harness()
                .doc
                .snapshot()
                .get(SLIDER)
                .is_some_and(
                    |slider| slider.role == Role::Slider && slider.state.enabled == Some(true)
                ),
            "{mode}: the fixture's duration control is an enabled range input"
        );
        assert!(
            reading(&timer) == Some(("15".to_string(), false, "15.0s".to_string())),
            "{mode}: the slider reads its boot value, unfocused, at boot"
        );
        let steps = sequence_a();
        assert_eq!(steps.len(), 9);
        let a = run(&mode, &mut timer, &steps);

        let mode = format!("incremental={incremental}: sequence B");
        let (mut timer, _ticks) = hold(LeanTask::Timer, incremental);
        for (index, named) in [&["back-btn"] as &[&str], &["back-btn", SLIDER]]
            .into_iter()
            .enumerate()
        {
            let Some(acted) = act(&mut timer, &press("tab", false)) else {
                panic!("{mode}: tab {index} runs");
            };
            assert!(
                acted.settled && acted.changed() == named,
                "{mode}: tab {index} moves the focus as stated"
            );
        }
        let label = slider_label(&timer);
        assert!(
            label
                .as_ref()
                .is_some_and(|(role, _)| *role == Role::Slider),
            "{mode}: the focused control reads role Slider"
        );
        let b = run(
            &mode,
            &mut timer,
            &[(press("arrow-right", false), Changes::Value, "15.5", "15.5s")],
        );
        let (in_snapshot, in_tree) = focused(&timer);
        assert!(
            in_snapshot == [SLIDER] && in_tree == in_snapshot,
            "{mode}: the slider still holds focus after the arrow step"
        );
        assert!(
            slider_label(&timer) == label,
            "{mode}: the slider reads the same role and name after the arrow step"
        );
        by_mode.push((a, b));
    }
    assert!(
        by_mode[0] == by_mode[1],
        "every diff is equal across the two layout modes"
    );
}
