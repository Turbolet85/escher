//! The driver's `scroll` brings the element an id names into view, so the `off-screen` remedy
//! can be followed, and its result says when it could not. In both layout modes. *On the
//! stand's CRUD task:* the driver creates rows until one lies past the list's box; a `click` on
//! that row is refused `off-screen`; `scroll` naming it returns settled and in view, the row
//! now inside the list's box; a `click` on it is then accepted and selects it; and the first
//! row, scrolled out the other way, is refused in its turn. *On a fixture whose viewport
//! scrolls:* `scroll` brings a far button inside the viewport and a `click` on it lands. *On a
//! fixture no scroll can reach* — a button above the document's origin — `scroll` runs, says
//! the button is not in view, and a `click` on it is still refused. A `scroll` naming a target
//! already in view changes nothing, one naming no element is refused `not-found`, and one
//! naming a disabled control is accepted. A failure message carries the layout mode and the
//! task or fixture, never an id, a coordinate or what a screen reads.

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use escher_driver::{Cause, Session, VERBS};
use seven_guis::stand::{self, LeanTask};

mod session_common;
use session_common::{act, click, hold, refused, refused_unchanged, scroll};

/// How many driver clicks on Create put a row's centre past the CRUD list's box at the stand's
/// viewport, as measured on the stand.
const CREATES: usize = 12;

/// The row the last of those clicks creates.
const LAST_ROW: &str = "crud-person-14";

/// An id no element of the stand or of a fixture carries.
const NOBODY: &str = "act-scroll-names-no-element";

/// A button in view, and a second one below a spacer taller than the viewport; each click on
/// either rewrites the count.
fn tall_fixture() -> Element {
    let mut presses = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            button { id: "fx-near", onclick: move |_| presses += 1, "Near" }
            p { id: "fx-presses", "{presses}" }
            div { id: "fx-spacer", style: "height: 900px;" }
            button { id: "fx-far", onclick: move |_| presses += 1, "Far" }
            div { id: "fx-tail", style: "height: 900px;" }
        }
    }
}

/// A button positioned above the document's origin, where no scroll offset goes.
fn unreachable_fixture() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; height: 400px;",
            button {
                id: "fx-above",
                style: "position: absolute; left: 20px; top: -300px; width: 120px; height: 40px;",
                "Above"
            }
            button { id: "fx-near", "Near" }
        }
    }
}

fn hold_fixture(fixture: fn() -> Element, incremental: bool) -> Session {
    Session::start("scroll", || {
        Harness::from_vdom(VirtualDom::new(fixture), stand::options(incremental))
    })
    .expect("the label is one of the closed set")
}

/// The left, top, right and bottom of the bounds the snapshot reads for `id`.
fn edges(session: &Session, id: &str) -> (f64, f64, f64, f64) {
    let snapshot = session.harness().doc.snapshot();
    let bounds = snapshot
        .get(id)
        .expect("the element is a snapshot node")
        .bounds;
    (
        bounds.x,
        bounds.y,
        bounds.x + bounds.width,
        bounds.y + bounds.height,
    )
}

fn centre(session: &Session, id: &str) -> (f64, f64) {
    let (left, top, right, bottom) = edges(session, id);
    ((left + right) / 2.0, (top + bottom) / 2.0)
}

fn inside(point: (f64, f64), (left, top, right, bottom): (f64, f64, f64, f64)) -> bool {
    point.0 >= left && point.0 <= right && point.1 >= top && point.1 <= bottom
}

/// The viewport's logical size as the edges of a rect.
fn viewport() -> (f64, f64, f64, f64) {
    (
        0.0,
        0.0,
        f64::from(stand::VIEWPORT_WIDTH),
        f64::from(stand::VIEWPORT_HEIGHT),
    )
}

fn viewport_scroll(session: &Session) -> (f64, f64) {
    let scroll = session.harness().base().viewport_scroll();
    (scroll.x, scroll.y)
}

