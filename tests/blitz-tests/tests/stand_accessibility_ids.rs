//! The accessibility tree of each lean stand task, booted headlessly in both layout modes,
//! carries every element's stable element id as the AccessKit `author_id` of that element's
//! node, and no id on any other node; every interactive control has a role and a non-empty
//! accessible name; the ids hold across a re-render; and the Tab order is unchanged.

use std::collections::{HashMap, HashSet};

use accesskit::{Node, NodeId as TreeNodeId, Role, TreeUpdate};
use blitz_dom::Document;
use blitz_test_harness::Harness;
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand::{self, LeanTask};

/// Each task's interactive controls with their HTML-AAM role, the shell's `back-btn` first.
fn controls(task: LeanTask) -> &'static [(&'static str, Role)] {
    match task {
        LeanTask::Counter => &[
            ("back-btn", Role::Button),
            ("counter-increment", Role::Button),
        ],
        LeanTask::FlightBooker => &[
            ("back-btn", Role::Button),
            ("flight-one-way", Role::Button),
            ("flight-return", Role::Button),
            ("flight-start", Role::TextInput),
            ("flight-return-date", Role::TextInput),
            ("flight-book", Role::Button),
        ],
        LeanTask::Timer => &[
            ("back-btn", Role::Button),
            ("timer-duration", Role::Slider),
            ("timer-reset", Role::Button),
        ],
        LeanTask::Crud => &[
            ("back-btn", Role::Button),
            ("crud-filter", Role::TextInput),
            ("crud-name", Role::TextInput),
            ("crud-surname", Role::TextInput),
            ("crud-create", Role::Button),
            ("crud-update", Role::Button),
            ("crud-delete", Role::Button),
        ],
    }
}

/// The accessible names the markup gives the stand's inputs, trimmed.
const INPUT_NAMES: [(&str, &str); 6] = [
    ("flight-start", "Departure date"),
    ("flight-return-date", "Return date"),
    ("timer-duration", "Duration:"),
    ("crud-filter", "Filter prefix:"),
    ("crud-name", "Name:"),
    ("crud-surname", "Surname:"),
];

/// Each task's focus sequence from the root element, measured on the markup before the
/// accessible-name attributes were added.
fn tab_order(task: LeanTask) -> &'static [&'static str] {
    match task {
        LeanTask::Counter => &["back-btn", "counter-increment"],
        LeanTask::FlightBooker => &[
            "back-btn",
            "flight-one-way",
            "flight-return",
            "flight-start",
            "flight-book",
        ],
        LeanTask::Timer => &["back-btn", "timer-duration", "timer-reset"],
        LeanTask::Crud => &[
            "back-btn",
            "crud-filter",
            "crud-name",
            "crud-surname",
            "crud-create",
        ],
    }
}

fn boot(task: LeanTask, incremental: bool) -> Harness<DioxusDocument> {
    stand::boot(task, stand::options(incremental))
}

/// The tree's nodes by id.
fn by_id(tree: &TreeUpdate) -> HashMap<TreeNodeId, &Node> {
    tree.nodes.iter().map(|(id, node)| (*id, node)).collect()
}

/// The node carrying `author_id`.
#[track_caller]
fn find<'t>(tree: &'t TreeUpdate, author_id: &str) -> (TreeNodeId, &'t Node) {
    let found: Vec<_> = tree
        .nodes
        .iter()
        .filter(|(_, node)| node.author_id() == Some(author_id))
        .collect();
    assert_eq!(found.len(), 1, "one node carries {author_id:?}");
    (found[0].0, &found[0].1)
}

/// A node's accessible name: its `label`, else the joined names of the nodes labelling it,
/// where a `TextRun`'s name is its value.
fn name(nodes: &HashMap<TreeNodeId, &Node>, node: &Node) -> String {
    if let Some(label) = node.label().filter(|label| !label.is_empty()) {
        return label.to_string();
    }
    if node.role() == Role::TextRun {
        return node.value().unwrap_or_default().to_string();
    }
    node.labelled_by()
        .iter()
        .filter_map(|id| nodes.get(id))
        .map(|labelling| name(nodes, labelling))
        .collect()
}

/// Every node standing for an element carries that element's stable id, and every other node
/// carries none. Returns the carried ids, asserted pairwise distinct and non-empty.
#[track_caller]
fn assert_carried(
    harness: &Harness<DioxusDocument>,
    tree: &TreeUpdate,
    task: LeanTask,
) -> Vec<String> {
    let ids: HashMap<u64, String> = harness
        .doc
        .element_ids()
        .into_iter()
        .map(|(node, id)| (node.as_u64(), id))
        .collect();
    let doc = harness.base();
    let mut carried = Vec::new();
    let mut seen = HashSet::new();
    for (node_id, node) in &tree.nodes {
        match ids.get(&node_id.0) {
            Some(id) => {
                assert_eq!(
                    node.author_id(),
                    Some(id.as_str()),
                    "{task:?}: node {node_id:?}"
                );
                let element = doc
                    .get_node(blitz_traits::node_id::NodeId::from_u64(node_id.0))
                    .and_then(|n| n.element_data())
                    .expect("an element id names a live element");
                assert_eq!(
                    node.html_tag(),
                    Some(&*element.name.local),
                    "{task:?}: {id:?} is keyed by its own element"
                );
                assert!(seen.insert(id.clone()), "{task:?}: {id:?} carried twice");
                carried.push(id.clone());
            }
            None => assert_eq!(node.author_id(), None, "{task:?}: node {node_id:?}"),
        }
    }
    assert!(!carried.is_empty(), "{task:?}: the tree carries ids");
    carried
}

