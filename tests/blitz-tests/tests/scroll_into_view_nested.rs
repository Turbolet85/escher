//! `BaseDocument::scroll_into_view` scrolls every scrolling box that holds its target,
//! innermost first, and then the viewport — the CSSOM View "scroll an element into view" steps
//! — and `BaseDocument::visible_region` tells the part of the viewport an element can be seen
//! through. On parsed pages at a fixed 800 × 600 viewport, in both layout modes: a box in view
//! is scrolled and the viewport is not; a box below the viewport and the viewport both move;
//! two nested boxes both move; a target already in view moves nothing; an `overflow: hidden`
//! box is scrolled and an `overflow: visible` one is not; under a smooth scroll the nested box
//! is written at once and the viewport travels; and a target above the document's origin stays
//! out of view. A page whose only scroller is the viewport is `fragment_navigation`'s to check.
//! A failure message carries the layout mode and the page, never a coordinate.

use blitz_dom::{DocumentConfig, ScrollBehavior, ScrollLogicalPosition};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::node_id::NodeId;
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

const VIEWPORT_HEIGHT: f64 = 600.0;

/// A 200px-tall scrolling box in view, its target 500px down inside it.
const BOX_IN_VIEW: &str = r#"<html><body style="margin:0">
    <div id="box" style="width:300px; height:200px; overflow:auto">
        <div style="height:500px"></div>
        <div id="target" style="width:100px; height:40px"></div>
        <div style="height:300px"></div>
    </div>
</body></html>"#;

/// The same box 900px down the page, below the viewport.
const BOX_BELOW_THE_VIEWPORT: &str = r#"<html><body style="margin:0">
    <div style="height:900px"></div>
    <div id="box" style="width:300px; height:200px; overflow:auto">
        <div style="height:500px"></div>
        <div id="target" style="width:100px; height:40px"></div>
        <div style="height:300px"></div>
    </div>
    <div style="height:900px"></div>
</body></html>"#;

/// A scrolling box inside a scrolling box, the target far down the inner one.
const TWO_BOXES: &str = r#"<html><body style="margin:0">
    <div id="outer" style="width:400px; height:300px; overflow:auto">
        <div style="height:600px"></div>
        <div id="inner" style="width:300px; height:150px; overflow:auto">
            <div style="height:400px"></div>
            <div id="target" style="width:100px; height:40px"></div>
            <div style="height:200px"></div>
        </div>
        <div style="height:600px"></div>
    </div>
</body></html>"#;

/// A target at the top of a scrolling box that has room to scroll.
const ALREADY_IN_VIEW: &str = r#"<html><body style="margin:0">
    <div id="box" style="width:300px; height:200px; overflow:auto">
        <div id="target" style="width:100px; height:40px"></div>
        <div style="height:800px"></div>
    </div>
</body></html>"#;

/// An `overflow: hidden` box holding an `overflow: visible` one that its content overflows.
const HIDDEN_AND_VISIBLE: &str = r#"<html><body style="margin:0">
    <div id="hidden" style="width:300px; height:200px; overflow:hidden">
        <div id="visible" style="height:100px">
            <div style="height:300px"></div>
            <div id="target" style="width:100px; height:40px"></div>
        </div>
    </div>
</body></html>"#;

/// A target positioned above the document's origin, where no scroll offset goes.
const ABOVE_THE_ORIGIN: &str = r#"<html><body style="margin:0">
    <div id="target" style="position:absolute; top:-300px; left:0; width:100px; height:40px"></div>
    <div style="height:2000px"></div>
</body></html>"#;

fn page(html: &str, incremental: bool) -> HtmlDocument {
    let mut doc = HtmlDocument::from_html(
        html,
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            ..Default::default()
        },
    );
    doc.set_incremental_layout(incremental);
    doc.resolve(0.0);
    doc
}

fn node(doc: &HtmlDocument, selector: &str) -> NodeId {
    doc.query_selector(selector)
        .expect("the selector parses")
        .expect("the page holds the element")
}

/// The vertical scroll offset of the box `selector` names.
fn offset(doc: &HtmlDocument, selector: &str) -> f64 {
    doc.get_node(node(doc, selector))
        .expect("a queried node resolves")
        .scroll_offset()
        .y
}

