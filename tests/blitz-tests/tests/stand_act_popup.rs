//! What the driver's verbs do to a select's open list, in both layout modes through `Session::run`.
//! On the stand's `select#flight-type`: a `click` opens it, its diff naming the rows' `bounds`; a
//! `click` on a row is accepted and the next snapshot reads its value; `press enter`, `arrow-down`,
//! `enter` chooses by key; `press escape` closes with the value unchanged, the rows back at the
//! select's client rect, the Tab walk holding. A closed option stays refused `covered`; a row past a
//! long list's scrollport is refused `off-screen` until `scroll` brings it in; a button under the
//! list is refused `covered`. `type` leaves no list open; `screenshot` of a row crops to it and
//! changes nothing; `lint` reads no `overlap` on a row, the planted pair still reading; the open
//! screen stays under the text budget; every actionable element stays keyed; two `click`s on one
//! point open and close, each focusing; and the `VirtualClock` stays where it was. A failure
//! message carries the layout mode and a row index, never an id or what a screen reads.

use dioxus::prelude::*;
use dioxus_native_dom::{Reading, SNAPSHOT_TEXT_BUDGET};
use escher_driver::{Cause, Defect, Outcome, Session};
use seven_guis::stand::LeanTask;

mod fixtures;
mod session_common;
use session_common::{
    Acted, act, click, focused, hold, lint, linted, press, refused, refused_unchanged,
    screenshot_of, scroll, shot, type_into,
};

fn flight(incremental: bool) -> Session {
    hold(LeanTask::FlightBooker, incremental).0
}

fn value(session: &Session, id: &str) -> Option<String> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.value.clone())
}

fn bounds(session: &Session, id: &str) -> Option<(f64, f64, f64, f64)> {
    let snapshot = session.harness().doc.snapshot();
    let node = snapshot.get(id)?;
    let read = node.bounds;
    Some((read.x, read.y, read.width, read.height))
}

fn differs(acted: &Acted, id: &str) -> Vec<Reading> {
    acted
        .diff
        .changed
        .iter()
        .find(|node| node.id == id)
        .map(|node| node.differs.clone())
        .unwrap_or_default()
}

const SELECT: &str = "flight-type";
const ROWS: [&str; 2] = ["flight-type-one-way", "flight-type-return"];

#[test]
fn a_click_opens_the_list_and_a_click_on_a_row_chooses_it() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        let at_boot = bounds(&session, ROWS[1]);
        assert!(
            at_boot == bounds(&session, SELECT),
            "{mode}: a closed option reads its select's box"
        );
        let opened = act(&mut session, &click(SELECT))
            .unwrap_or_else(|| panic!("{mode}: the click is accepted"));
        assert!(opened.settled, "{mode}: the click settles");
        for (row, id) in ROWS.into_iter().enumerate() {
            assert!(
                differs(&opened, id).contains(&Reading::Bounds),
                "{mode}: row {row}: the diff names the row's bounds"
            );
        }
        assert!(
            bounds(&session, ROWS[1]) != bounds(&session, SELECT),
            "{mode}: an open row reads a box of its own"
        );
        let chosen = act(&mut session, &click(ROWS[1]))
            .unwrap_or_else(|| panic!("{mode}: the click on the row is accepted"));
        assert!(chosen.settled, "{mode}: the choice settles");
        assert!(
            differs(&chosen, SELECT).contains(&Reading::Value),
            "{mode}: the diff names the select's value"
        );
        assert!(
            differs(&chosen, ROWS[1]).contains(&Reading::Selected),
            "{mode}: the diff names the row's selection"
        );
        assert!(
            value(&session, SELECT).as_deref() == Some("return"),
            "{mode}: the next snapshot reads the choice"
        );
        assert!(
            focused(&session).0 == [SELECT],
            "{mode}: focus stays on the select"
        );
    }
}

#[test]
fn the_keys_alone_open_move_and_choose() {
    let keys = ["tab", "tab", "enter", "arrow-down", "enter"];
    assert_eq!(keys.len(), 5);
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        for (row, key) in keys.into_iter().enumerate() {
            assert!(
                act(&mut session, &press(key, false)).is_some_and(|acted| acted.settled),
                "{mode}: row {row}: the press is accepted and settled"
            );
        }
        assert!(
            focused(&session).0 == [SELECT],
            "{mode}: the Tab walk reached the select"
        );
        assert!(
            value(&session, SELECT).as_deref() == Some("return"),
            "{mode}: the keys chose the row"
        );
    }
}