/// Move focus forward from the root element until it stops or repeats, reading each focused
/// element's stable id.
fn focus_sequence(harness: &mut Harness<DioxusDocument>) -> Vec<String> {
    let mut sequence = Vec::new();
    loop {
        let next = harness.base_mut().focus_next_node();
        let Some(node) = next else { break };
        let id = harness
            .doc
            .element_id(node)
            .expect("a focused element reads an id");
        if sequence.contains(&id) {
            break;
        }
        sequence.push(id);
    }
    sequence
}

#[test]
fn element_nodes_carry_their_stable_id() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let tree = harness.doc.accessibility_tree();
            let carried = assert_carried(&harness, &tree, task);
            for (control, _) in controls(task) {
                assert!(
                    carried.iter().any(|id| id == control),
                    "{task:?}: {control:?} is carried in {carried:?}"
                );
            }
        }
    }
}

#[test]
fn non_elements_carry_no_id() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let tree = harness.doc.accessibility_tree();
            let nodes = by_id(&tree);

            let text_runs: Vec<_> = tree
                .nodes
                .iter()
                .filter(|(_, node)| node.role() == Role::TextRun)
                .collect();
            assert!(!text_runs.is_empty(), "{task:?}: the tree has text");
            for (id, node) in text_runs {
                assert_eq!(node.author_id(), None, "{task:?}: TextRun {id:?}");
            }

            let window = nodes
                .get(&TreeNodeId(u64::MAX))
                .expect("the tree has its window");
            assert_eq!(window.author_id(), None, "{task:?}: the window");

            let root = harness.base().root_node().id.as_u64();
            let root = nodes
                .get(&TreeNodeId(root))
                .expect("the tree has the document root");
            assert_eq!(root.author_id(), None, "{task:?}: the document root");
        }
    }
}

#[test]
fn controls_carry_role_and_name() {
    let mut checked = HashSet::new();
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let tree = harness.doc.accessibility_tree();
            let nodes = by_id(&tree);
            for (control, role) in controls(task) {
                let (_, node) = find(&tree, control);
                assert_eq!(node.role(), *role, "{task:?}: {control:?}");
                let read = name(&nodes, node);
                assert!(!read.trim().is_empty(), "{task:?}: {control:?} is named");
                if let Some((_, expected)) = INPUT_NAMES.iter().find(|(id, _)| id == control) {
                    assert_eq!(read.trim(), *expected, "{task:?}: {control:?}");
                }
                checked.insert(*control);
            }
        }
    }
    assert_eq!(checked.len(), 15, "the stand's 15 controls");
}

#[test]
fn ids_hold_after_a_rerender() {
    for incremental in [false, true] {
        let mut counter = boot(LeanTask::Counter, incremental);
        let before = counter.text_content("#counter-value");
        counter.click("#counter-increment");
        assert_ne!(
            counter.text_content("#counter-value"),
            before,
            "the counter re-rendered"
        );
        let tree = counter.doc.accessibility_tree();
        assert_carried(&counter, &tree, LeanTask::Counter);

        let mut crud = boot(LeanTask::Crud, incremental);
        crud.click("#crud-create");
        let rows = crud.query_all(".list > .list-item");
        assert_eq!(rows.len(), 4, "Create added a row");
        let tree = crud.doc.accessibility_tree();
        assert_carried(&crud, &tree, LeanTask::Crud);
        let (created, _) = find(&tree, "crud-person-3");
        assert_eq!(
            created.0,
            rows[3].as_u64(),
            "the created row carries its id"
        );

        crud.click(".list > .list-item");
        assert!(
            crud.attr("#crud-delete", "disabled").is_none(),
            "a row is selected"
        );
        crud.click("#crud-delete");
        crud.pump();
        let rows_after = crud.query_all(".list > .list-item");
        assert_eq!(rows_after.len(), 3, "Delete removed a row");
        let removed: Vec<_> = rows
            .iter()
            .filter(|row| !rows_after.contains(row))
            .collect();
        assert_eq!(removed.len(), 1, "one row node left the list");

        let tree = crud.doc.accessibility_tree();
        assert_carried(&crud, &tree, LeanTask::Crud);
        assert!(
            tree.nodes.iter().all(|(id, _)| id.0 != removed[0].as_u64()),
            "the dropped row's node is gone from the tree"
        );
    }
}

#[test]
fn tab_order_is_unchanged() {
    for incremental in [false, true] {
        let measured: Vec<(LeanTask, Vec<String>)> = LeanTask::ALL
            .into_iter()
            .map(|task| (task, focus_sequence(&mut boot(task, incremental))))
            .collect();
        let expected: Vec<(LeanTask, Vec<String>)> = LeanTask::ALL
            .into_iter()
            .map(|task| {
                (
                    task,
                    tab_order(task).iter().map(|id| id.to_string()).collect(),
                )
            })
            .collect();
        assert_eq!(measured, expected, "incremental {incremental}");
    }
}