/// The top and bottom of the target's client rect, relative to the viewport.
fn target_edges(doc: &HtmlDocument) -> (f64, f64) {
    let rect = doc
        .get_client_bounding_rect(node(doc, "#target"))
        .expect("the target has a client rect");
    (rect.y, rect.y + rect.height)
}

/// Whether the reader's answer for the target holds the centre of the target's client rect.
fn reader_holds_the_target(doc: &HtmlDocument) -> bool {
    let target = node(doc, "#target");
    let rect = doc
        .get_client_bounding_rect(target)
        .expect("the target has a client rect");
    let (x, y) = (rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
    doc.visible_region(target).is_some_and(|seen| {
        x >= seen.x && x <= seen.x + seen.width && y >= seen.y && y <= seen.y + seen.height
    })
}

fn scroll_to_target(doc: &mut HtmlDocument, behavior: ScrollBehavior) {
    let target = node(doc, "#target");
    doc.scroll_into_view(
        target,
        behavior,
        ScrollLogicalPosition::Nearest,
        ScrollLogicalPosition::Nearest,
    );
}

#[test]
fn a_box_in_view_is_scrolled_and_the_viewport_is_not() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: box in view");
        let mut doc = page(BOX_IN_VIEW, incremental);
        let (top, bottom) = target_edges(&doc);
        assert!(
            top >= 200.0 && bottom <= VIEWPORT_HEIGHT,
            "{mode}: the target starts below the box's scrollport and inside the viewport"
        );
        assert!(
            offset(&doc, "#box") == 0.0 && doc.viewport_scroll().y == 0.0,
            "{mode}: nothing is scrolled at first"
        );
        assert!(
            !reader_holds_the_target(&doc),
            "{mode}: the reader excludes the target before the scroll"
        );

        scroll_to_target(&mut doc, ScrollBehavior::Instant);

        assert!(offset(&doc, "#box") > 0.0, "{mode}: the box is scrolled");
        assert!(
            doc.viewport_scroll().y == 0.0,
            "{mode}: the viewport is not scrolled"
        );
        let (top, bottom) = target_edges(&doc);
        assert!(
            top >= 0.0 && bottom <= 200.0,
            "{mode}: the target lies inside the box's scrollport"
        );
        assert!(
            reader_holds_the_target(&doc),
            "{mode}: the reader holds the target after the scroll"
        );
    }
}

#[test]
fn a_box_below_the_viewport_and_the_viewport_both_move() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: box below the viewport");
        let mut doc = page(BOX_BELOW_THE_VIEWPORT, incremental);
        let (top, _) = target_edges(&doc);
        assert!(
            top >= VIEWPORT_HEIGHT,
            "{mode}: the target starts below the viewport"
        );
        assert!(
            !reader_holds_the_target(&doc),
            "{mode}: the reader excludes the target before the scroll"
        );

        scroll_to_target(&mut doc, ScrollBehavior::Instant);

        assert!(offset(&doc, "#box") > 0.0, "{mode}: the box is scrolled");
        assert!(
            doc.viewport_scroll().y > 0.0,
            "{mode}: the viewport is scrolled"
        );
        let (top, bottom) = target_edges(&doc);
        assert!(
            top >= 0.0 && bottom <= VIEWPORT_HEIGHT,
            "{mode}: the target lies inside the viewport"
        );
        assert!(
            reader_holds_the_target(&doc),
            "{mode}: the reader holds the target after the scroll"
        );
    }
}

#[test]
fn two_nested_boxes_both_move() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: two boxes");
        let mut doc = page(TWO_BOXES, incremental);
        assert!(
            offset(&doc, "#outer") == 0.0 && offset(&doc, "#inner") == 0.0,
            "{mode}: nothing is scrolled at first"
        );
        assert!(
            !reader_holds_the_target(&doc),
            "{mode}: the reader excludes the target before the scroll"
        );

        scroll_to_target(&mut doc, ScrollBehavior::Instant);

        assert!(
            offset(&doc, "#inner") > 0.0,
            "{mode}: the inner box is scrolled"
        );
        assert!(
            offset(&doc, "#outer") > 0.0,
            "{mode}: the outer box is scrolled"
        );
        assert!(
            doc.viewport_scroll().y == 0.0,
            "{mode}: the viewport is not scrolled"
        );
        // The outer box sits at the top of the page and is 300px tall, so its scrollport is
        // the viewport's first 300px; the inner one is narrower still.
        let (top, bottom) = target_edges(&doc);
        assert!(
            top >= 0.0 && bottom <= 300.0,
            "{mode}: the target lies inside the outer box's scrollport"
        );
        assert!(
            reader_holds_the_target(&doc),
            "{mode}: the reader, which narrows to both scrollports, holds the target"
        );
    }
}

