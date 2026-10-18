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
//! a `contain: paint` box whose `overflow` is `visible` is stopped at its padding box too, as
//! paint clips it; an `overflow: visible` box's overflowing content is still hit outside the
//! box; and a scrolled box's overlay scrollbar thumb is still resolved. *Three boxes of their
//! own, measured (2026-10-14):* a hit at an image answers the image, a hit inside an iframe's
//! `srcdoc` sub-document answers the iframe, and a hit inside a filled text input answers the
//! input. A box both transformed and scrolled is hit at the row showing where the row shows,
//! where its client rect reads it, and inside the box where a scrolled-out row would lie
//! unscrolled it answers the box. A failure message carries the layout mode and the page, never
//! a coordinate.

use blitz_dom::node::{ImageData, RasterImageData, ScrollbarRef, SpecialElementData};
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

/// Paint clips the content of a `contain: paint` box to its padding box whatever its
/// `overflow`, and the hit stops where paint clips: the content clipped away below the box is
/// not reached. Before the fix (measured 2026-10-10) the hit took paint's `overflow` rule only
/// and answered the clipped-out row.
#[test]
fn a_contain_paint_box_stops_the_hit_at_its_padding_box() {
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
            answer(&doc, at.below) == Some(node(&doc, "#stage")),
            "{mode}: the hit below the box answers the stage, not the row paint clips away there"
        );
    }
}

/// A page holding, in turn, an image 120 × 80, an iframe whose `srcdoc` sub-document holds one
/// block 100 × 60, and a text input 200 wide holding text.
const OWN_BOX_PAGE: &str = r#"<html><body style="margin:0">
    <img id="picture" src="https://example.com/picture.png" style="display:block; width:120px; height:80px">
    <iframe id="frame" style="display:block; width:200px; height:100px; border:0"
        srcdoc="<html><body style='margin:0'><div id='inner' style='width:100px; height:60px'></div></body></html>"></iframe>
    <input id="field" value="filled" style="display:block; width:200px">
</body></html>"#;

/// The page, its image's data injected as loaded, resolved again.
fn own_box_page(incremental: bool) -> HtmlDocument {
    let mut doc = page(OWN_BOX_PAGE, incremental);
    let picture = node(&doc, "#picture");
    let pixels = vec![255_u8; 120 * 80 * 4];
    doc.get_node_mut(picture)
        .and_then(|picture| picture.element_data_mut())
        .expect("the image is an element")
        .special_data = SpecialElementData::Image(Box::new(ImageData::Raster(
        RasterImageData::new(120, 80, Arc::new(pixels)),
    )));
    doc.resolve(0.0);
    doc
}

fn centre_of(doc: &HtmlDocument, selector: &str) -> (f64, f64) {
    let (left, top, right, bottom) = edges(doc, selector);
    ((left + right) / 2.0, (top + bottom) / 2.0)
}

#[test]
fn a_hit_at_an_image_answers_the_image() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: an image with its data loaded");
        let doc = own_box_page(incremental);
        let picture = node(&doc, "#picture");
        let loaded = doc
            .get_node(picture)
            .and_then(|picture| picture.element_data())
            .and_then(|picture| picture.image_data())
            .is_some_and(|data| matches!(data, ImageData::Raster(_)));
        assert!(loaded, "{mode}: the image holds raster data");
        assert!(
            edges(&doc, "#picture") == (0.0, 0.0, 120.0, 80.0),
            "{mode}: the image lays out at its stated size"
        );

        assert!(
            answer(&doc, centre_of(&doc, "#picture")) == Some(picture),
            "{mode}: the hit at the image's centre answers the image"
        );
    }
}

/// The sub-document is a document of its own; the parent's hit walk does not enter it. The
/// reading is pinned as measured on the engine as built (2026-10-14).
#[test]
fn a_hit_inside_a_sub_document_answers_the_iframe() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: an iframe holding a srcdoc document");
        let doc = own_box_page(incremental);
        let frame = node(&doc, "#frame");
        let attached = doc
            .get_node(frame)
            .and_then(|frame| frame.element_data())
            .is_some_and(|frame| matches!(frame.special_data, SpecialElementData::SubDocument(_)));
        assert!(attached, "{mode}: the iframe holds its sub-document");
        let (left, top, right, bottom) = edges(&doc, "#frame");
        assert!(
            (right - left, bottom - top) == (200.0, 100.0),
            "{mode}: the iframe lays out at its stated size"
        );

        // Inside the sub-document's block, 100 × 60 at the iframe's corner.
        let inside_the_block = (left + 50.0, top + 30.0);
        assert!(
            answer(&doc, inside_the_block) == Some(frame),
            "{mode}: the hit inside the sub-document's block answers the iframe"
        );
    }
}

