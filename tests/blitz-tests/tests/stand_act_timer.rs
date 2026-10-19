//! Time on a held instance moves by the driver's `advance` and by nothing else. On the stand's
//! Timer, in both layout modes, a driver `click` on Reset followed by a driver `advance` returns
//! settled with the delayed update already in the returned diff; an `advance` moves the
//! session's clock by exactly the milliseconds asked and reports them, while the Timer reads the
//! clock's whole ticks, a remainder carried to the next `advance`. A session whose caller handed
//! it no time step still takes `advance`: its clock moves and, with nothing that reads it on the
//! screen, its diff is empty. On a fixture booted with the stand's options, an `advance` reports
//! exactly what was asked whatever the caller's step does, and a click beside a running
//! animation returns settled without waiting on it and moves no clock. No check here
//! sleeps, reads a wall clock or makes a pass of its own: every pass is the driver's. A failure
//! message carries the layout mode, the task and a step index, never an id or what a screen
//! reads.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_native_dom::DiffNode;
use escher_driver::Session;
use seven_guis::stand::LeanTask;

mod fixtures;
mod session_common;
use session_common::{Acted, act, advance, click, hold};

/// The name the diff returns for the Timer's elapsed time: `None` when the diff does not name
/// it changed.
fn elapsed(acted: &Acted) -> Option<&str> {
    acted
        .diff
        .changed
        .iter()
        .find(|entry| entry.id == "timer-elapsed")
        .map(|entry: &DiffNode| entry.name.as_str())
}

#[test]
fn a_reset_then_an_advance_returns_settled_with_the_delayed_update_present() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Timer");
        let (mut timer, _ticks) = hold(LeanTask::Timer, incremental);
        assert!(
            timer.harness().time() == 0.0,
            "{mode}: the clock starts at zero"
        );

        // Milliseconds asked, milliseconds the clock moved, and the elapsed time the diff
        // returns: the 50 ms are carried, and the 250 ms after them make three ticks.
        let before_reset = [(1000, 1000, Some("Elapsed: 1.0s"))];
        let after_reset = [
            (300, 300, Some("Elapsed: 0.3s")),
            (50, 50, None),
            (250, 250, Some("Elapsed: 0.6s")),
        ];
        assert!(before_reset.len() + after_reset.len() == 4, "four advances");

        let mut step = 0;
        let mut clock_ms = 0;
        let mut run = |timer: &mut Session, rows: &[(i64, u32, Option<&str>)]| {
            for (asked, moved, reads) in rows {
                let Some(advanced) = act(timer, &advance(*asked)) else {
                    panic!("{mode}: step {step}: the advance runs");
                };
                assert!(
                    advanced.settled && advanced.busy.is_none(),
                    "{mode}: step {step}: the advance returns settled"
                );
                assert!(
                    advanced.advanced_ms == Some(*moved),
                    "{mode}: step {step}: the advance reports the milliseconds the clock moved"
                );
                assert!(
                    elapsed(&advanced) == *reads,
                    "{mode}: step {step}: the elapsed time is in the returned diff as it reads \
                     after the advance"
                );
                assert!(
                    advanced.diff.is_empty() == reads.is_none(),
                    "{mode}: step {step}: the diff is empty exactly when no whole tick was moved"
                );
                clock_ms += u64::from(*moved);
                assert!(
                    timer.harness().time() == Duration::from_millis(clock_ms).as_secs_f64(),
                    "{mode}: step {step}: the clock moved by exactly the milliseconds asked"
                );
                step += 1;
            }
        };

        run(&mut timer, &before_reset);
        let clock = timer.harness().time();
        let Some(reset) = act(&mut timer, &click("timer-reset")) else {
            panic!("{mode}: the click on Reset runs");
        };
        assert!(reset.settled, "{mode}: the click on Reset returns settled");
        assert!(
            elapsed(&reset) == Some("Elapsed: 0.0s"),
            "{mode}: Reset returns the elapsed time at zero"
        );
        assert!(
            timer.harness().time() == clock,
            "{mode}: the click leaves the clock where it was"
        );
        run(&mut timer, &after_reset);
    }
}

