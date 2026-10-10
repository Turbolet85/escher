//! A hit never reaches content a box clips away by `overflow` — CSS Overflow: a box whose
//! `overflow` is not `visible` clips its content to its padding box, and what is clipped away
//! is not there to be hit. This guards the engine's hit walk (`Node::hit`), which tested a
//! scrolled box's own area in scrolled coordinates — so a point up to the scroll offset above
//! the box answered the box — and walked into a box's content wherever the content's layout
//! extends, clipped or not. On parsed pages at a fixed 800 × 600 viewport, in both layout
//! modes: a point above a scrolled box, over an earlier sibling, answers the sibling, and a
//! pointer moved there hovers it; a point below an unscrolled box, where its rows overflow,
//! answers what shows there; a point inside the box answers the row showing there, scrolled or
//! not; an `overflow: hidden` box scrolled by a program and an `overflow: clip` box read the
//! same; a box clipped on one axis only is stopped at its whole padding box, as paint clips it;
//! an `overflow: visible` box's overflowing content is still hit outside the box; and a
//! scrolled box's overlay scrollbar thumb is still resolved. *What the walk does not stop at,
//! pinned as it reads on the engine as built (2026-10-10):* a `contain: paint` box whose
//! `overflow` is `visible` — paint clips its content, and a hit reaches the clipped-out
//! content. A failure message carries the layout mode and the page, never a coordinate.

use blitz_dom::node::ScrollbarRef;
use blitz_dom::{DocumentConfig, EventDriver, NoopEventHandler, ScrollBehavior};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::events::{
    BlitzPointerEvent, BlitzPointerId, MouseEventButton, MouseEventButtons, Point, PointerCoords,
    PointerDetails, UiEvent,
};
use blitz_traits::node_id::NodeId;
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;
use style::values::computed::{Contain, Overflow};
use taffy::AbsoluteAxis;

/// How far down its rows a test scrolls the box: seven of its ten rows, so the seventh row's
/// centre lies at the centre of the sibling above the box.
const SCROLLED: f64 = 280.0;