#[test]
fn a_row_past_the_list_is_refused_scrolled_into_view_and_then_clicked() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        for _ in 0..CREATES {
            assert!(
                act(&mut session, &click("crud-create")).is_some_and(|created| created.settled),
                "{mode}: the driver creates a row"
            );
        }
        // The list has not been scrolled yet, so its bounds are the box it occupies. A box's
        // bounds read after it is scrolled are shifted by its own scroll offset, so the box is
        // read here, once, for every comparison below.
        let list = edges(&session, "crud-list");
        assert!(
            viewport_scroll(&session) == (0.0, 0.0),
            "{mode}: the viewport is not scrolled"
        );
        assert!(
            inside(centre(&session, "crud-person-0"), list),
            "{mode}: the first row's centre lies inside the list's box"
        );
        assert!(
            !inside(centre(&session, LAST_ROW), list)
                && inside(centre(&session, LAST_ROW), viewport()),
            "{mode}: the last row's centre lies outside the list's box and inside the viewport"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &click(LAST_ROW));
        assert!(
            cause == Some(Cause::OffScreen),
            "{mode}: a click on the row past the list's box is refused off-screen"
        );
        assert!(
            unchanged,
            "{mode}: the refused click leaves the snapshot and the focus as they were"
        );

        let Some(scrolled) = act(&mut session, &scroll(LAST_ROW)) else {
            panic!("{mode}: the scroll naming the row runs");
        };
        assert!(
            scrolled.settled && scrolled.busy.is_none(),
            "{mode}: the scroll returns settled"
        );
        assert!(
            scrolled.in_view == Some(true),
            "{mode}: the scroll says the row is in view"
        );
        assert!(
            scrolled.changed().contains(&LAST_ROW)
                && scrolled.added().is_empty()
                && scrolled.removed().is_empty(),
            "{mode}: the scroll's diff names the row among the changed and adds and removes \
             nothing"
        );
        assert!(
            inside(centre(&session, LAST_ROW), list),
            "{mode}: the row's centre lies inside the list's box after the scroll"
        );
        assert!(
            viewport_scroll(&session) == (0.0, 0.0),
            "{mode}: the list was scrolled, not the viewport"
        );
        assert!(
            act(&mut session, &scroll(LAST_ROW)).is_some_and(|again| again.settled
                && again.in_view == Some(true)
                && again.diff.is_empty()),
            "{mode}: a second scroll finds nothing left to move — the first was instant"
        );

        let Some(clicked) = act(&mut session, &click(LAST_ROW)) else {
            panic!("{mode}: the click on the row is accepted after the scroll");
        };
        let delete_enabled = session
            .harness()
            .doc
            .snapshot()
            .get("crud-delete")
            .and_then(|node| node.state.enabled);
        assert!(
            clicked.settled
                && clicked.changed().contains(&"crud-delete")
                && delete_enabled == Some(true),
            "{mode}: the click selected the row — Delete reads enabled in the returned diff"
        );
        assert!(
            act(&mut session, &click("crud-delete"))
                .is_some_and(|deleted| deleted.removed() == [LAST_ROW]),
            "{mode}: the row the click selected is the row it named"
        );

        assert!(
            !inside(centre(&session, "crud-person-0"), list),
            "{mode}: the first row's centre now lies outside the list's box"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &click("crud-person-0"));
        assert!(
            cause == Some(Cause::OffScreen) && unchanged,
            "{mode}: the first row, scrolled out the other way, is refused off-screen"
        );
    }
}