#[test]
fn an_advance_on_a_session_with_no_time_step_moves_the_clock_and_settles() {
    for incremental in [false, true] {
        for task in [LeanTask::Counter, LeanTask::FlightBooker, LeanTask::Crud] {
            let mode = format!("incremental={incremental}: {task:?}");
            let (mut session, _ticks) = hold(task, incremental);
            let before = session.harness().doc.snapshot();
            assert!(before.nodes().count() > 0, "{mode}: the snapshot has nodes");

            let Some(advanced) = act(&mut session, &advance(300)) else {
                panic!("{mode}: the advance is accepted");
            };
            assert!(
                advanced.settled && advanced.busy.is_none(),
                "{mode}: the advance returns settled"
            );
            assert!(
                advanced.advanced_ms == Some(300),
                "{mode}: the advance reports the milliseconds asked"
            );
            assert!(
                advanced.diff.is_empty(),
                "{mode}: nothing on the screen reads the clock, so the diff is empty"
            );
            assert!(
                session.harness().time() == Duration::from_millis(300).as_secs_f64(),
                "{mode}: the clock moved by exactly the milliseconds asked"
            );
            assert!(
                session.harness().doc.snapshot() == before,
                "{mode}: the snapshots before and after the advance are equal"
            );
        }
    }
}

const ANIMATED_CSS: &str = "
    @keyframes fx-slide { from { margin-left: 0px; } to { margin-left: 100px; } }
    #fx-animated { width: 50px; height: 50px; animation: fx-slide 1s linear infinite; }
";

/// A box that animates for ever beside a button that counts its clicks.
fn animated_fixture() -> Element {
    let mut presses = use_signal(|| 0);
    rsx! {
        style { {ANIMATED_CSS} }
        div { id: "fx-root",
            button { id: "fx-press", onclick: move |_| presses += 1, "Pressed {presses}" }
            div { id: "fx-animated" }
        }
    }
}

#[test]
fn an_advance_reports_exactly_what_was_asked_whatever_the_step_does() {
    // Milliseconds asked, and whether the caller's step moves the screen.
    let rows = [(100, false), (100, true), (1, true), (60_000, false)];
    assert_eq!(rows.len(), 4);
    for incremental in [false, true] {
        for (row, (asked, presses)) in rows.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: fixture: row {row}");
            let handed = std::rc::Rc::new(std::cell::Cell::new(None));
            let seen = handed.clone();
            let mut session = fixtures::hold("claims", animated_fixture, incremental).with_time(
                move |harness, ms| {
                    seen.set(Some(ms));
                    if presses {
                        harness.click("#fx-press");
                    }
                },
            );

            let Some(advanced) = act(&mut session, &advance(asked)) else {
                panic!("{mode}: the advance runs");
            };
            assert!(
                handed.get() == u32::try_from(asked).ok(),
                "{mode}: the step is handed the milliseconds asked"
            );
            assert!(
                advanced.advanced_ms == u32::try_from(asked).ok(),
                "{mode}: the advance reports the milliseconds asked"
            );
            assert!(
                session.harness().time()
                    == Duration::from_millis(asked.unsigned_abs()).as_secs_f64(),
                "{mode}: the clock moved by exactly the milliseconds asked"
            );
            assert!(
                advanced.settled && advanced.added().is_empty() && advanced.removed().is_empty(),
                "{mode}: the advance returns settled and adds or removes nothing"
            );
            assert!(
                advanced.changed().contains(&"fx-press") == presses,
                "{mode}: the button reads changed exactly when the step pressed it"
            );
        }
    }
}

#[test]
fn a_click_beside_a_running_animation_returns_settled_and_moves_no_clock() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: fixture");
        let mut session = fixtures::hold("animated", animated_fixture, incremental);
        assert!(
            session.harness().base().is_animating(),
            "{mode}: the fixture is animating at boot"
        );
        let clock = session.harness().time();

        let Some(pressed) = act(&mut session, &click("fx-press")) else {
            panic!("{mode}: the click runs");
        };
        assert!(
            pressed.settled && pressed.busy.is_none(),
            "{mode}: the click returns settled beside the running animation"
        );
        assert!(
            pressed.changed() == ["fx-press"]
                && pressed.added().is_empty()
                && pressed.removed().is_empty(),
            "{mode}: the click names the button it rewrote, and not the animated box"
        );
        assert!(
            session.harness().time() == clock,
            "{mode}: the click leaves the animation clock where it was"
        );
        assert!(
            session.harness().base().is_animating(),
            "{mode}: the animation is still running after the click"
        );
    }
}
