//! A headless stand session reads one clock, and only the driver's `advance` moves it, so two
//! shots of one state read the same pixels however much wall time passes between them. On a
//! held CRUD session, in both layout modes: after twelve Creates and a driver `scroll` to the last
//! row, `crud-list`'s shot taken after an 800 ms sleep with no `advance` is byte-identical to the
//! one taken right after the scroll; `advance --ms 600` changes it, the thumb part faded;
//! `advance --ms 100` changes it again, the thumb gone; `advance --ms 1000` leaves it as it was;
//! the shot right after the scroll differs from the faded one, so a thumb was painted; and the
//! session's clock then reads exactly 1.7 s. On the same clock two driver `click`s on one point
//! with no `advance` between them are a double click: after a `type` into `crud-name` and an
//! `advance --ms 500` (the `type` clicks its target first), two clicks select a word where a
//! click, an `advance --ms 500` and a click leave the caret; the two shots differ, and the
//! double click's shot is the same across fresh sessions.
//!
//! The 800 ms `std::thread::sleep` here is the stand checks' one recorded exception to their
//! no-sleep rule: what this file proves is that wall time passing moves nothing. A failure
//! message carries the layout mode, the step and an index, never an id or what a screen reads.

use std::time::Duration;

use escher_driver::{Call, Session};
use seven_guis::stand::LeanTask;

mod session_common;
use session_common::{act, advance, click, hold, screenshot_of, scroll, shot, type_into};

/// The Creates that put the last row past the list's box.
const CREATES: usize = 12;

/// The wall time slept between the first two shots.
const SLEEP: Duration = Duration::from_millis(800);

/// Runs an acting `call` that must be accepted and settle.
#[track_caller]
fn acts(session: &mut Session, call: &Call, at: &str) {
    let Some(acted) = act(session, call) else {
        panic!("{at}: the call is accepted");
    };
    assert!(acted.settled, "{at}: the call settles");
}

/// The PNG of `crud-name` or `crud-list`, as the driver shoots it.
#[track_caller]
fn png(session: &mut Session, id: &str, at: &str) -> Vec<u8> {
    let Some(shot) = shot(session, &screenshot_of(id)) else {
        panic!("{at}: the shot is taken");
    };
    shot.png
}

#[test]
fn a_shot_after_a_scroll_repeats_until_advance_moves_the_fade() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Crud: fade");
        let (mut crud, _ticks) = hold(LeanTask::Crud, incremental);
        let mut last_row = None;
        for index in 0..CREATES {
            let Some(created) = act(&mut crud, &click("crud-create")) else {
                panic!("{mode}: create {index}: the click is accepted");
            };
            last_row = created.diff.added.first().map(|row| row.id.clone());
        }
        let Some(last_row) = last_row else {
            panic!("{mode}: the last Create adds a row");
        };
        let Some(scrolled) = act(&mut crud, &scroll(&last_row)) else {
            panic!("{mode}: the scroll is accepted");
        };
        assert!(
            scrolled.in_view == Some(true) && !scrolled.diff.is_empty(),
            "{mode}: the scroll moves the list and brings the row into view"
        );

        let s1 = png(&mut crud, "crud-list", &format!("{mode}: shot 1"));
        std::thread::sleep(SLEEP);
        let s2 = png(&mut crud, "crud-list", &format!("{mode}: shot 2"));
        acts(&mut crud, &advance(600), &format!("{mode}: advance 1"));
        let s3 = png(&mut crud, "crud-list", &format!("{mode}: shot 3"));
        acts(&mut crud, &advance(100), &format!("{mode}: advance 2"));
        let s4 = png(&mut crud, "crud-list", &format!("{mode}: shot 4"));
        acts(&mut crud, &advance(1000), &format!("{mode}: advance 3"));
        let s5 = png(&mut crud, "crud-list", &format!("{mode}: shot 5"));

        assert!(s2 == s1, "{mode}: shot 2 repeats shot 1 across the sleep");
        assert!(s3 != s1, "{mode}: shot 3 differs from shot 1");
        assert!(s4 != s3, "{mode}: shot 4 differs from shot 3");
        assert!(s5 == s4, "{mode}: shot 5 repeats shot 4");
        assert!(
            s1 != s4,
            "{mode}: shot 1 painted a thumb that shot 4 does not"
        );
        assert!(
            crud.harness().time() == Duration::from_millis(1700).as_secs_f64(),
            "{mode}: the clock reads the three advances"
        );
    }
}

/// A fresh CRUD session with a two-word name typed into `crud-name`, the clock then moved past
/// the double-click window of the `type`'s own click, and two clicks on the field with
/// `between` milliseconds of clock between them: the field's shot after them.
fn clicked_twice(incremental: bool, between: Option<i64>, at: &str) -> Vec<u8> {
    let (mut crud, _ticks) = hold(LeanTask::Crud, incremental);
    acts(
        &mut crud,
        &type_into("crud-name", "Grace Hopper"),
        &format!("{at}: type"),
    );
    acts(&mut crud, &advance(500), &format!("{at}: advance 1"));
    acts(&mut crud, &click("crud-name"), &format!("{at}: click 1"));
    if let Some(ms) = between {
        acts(&mut crud, &advance(ms), &format!("{at}: advance 2"));
    }
    acts(&mut crud, &click("crud-name"), &format!("{at}: click 2"));
    png(&mut crud, "crud-name", &format!("{at}: shot"))
}

#[test]
fn two_clicks_with_no_advance_between_them_are_a_double_click() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Crud: double click");
        let double = clicked_twice(incremental, None, &format!("{mode}: session 1"));
        let again = clicked_twice(incremental, None, &format!("{mode}: session 2"));
        let single = clicked_twice(incremental, Some(500), &format!("{mode}: session 3"));
        assert!(
            double != single,
            "{mode}: the double click's shot differs from two single clicks'"
        );
        assert!(
            double == again,
            "{mode}: the double click's shot repeats across fresh sessions"
        );
    }
}
