//! A driver action aimed at a target that is covered or out of view is refused with that cause,
//! and where two causes hold the refusal names the earlier in the driver's order. The lean
//! tasks hold neither case at boot, so both are proven on fixtures in this file, booted with
//! the stand's options, in both layout modes. *Covered:* a button under a positioned overlay
//! with a `z-index` above it is refused `covered`; a button under an overlay that is
//! transparent to hits is clicked; once the driver dismisses the overlay the first button is
//! clicked too. *Off-screen:* a button below a spacer taller than the viewport is refused
//! `off-screen` for a `click` and for a `type`, a button inside the viewport is clicked, and a
//! button scrolled out of a scrolling box that sits inside the viewport is refused
//! `off-screen`, never `covered`. *Order:* disabled and covered reads `disabled`, disabled and
//! out of view reads `disabled`, out of view and covered reads `off-screen`. Every refused call
//! leaves the snapshot and both focus readings as they were. *A measured limit, pinned as it
//! reads:* the engine's hit reaches content scrolled out of a scrolling box, so a button lying
//! before such a box, where its scrolled-out rows extend, is refused `covered` though nothing
//! shows over it; a button after the box is not, and no control of the stand's CRUD is. A
//! failure message carries the layout mode and the fixture, never an id, a coordinate or what a
//! screen reads.

use blitz_test_harness::Harness;
use blitz_traits::node_id::NodeId;
use dioxus::prelude::*;
use escher_driver::{Cause, Session};
use seven_guis::stand::{self, LeanTask};

mod common;
mod session_common;
use session_common::{act, click, hold as hold_task, refused_unchanged, scroll, type_into};

/// Two buttons under one opaque overlay — the second of them disabled — a third under an
/// overlay transparent to hits, and a control that removes the opaque overlay.
fn covered_fixture() -> Element {
    let mut presses = use_signal(|| 0u32);
    let mut dismissed = use_signal(|| false);
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 600px; height: 400px;",
            button {
                id: "fx-under",
                style: "position: absolute; left: 20px; top: 20px; width: 120px; height: 40px;",
                onclick: move |_| presses += 1,
                "Under"
            }
            button {
                id: "fx-under-off",
                style: "position: absolute; left: 20px; top: 80px; width: 120px; height: 40px;",
                disabled: true,
                "Under and off"
            }
            if !dismissed() {
                div {
                    id: "fx-cover",
                    style: "position: absolute; left: 0; top: 0; width: 200px; height: 140px; z-index: 5; background: #cccccc;",
                }
            }
            button {
                id: "fx-through",
                style: "position: absolute; left: 300px; top: 20px; width: 120px; height: 40px;",
                onclick: move |_| presses += 1,
                "Through"
            }
            div {
                id: "fx-glass",
                style: "position: absolute; left: 280px; top: 0; width: 200px; height: 140px; z-index: 5; pointer-events: none;",
            }
            button {
                id: "fx-dismiss",
                style: "position: absolute; left: 20px; top: 200px; width: 120px; height: 40px;",
                onclick: move |_| dismissed.set(true),
                "Dismiss"
            }
            p {
                id: "fx-presses",
                style: "position: absolute; left: 20px; top: 300px; margin: 0;",
                "{presses}"
            }
        }
    }
}

/// A button in view, a scrolling box whose own button is scrolled out of it, and below a
/// spacer taller than the viewport three more buttons: one plain, one disabled, one under an
/// overlay.
fn tall_fixture() -> Element {
    let mut presses = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            button { id: "fx-near", onclick: move |_| presses += 1, "Near" }
            p { id: "fx-presses", "{presses}" }
            div { id: "fx-box", style: "height: 100px; overflow-y: auto;",
                div { id: "fx-box-spacer", style: "height: 300px;" }
                button { id: "fx-boxed", onclick: move |_| presses += 1, "Boxed" }
            }
            div { id: "fx-spacer", style: "height: 900px;" }
            button { id: "fx-far", onclick: move |_| presses += 1, "Far" }
            button { id: "fx-far-off", disabled: true, "Far and off" }
            div { id: "fx-far-wrap", style: "position: relative; height: 60px;",
                button {
                    id: "fx-far-under",
                    style: "position: absolute; left: 20px; top: 10px; width: 120px; height: 40px;",
                    "Far and under"
                }
                div {
                    id: "fx-far-cover",
                    style: "position: absolute; left: 0; top: 0; width: 200px; height: 60px; z-index: 5; background: #cccccc;",
                }
            }
        }
    }
}

