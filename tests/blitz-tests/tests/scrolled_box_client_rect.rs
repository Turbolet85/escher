//! A box's client rect is the box's own position: scrolling a box moves what it holds and never
//! the box — CSSOM View, `getBoundingClientRect`. This guards the engine's bounds reader
//! (`BaseDocument::physical_unrounded_geometry`, behind `get_client_bounding_rect`), which
//! counted a box's own scroll offset against the box itself, so a scrolled box read its rect
//! that far from where it stands. On parsed pages at a fixed 800 × 600 viewport, in both layout
//! modes: a box scrolled down its content, across it and both ways reads the rect it read
//! unscrolled; a scrolled box inside a scrolled box moves by the outer box's offset alone; and
//! the readers that take a box as the origin of its content hold — an element inside a
//! scrolled box moves by the box's offset, an inline element's fragment rects move with its
//! scrolling inline root, an inline element's offset rect ignores its offset parent's scroll,
//! and the visible region of an element inside a scrolled box is the box's padding box cut by
//! the viewport, scrolled or not. The reader has one body per setting of blitz-dom's
//! `writing-mode` feature, so this file is read under a per-package build and under a
//! workspace build. A failure message carries the layout mode and the page, never a coordinate.

use blitz_dom::{DocumentConfig, ScrollBehavior};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::node_id::NodeId;
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

/// A 300 × 200 scrolling box 40px in and 50px down the page, its content 900px each way.
const ONE_BOX: &str = r#"<html><body style="margin:0">
    <div style="height:50px"></div>
    <div id="box" style="margin-left:40px; width:300px; height:200px; overflow:auto">
        <div id="content" style="width:900px; height:900px"></div>
    </div>
</body></html>"#;

/// A scrolling box 100px down the content of another scrolling box.
const TWO_BOXES: &str = r#"<html><body style="margin:0">
    <div id="outer" style="width:400px; height:300px; overflow:auto">
        <div style="height:100px"></div>
        <div id="inner" style="width:300px; height:150px; overflow:auto">
            <div style="height:600px"></div>
        </div>
        <div style="height:600px"></div>
    </div>
</body></html>"#;

/// A scrolling box that is an inline root: six lines of text, an inline element on the fourth.
const SCROLLING_INLINE_ROOT: &str = r#"<html><body style="margin:0">
    <div style="height:50px"></div>
    <div id="scroller" style="width:300px; height:40px; overflow:auto; font-size:16px; line-height:20px">aaaa<br>aaaa<br>aaaa<br>aaaa <span id="inline">bbbb</span><br>aaaa<br>aaaa</div>
</body></html>"#;

/// The same inline root, positioned: it is its inline element's offset parent.
const POSITIONED_INLINE_ROOT: &str = r#"<html><body style="margin:0">
    <div style="height:50px"></div>
    <div id="scroller" style="position:relative; width:300px; height:40px; overflow:auto; font-size:16px; line-height:20px">aaaa<br>aaaa<br>aaaa<br>aaaa <span id="inline">bbbb</span><br>aaaa<br>aaaa</div>
</body></html>"#;

/// A positioned scrolling box holding a paragraph: the box is the offset parent of the
/// paragraph's inline element.
const POSITIONED_BOX_AROUND_A_PARAGRAPH: &str = r#"<html><body style="margin:0">
    <div style="height:50px"></div>
    <div id="scroller" style="position:relative; width:300px; height:100px; overflow:auto; border:5px solid">
        <div style="height:60px"></div>
        <p style="margin:0; font-size:16px; line-height:20px">aaaa <span id="inline">bbbb</span></p>
        <div style="height:600px"></div>
    </div>
</body></html>"#;

/// A scrolling box with a border and padding that runs past the viewport's right edge, its
/// target 400px down its content.
const BOX_PAST_THE_VIEWPORT_EDGE: &str = r#"<html><body style="margin:0">
    <div style="height:50px"></div>
    <div id="box" style="box-sizing:border-box; margin-left:600px; width:300px; height:200px; overflow:auto; border:5px solid; padding:10px">
        <div style="height:400px"></div>
        <div id="target" style="width:100px; height:40px"></div>
        <div style="height:400px"></div>
    </div>
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

