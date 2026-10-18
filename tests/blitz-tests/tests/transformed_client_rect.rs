//! An element's client rect counts the transforms on its containing-block chain, as CSSOM View
//! states (<https://drafts.csswg.org/cssom-view/#dom-element-getboundingclientrect>): the border
//! box is mapped through the element's own transform and each containing block's, as the hit
//! and paint map it, and the rect is the axis-aligned box of the mapped corners. Before the fix
//! (measured 2026-10-14) the rect read every box where layout puts it, transformed or not.
//! On parsed pages booted with the stand's options, in both layout modes: a translated box moves
//! by its translation; a scaled box reads its scaled box about its centre; an untransformed child
//! of a translated parent, and an inline span inside it, move with the parent, while
//! `offset_rect` — CSSOM's `offsetTop` and kin, which ignore transforms — reads as on the
//! untransformed page. *A limit, measured:* `visible_region` reads a scrolling box where it
//! stands untransformed, so an element inside a translated scrolling box, shown in the viewport,
//! has its centre outside its own visible region — the driver reads it `off-screen` though it
//! shows. A failure message carries the layout mode and the page, never a coordinate.

use blitz_dom::BoundingRect;
use blitz_html::HtmlDocument;
use blitz_test_harness::Harness;
use seven_guis::stand;

fn page(html: &str, incremental: bool) -> Harness<HtmlDocument> {
    Harness::from_html_with(html, stand::options(incremental))
}

/// A page whose `#box` is a 100 × 40 box 10px in and 20px down, styled by `style`.
fn one_box(style: &str) -> String {
    format!(
        r#"<html><body style="margin:0">
    <div style="height:20px"></div>
    <div id="box" style="margin-left:10px; width:100px; height:40px; {style}"></div>
</body></html>"#
    )
}

/// A 300 × 200 parent at the page's corner, styled by `style`, holding a 50 × 50 child 20px in
/// and 10px down and a line of text with an inline span.
fn parent_and_child(style: &str) -> String {
    format!(
        r#"<html><body style="margin:0">
    <div id="parent" style="width:300px; height:200px; {style}">
        <div id="child" style="margin-left:20px; margin-top:10px; width:50px; height:50px"></div>
        <p style="margin:0">Before <span id="span">inside</span> after</p>
    </div>
</body></html>"#
    )
}

fn rect(harness: &Harness<HtmlDocument>, selector: &str) -> (f64, f64, f64, f64) {
    let node = harness.node(selector);
    let BoundingRect {
        x,
        y,
        width,
        height,
    } = harness
        .base()
        .get_client_bounding_rect(node)
        .expect("the element has a client rect");
    (x, y, width, height)
}

fn offset_rect(harness: &Harness<HtmlDocument>, selector: &str) -> (f64, f64, f64, f64) {
    let node = harness.node(selector);
    let BoundingRect {
        x,
        y,
        width,
        height,
    } = harness
        .base()
        .offset_rect(node)
        .expect("the element has an offset rect");
    (x, y, width, height)
}

fn moved((x, y, width, height): (f64, f64, f64, f64), by: (f64, f64)) -> (f64, f64, f64, f64) {
    (x + by.0, y + by.1, width, height)
}

#[test]
fn a_translated_box_reads_its_rect_moved_by_the_translation() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: one box");
        let plain = page(&one_box(""), incremental);
        assert!(
            rect(&plain, "#box") == (10.0, 20.0, 100.0, 40.0),
            "{mode}: untransformed, the box reads where layout puts it"
        );
        let translated = page(&one_box("transform:translate(50px, 30px)"), incremental);
        assert!(
            rect(&translated, "#box") == (60.0, 50.0, 100.0, 40.0),
            "{mode}: translated, the box reads moved by (50, 30)"
        );
    }
}