/// A scrolling box a hundred high holding ten rows forty high, a button before it and a
/// button after it; a click on a button rewrites one count and a click on a row the other.
fn boxed_rows_fixture() -> Element {
    let mut presses = use_signal(|| 0u32);
    let mut rows = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            button { id: "fx-before", onclick: move |_| presses += 1, "Before" }
            div { id: "fx-box", style: "height: 100px; overflow-y: auto;",
                for index in 0..10 {
                    div {
                        key: "{index}",
                        id: "fx-row-{index}",
                        style: "height: 40px;",
                        onclick: move |_| rows += 1,
                        "Row {index}"
                    }
                }
            }
            button { id: "fx-after", onclick: move |_| presses += 1, "After" }
            p { id: "fx-presses", "{presses}" }
            p { id: "fx-rows", "{rows}" }
        }
    }
}

fn hold(fixture: fn() -> Element, incremental: bool) -> Session {
    Session::start("obstructed", || {
        Harness::from_vdom(VirtualDom::new(fixture), stand::options(incremental))
    })
    .expect("the label is one of the closed set")
}

/// The left, top, right and bottom of the bounds the snapshot reads for `id`.
fn edges(session: &Session, id: &str) -> (f64, f64, f64, f64) {
    let snapshot = session.harness().doc.snapshot();
    let bounds = snapshot
        .get(id)
        .expect("the fixture's element is a snapshot node")
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

fn has_area(session: &Session, id: &str) -> bool {
    let (left, top, right, bottom) = edges(session, id);
    right > left && bottom > top
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

/// The node the harness's own hit answers at the centre of `id`'s bounds, on a viewport that
/// is not scrolled.
fn hit_at_centre(session: &Session, id: &str) -> Option<NodeId> {
    let (x, y) = centre(session, id);
    session
        .harness()
        .hit(x as f32, y as f32)
        .map(|hit| hit.node_id)
}

/// Whether `node` is `ancestor` or lies inside it.
fn within(session: &Session, node: Option<NodeId>, ancestor: NodeId) -> bool {
    let doc = session.harness().base();
    let mut at = node;
    while let Some(node) = at {
        if node == ancestor {
            return true;
        }
        at = doc.get_node(node).and_then(|node| node.parent);
    }
    false
}

/// Whether the harness's own hit at the centre of `id`'s bounds answers the element `id` names
/// or one inside it, on a viewport that is not scrolled.
fn hit_answers(session: &Session, id: &str) -> bool {
    let target = session.harness().node(&format!("#{id}"));
    within(session, hit_at_centre(session, id), target)
}

fn enabled(session: &Session, id: &str) -> Option<bool> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.enabled)
}

#[track_caller]
fn assert_fixture_is_keyed_and_unscrolled(session: &Session, mode: &str) {
    assert!(
        session.harness().doc.unkeyed_actionable().is_empty(),
        "{mode}: every actionable element of the fixture reads an author key"
    );
    let scroll = session.harness().base().viewport_scroll();
    assert!(
        scroll.x == 0.0 && scroll.y == 0.0,
        "{mode}: the viewport is not scrolled"
    );
}

#[test]
fn a_covered_target_is_refused_until_its_cover_is_gone_or_transparent() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: covered fixture");
        let mut session = hold(covered_fixture, incremental);
        assert_fixture_is_keyed_and_unscrolled(&session, &mode);
        for id in ["fx-under", "fx-cover", "fx-through", "fx-glass"] {
            assert!(
                has_area(&session, id),
                "{mode}: each target and each cover reads non-zero bounds"
            );
        }
        let cover = session.harness().query("#fx-cover");
        assert!(
            cover.is_some() && hit_at_centre(&session, "fx-under") == cover,
            "{mode}: the harness's own hit at the first button's centre answers its cover"
        );
        let glass = session.harness().query("#fx-glass");
        assert!(
            inside(centre(&session, "fx-through"), edges(&session, "fx-glass"))
                && hit_at_centre(&session, "fx-through") != glass,
            "{mode}: the second overlay lies over its button and is not what a hit answers"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-under"));
        assert!(
            cause == Some(Cause::Covered),
            "{mode}: a click on the covered button is refused covered"
        );
        assert!(
            unchanged,
            "{mode}: the refused click leaves the snapshot and the focus as they were"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &type_into("fx-under", "zq"));
        assert!(
            cause == Some(Cause::Covered) && unchanged,
            "{mode}: a type on the covered button is refused covered, the instance unchanged"
        );

        assert!(
            act(&mut session, &click("fx-through"))
                .is_some_and(|clicked| clicked.settled && clicked.changed() == ["fx-presses"]),
            "{mode}: a click under an overlay transparent to hits is accepted and lands"
        );

        assert!(
            act(&mut session, &click("fx-dismiss"))
                .is_some_and(|dismissed| dismissed.settled && dismissed.removed() == ["fx-cover"]),
            "{mode}: the driver's click on the dismiss control removes the cover"
        );
        assert!(
            act(&mut session, &click("fx-under"))
                .is_some_and(|clicked| clicked.settled && clicked.changed() == ["fx-presses"]),
            "{mode}: with the cover dismissed the click is accepted and lands"
        );
    }
}

