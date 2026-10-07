//! Time on a held instance moves by the driver's `advance` and by nothing else. On the stand's
//! Timer, in both layout modes, a driver `click` on Reset followed by a driver `advance` returns
//! settled with the delayed update already in the returned diff; an `advance` reports the time
//! the app actually moved — whole ticks, never more than was asked — and leaves the harness's
//! animation clock where it was. A session whose caller handed it no time step refuses
//! `advance` and changes nothing. On a fixture booted with the stand's options, a time step
//! that claims more than it was asked is reported as what was asked, and a click beside a
//! running animation returns settled without waiting on it and moves no clock. No check here
//! sleeps, reads a wall clock or makes a pass of its own: every pass is the driver's. A failure
//! message carries the layout mode, the task and a step index, never an id or what a screen
//! reads.

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::DiffNode;
use escher_driver::{Cause, Refusal, Session};
use seven_guis::stand::{self, LeanTask};

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
        let clock = timer.harness().time();

        // Milliseconds asked, milliseconds moved, and the elapsed time the diff returns.
        let before_reset = [(1000, 1000, Some("Elapsed: 1.0s"))];
        let after_reset = [
            (300, 300, Some("Elapsed: 0.3s")),
            (50, 0, None),
            (250, 200, Some("Elapsed: 0.5s")),
        ];
        assert!(before_reset.len() + after_reset.len() == 4, "four advances");

        let mut step = 0;
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
                    "{mode}: step {step}: the advance reports the whole ticks it moved"
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
                assert!(
                    timer.harness().time() == clock,
                    "{mode}: step {step}: the animation clock is where it was"
                );
                step += 1;
            }
        };

        run(&mut timer, &before_reset);
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
            "{mode}: the click leaves the animation clock where it was"
        );
        run(&mut timer, &after_reset);
    }
}

#[test]
fn a_session_with_no_time_step_refuses_advance_and_changes_nothing() {
    for incremental in [false, true] {
        for task in [LeanTask::Counter, LeanTask::FlightBooker, LeanTask::Crud] {
            let mode = format!("incremental={incremental}: {task:?}");
            let (mut session, _ticks) = hold(task, incremental);
            let before = session.harness().doc.snapshot();
            assert!(before.nodes().count() > 0, "{mode}: the snapshot has nodes");

            let refused = session.run(&advance(300));
            assert!(
                refused == Err(Refusal::new(Cause::TimeUnavailable)),
                "{mode}: the advance is refused time-unavailable"
            );
            assert!(
                session.harness().doc.snapshot() == before,
                "{mode}: the snapshots before and after the refused advance are equal"
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
fn an_advance_reports_no_more_than_it_was_asked() {
    // Milliseconds asked, what the caller's step claims it moved, and what the driver reports.
    let rows = [
        (100, 100, 100),
        (100, 40, 40),
        (100, 0, 0),
        (100, 600, 100),
        (60_000, u32::MAX, 60_000),
    ];
    assert_eq!(rows.len(), 5);
    for incremental in [false, true] {
        for (row, (asked, claimed, reported)) in rows.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: fixture: row {row}");
            let mut session = Session::start("claims", || {
                Harness::from_vdom(
                    VirtualDom::new(animated_fixture),
                    stand::options(incremental),
                )
            })
            .expect("the label is one of the closed set")
            .with_time(move |_, _| claimed);

            let Some(advanced) = act(&mut session, &advance(asked)) else {
                panic!("{mode}: the advance runs");
            };
            assert!(
                advanced.advanced_ms == Some(reported),
                "{mode}: the advance reports the lesser of what was asked and what the step claims"
            );
            assert!(
                advanced.settled && advanced.diff.is_empty(),
                "{mode}: a step that moves nothing returns settled with an empty diff"
            );
        }
    }
}

#[test]
fn a_click_beside_a_running_animation_returns_settled_and_moves_no_clock() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: fixture");
        let mut session = Session::start("animated", || {
            Harness::from_vdom(
                VirtualDom::new(animated_fixture),
                stand::options(incremental),
            )
        })
        .expect("the label is one of the closed set");
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
