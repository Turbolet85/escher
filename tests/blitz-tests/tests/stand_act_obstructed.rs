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
//! leaves the snapshot and both focus readings as they were. A failure message carries the
//! layout mode and the fixture, never an id, a coordinate or what a screen reads.

use blitz_test_harness::Harness;
use blitz_traits::node_id::NodeId;
use dioxus::prelude::*;
use escher_driver::{Cause, Session};
use seven_guis::stand;

mod session_common;
use session_common::{act, click, refused_unchanged, type_into};

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