#[test]
fn escape_closes_with_the_value_unchanged_and_the_rows_back_at_the_selects_box() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        let closed = session.harness().doc.snapshot();
        assert!(act(&mut session, &click(SELECT)).is_some(), "{mode}: opens");
        let shut = act(&mut session, &press("escape", false))
            .unwrap_or_else(|| panic!("{mode}: the press is accepted"));
        assert!(shut.settled, "{mode}: the press settles");
        assert!(
            value(&session, SELECT).as_deref() == Some("one-way"),
            "{mode}: the value stands"
        );
        for (row, id) in ROWS.into_iter().enumerate() {
            assert!(
                bounds(&session, id) == bounds(&session, SELECT),
                "{mode}: row {row}: back at the select's box"
            );
        }
        let after = session.harness().doc.snapshot();
        let mut diff = closed.diff(&after);
        diff.changed.retain(|node| node.id != SELECT);
        assert!(
            diff.added.is_empty() && diff.removed.is_empty() && diff.changed.is_empty(),
            "{mode}: past the select's focus the screen reads as at boot"
        );
    }
}

#[test]
fn a_closed_option_stays_refused_covered_and_changes_nothing() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        let (cause, unchanged) = refused_unchanged(&mut session, &click(ROWS[1]));
        assert!(cause == Some(Cause::Covered), "{mode}: the cause");
        assert!(unchanged, "{mode}: the refused call changes nothing");
    }
}

/// A select of 40 rows, a button its list opens over, and a planted overlap apart from it.
fn long_page() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 760px; height: 560px;",
            select { id: "fx-long",
                for row in 0..40 {
                    option { key: "{row}", id: "fx-row-{row}", value: "{row}", "Row {row}" }
                }
            }
            button {
                id: "fx-under",
                style: "position: absolute; left: 0; top: 60px;",
                "Under"
            }
            div {
                id: "fx-overlap-a",
                style: "position: absolute; left: 500px; top: 20px; width: 100px; height: 40px;",
            }
            div {
                id: "fx-overlap-b",
                style: "position: absolute; left: 560px; top: 40px; width: 100px; height: 40px;",
            }
        }
    }
}

fn hold_long(incremental: bool) -> Session {
    fixtures::hold("select", long_page, incremental)
}

#[test]
fn a_row_past_the_scrollport_is_off_screen_until_scroll_brings_it_in() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_long(incremental);
        assert!(
            act(&mut session, &click("fx-long")).is_some(),
            "{mode}: opens"
        );
        assert!(
            refused(&mut session, &click("fx-row-39")) == Some(Cause::OffScreen),
            "{mode}: a row past the scrollport is off-screen"
        );
        let scrolled = act(&mut session, &scroll("fx-row-39"))
            .unwrap_or_else(|| panic!("{mode}: the scroll is accepted"));
        assert!(
            scrolled.in_view == Some(true),
            "{mode}: the scroll brings the row in"
        );
        assert!(
            act(&mut session, &click("fx-row-39")).is_some(),
            "{mode}: the row then takes the click"
        );
        assert!(
            value(&session, "fx-long").as_deref() == Some("39"),
            "{mode}: and the select reads it"
        );
    }
}

#[test]
fn a_button_under_the_list_is_refused_covered() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_long(incremental);
        assert!(
            act(&mut session, &click("fx-under")).is_some(),
            "{mode}: the closed page's button takes a click"
        );
        assert!(
            act(&mut session, &click("fx-long")).is_some(),
            "{mode}: opens"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-under"));
        assert!(cause == Some(Cause::Covered), "{mode}: the cause");
        assert!(unchanged, "{mode}: the refused call changes nothing");
    }
}

#[test]
fn type_leaves_no_list_open_and_chooses_by_its_text() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        assert!(
            act(&mut session, &type_into(SELECT, "Return")).is_some(),
            "{mode}: a type on a closed select is accepted"
        );
        assert!(
            session.harness().base().open_select().is_none(),
            "{mode}: it opens nothing"
        );
        assert!(
            value(&session, SELECT).as_deref() == Some("return"),
            "{mode}: the text chose its option"
        );
        assert!(
            act(&mut session, &press("enter", false)).is_some(),
            "{mode}"
        );
        assert!(
            session.harness().base().open_select().is_some(),
            "{mode}: the fixture's list opens"
        );
        assert!(
            act(&mut session, &type_into(SELECT, "One")).is_some(),
            "{mode}: a type on an open select is accepted"
        );
        assert!(
            session.harness().base().open_select().is_none(),
            "{mode}: and closes its list"
        );
        assert!(
            value(&session, SELECT).as_deref() == Some("one-way"),
            "{mode}: the text chose its option"
        );
    }
}