#[test]
fn a_target_out_of_view_is_refused_off_screen() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: tall fixture");
        let mut session = hold(tall_fixture, incremental);
        assert_fixture_is_keyed_and_unscrolled(&session, &mode);
        for id in ["fx-near", "fx-far", "fx-boxed", "fx-box"] {
            assert!(
                has_area(&session, id),
                "{mode}: each target reads non-zero bounds"
            );
        }
        let (_, far_top, _, _) = edges(&session, "fx-far");
        assert!(
            far_top >= f64::from(stand::VIEWPORT_HEIGHT),
            "{mode}: the far button lies below the viewport"
        );
        assert!(
            inside(centre(&session, "fx-near"), viewport()),
            "{mode}: the near button lies inside the viewport"
        );
        let boxed = edges(&session, "fx-boxed");
        assert!(
            inside((boxed.0, boxed.1), viewport()) && inside((boxed.2, boxed.3), viewport()),
            "{mode}: the boxed button's bounds lie inside the viewport"
        );
        assert!(
            !inside(centre(&session, "fx-boxed"), edges(&session, "fx-box")),
            "{mode}: the boxed button's centre lies outside its scrolling box"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-far"));
        assert!(
            cause == Some(Cause::OffScreen),
            "{mode}: a click on the far button is refused off-screen"
        );
        assert!(
            unchanged,
            "{mode}: the refused click leaves the snapshot and the focus as they were"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &type_into("fx-far", "zq"));
        assert!(
            cause == Some(Cause::OffScreen) && unchanged,
            "{mode}: a type on the far button is refused off-screen, the instance unchanged"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-boxed"));
        assert!(
            cause == Some(Cause::OffScreen),
            "{mode}: a click on the button scrolled out of its box is refused off-screen, \
             never covered"
        );
        assert!(
            unchanged,
            "{mode}: that refused click leaves the snapshot and the focus as they were"
        );

        assert!(
            act(&mut session, &click("fx-near"))
                .is_some_and(|clicked| clicked.settled && clicked.changed() == ["fx-presses"]),
            "{mode}: a click on the button inside the viewport is accepted and lands"
        );
    }
}

/// The engine's hit walk passes a point outside a scrolling box to the box's scrolled-out
/// content. The readings are pinned as measured on the engine as built (2026-10-10): this
/// check turns red by design when the walk is clipped at a scrolling box, and is then restated
/// — the button before the box is clicked.
#[test]
fn a_button_where_scrolled_out_rows_extend_reads_covered_only_before_its_box() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: boxed rows fixture");
        let mut session = hold(boxed_rows_fixture, incremental);
        assert_fixture_is_keyed_and_unscrolled(&session, &mode);
        let scrolling_box = edges(&session, "fx-box");
        let box_node = session.harness().node("#fx-box");
        for id in ["fx-before", "fx-box", "fx-after"] {
            assert!(
                has_area(&session, id),
                "{mode}: each target reads non-zero bounds"
            );
        }

        // The box is not scrolled: its rows overflow below it, across the button after it.
        let after = centre(&session, "fx-after");
        assert!(
            !inside(after, scrolling_box) && inside(after, edges(&session, "fx-row-2")),
            "{mode}: a row scrolled out below the box extends across the later button's centre"
        );
        assert!(
            hit_answers(&session, "fx-after"),
            "{mode}: the hit at the later button's centre answers the button"
        );
        assert!(
            act(&mut session, &click("fx-after"))
                .is_some_and(|clicked| clicked.settled && clicked.changed() == ["fx-presses"]),
            "{mode}: a click on the later button is accepted and lands on it"
        );

        // The box scrolled to its end: its rows overflow above it, across the button before
        // it. The rows' bounds are read true; the box's own are not, so it is not read here.
        assert!(
            act(&mut session, &scroll("fx-row-9"))
                .is_some_and(|scrolled| scrolled.settled && scrolled.in_view == Some(true)),
            "{mode}: the scroll brings the last row into view"
        );
        let before = centre(&session, "fx-before");
        assert!(
            !inside(before, scrolling_box) && inside(before, edges(&session, "fx-row-7")),
            "{mode}: a row scrolled out above the box extends across the earlier button's centre"
        );
        assert!(
            inside(before, viewport()),
            "{mode}: the earlier button lies inside the viewport"
        );
        let hit = hit_at_centre(&session, "fx-before");
        assert!(
            !hit_answers(&session, "fx-before") && within(&session, hit, box_node),
            "{mode}: the hit at the earlier button's centre answers content of the box"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-before"));
        assert!(
            cause == Some(Cause::Covered),
            "{mode}: a click on the earlier button is refused covered"
        );
        assert!(
            unchanged,
            "{mode}: the refused click leaves the snapshot and the focus as they were"
        );
    }
}

