//! Controls take their accessible name from `aria-label` and from an associated `<label>`.
//!
//! The tree once named a control only by its own text children, so an input labelled by a
//! `<label for>`, a wrapping `<label>` or an `aria-label` reached assistive technology
//! nameless. A document with no wrapper overriding [`Document::accessibility_tree`] builds the
//! same tree as [`BaseDocument::build_accessibility_tree`], and carries no `author_id`.
//!
//! <https://www.w3.org/TR/accname-1.2/#step2C>
//! <https://www.w3.org/TR/html-aam-1.0/#accessible-name-computations-by-html-element>
//!
//! [`BaseDocument::build_accessibility_tree`]: blitz_dom::BaseDocument::build_accessibility_tree

use accesskit::{Node, NodeId, Role, TreeUpdate};
use blitz_dom::Document;
use blitz_test_harness::Harness;

/// The resolved document's accessibility tree, built through [`Document::accessibility_tree`].
fn tree(html: &str) -> (Harness, TreeUpdate) {
    let harness = Harness::from_html(html);
    let tree = harness.doc.accessibility_tree();
    (harness, tree)
}

/// The tree node of the element matching `selector`.
#[track_caller]
fn node<'t>(harness: &Harness, tree: &'t TreeUpdate, selector: &str) -> (NodeId, &'t Node) {
    let id = NodeId(harness.node(selector).as_u64());
    let node = tree
        .nodes
        .iter()
        .find(|(node_id, _)| *node_id == id)
        .map(|(_, node)| node)
        .unwrap_or_else(|| panic!("{selector} has a tree node"));
    (id, node)
}

/// The `<label>` labels the input, and its own text run reads `text`.
#[track_caller]
fn assert_labelled(html: &str, text: &str) {
    let (harness, tree) = tree(html);
    let (label_id, label) = node(&harness, &tree, "label");
    let (_, input) = node(&harness, &tree, "input");
    assert_eq!(label.role(), Role::Label);
    assert_eq!(input.role(), Role::TextInput);

    assert_eq!(
        input.labelled_by(),
        &[label_id],
        "the label names the input"
    );
    let runs: Vec<&str> = label
        .labelled_by()
        .iter()
        .filter_map(|id| tree.nodes.iter().find(|(node_id, _)| node_id == id))
        .filter(|(_, node)| node.role() == Role::TextRun)
        .filter_map(|(_, node)| node.value())
        .collect();
    assert_eq!(
        runs.iter().map(|run| run.trim()).collect::<Vec<_>>(),
        [text]
    );
}

#[test]
fn aria_label_names_a_control() {
    let (harness, tree) = tree(r#"<input id="a" aria-label="X">"#);
    let (_, input) = node(&harness, &tree, "#a");
    assert_eq!(input.role(), Role::TextInput);
    assert_eq!(input.label(), Some("X"));
}

#[test]
fn empty_aria_label_is_ignored() {
    let (harness, tree) = tree(r#"<input id="a" aria-label="   ">"#);
    let (_, input) = node(&harness, &tree, "#a");
    assert_eq!(
        harness.attr("#a", "aria-label").as_deref(),
        Some("   "),
        "the fixture carries a whitespace-only aria-label"
    );
    assert_eq!(input.label(), None);
}

#[test]
fn label_for_names_its_input() {
    assert_labelled(r#"<label for="a">A</label><input id="a">"#, "A");
    assert_labelled(r#"<input id="a"><label for="a">A</label>"#, "A");
}

#[test]
fn nested_label_names_its_input() {
    assert_labelled(r#"<label>A <input></label>"#, "A");
}

#[test]
fn plain_html_tree_carries_no_author_id() {
    let html =
        r#"<main id="m"><label for="a">A</label><input id="a" aria-label="X"><p>text</p></main>"#;
    let harness = Harness::from_html(html);
    let mut through_trait = harness.doc.accessibility_tree();
    let mut built = harness.base().build_accessibility_tree();
    assert!(through_trait.nodes.len() > 1, "the tree has nodes");

    through_trait.nodes.sort_by_key(|(id, _)| *id);
    built.nodes.sort_by_key(|(id, _)| *id);
    assert_eq!(through_trait, built, "node for node");
    assert!(
        through_trait
            .nodes
            .iter()
            .all(|(_, node)| node.author_id().is_none()),
        "no node carries an author id"
    );
}
