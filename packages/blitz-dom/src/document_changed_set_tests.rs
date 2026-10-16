//! Guards the document's changed set: a fresh document reads no change; an attribute write, a
//! focus move and a checked change mark their nodes until `take_changed_nodes` drains them; a write
//! outside the document, a hover and an idle resolve mark nothing, in either layout mode; and a
//! drained id of a dropped node reads `None`. A child module of `document`, because it reads the
//! crate-private node tree and the module's own imports.

use super::*;
use crate::mutator::DocumentMutator;
use crate::{Attribute, qual_name};
use blitz_traits::shell::ColorScheme;

/// Build `<html><body>` manually (the HTML parser lives in blitz-html, which would be a
/// circular dev-dependency), append the nodes `build` creates to the body and resolve once.
/// Elements are created in the HTML namespace: the default stylesheet's type selectors
/// match no other, and an unmatched `body` or `div` lays out inline.
fn make_doc(
    incremental: bool,
    build: impl FnOnce(&mut DocumentMutator<'_>) -> Vec<NodeId>,
) -> (BaseDocument, Vec<NodeId>) {
    let mut doc = BaseDocument::new(DocumentConfig {
        viewport: Some(Viewport::new(400, 300, 1.0, ColorScheme::Light)),
        incremental: Some(incremental),
        ..Default::default()
    });
    let root_id = doc.root_node().id;

    let mut mutator = doc.mutate();
    let html = mutator.create_element(qual_name!("html", html), vec![]);
    let body = mutator.create_element(qual_name!("body", html), vec![]);
    let children = build(&mut mutator);
    mutator.append_children(body, &children);
    mutator.append_children(html, &[body]);
    mutator.append_children(root_id, &[html]);
    drop(mutator);

    doc.resolve(0.0);
    (doc, children)
}

fn one_div(mutator: &mut DocumentMutator<'_>) -> Vec<NodeId> {
    vec![mutator.create_element(qual_name!("div", html), vec![])]
}

#[test]
fn changed_set_fresh_document_reads_no_change() {
    let mut doc = BaseDocument::new(DocumentConfig::default());
    assert!(!doc.has_changes(), "a fresh document reads no change");
    assert!(
        doc.take_changed_nodes().is_empty(),
        "a fresh document's changed set is empty"
    );
}

#[test]
fn changed_set_attribute_write_marks_its_node_until_drained() {
    let (mut doc, ids) = make_doc(true, one_div);
    let div = ids[0];
    assert!(
        doc.nodes[div].flags.is_in_document(),
        "the fixture's element is in the document"
    );

    doc.mutate()
        .set_attribute(div, qual_name!("title"), "written");
    assert!(doc.has_changes(), "a tracked write reads as a change");
    assert!(
        doc.take_changed_nodes().contains(&div),
        "the drain returns the written node"
    );
    assert!(!doc.has_changes(), "the drain leaves no change");
    assert!(
        doc.take_changed_nodes().is_empty(),
        "a second drain is empty"
    );

    doc.mutate()
        .set_attribute(div, qual_name!("title"), "written again");
    assert!(doc.has_changes(), "a write after a drain reads as a change");
    assert_eq!(
        doc.take_changed_nodes(),
        HashSet::from([div]),
        "the drain returns the written node and no other"
    );
}

#[test]
fn changed_set_write_outside_the_document_marks_nothing() {
    let (mut doc, _) = make_doc(true, |_| Vec::new());
    doc.take_changed_nodes();

    let mut mutator = doc.mutate();
    let detached = mutator.create_element(qual_name!("div", html), vec![]);
    let text = mutator.create_text_node("before");
    mutator.append_children(detached, &[text]);
    mutator.set_attribute(detached, qual_name!("title"), "written");
    mutator.set_node_text(text, "after");
    drop(mutator);

    assert!(
        !doc.nodes[detached].flags.is_in_document(),
        "the fixture's element is outside the document"
    );
    assert_eq!(doc.nodes[text].text_content(), "after");
    assert!(
        !doc.has_changes(),
        "a write outside the document is no change"
    );
    assert!(doc.take_changed_nodes().is_empty());
}

#[test]
fn changed_set_focus_move_marks_both_nodes_and_hover_marks_nothing() {
    let (mut doc, ids) = make_doc(true, |mutator| {
        vec![
            mutator.create_element(qual_name!("button", html), vec![]),
            mutator.create_element(qual_name!("button", html), vec![]),
        ]
    });
    let (first, second) = (ids[0], ids[1]);
    assert!(
        doc.set_focus_to(first),
        "the fixture focuses its first button"
    );
    doc.take_changed_nodes();

    assert!(doc.set_focus_to(second), "focus moves to the second button");
    assert!(doc.has_changes(), "a focus move reads as a change");
    assert_eq!(
        doc.take_changed_nodes(),
        HashSet::from([first, second]),
        "the node that lost focus and the node that gained it"
    );

    doc.snapshot_node_and(first, ElementState::HOVER, |node| node.hover());
    doc.snapshot_node_and(first, ElementState::ACTIVE, |node| node.active());
    assert!(
        doc.nodes[first].is_hovered(),
        "the fixture's first button is hovered"
    );
    assert!(!doc.has_changes(), "hover and active are no change");

    doc.clear_focus();
    assert_eq!(
        doc.take_changed_nodes(),
        HashSet::from([second]),
        "the node that lost focus"
    );
}

#[test]
fn changed_set_checked_change_marks_its_node() {
    let (mut doc, ids) = make_doc(true, |mutator| {
        let checkbox = Attribute {
            name: qual_name!("type"),
            value: "checkbox".to_string(),
        };
        vec![mutator.create_element(qual_name!("input", html), vec![checkbox])]
    });
    let checkbox = ids[0];
    let checked = |doc: &BaseDocument| {
        doc.nodes[checkbox]
            .element_data()
            .and_then(|element| element.checkbox_input_checked())
    };
    assert_eq!(
        checked(&doc),
        Some(false),
        "the fixture is an unchecked checkbox"
    );
    doc.take_changed_nodes();

    doc.snapshot_node_and(checkbox, ElementState::CHECKED, |node| {
        if let Some(element) = node.element_data_mut() {
            BaseDocument::toggle_checkbox(element);
        }
    });
    assert_eq!(checked(&doc), Some(true), "the checkbox is checked");
    assert!(doc.has_changes(), "a checked change reads as a change");
    assert_eq!(doc.take_changed_nodes(), HashSet::from([checkbox]));
}

/// Bare text next to a block sibling is wrapped in an anonymous block, a node layout
/// creates.
#[test]
fn changed_set_idle_resolve_marks_nothing_in_either_layout_mode() {
    for incremental in [false, true] {
        let (mut doc, _) = make_doc(incremental, |mutator| {
            let container = mutator.create_element(qual_name!("div", html), vec![]);
            let text = mutator.create_text_node("some text");
            let block = mutator.create_element(qual_name!("div", html), vec![]);
            mutator.append_children(container, &[text, block]);
            vec![container]
        });
        assert!(
            doc.nodes.iter().any(|(_, node)| node.is_anonymous()),
            "incremental={incremental}: layout built an anonymous block"
        );
        doc.take_changed_nodes();

        doc.resolve(0.0);
        assert!(
            !doc.has_changes(),
            "incremental={incremental}: a resolve with nothing written is no change"
        );
        assert!(
            doc.take_changed_nodes().is_empty(),
            "incremental={incremental}: a resolve with nothing written marks no node"
        );
    }
}

#[test]
fn changed_set_drained_id_of_a_dropped_node_reads_none() {
    let (mut doc, ids) = make_doc(true, one_div);
    let div = ids[0];
    doc.take_changed_nodes();

    let mut mutator = doc.mutate();
    mutator.set_attribute(div, qual_name!("title"), "written");
    mutator.remove_and_drop_node(div);
    drop(mutator);

    let taken = doc.take_changed_nodes();
    assert!(
        taken.contains(&div),
        "the drained set holds the dropped node's id"
    );
    for id in taken {
        assert_eq!(
            doc.get_node(id).is_none(),
            id == div,
            "only the dropped node's id reads None"
        );
    }
}