#[test]
fn a_scaled_box_reads_its_scaled_box_about_its_centre() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: one box");
        let scaled = page(
            &one_box("margin-left:100px; margin-top:80px; transform:scale(2)"),
            incremental,
        );
        let plain = page(&one_box("margin-left:100px; margin-top:80px"), incremental);
        assert!(
            rect(&plain, "#box") == (100.0, 100.0, 100.0, 40.0),
            "{mode}: untransformed, the box is 100 × 40 at (100, 100)"
        );
        assert!(
            rect(&scaled, "#box") == (50.0, 80.0, 200.0, 80.0),
            "{mode}: scaled by 2 about its centre, the box reads 200 × 80 about the same centre"
        );
    }
}

#[test]
fn a_child_of_a_translated_parent_moves_with_it_and_its_offset_rect_does_not() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: parent and child");
        let plain = page(&parent_and_child(""), incremental);
        let translated = page(
            &parent_and_child("transform:translate(50px, 30px)"),
            incremental,
        );
        assert!(
            rect(&plain, "#child") == (20.0, 10.0, 50.0, 50.0),
            "{mode}: untransformed, the child reads where layout puts it"
        );
        let span = rect(&plain, "#span");
        assert!(
            span.2 > 0.0 && span.3 > 0.0,
            "{mode}: the span lays out with an area"
        );
        assert!(
            translated
                .base()
                .get_node(translated.node("#child"))
                .is_some_and(|child| child.transform().is_none()),
            "{mode}: the child carries no transform of its own"
        );

        for (index, selector) in ["#parent", "#child", "#span"].into_iter().enumerate() {
            assert!(
                rect(&translated, selector) == moved(rect(&plain, selector), (50.0, 30.0)),
                "{mode}: element {index} reads moved by the parent's translation"
            );
            assert!(
                offset_rect(&translated, selector) == offset_rect(&plain, selector),
                "{mode}: element {index}'s offset rect reads as on the untransformed page"
            );
        }
    }
}

/// A 300 × 100 scrolling box at the page's corner, styled by `style`, holding a 100 × 40
/// target at its top.
fn scrolling_box(style: &str) -> String {
    format!(
        r#"<html><body style="margin:0">
    <div id="scroller" style="width:300px; height:100px; overflow:auto; {style}">
        <div id="target" style="width:100px; height:40px"></div>
        <div style="height:400px"></div>
    </div>
</body></html>"#
    )
}

fn centre((x, y, width, height): (f64, f64, f64, f64)) -> (f64, f64) {
    (x + width / 2.0, y + height / 2.0)
}

fn region(harness: &Harness<HtmlDocument>, selector: &str) -> Option<(f64, f64, f64, f64)> {
    harness
        .base()
        .visible_region(harness.node(selector))
        .map(|seen| (seen.x, seen.y, seen.width, seen.height))
}

fn inside(point: (f64, f64), (x, y, width, height): (f64, f64, f64, f64)) -> bool {
    point.0 >= x && point.0 <= x + width && point.1 >= y && point.1 <= y + height
}

/// The reading as measured: the scrolling box's clip is read where the box stands untransformed,
/// while the target's rect reads where it shows.
#[test]
fn a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: a scrolling box");
        let plain = page(&scrolling_box(""), incremental);
        let plain_target = rect(&plain, "#target");
        assert!(
            region(&plain, "#target").is_some_and(|seen| inside(centre(plain_target), seen)),
            "{mode}: untransformed, the target's centre lies in its visible region"
        );

        let translated = page(&scrolling_box("transform:translate(300px, 0)"), incremental);
        let target = rect(&translated, "#target");
        assert!(
            target == moved(plain_target, (300.0, 0.0)),
            "{mode}: translated, the target reads where it shows"
        );
        let (x, y, width, height) = target;
        assert!(
            x >= 0.0
                && y >= 0.0
                && x + width <= f64::from(stand::VIEWPORT_WIDTH)
                && y + height <= f64::from(stand::VIEWPORT_HEIGHT),
            "{mode}: the target shows inside the viewport"
        );
        let seen = region(&translated, "#target");
        assert!(
            seen == region(&plain, "#target"),
            "{mode}: the visible region reads as on the untransformed page"
        );
        assert!(
            seen.is_some_and(|seen| !inside(centre(target), seen)),
            "{mode}: the target's centre lies outside its own visible region"
        );
    }
}
