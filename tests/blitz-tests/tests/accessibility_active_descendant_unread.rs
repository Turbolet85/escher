//! The accessibility tree reads no `aria-activedescendant`, measured and pinned as it reads on the
//! engine as built, in both layout modes: a focused `role="listbox"` stating it on one of its
//! `role="option"` children reads the tree's focus on the listbox and no active descendant, so
//! an assistive technology is told the list holds focus and never which option is current. The
//! stand's CRUD list `crud-list` is such a list. Kept a stated limit by the founder's F2 of
//! 2026-10-14 and this chunk's F3 B (2026-10-18): the gap is SC 4.1.2 work, outside the tier-0
//! target (SC 2.1.1 · 1.4.3 · 2.4.3). This check turns red by design once the builder reads the
//! attribute, and is restated then.
//!
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-activedescendant>

use accesskit::NodeId;
use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

const LIST: &str = r#"<html><body>
    <div id="list" role="listbox" aria-label="People" tabindex="0" aria-activedescendant="second">
        <div id="first" role="option" aria-selected="false">First</div>
        <div id="second" role="option" aria-selected="true">Second</div>
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

#[test]
fn a_focused_listbox_reads_focus_and_no_active_descendant() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut doc = page(LIST, incremental);
        let [list, second] = ["list", "second"].map(|id| {
            doc.get_element_by_id(id)
                .unwrap_or_else(|| panic!("{mode}: the page holds {id:?}"))
        });
        assert!(
            doc.get_node(list)
                .and_then(|node| node.attr(blitz_dom::local_name!("aria-activedescendant")))
                == Some("second"),
            "{mode}: the listbox states its active descendant"
        );
        assert!(
            doc.get_node(list).is_some_and(|node| node.is_focussable()),
            "{mode}: the listbox takes focus"
        );
        doc.set_focus_to(list);
        doc.resolve(0.0);

        let tree = doc.build_accessibility_tree();
        assert!(
            tree.focus == NodeId(list.as_u64()),
            "{mode}: the tree's focus is the listbox"
        );
        let node = tree
            .nodes
            .iter()
            .find(|(id, _)| *id == NodeId(list.as_u64()))
            .map(|(_, node)| node)
            .unwrap_or_else(|| panic!("{mode}: the tree keeps the listbox"));
        assert!(
            node.children().contains(&NodeId(second.as_u64())),
            "{mode}: the option is a child of the listbox in the tree"
        );
        assert!(
            node.active_descendant().is_none(),
            "{mode}: the listbox reads no active descendant"
        );
    }
}
