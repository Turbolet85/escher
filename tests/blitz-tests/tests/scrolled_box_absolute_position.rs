//! `Node::absolute_position` for a box that is itself scrolled, in both layout modes: the reader
//! counts the scroll offset of every box that holds a node and not the node's own, since
//! scrolling a box moves what it holds and never the box (CSSOM View,
//! <https://drafts.csswg.org/cssom-view/#dom-element-getboundingclientrect>). A scrolled box
//! reads where it stands at every scroll offset, as the bounds reader does
//! (`scrolled_box_client_rect`), while the content it holds reads moved by its offset. Before
//! the fix (measured 2026-10-10) the reader counted the box's own offset against it, so a box at
//! (40, 50) scrolled by (70, 120) read (−30, −70). The harness's `layout_rect`, `layout_rect_of`
//! and `center_of` are built on this reader. A failure message carries the layout mode and the
//! page, never a coordinate.

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

/// The scroll offsets the box is read at, the first leaving it unscrolled.
const OFFSETS: [(f64, f64); 4] = [(0.0, 0.0), (70.0, 120.0), (300.0, 0.0), (0.0, 400.0)];

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

/// What the reader answers for the border-box corner of `node`.
fn absolute_position(doc: &HtmlDocument, node: NodeId) -> (f32, f32) {
    let position = doc
        .get_node(node)
        .expect("a queried node resolves")
        .absolute_position(0.0, 0.0);
    (position.x, position.y)
}

#[test]
fn a_scrolled_box_reads_its_absolute_position_where_it_stands() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: one box");
        let mut doc = page(ONE_BOX, incremental);
        let [scroller, content] = ["#box", "#content"].map(|selector| {
            doc.query_selector(selector)
                .expect("the selector parses")
                .expect("the page holds the node")
        });
        for (index, (x, y)) in OFFSETS.into_iter().enumerate() {
            doc.scroll_to(scroller, x, y, ScrollBehavior::Instant);
            let offset = doc
                .get_node(scroller)
                .map(|node| (node.scroll_offset().x, node.scroll_offset().y));
            assert!(
                offset == Some((x, y)) && doc.viewport_scroll().y == 0.0,
                "{mode}: offset {index}: the box is scrolled by the offset asked and the viewport \
                 is not"
            );
            assert!(
                absolute_position(&doc, scroller) == (40.0, 50.0),
                "{mode}: offset {index}: the box reads the position the page puts it at"
            );
            assert!(
                absolute_position(&doc, content) == (40.0 - x as f32, 50.0 - y as f32),
                "{mode}: offset {index}: the content reads moved by the box's offset"
            );
        }
    }
}