#[test]
fn a_target_already_in_view_moves_nothing() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: already in view");
        let mut doc = page(ALREADY_IN_VIEW, incremental);
        let scrollable = doc
            .get_node(node(&doc, "#box"))
            .is_some_and(|scroller| scroller.final_layout().scroll_height() > 0.0);
        assert!(scrollable, "{mode}: the box has room to scroll");
        assert!(
            reader_holds_the_target(&doc),
            "{mode}: the reader holds the target before the scroll"
        );

        scroll_to_target(&mut doc, ScrollBehavior::Instant);

        assert!(
            offset(&doc, "#box") == 0.0 && doc.viewport_scroll().y == 0.0,
            "{mode}: no offset moves"
        );
    }
}

#[test]
fn a_hidden_box_is_scrolled_and_a_visible_one_is_not() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: hidden and visible");
        let mut doc = page(HIDDEN_AND_VISIBLE, incremental);
        let (top, _) = target_edges(&doc);
        assert!(
            top >= 200.0,
            "{mode}: the target starts below the hidden box's scrollport"
        );
        assert!(
            !reader_holds_the_target(&doc),
            "{mode}: the reader excludes the target before the scroll"
        );

        scroll_to_target(&mut doc, ScrollBehavior::Instant);

        assert!(
            offset(&doc, "#hidden") > 0.0,
            "{mode}: the hidden box is scrolled"
        );
        assert!(
            offset(&doc, "#visible") == 0.0,
            "{mode}: the visible box is not scrolled"
        );
        let (top, bottom) = target_edges(&doc);
        assert!(
            top >= 0.0 && bottom <= 200.0,
            "{mode}: the target lies inside the hidden box's scrollport"
        );
        assert!(
            reader_holds_the_target(&doc),
            "{mode}: the reader holds the target after the scroll"
        );
    }
}

#[test]
fn a_smooth_scroll_writes_the_nested_box_at_once_and_animates_the_viewport() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: box below the viewport, smooth");
        let mut doc = page(BOX_BELOW_THE_VIEWPORT, incremental);
        assert!(
            offset(&doc, "#box") == 0.0 && doc.viewport_scroll().y == 0.0,
            "{mode}: nothing is scrolled at first"
        );

        scroll_to_target(&mut doc, ScrollBehavior::Smooth);

        assert!(
            offset(&doc, "#box") > 0.0,
            "{mode}: the box is scrolled when the call returns"
        );
        // The instant scroll of this page moves the viewport at once (the check above); a
        // smooth one has only started its travel, and no frame is driven here.
        assert!(
            doc.viewport_scroll().y == 0.0,
            "{mode}: the viewport has not moved when the call returns"
        );
        assert!(doc.is_animating(), "{mode}: the document reads animating");
    }
}

#[test]
fn a_target_above_the_origin_stays_out_of_view() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: above the origin");
        let mut doc = page(ABOVE_THE_ORIGIN, incremental);
        let (_, bottom) = target_edges(&doc);
        assert!(bottom <= 0.0, "{mode}: the target lies above the viewport");
        assert!(
            !reader_holds_the_target(&doc),
            "{mode}: the reader excludes the target before the scroll"
        );

        scroll_to_target(&mut doc, ScrollBehavior::Instant);

        assert!(
            doc.viewport_scroll().y == 0.0 && doc.viewport_scroll().x == 0.0,
            "{mode}: no offset brings the target in"
        );
        assert!(
            !reader_holds_the_target(&doc),
            "{mode}: the reader still excludes the target"
        );
    }
}