/// On the stand's CRUD no control reads `covered` this way, in any state measured
/// (2026-10-10) — one a scrolled-out row extends across included.
#[test]
fn no_stand_control_is_hit_through_by_a_row_scrolled_out_of_the_list() {
    let controls = common::controls(LeanTask::Crud);
    assert_eq!(controls.len(), 7);
    for incremental in [false, true] {
        for creates in [12, 14] {
            let mode = format!("incremental={incremental}: Crud, {creates} rows created");
            let (mut session, _ticks) = hold_task(LeanTask::Crud, incremental);
            for _ in 0..creates {
                assert!(
                    act(&mut session, &click("crud-create")).is_some_and(|created| created.settled),
                    "{mode}: the driver creates a row"
                );
            }
            let answered = |session: &Session| {
                controls
                    .iter()
                    .filter(|(id, _)| hit_answers(session, id))
                    .count()
            };
            assert!(
                answered(&session) == controls.len(),
                "{mode}: with the list not scrolled the hit at each control's centre answers \
                 the control ({} of {})",
                answered(&session),
                controls.len()
            );

            let last_row = format!("crud-person-{}", creates + 2);
            assert!(
                act(&mut session, &scroll(&last_row))
                    .is_some_and(|scrolled| scrolled.settled && scrolled.in_view == Some(true)),
                "{mode}: the scroll brings the last row into view"
            );
            assert!(
                answered(&session) == controls.len(),
                "{mode}: with the list scrolled the hit at each control's centre answers the \
                 control ({} of {})",
                answered(&session),
                controls.len()
            );

            if creates == 14 {
                assert!(
                    inside(
                        centre(&session, "crud-filter"),
                        edges(&session, "crud-person-1")
                    ),
                    "{mode}: a row scrolled out above the list extends across the filter's centre"
                );
                assert!(
                    act(&mut session, &click("crud-filter")).is_some_and(|clicked| clicked.settled),
                    "{mode}: a click on the filter is accepted"
                );
            }
        }
    }
}

#[test]
fn a_target_holding_two_causes_reads_the_earlier_one() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: covered fixture");
        let mut session = hold(covered_fixture, incremental);
        let cover = session.harness().query("#fx-cover");
        assert!(
            enabled(&session, "fx-under-off") == Some(false)
                && cover.is_some()
                && hit_at_centre(&session, "fx-under-off") == cover,
            "{mode}: one button reads not enabled and is covered"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-under-off"));
        assert!(
            cause == Some(Cause::Disabled) && unchanged,
            "{mode}: disabled and covered reads disabled"
        );

        let mode = format!("incremental={incremental}: tall fixture");
        let mut session = hold(tall_fixture, incremental);
        let below =
            |session: &Session, id: &str| edges(session, id).1 >= f64::from(stand::VIEWPORT_HEIGHT);
        assert!(
            enabled(&session, "fx-far-off") == Some(false) && below(&session, "fx-far-off"),
            "{mode}: one button reads not enabled and lies below the viewport"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-far-off"));
        assert!(
            cause == Some(Cause::Disabled) && unchanged,
            "{mode}: disabled and out of view reads disabled"
        );

        let far_cover = session.harness().query("#fx-far-cover");
        assert!(
            below(&session, "fx-far-under")
                && far_cover.is_some()
                && hit_at_centre(&session, "fx-far-under") == far_cover,
            "{mode}: one button lies below the viewport and is covered"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-far-under"));
        assert!(
            cause == Some(Cause::OffScreen) && unchanged,
            "{mode}: out of view and covered reads off-screen"
        );
    }
}