#[test]
fn a_hit_inside_a_filled_text_input_answers_the_input() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: a text input holding text");
        let doc = own_box_page(incremental);
        let field = node(&doc, "#field");
        let filled = doc
            .get_node(field)
            .and_then(|field| field.element_data())
            .and_then(|field| field.text_input_data())
            .is_some_and(|data| !data.editor.text().to_string().is_empty());
        assert!(filled, "{mode}: the input's editor holds text");
        let (left, top, right, bottom) = edges(&doc, "#field");
        assert!(
            right - left > 0.0 && bottom - top > 0.0,
            "{mode}: the input reads non-zero bounds"
        );

        assert!(
            answer(&doc, centre_of(&doc, "#field")) == Some(field),
            "{mode}: the hit at the input's centre answers the input"
        );
    }
}

/// How far the transformed box's transform moves it, across and down.
const MOVED: (f64, f64) = (50.0, 30.0);

/// A box 300 × 100, moved by [`MOVED`] by a transform, scrolled 280 down ten rows: the first row
/// as wide as the box, every other row 200 wide.
const TRANSFORMED_PAGE: &str = r#"<html><body style="margin:0">
    <div id="stage" style="width:400px; height:500px">
        <div id="before" style="width:300px; height:40px"></div>
        <div id="box" style="width:300px; height:100px; overflow:auto; transform:translate(50px, 30px)">
            <div id="row-0" style="width:300px; height:40px"></div>
            <div id="row-1" style="width:200px; height:40px"></div>
            <div id="row-2" style="width:200px; height:40px"></div>
            <div id="row-3" style="width:200px; height:40px"></div>
            <div id="row-4" style="width:200px; height:40px"></div>
            <div id="row-5" style="width:200px; height:40px"></div>
            <div id="row-6" style="width:200px; height:40px"></div>
            <div id="row-7" style="width:200px; height:40px"></div>
            <div id="row-8" style="width:200px; height:40px"></div>
            <div id="row-9" style="width:200px; height:40px"></div>
        </div>
    </div>
</body></html>"#;

/// The walk tests a box's own area and its padding box on the point carried through its inverse
/// transform, unscrolled, and its content on that point scrolled: a box both transformed and
/// scrolled is hit at the row showing where the row shows, and a point inside the box where a
/// scrolled-out row would lie unscrolled answers the box. The client rects read each element
/// where it shows, moved by the transform — before the fix (measured 2026-10-14) they read it
/// where layout puts it — so the points are those rects' own points.
#[test]
fn a_box_both_transformed_and_scrolled_is_hit_where_its_rows_show() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: rows in a transformed overflow:auto box");
        let mut doc = page(TRANSFORMED_PAGE, incremental);
        let shows_at = edges(&doc, "#box");
        assert!(
            shows_at == (MOVED.0, 40.0 + MOVED.1, 300.0 + MOVED.0, 140.0 + MOVED.1),
            "{mode}: the box's client rect reads where the transform moves it"
        );
        let unscrolled_first_row = edges(&doc, "#row-0");
        scroll_the_box(&mut doc);
        assert!(
            offset(&doc) == SCROLLED && doc.viewport_scroll().y == 0.0,
            "{mode}: the box is scrolled by the offset asked and the viewport is not"
        );
        assert!(
            edges(&doc, "#box") == shows_at,
            "{mode}: the scrolled box reads where it stands"
        );

        let row = centre_of(&doc, "#row-8");
        assert!(
            inside(row, shows_at),
            "{mode}: the ninth row shows inside the box"
        );
        assert!(
            answer(&doc, row) == Some(node(&doc, "#row-8")),
            "{mode}: the hit where the ninth row's centre shows answers the row"
        );

        let beside_the_rows = (shows_at.0 + 250.0, shows_at.1 + 20.0);
        assert!(
            inside(beside_the_rows, unscrolled_first_row)
                && !inside(beside_the_rows, edges(&doc, "#row-7")),
            "{mode}: the point lies where the first row lies unscrolled and beside the row \
             showing there"
        );
        assert!(
            inside(beside_the_rows, shows_at),
            "{mode}: the point lies inside the box as it shows"
        );
        assert!(
            answer(&doc, beside_the_rows) == Some(node(&doc, "#box")),
            "{mode}: the hit there answers the box, not a row"
        );
    }
}
