//! `BaseDocument::text_extent` reads where an inline root's laid-out lines lie beside its
//! content box, on pages booted with the stand's options (bundled DejaVu Sans, pinned viewport),
//! in both layout modes: a line that cannot wrap reads wider than its 80 px box, the same text
//! wrapping reads inside it, and an element with no inline content and an id that no longer
//! resolves read nothing. A failure message carries a layout mode only.

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