#[test]
fn a_far_button_is_scrolled_into_a_viewport_that_scrolls_and_then_clicked() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: tall fixture");
        let mut session = hold_fixture(tall_fixture, incremental);
        assert!(
            session.harness().doc.unkeyed_actionable().is_empty(),
            "{mode}: every actionable element of the fixture reads an author key"
        );
        assert!(
            !inside(centre(&session, "fx-far"), viewport()),
            "{mode}: the far button lies outside the viewport"
        );
        assert!(
            refused(&mut session, &click("fx-far")) == Some(Cause::OffScreen),
            "{mode}: a click on the far button is refused off-screen"
        );

        let Some(scrolled) = act(&mut session, &scroll("fx-far")) else {
            panic!("{mode}: the scroll naming the far button runs");
        };
        assert!(
            scrolled.settled && scrolled.in_view == Some(true),
            "{mode}: the scroll returns settled and says the button is in view"
        );
        assert!(
            viewport_scroll(&session).1 > 0.0,
            "{mode}: the viewport is scrolled"
        );
        let far = edges(&session, "fx-far");
        assert!(
            inside((far.0, far.1), viewport()) && inside((far.2, far.3), viewport()),
            "{mode}: the far button's bounds lie inside the viewport after the scroll"
        );
        assert!(
            act(&mut session, &scroll("fx-far")).is_some_and(|again| again.settled
                && again.in_view == Some(true)
                && again.diff.is_empty()),
            "{mode}: a second scroll finds nothing left to move — the first was instant"
        );

        assert!(
            act(&mut session, &click("fx-far")).is_some_and(|clicked| clicked.settled
                && clicked.changed().contains(&"fx-presses")),
            "{mode}: the click on the far button lands under the scrolled viewport — the text \
             its handler rewrites is in the returned diff"
        );
        assert!(
            refused(&mut session, &click("fx-near")) == Some(Cause::OffScreen),
            "{mode}: the near button, scrolled out of the viewport, is refused in its turn"
        );
    }
}

#[test]
fn a_target_no_scroll_can_reach_is_told_in_the_result() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: unreachable fixture");
        let mut session = hold_fixture(unreachable_fixture, incremental);
        assert!(
            session.harness().doc.unkeyed_actionable().is_empty(),
            "{mode}: every actionable element of the fixture reads an author key"
        );
        let (_, _, _, bottom) = edges(&session, "fx-above");
        assert!(
            bottom <= 0.0,
            "{mode}: the button lies above the viewport, above the document's origin"
        );
        assert!(
            refused(&mut session, &click("fx-above")) == Some(Cause::OffScreen),
            "{mode}: a click on the button is refused off-screen"
        );

        let Some(scrolled) = act(&mut session, &scroll("fx-above")) else {
            panic!("{mode}: the scroll naming the button runs");
        };
        assert!(scrolled.settled, "{mode}: the scroll returns settled");
        assert!(
            scrolled.in_view == Some(false),
            "{mode}: the scroll says the button is not in view"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-above"));
        assert!(
            cause == Some(Cause::OffScreen) && unchanged,
            "{mode}: a click on the button is still refused off-screen"
        );
    }
}

#[test]
fn a_scroll_needs_a_target_on_the_screen_and_nothing_else_of_it() {
    assert_eq!(VERBS.len(), 6);
    let sixth = VERBS[5].name;
    assert!(
        Cause::OffScreen.remedy().contains(sixth),
        "the off-screen remedy names the verb table's sixth verb"
    );
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);

        assert!(
            act(&mut session, &scroll("crud-create")).is_some_and(|scrolled| scrolled.settled
                && scrolled.in_view == Some(true)
                && scrolled.diff.is_empty()),
            "{mode}: a scroll naming a target already in view says so and changes nothing"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &scroll(NOBODY));
        assert!(
            cause == Some(Cause::NotFound) && unchanged,
            "{mode}: a scroll naming no element is refused not-found, the instance unchanged"
        );

        let disabled = session
            .harness()
            .doc
            .snapshot()
            .get("crud-delete")
            .and_then(|node| node.state.enabled);
        assert!(
            disabled == Some(false),
            "{mode}: Delete reads not enabled at boot"
        );
        assert!(
            act(&mut session, &scroll("crud-delete"))
                .is_some_and(|scrolled| scrolled.settled && scrolled.in_view == Some(true)),
            "{mode}: a scroll naming a disabled control is accepted"
        );
    }
}