/// The left, top, width and height of the client rect of the element `selector` names.
fn rect(doc: &HtmlDocument, selector: &str) -> (f64, f64, f64, f64) {
    let rect = doc
        .get_client_bounding_rect(node(doc, selector))
        .expect("the element has a client rect");
    (rect.x, rect.y, rect.width, rect.height)
}

/// How far the box `selector` names is scrolled, across and down its content.
fn offset(doc: &HtmlDocument, selector: &str) -> (f64, f64) {
    let offset = doc
        .get_node(node(doc, selector))
        .expect("a queried node resolves")
        .scroll_offset();
    (offset.x, offset.y)
}

fn scroll(doc: &mut HtmlDocument, selector: &str, x: f64, y: f64) {
    let scroller = node(doc, selector);
    doc.scroll_to(scroller, x, y, ScrollBehavior::Instant);
}

/// The left, top, width and height of each fragment rect of the element `selector` names.
fn fragments(doc: &HtmlDocument, selector: &str) -> Vec<(f64, f64, f64, f64)> {
    doc.node_client_rects(node(doc, selector))
        .into_iter()
        .map(|rect| (rect.x, rect.y, rect.width, rect.height))
        .collect()
}

#[test]
fn a_scrolled_box_reads_the_client_rect_it_read_unscrolled() {
    // Down the box's content, across it, and both.
    let offsets = [(0.0, 120.0), (70.0, 0.0), (70.0, 120.0)];
    assert_eq!(offsets.len(), 3);
    for incremental in [false, true] {
        for (row, (x, y)) in offsets.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: one box, offset {row}");
            let mut doc = page(ONE_BOX, incremental);
            let unscrolled = rect(&doc, "#box");
            assert!(
                unscrolled == (40.0, 50.0, 300.0, 200.0) && offset(&doc, "#box") == (0.0, 0.0),
                "{mode}: the box stands where the page puts it, unscrolled"
            );

            scroll(&mut doc, "#box", x, y);

            assert!(
                offset(&doc, "#box") == (x, y) && doc.viewport_scroll().y == 0.0,
                "{mode}: the box is scrolled by the offset asked and the viewport is not"
            );
            assert!(
                rect(&doc, "#box") == unscrolled,
                "{mode}: the scrolled box reads the client rect it read unscrolled"
            );
        }
    }
}

#[test]
fn a_scrolled_box_inside_a_scrolled_box_moves_by_the_outer_offset_alone() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: two boxes");
        let mut doc = page(TWO_BOXES, incremental);
        let outer = rect(&doc, "#outer");
        let inner = rect(&doc, "#inner");
        assert!(
            outer == (0.0, 0.0, 400.0, 300.0) && inner == (0.0, 100.0, 300.0, 150.0),
            "{mode}: both boxes stand where the page puts them"
        );

        scroll(&mut doc, "#outer", 0.0, 60.0);
        scroll(&mut doc, "#inner", 0.0, 90.0);

        assert!(
            offset(&doc, "#outer") == (0.0, 60.0) && offset(&doc, "#inner") == (0.0, 90.0),
            "{mode}: each box is scrolled by the offset asked"
        );
        assert!(
            rect(&doc, "#outer") == outer,
            "{mode}: the outer box reads the client rect it read unscrolled"
        );
        assert!(
            rect(&doc, "#inner") == (inner.0, inner.1 - 60.0, inner.2, inner.3),
            "{mode}: the inner box moved by the outer box's offset and not by its own"
        );
    }
}

#[test]
fn an_element_inside_a_scrolled_box_moves_by_the_box_offset() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: one box");
        let mut doc = page(ONE_BOX, incremental);
        let unscrolled = rect(&doc, "#content");
        assert!(
            unscrolled == (40.0, 50.0, 900.0, 900.0),
            "{mode}: the content starts at the box's own corner"
        );

        scroll(&mut doc, "#box", 70.0, 120.0);

        assert!(
            offset(&doc, "#box") == (70.0, 120.0),
            "{mode}: the box is scrolled by the offset asked"
        );
        assert!(
            rect(&doc, "#content")
                == (
                    unscrolled.0 - 70.0,
                    unscrolled.1 - 120.0,
                    unscrolled.2,
                    unscrolled.3
                ),
            "{mode}: the content moved by the box's offset"
        );
    }
}

