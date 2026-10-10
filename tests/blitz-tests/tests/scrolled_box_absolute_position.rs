//! `Node::absolute_position` for a box that is itself scrolled, measured and pinned as it reads
//! on the engine as built (2026-10-10), in both layout modes: the reader counts the box's own
//! scroll offset against the box, so a scrolled box reads its position that far from where it
//! stands though the box has not moved. This is the second reader with the shift the bounds
//! reader had (`scrolled_box_client_rect`); it is not fixed here. This check turns red by design
//! when the reader is fixed, and is then restated to the box's own position. The harness's
//! `layout_rect`, `layout_rect_of` and `center_of` are built on this reader, so they read a
//! scrolled box the same way. A failure message carries the layout mode and the page, never a
//! coordinate.

use blitz_dom::{DocumentConfig, ScrollBehavior};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::node_id::NodeId;
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

/// A 300 × 200 scrolling box 40px in and 50px down the page, its content 900px each way.
const ONE_BOX: &str = r#"<html><body style="margin:0">
    <div style="height:50px"></div>
    <div id="box" style="margin-left:40px; width:300px; height:200px; overflow:auto">
        <div style="width:900px; height:900px"></div>
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

/// What the reader answers for the border-box corner of `node`.
fn absolute_position(doc: &HtmlDocument, node: NodeId) -> (f32, f32) {
    let position = doc
        .get_node(node)
        .expect("a queried node resolves")
        .absolute_position(0.0, 0.0);
    (position.x, position.y)
}

#[test]
fn a_scrolled_box_reads_its_absolute_position_less_its_own_scroll_offset() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: one box");
        let mut doc = page(ONE_BOX, incremental);
        let scroller = doc
            .query_selector("#box")
            .expect("the selector parses")
            .expect("the page holds the box");
        let unscrolled = absolute_position(&doc, scroller);
        assert!(
            unscrolled == (40.0, 50.0),
            "{mode}: unscrolled, the box reads the position the page puts it at"
        );

        doc.scroll_to(scroller, 70.0, 120.0, ScrollBehavior::Instant);

        let offset = doc
            .get_node(scroller)
            .map(|node| (node.scroll_offset().x, node.scroll_offset().y));
        assert!(
            offset == Some((70.0, 120.0)) && doc.viewport_scroll().y == 0.0,
            "{mode}: the box is scrolled by the offset asked and the viewport is not"
        );
        // The reading, as measured: the box has not moved, and it reads moved by its own offset.
        assert!(
            absolute_position(&doc, scroller) == (unscrolled.0 - 70.0, unscrolled.1 - 120.0),
            "{mode}: the scrolled box reads its position less its own scroll offset"
        );
    }
}