/// A 400 × 500 stage holding a sibling 40px high and, below it, a 300 × 100 box styled by
/// `box_style` that holds ten rows 40px high. No element holds text, so a hit answers the
/// element itself.
fn rows_page(box_style: &str) -> String {
    let rows: String = (0..10)
        .map(|index| format!(r#"<div id="row-{index}" style="height:40px"></div>"#))
        .collect();
    format!(
        r#"<html><body style="margin:0">
    <div id="stage" style="width:400px; height:500px">
        <div id="before" style="width:300px; height:40px"></div>
        <div id="box" style="width:300px; height:100px; {box_style}">{rows}</div>
    </div>
</body></html>"#
    )
}

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

/// The left, top, right and bottom of the client rect of the element `selector` names.
fn edges(doc: &HtmlDocument, selector: &str) -> (f64, f64, f64, f64) {
    let rect = doc
        .get_client_bounding_rect(node(doc, selector))
        .expect("the element has a client rect");
    (rect.x, rect.y, rect.x + rect.width, rect.y + rect.height)
}

fn inside(point: (f64, f64), (left, top, right, bottom): (f64, f64, f64, f64)) -> bool {
    point.0 > left && point.0 < right && point.1 > top && point.1 < bottom
}

/// The three points every case reads, from client rects read while the box is unscrolled: the
/// centre of the sibling above the box, a point inside the box, and a point below it.
struct Points {
    above: (f64, f64),
    within: (f64, f64),
    below: (f64, f64),
    scrolling_box: (f64, f64, f64, f64),
}

fn points(doc: &HtmlDocument) -> Points {
    let before = edges(doc, "#before");
    let scrolling_box = edges(doc, "#box");
    let middle = (scrolling_box.0 + scrolling_box.2) / 2.0;
    Points {
        above: ((before.0 + before.2) / 2.0, (before.1 + before.3) / 2.0),
        within: (middle, scrolling_box.1 + 60.0),
        below: (middle, scrolling_box.3 + 80.0),
        scrolling_box,
    }
}

/// The node the document's hit answers at `point`, on a viewport that is not scrolled.
fn answer(doc: &HtmlDocument, point: (f64, f64)) -> Option<NodeId> {
    doc.hit(point.0 as f32, point.1 as f32)
        .map(|hit| hit.node_id)
}

/// How far the box is scrolled down its rows.
fn offset(doc: &HtmlDocument) -> f64 {
    doc.get_node(node(doc, "#box"))
        .expect("a queried node resolves")
        .scroll_offset()
        .y
}

fn scroll_the_box(doc: &mut HtmlDocument) {
    let scroller = node(doc, "#box");
    doc.scroll_to(scroller, 0.0, SCROLLED, ScrollBehavior::Instant);
}

/// The computed `overflow` of the box, across and down.
fn overflow(doc: &HtmlDocument) -> Option<(Overflow, Overflow)> {
    doc.get_node(node(doc, "#box"))
        .and_then(|scroller| scroller.primary_styles())
        .map(|styles| (styles.clone_overflow_x(), styles.clone_overflow_y()))
}

fn move_the_pointer(doc: &mut HtmlDocument, point: (f64, f64)) {
    let (x, y) = (point.0 as f32, point.1 as f32);
    let mut driver = EventDriver::new(doc, NoopEventHandler);
    driver.handle_ui_event(UiEvent::PointerMove(BlitzPointerEvent {
        id: BlitzPointerId::Mouse,
        is_primary: true,
        coords: PointerCoords {
            page_x: x,
            page_y: y,
            screen_x: x,
            screen_y: y,
            client_x: x,
            client_y: y,
        },
        button: MouseEventButton::Main,
        buttons: MouseEventButtons::None,
        mods: Default::default(),
        details: PointerDetails::default(),
        element: Point::default(),
        active_pointers: Default::default(),
    }));
}

/// With the box scrolled, the point over the sibling above it lies outside the box and inside
/// the seventh row as the row's own client rect reads.
#[track_caller]
fn assert_a_scrolled_out_row_extends_over_the_sibling(doc: &HtmlDocument, at: &Points, mode: &str) {
    assert!(
        offset(doc) == SCROLLED && doc.viewport_scroll().y == 0.0,
        "{mode}: the box is scrolled by the offset asked and the viewport is not"
    );
    assert!(
        !inside(at.above, at.scrolling_box)
            && inside(at.above, edges(doc, "#before"))
            && inside(at.above, edges(doc, "#row-6")),
        "{mode}: a row scrolled out above the box extends across the sibling's centre"
    );
}

/// With the box unscrolled, the point below it lies outside the box, inside the stage and
/// inside the fifth row as the row's own client rect reads.
#[track_caller]
fn assert_an_overflowing_row_extends_below_the_box(doc: &HtmlDocument, at: &Points, mode: &str) {
    assert!(offset(doc) == 0.0, "{mode}: the box is not scrolled");
    assert!(
        !inside(at.below, at.scrolling_box)
            && inside(at.below, edges(doc, "#stage"))
            && inside(at.below, edges(doc, "#row-4")),
        "{mode}: a row overflowing below the box extends across the point, over the stage"
    );
}

#[test]
fn a_point_above_a_scrolled_box_answers_the_sibling_there() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in an overflow:auto box");
        let mut doc = page(&rows_page("overflow:auto"), incremental);
        let at = points(&doc);
        let sibling = node(&doc, "#before");
        scroll_the_box(&mut doc);
        assert_a_scrolled_out_row_extends_over_the_sibling(&doc, &at, &mode);

        assert!(
            answer(&doc, at.above) == Some(sibling),
            "{mode}: the hit above the scrolled box answers the sibling, not the box or a row"
        );
        move_the_pointer(&mut doc, at.above);
        assert!(
            doc.get_hover_node_id() == Some(sibling),
            "{mode}: a pointer moved there hovers the sibling"
        );
    }
}

#[test]
fn a_point_below_an_unscrolled_box_answers_what_shows_there() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in an overflow:auto box");
        let doc = page(&rows_page("overflow:auto"), incremental);
        let at = points(&doc);
        assert_an_overflowing_row_extends_below_the_box(&doc, &at, &mode);

        assert!(
            answer(&doc, at.below) == Some(node(&doc, "#stage")),
            "{mode}: the hit below the box answers the stage, not a row"
        );
    }
}

#[test]
fn a_point_inside_the_box_answers_the_row_showing_there() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in an overflow:auto box");
        let mut doc = page(&rows_page("overflow:auto"), incremental);
        let at = points(&doc);
        assert!(
            inside(at.within, at.scrolling_box) && inside(at.within, edges(&doc, "#row-1")),
            "{mode}: unscrolled, the second row shows at the point inside the box"
        );
        assert!(
            answer(&doc, at.within) == Some(node(&doc, "#row-1")),
            "{mode}: unscrolled, the hit inside the box answers the row showing there"
        );

        scroll_the_box(&mut doc);

        assert!(
            offset(&doc) == SCROLLED && inside(at.within, edges(&doc, "#row-8")),
            "{mode}: scrolled, the ninth row shows at the same point"
        );
        assert!(
            answer(&doc, at.within) == Some(node(&doc, "#row-8")),
            "{mode}: scrolled, the hit inside the box answers the row showing there"
        );
    }
}

#[test]
fn a_hidden_overflow_box_scrolled_by_a_program_reads_the_same() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in an overflow:hidden box");
        let mut doc = page(&rows_page("overflow:hidden"), incremental);
        let at = points(&doc);
        assert!(
            overflow(&doc) == Some((Overflow::Hidden, Overflow::Hidden)),
            "{mode}: the box computes overflow hidden on both axes"
        );
        scroll_the_box(&mut doc);
        assert_a_scrolled_out_row_extends_over_the_sibling(&doc, &at, &mode);

        assert!(
            answer(&doc, at.above) == Some(node(&doc, "#before")),
            "{mode}: the hit above the scrolled box answers the sibling, not the box or a row"
        );
    }
}