#[test]
fn an_inline_element_moves_with_its_scrolling_inline_root() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: scrolling inline root");
        let mut doc = page(SCROLLING_INLINE_ROOT, incremental);
        let inline = node(&doc, "#inline");
        assert!(
            doc.inline_fragment_rects(inline).is_some(),
            "{mode}: the element is laid out inside an inline root"
        );
        let unscrolled = fragments(&doc, "#inline");
        assert!(
            !unscrolled.is_empty()
                && unscrolled
                    .iter()
                    .all(|fragment| fragment.2 > 0.0 && fragment.3 > 0.0),
            "{mode}: the inline element has a fragment, each with an area"
        );

        scroll(&mut doc, "#scroller", 0.0, 30.0);

        assert!(
            offset(&doc, "#scroller") == (0.0, 30.0),
            "{mode}: the inline root is scrolled by the offset asked"
        );
        let moved: Vec<_> = unscrolled
            .iter()
            .map(|fragment| (fragment.0, fragment.1 - 30.0, fragment.2, fragment.3))
            .collect();
        assert!(
            fragments(&doc, "#inline") == moved,
            "{mode}: each fragment moved by the inline root's offset"
        );
    }
}

#[test]
fn an_inline_element_offset_rect_ignores_its_offset_parent_scroll() {
    let pages = [
        ("positioned inline root", POSITIONED_INLINE_ROOT),
        (
            "positioned box around a paragraph",
            POSITIONED_BOX_AROUND_A_PARAGRAPH,
        ),
    ];
    assert_eq!(pages.len(), 2);
    for incremental in [false, true] {
        for (name, html) in pages {
            let mode = format!("incremental={incremental}: {name}");
            let mut doc = page(html, incremental);
            let inline = node(&doc, "#inline");
            let offset_rect = |doc: &HtmlDocument| {
                doc.offset_rect(inline)
                    .map(|rect| (rect.x, rect.y, rect.width, rect.height))
            };
            assert!(
                doc.inline_fragment_rects(inline).is_some(),
                "{mode}: the element is laid out inside an inline root"
            );
            let unscrolled = offset_rect(&doc);
            assert!(
                unscrolled.is_some_and(|rect| rect.2 > 0.0 && rect.3 > 0.0),
                "{mode}: the inline element has an offset rect with an area"
            );
            let fragments_unscrolled = fragments(&doc, "#inline");

            scroll(&mut doc, "#scroller", 0.0, 30.0);

            assert!(
                offset(&doc, "#scroller") == (0.0, 30.0),
                "{mode}: the offset parent is scrolled by the offset asked"
            );
            assert!(
                fragments(&doc, "#inline") != fragments_unscrolled,
                "{mode}: the scroll moved the inline element on the screen"
            );
            assert!(
                offset_rect(&doc) == unscrolled,
                "{mode}: the offset rect reads as it read before the scroll"
            );
        }
    }
}

#[test]
fn the_visible_region_inside_a_scrolled_box_is_its_padding_box_cut_by_the_viewport() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: box past the viewport's edge");
        let mut doc = page(BOX_PAST_THE_VIEWPORT_EDGE, incremental);
        let target = node(&doc, "#target");
        let region = |doc: &HtmlDocument| {
            doc.visible_region(target)
                .map(|seen| (seen.x, seen.y, seen.width, seen.height))
        };
        assert!(
            rect(&doc, "#box") == (600.0, 50.0, 300.0, 200.0),
            "{mode}: the box runs past the viewport's right edge"
        );
        // The box's border is 5px, so its padding box starts 5px inside its border box; the
        // viewport ends at 800.
        let padding_box_in_the_viewport = Some((605.0, 55.0, 195.0, 190.0));
        assert!(
            region(&doc) == padding_box_in_the_viewport,
            "{mode}: unscrolled, the region is the box's padding box cut by the viewport"
        );

        scroll(&mut doc, "#box", 0.0, 120.0);

        assert!(
            offset(&doc, "#box") == (0.0, 120.0),
            "{mode}: the box is scrolled by the offset asked"
        );
        assert!(
            region(&doc) == padding_box_in_the_viewport,
            "{mode}: scrolled, the region reads as it read before the scroll"
        );
    }
}
