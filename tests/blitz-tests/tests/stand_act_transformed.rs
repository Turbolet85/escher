//! A driver action reaches a control a transform moves, where the control shows: the snapshot's
//! `bounds` read the control's client rect, which counts the transforms on its containing-block
//! chain (CSSOM View, <https://drafts.csswg.org/cssom-view/#dom-element-getboundingclientrect>),
//! and the action lands at the centre of those bounds, where the hit answers the control. The
//! lean tasks hold no transform, so the case is proven on a fixture held in a session, booted
//! with the stand's options, in both layout modes. Before the fix (measured 2026-10-14) the
//! bounds read the control where layout puts it, so a click on a control moved off that place
//! landed beside it and was refused `covered`. A control inside a scrolling box that itself lies
//! under a transform, a node body in a panned and zoomed world, is reached the same way: a `click`
//! is accepted and lands, a lint reads neither `off-screen` nor `truncated` there and a shot's crop
//! lies inside the frame. Before the fix (measured 2026-10-31) that `click` was refused
//! `off-screen`. A failure message carries the layout mode and the fixture, never an id, a
//! coordinate or what a screen reads.

use dioxus::prelude::*;
use escher_driver::{Defect, Session};
use seven_guis::stand;

mod fixtures;
mod session_common;
use session_common::{act, click, lint, linted, screenshot_of, shot};

/// How far the button's transform moves it across.
const MOVED_X: f64 = 120.0;

/// A button moved 120px across by a transform, and the count of its clicks.
fn transformed_fixture() -> Element {
    let mut presses = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 600px; height: 400px;",
            button {
                id: "fx-moved",
                style: "position: absolute; left: 20px; top: 20px; width: 120px; height: 40px; transform: translate(120px, 0);",
                onclick: move |_| presses += 1,
                "Moved"
            }
            p {
                id: "fx-presses",
                style: "position: absolute; left: 20px; top: 300px; margin: 0;",
                "{presses}"
            }
        }
    }
}

/// The left, top, width and height of the bounds the snapshot reads for `id`.
fn bounds(session: &Session, id: &str) -> (f64, f64, f64, f64) {
    let snapshot = session.harness().doc.snapshot();
    let bounds = snapshot
        .get(id)
        .expect("the fixture's element is a snapshot node")
        .bounds;
    (bounds.x, bounds.y, bounds.width, bounds.height)
}

/// Asserts the fixture's condition: the button is transformed, and where layout puts it lies
/// beside where it shows.
#[track_caller]
fn assert_the_button_is_moved(session: &Session, mode: &str) {
    let harness = session.harness();
    let button = harness.node("#fx-moved");
    assert!(
        harness
            .base()
            .get_node(button)
            .is_some_and(|node| node.transform().is_some()),
        "{mode}: the button computes a transform"
    );
    assert!(
        harness.doc.unkeyed_actionable().is_empty(),
        "{mode}: every actionable element of the fixture reads an author key"
    );
    let laid_out = harness.layout_rect("#fx-moved");
    let (root_x, root_y, _, _) = bounds(session, "fx-root");
    assert!(
        (f64::from(laid_out.x), f64::from(laid_out.y)) == (root_x + 20.0, root_y + 20.0),
        "{mode}: layout puts the button at its stated offset"
    );
}

#[test]
fn a_click_on_a_translated_button_is_accepted_and_lands() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: transformed fixture");
        let mut session = fixtures::hold("transformed", transformed_fixture, incremental);
        assert_the_button_is_moved(&session, &mode);
        assert!(
            session.harness().text_content("#fx-presses") == "0",
            "{mode}: the button is not pressed first"
        );

        let clicked = act(&mut session, &click("fx-moved"));
        assert!(
            clicked
                .as_ref()
                .is_some_and(|clicked| clicked.settled && clicked.changed().contains(&"fx-presses")),
            "{mode}: the click is accepted and its diff names the count it changes"
        );
        assert!(
            session.harness().text_content("#fx-presses") == "1",
            "{mode}: the click lands on the button"
        );
    }
}

/// A node body scrolling what it holds, in a world moved and scaled by 2 under an untransformed
/// clip, and the count of its button's clicks.
fn zoomed_world_fixture() -> Element {
    let mut presses = use_signal(|| 0u32);
    rsx! {
        div {
            id: "fx-viewport",
            style: "position: absolute; left: 50px; top: 30px; width: 500px; height: 400px; overflow: hidden;",
            div {
                id: "fx-world",
                style: "position: absolute; left: 0; top: 0; width: 1000px; height: 1000px; transform-origin: 0 0; transform: translate(200px, 100px) scale(2);",
                div {
                    id: "fx-body",
                    style: "position: absolute; left: 20px; top: 10px; width: 200px; height: 100px; overflow: auto;",
                    button {
                        id: "fx-inner",
                        style: "display: block; width: 100px; height: 40px;",
                        onclick: move |_| presses += 1,
                        "In"
                    }
                    div { id: "fx-spacer", style: "height: 400px;" }
                }
            }
        }
        p {
            id: "fx-presses",
            style: "position: absolute; left: 600px; top: 500px; margin: 0;",
            "{presses}"
        }
    }
}

#[test]
fn a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: zoomed world");
        let mut session = fixtures::hold("transformed", zoomed_world_fixture, incremental);

        let clicked = act(&mut session, &click("fx-inner"));
        assert!(
            clicked
                .as_ref()
                .is_some_and(|clicked| clicked.settled && clicked.changed().contains(&"fx-presses")),
            "{mode}: the click is accepted and its diff names the count it changes"
        );
        assert!(
            session.harness().text_content("#fx-presses") == "1",
            "{mode}: the click lands on the button"
        );

        let findings = linted(&mut session, &lint()).expect("the lint is answered");
        assert!(
            !findings.iter().any(|finding| finding.id == "fx-inner"
                && matches!(finding.defect, Defect::OffScreen | Defect::Truncated)),
            "{mode}: the lint reads the button neither off-screen nor truncated"
        );

        let cropped = shot(&mut session, &screenshot_of("fx-inner"))
            .unwrap_or_else(|| panic!("{mode}: the shot of the button is accepted"));
        let (_, _, width, height) = bounds(&session, "fx-inner");
        assert!(
            cropped.width > 0
                && cropped.height > 0
                && f64::from(cropped.width) <= width.ceil()
                && f64::from(cropped.height) <= height.ceil()
                && cropped.width <= stand::VIEWPORT_WIDTH
                && cropped.height <= stand::VIEWPORT_HEIGHT,
            "{mode}: the crop holds the button's visible part, inside the frame"
        );
    }
}

#[test]
fn the_snapshot_bounds_of_a_translated_button_read_where_it_shows() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: transformed fixture");
        let session = fixtures::hold("transformed", transformed_fixture, incremental);
        assert_the_button_is_moved(&session, &mode);
        let (root_x, root_y, _, _) = bounds(&session, "fx-root");
        let laid_out = session.harness().layout_rect("#fx-moved");
        let size = (f64::from(laid_out.width), f64::from(laid_out.height));
        assert!(
            bounds(&session, "fx-moved")
                == (root_x + 20.0 + MOVED_X, root_y + 20.0, size.0, size.1),
            "{mode}: the bounds read the button's box moved by its transform"
        );
    }
}
