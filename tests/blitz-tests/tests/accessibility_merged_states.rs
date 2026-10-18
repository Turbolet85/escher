//! The accessibility tree reads three ARIA attributes upstream's builder brought in, as upstream
//! wrote them: `aria-expanded` as its node's `expanded`, `aria-disabled` as its `disabled` and
//! `aria-level` as its `level`.
//!
//! On parsed pages at a fixed 800 × 600 viewport, in both layout modes: `aria-expanded` reads
//! `true` and `false` and nothing for a value that does not parse as either; `aria-disabled`
//! sets `disabled` for the exact value `true` alone; `aria-level` reads its number less one, as
//! AccessKit's levels are 0-based, and nothing for `0` or a value that is not a number, while
//! `h1`–`h6` with no attribute read their tag's level and `role="heading"` its implicit level 2.
//! The snapshot reads none of the three: `aria-disabled` moves no `enabled` reading.
//!
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-expanded>
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-disabled>
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-level>

use accesskit::{Node, NodeId, TreeUpdate};
use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

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

/// A page holding, per row, the element `{open} id="r{row}"…>{close}`, carrying `attribute`
/// with the row's value, or no attribute for `None`.
fn carriers(attribute: &str, open: &str, close: &str, values: &[Option<&str>]) -> String {
    let rows: String = values
        .iter()
        .enumerate()
        .map(|(row, value)| {
            let stated = value
                .map(|value| format!(r#" {attribute}="{value}""#))
                .unwrap_or_default();
            format!(r#"{open} id="r{row}"{stated}>{close}"#)
        })
        .collect();
    format!("<html><body>{rows}</body></html>")
}

/// The tree node the element `id` builds in `tree`.
#[track_caller]
fn read<'t>(doc: &HtmlDocument, tree: &'t TreeUpdate, id: &str) -> &'t Node {
    let node_id = doc
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("the page holds {id:?}"));
    tree.nodes
        .iter()
        .find(|(tree_id, _)| *tree_id == NodeId(node_id.as_u64()))
        .map(|(_, node)| node)
        .unwrap_or_else(|| panic!("the tree keeps {id:?}"))
}

/// Asserts, in both layout modes, that every row of the page `html` reads what `reading` takes
/// of its tree node as stated.
#[track_caller]
fn assert_rows<T: PartialEq + std::fmt::Debug>(
    html: &str,
    stated: &[T],
    reading: impl Fn(&Node) -> T,
    what: &str,
) {
    for incremental in [false, true] {
        let doc = page(html, incremental);
        let tree = doc.build_accessibility_tree();
        for (row, stated) in stated.iter().enumerate() {
            let node = read(&doc, &tree, &format!("r{row}"));
            assert_eq!(
                reading(node),
                *stated,
                "incremental={incremental}: {what}: row {row}"
            );
        }
    }
}

#[test]
fn aria_expanded_reads_true_and_false_and_nothing_else() {
    let rows = [
        (Some("true"), Some(true)),
        (Some("false"), Some(false)),
        (None, None),
        (Some(""), None),
        (Some("TRUE"), None),
        (Some("yes"), None),
        (Some(" true "), None),
    ];
    assert_eq!(rows.len(), 7);
    let values: Vec<_> = rows.iter().map(|(value, _)| *value).collect();
    let stated: Vec<_> = rows.iter().map(|(_, stated)| *stated).collect();
    let html = carriers("aria-expanded", "<button", "</button>", &values);
    assert_rows(&html, &stated, Node::is_expanded, "aria-expanded");
}

#[test]
fn aria_disabled_sets_disabled_for_true_alone() {
    let rows = [
        (Some("true"), true),
        (Some("false"), false),
        (None, false),
        (Some(""), false),
        (Some("TRUE"), false),
        (Some("zqx-no-such-value"), false),
    ];
    assert_eq!(rows.len(), 6);
    let values: Vec<_> = rows.iter().map(|(value, _)| *value).collect();
    let stated: Vec<_> = rows.iter().map(|(_, stated)| *stated).collect();
    let html = carriers("aria-disabled", "<button", "</button>", &values);
    assert_rows(&html, &stated, Node::is_disabled, "aria-disabled");
}

#[test]
fn aria_level_reads_its_number_less_one_and_the_heading_defaults() {
    let rows = [
        (Some("1"), Some(0)),
        (Some("3"), Some(2)),
        (Some("6"), Some(5)),
        (Some("0"), None),
        (Some("x"), None),
        (None, None),
    ];
    assert_eq!(rows.len(), 6);
    let values: Vec<_> = rows.iter().map(|(value, _)| *value).collect();
    let stated: Vec<_> = rows.iter().map(|(_, stated)| *stated).collect();
    let html = carriers("aria-level", "<div", "</div>", &values);
    assert_rows(&html, &stated, Node::level, "aria-level");

    let defaults = [
        ("<h1", "</h1>", Some(0)),
        ("<h2", "</h2>", Some(1)),
        ("<h3", "</h3>", Some(2)),
        ("<h4", "</h4>", Some(3)),
        ("<h5", "</h5>", Some(4)),
        ("<h6", "</h6>", Some(5)),
        (r#"<div role="heading""#, "</div>", Some(1)),
    ];
    assert_eq!(defaults.len(), 7);
    let html: String = defaults
        .iter()
        .enumerate()
        .map(|(row, (open, close, _))| format!(r#"{open} id="r{row}">Title{close}"#))
        .collect();
    let html = format!("<html><body>{html}</body></html>");
    let stated: Vec<_> = defaults.iter().map(|(_, _, stated)| *stated).collect();
    assert_rows(&html, &stated, Node::level, "the default levels");
}