#[test]
fn a_shot_of_a_row_crops_to_it_and_leaves_the_list_open() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        assert!(act(&mut session, &click(SELECT)).is_some(), "{mode}: opens");
        let before = session.harness().doc.snapshot();
        let taken = shot(&mut session, &screenshot_of(ROWS[1]))
            .unwrap_or_else(|| panic!("{mode}: the shot is accepted"));
        let rect = bounds(&session, ROWS[1]).unwrap_or_else(|| panic!("{mode}: the row reads"));
        assert!(
            f64::from(taken.width) >= rect.2 && f64::from(taken.width) <= rect.2 + 2.0,
            "{mode}: the shot is the row's width"
        );
        assert!(
            f64::from(taken.height) >= rect.3 && f64::from(taken.height) <= rect.3 + 2.0,
            "{mode}: the shot is the row's height"
        );
        assert!(
            session.harness().base().open_select().is_some(),
            "{mode}: the list stands"
        );
        let after = session.harness().doc.snapshot();
        let diff = before.diff(&after);
        assert!(
            diff.added.is_empty() && diff.removed.is_empty() && diff.changed.is_empty(),
            "{mode}: the shot changes nothing"
        );
    }
}

#[test]
fn the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_long(incremental);
        assert!(
            act(&mut session, &click("fx-long")).is_some(),
            "{mode}: opens"
        );
        let findings =
            linted(&mut session, &lint()).unwrap_or_else(|| panic!("{mode}: the lint is accepted"));
        let overlaps: Vec<_> = findings
            .iter()
            .filter(|finding| finding.defect == Defect::Overlap)
            .collect();
        assert!(
            overlaps
                .iter()
                .all(|finding| !finding.id.starts_with("fx-row")),
            "{mode}: no row reads an overlap"
        );
        assert!(
            overlaps.iter().any(|finding| finding.id == "fx-overlap-a"),
            "{mode}: the planted pair still reads"
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.id == "fx-row-39" && finding.defect == Defect::OffScreen),
            "{mode}: a row past the scrollport keeps its other readings"
        );
    }
}

#[test]
fn the_open_screen_stays_under_its_budget_and_every_actionable_element_keyed() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        let rows = [(false, 0), (true, 1)];
        assert_eq!(rows.len(), 2);
        for (open, row) in rows {
            assert!(
                session.harness().doc.unkeyed_actionable().is_empty(),
                "{mode}: row {row}: every actionable element reads a key"
            );
            assert!(
                session.harness().doc.snapshot().to_text().len() <= SNAPSHOT_TEXT_BUDGET,
                "{mode}: row {row}: the screen is under the budget"
            );
            assert!(
                session.harness().base().open_select().is_some() == open,
                "{mode}: row {row}: the fixture's state"
            );
            if !open {
                assert!(act(&mut session, &click(SELECT)).is_some(), "{mode}: opens");
            }
        }
    }
}

#[test]
fn two_clicks_on_one_point_open_and_close_each_focusing() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        let time = session.harness().time();
        let rows = [(true, 0), (false, 1)];
        assert_eq!(rows.len(), 2);
        for (stands, row) in rows {
            assert!(
                act(&mut session, &click(SELECT)).is_some(),
                "{mode}: row {row}: the click is accepted"
            );
            assert!(
                session.harness().base().open_select().is_some() == stands,
                "{mode}: row {row}: the list's state"
            );
            assert!(
                focused(&session).0 == [SELECT],
                "{mode}: row {row}: the click focuses"
            );
        }
        assert!(
            session.harness().time() == time,
            "{mode}: opening and closing moves no time"
        );
    }
}

#[test]
fn a_screen_read_after_a_step_reads_the_new_state_with_no_advance() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = flight(incremental);
        let answer = session.run(&click(SELECT));
        assert!(
            matches!(answer, Ok(Outcome::Acted { settled: true, .. })),
            "{mode}: the click settles"
        );
        assert!(
            bounds(&session, ROWS[0]) != bounds(&session, SELECT),
            "{mode}: the next read shows the rows"
        );
    }
}
