//! What the driver's verbs do to a range input, measured and pinned: the stand's Timer holds
//! one, its duration slider, and through the driver it cannot be changed. In both layout modes a
//! driver `click` on the slider's id is accepted and changes nothing — it does not focus the
//! slider — and a `press` of the right arrow, the left arrow, Home and End each returns settled
//! with an empty diff, whether the slider is unfocused after that click or focused by two Tab
//! presses; the slider reads its boot value and the display beside it its boot text after every
//! call. This file asserts the reading as it was measured, not a behaviour the engine is meant
//! to have: a range interaction model would turn it red, and that is its use. A failure message
//! carries the layout mode, a sequence letter and a step index, never an id or what a screen
//! reads.

use accesskit::Role;
use escher_driver::{Call, Session};
use seven_guis::stand::LeanTask;

mod session_common;
use session_common::{act, click, focused, hold, press};

/// The slider's value and the display's text at boot.
const BOOT: (&str, &str) = ("15", "15.0s");

/// The four keys the measurement presses.
const KEYS: [&str; 4] = ["arrow-right", "arrow-left", "home", "end"];

/// What the slider and the display beside it read now: the slider's value and whether it reads
/// focused, and the display's name.
fn reading(session: &Session) -> Option<(String, bool, String)> {
    let snapshot = session.harness().doc.snapshot();
    let slider = snapshot.get("timer-duration")?;
    let display = snapshot.get("timer-duration-value")?;
    Some((
        slider.state.value.clone()?,
        slider.state.focused,
        display.name.clone(),
    ))
}

/// Runs each call in turn through the driver and asserts it returns settled, names exactly the
/// controls stated as changed, and leaves the slider at its boot value with the focus stated.
#[track_caller]
fn run(mode: &str, session: &mut Session, steps: &[(Call, &[&str], bool)]) {
    for (index, (call, named, slider_focused)) in steps.iter().enumerate() {
        let Some(acted) = act(session, call) else {
            panic!("{mode}: step {index} runs");
        };
        assert!(
            acted.settled && acted.busy.is_none(),
            "{mode}: step {index} returns settled"
        );
        assert!(
            acted.changed() == *named && acted.added().is_empty() && acted.removed().is_empty(),
            "{mode}: step {index} names exactly the controls stated for it"
        );
        assert!(
            reading(session) == Some((BOOT.0.to_string(), *slider_focused, BOOT.1.to_string())),
            "{mode}: step {index} leaves the slider at its boot value, focused as stated"
        );
    }
}

#[test]
fn a_range_input_reads_its_boot_value_after_every_driver_call() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: sequence A");
        let (mut timer, _ticks) = hold(LeanTask::Timer, incremental);
        assert!(
            timer
                .harness()
                .doc
                .snapshot()
                .get("timer-duration")
                .is_some_and(
                    |slider| slider.role == Role::Slider && slider.state.enabled == Some(true)
                ),
            "{mode}: the fixture's duration control is an enabled range input"
        );
        assert!(
            reading(&timer) == Some((BOOT.0.to_string(), false, BOOT.1.to_string())),
            "{mode}: the slider reads its boot value, unfocused, at boot"
        );

        let mut by_click: Vec<(Call, &[&str], bool)> = vec![(click("timer-duration"), &[], false)];
        by_click.extend(KEYS.map(|key| (press(key, false), &[] as &[&str], false)));
        assert_eq!(by_click.len(), 5);
        run(&mode, &mut timer, &by_click);
        let (in_snapshot, in_tree) = focused(&timer);
        assert!(
            in_snapshot.is_empty() && in_tree.is_empty(),
            "{mode}: nothing reads focused after the click and the four keys"
        );

        let mode = format!("incremental={incremental}: sequence B");
        let (mut timer, _ticks) = hold(LeanTask::Timer, incremental);
        let mut by_tab: Vec<(Call, &[&str], bool)> = vec![
            (press("tab", false), &["back-btn"], false),
            (press("tab", false), &["back-btn", "timer-duration"], true),
        ];
        by_tab.extend(KEYS.map(|key| (press(key, false), &[] as &[&str], true)));
        assert_eq!(by_tab.len(), 6);
        run(&mode, &mut timer, &by_tab);
        let (in_snapshot, in_tree) = focused(&timer);
        assert!(
            in_snapshot == ["timer-duration"] && in_tree == in_snapshot,
            "{mode}: the slider still holds focus after the four keys"
        );
    }
}
