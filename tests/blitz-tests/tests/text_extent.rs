//! `BaseDocument::text_extent` reads where an inline root's laid-out lines lie beside its
//! content box, on pages booted with the stand's options (bundled DejaVu Sans, pinned viewport),
//! in both layout modes: a line that cannot wrap reads wider than its 80 px box, the same text
//! wrapping reads inside it, and an element with no inline content and an id that no longer
//! resolves read nothing. Its three stated limits, each read against a left-to-right twin: a
//! right-to-left line reads as a left-to-right one from the content box's left edge, vertical
//! text reads as one horizontal line, and a box scaled about its centre reads its content box at
//! its client rect with its lines unscaled. A failure message carries a layout mode only.

use blitz_dom::{TextExtent, local_name};
use blitz_test_harness::Harness;
use blitz_traits::node_id::NodeId;
use seven_guis::stand;

/// A line the bundled font lays out wider than the 80 px each paragraph is given.
const PAGE: &str = r#"
<style>
  body { margin: 0; }
  p { width: 80px; margin: 0 0 8px 0; }
</style>
<p id="nowrap" style="white-space: nowrap">A line too long for its box</p>
<p id="wrapped">A line too long for its box</p>
<div id="empty"></div>
<div id="gone">Removed</div>
<p id="ltr" style="width: 300px; white-space: nowrap">Word</p>
<p id="rtl" dir="rtl" style="width: 300px; white-space: nowrap">Word</p>
<p id="vertical" style="writing-mode: vertical-lr; white-space: nowrap">Word</p>
<p id="scaled" style="transform: scale(2)">Word</p>
"#;

fn boot(incremental: bool) -> Harness<blitz_html::HtmlDocument> {
    Harness::from_html_with(PAGE, stand::options(incremental))
}

#[track_caller]
fn node(harness: &Harness<blitz_html::HtmlDocument>, selector: &str, at: &str) -> NodeId {
    let node = harness.query(selector);
    assert!(node.is_some(), "{at}: the selector resolves");
    node.unwrap()
}

#[test]
fn a_line_that_cannot_wrap_reads_wider_than_its_box() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(incremental);
        let id = node(&harness, "#nowrap", &at);
        let extent = harness.base().text_extent(id);
        assert!(extent.is_some(), "{at}: the paragraph reads an extent");
        let extent = extent.unwrap();
        assert!(
            extent.content.width == 80.0,
            "{at}: the content box is 80 px"
        );
        assert!(
            extent.text.width > extent.content.width,
            "{at}: the unbroken line reads wider than its box"
        );
        assert!(
            extent.text.x == extent.content.x && extent.text.y >= extent.content.y,
            "{at}: the lines start at the content box"
        );
    }
}

#[test]
fn a_wrapped_paragraph_reads_inside_its_box() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(incremental);
        let unbroken = harness
            .base()
            .text_extent(node(&harness, "#nowrap", &at))
            .map(|extent| extent.text.width);
        assert!(
            unbroken.is_some_and(|width| width > 80.0),
            "{at}: the line measures wider than the box"
        );
        let extent = harness.base().text_extent(node(&harness, "#wrapped", &at));
        assert!(extent.is_some(), "{at}: the paragraph reads an extent");
        let extent = extent.unwrap();
        assert!(
            extent.text.width <= extent.content.width,
            "{at}: the wrapped lines read inside their box"
        );
        assert!(
            extent.text.height > extent.content.height / 2.0 && extent.text.height > 0.0,
            "{at}: the wrapped lines take height"
        );
    }
}

#[test]
fn an_element_with_no_inline_content_reads_none() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut harness = boot(incremental);
        let empty = node(&harness, "#empty", &at);
        assert!(
            harness.base().text_extent(empty).is_none(),
            "{at}: an empty block reads none"
        );
        let gone = node(&harness, "#gone", &at);
        assert!(
            harness.base().text_extent(gone).is_some(),
            "{at}: the block reads an extent before it is removed"
        );
        harness.base_mut().mutate().remove_and_drop_node(gone);
        assert!(
            harness.base().get_node(gone).is_none(),
            "{at}: the removed id no longer resolves"
        );
        assert!(
            harness.base().text_extent(gone).is_none(),
            "{at}: a stale id reads none"
        );
    }
}

#[track_caller]
fn extent(harness: &Harness<blitz_html::HtmlDocument>, selector: &str, at: &str) -> TextExtent {
    let extent = harness.base().text_extent(node(harness, selector, at));
    assert!(extent.is_some(), "{at}: the paragraph reads an extent");
    extent.unwrap()
}

#[test]
fn right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(incremental);
        let rtl = node(&harness, "#rtl", &at);
        assert!(
            harness
                .base()
                .get_node(rtl)
                .and_then(|node| node.attr(local_name!("dir")))
                == Some("rtl"),
            "{at}: the paragraph is right-to-left"
        );
        let ltr = extent(&harness, "#ltr", &at);
        let rtl = extent(&harness, "#rtl", &at);
        assert!(
            rtl.text.width == ltr.text.width && rtl.text.width < rtl.content.width,
            "{at}: the line reads as wide as its left-to-right twin, inside its box"
        );
        assert!(
            rtl.text.x == rtl.content.x,
            "{at}: the line reads from the content box's left edge"
        );
    }
}

#[test]
fn vertical_text_reads_as_a_horizontal_line() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(incremental);
        let ltr = extent(&harness, "#ltr", &at);
        let vertical = extent(&harness, "#vertical", &at);
        assert!(
            ltr.text.width > ltr.text.height,
            "{at}: the twin's line is wider than it is high"
        );
        assert!(
            vertical.text.width == ltr.text.width && vertical.text.height == ltr.text.height,
            "{at}: the vertical text reads as one horizontal line"
        );
        assert!(
            vertical.text.x == vertical.content.x,
            "{at}: the line reads from the content box's left edge"
        );
    }
}

#[test]
fn a_transformed_box_reads_its_lines_at_its_client_rect() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(incremental);
        let scaled = node(&harness, "#scaled", &at);
        let client = harness.base().get_client_bounding_rect(scaled);
        let laid_out = harness.base().offset_rect(scaled);
        assert!(
            client.is_some() && laid_out.is_some(),
            "{at}: the box reads a client rect and an offset rect"
        );
        let (client, laid_out) = (client.unwrap(), laid_out.unwrap());
        assert!(
            client.width == 2.0 * laid_out.width
                && client.x + client.width / 2.0 == laid_out.x + laid_out.width / 2.0,
            "{at}: the box is scaled twice about its centre"
        );
        let ltr = extent(&harness, "#ltr", &at);
        let extent = extent(&harness, "#scaled", &at);
        assert!(
            extent.content == client,
            "{at}: the content box is the client rect"
        );
        assert!(
            extent.text.width == ltr.text.width && extent.text.x == client.x,
            "{at}: the lines read unscaled from the client rect's left edge"
        );
    }
}