#[test]
fn an_overflow_clip_box_keeps_its_clipped_out_content_from_a_hit() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in an overflow:clip box");
        let doc = page(&rows_page("overflow:clip"), incremental);
        let at = points(&doc);
        assert!(
            overflow(&doc) == Some((Overflow::Clip, Overflow::Clip)),
            "{mode}: the box computes overflow clip on both axes"
        );
        assert_an_overflowing_row_extends_below_the_box(&doc, &at, &mode);

        assert!(
            answer(&doc, at.below) == Some(node(&doc, "#stage")),
            "{mode}: the hit below the box answers the stage, not a row"
        );
    }
}

/// Paint clips a box by `overflow` without telling its two axes apart, and the hit stops where
/// paint clips: content overflowing along the axis that computes `visible` is not reached.
/// `BaseDocument::visible_region` narrows per axis, so it does not cut that content away.
#[test]
fn a_box_clipped_on_one_axis_is_stopped_at_its_whole_padding_box() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in a box clipped across only");
        let doc = page(
            &rows_page("overflow-x:clip; overflow-y:visible"),
            incremental,
        );
        let at = points(&doc);
        assert!(
            overflow(&doc) == Some((Overflow::Clip, Overflow::Visible)),
            "{mode}: the box computes clip across and visible down, as written"
        );
        assert_an_overflowing_row_extends_below_the_box(&doc, &at, &mode);

        assert!(
            answer(&doc, at.below) == Some(node(&doc, "#stage")),
            "{mode}: the hit below the box answers the stage, not a row"
        );
    }
}

#[test]
fn an_overflow_visible_box_is_hit_through_to_its_overflowing_content() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in an overflow:visible box");
        let doc = page(&rows_page("overflow:visible"), incremental);
        let at = points(&doc);
        assert!(
            overflow(&doc) == Some((Overflow::Visible, Overflow::Visible)),
            "{mode}: the box computes overflow visible on both axes"
        );
        assert_an_overflowing_row_extends_below_the_box(&doc, &at, &mode);

        assert!(
            answer(&doc, at.below) == Some(node(&doc, "#row-4")),
            "{mode}: the hit below the box answers the row overflowing there"
        );
    }
}

#[test]
fn a_scrolled_box_overlay_scrollbar_thumb_is_still_resolved() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in an overflow:auto box");
        let mut doc = page(&rows_page("overflow:auto"), incremental);
        let at = points(&doc);
        let scroller = node(&doc, "#box");
        scroll_the_box(&mut doc);
        assert!(
            offset(&doc) == SCROLLED,
            "{mode}: the box is scrolled by the offset asked"
        );
        // The thumb's rect is relative to the box's border-box corner.
        let thumb = doc
            .get_node(scroller)
            .and_then(|scroller| scroller.scrollbar_thumb(AbsoluteAxis::Vertical))
            .map(|thumb| thumb.center());
        let Some(thumb) = thumb else {
            panic!("{mode}: the scrolled box has a vertical scrollbar thumb");
        };
        let on_the_thumb = (at.scrolling_box.0 + thumb.x, at.scrolling_box.1 + thumb.y);
        assert!(
            inside(on_the_thumb, at.scrolling_box),
            "{mode}: the thumb's centre lies inside the box"
        );

        move_the_pointer(&mut doc, on_the_thumb);

        assert!(
            doc.hovered_scrollbar()
                == Some(ScrollbarRef {
                    node_id: scroller,
                    axis: AbsoluteAxis::Vertical,
                }),
            "{mode}: a pointer moved onto the thumb resolves the box's vertical scrollbar"
        );
    }
}

/// What the hit walk does not stop at. Paint clips the content of a `contain: paint` box to
/// its padding box whatever its `overflow`; the hit takes paint's `overflow` rule only. The
/// reading is pinned as measured on the engine as built (2026-10-10): this check turns red by
/// design if the walk is made to stop at such a box, and is then restated.
#[test]
fn a_contain_paint_box_with_visible_overflow_is_still_hit_through() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in a contain:paint box");
        let doc = page(&rows_page("contain:paint"), incremental);
        let at = points(&doc);
        let contains_paint = doc
            .get_node(node(&doc, "#box"))
            .and_then(|scroller| scroller.primary_styles())
            .map(|styles| styles.clone_contain().contains(Contain::PAINT));
        assert!(
            contains_paint == Some(true)
                && overflow(&doc) == Some((Overflow::Visible, Overflow::Visible)),
            "{mode}: the box computes paint containment and overflow visible on both axes"
        );
        assert_an_overflowing_row_extends_below_the_box(&doc, &at, &mode);

        assert!(
            answer(&doc, at.below) == Some(node(&doc, "#row-4")),
            "{mode}: the hit below the box answers the row paint clips away there"
        );
    }
}
