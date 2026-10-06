//! The snapshot of each lean stand task, booted headlessly in both layout modes: every node
//! is an element the accessibility tree keeps, carrying its stable element id, the role and
//! name of its accessibility node, its state as the engine reads it and its bounding client
//! rect; the tree follows the TaskShell; and the snapshot is deterministic and follows a
//! re-render.

use std::collections::{HashMap, HashSet};

use accesskit::{Node, NodeId as TreeNodeId, Role, TreeUpdate};
use blitz_dom::Document;
use blitz_test_harness::Harness;
use blitz_traits::node_id::NodeId;
use dioxus_native_dom::{DioxusDocument, Snapshot, SnapshotNode};
use seven_guis::stand::{self, LeanTask, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

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

/// Each task's author-id controls and value displays rendered at boot with a box of their
/// own. `flight-booked` renders only after a booking, and `timer-progress` is 0 wide until
/// a tick is delivered.
fn rendered(task: LeanTask) -> &'static [&'static str] {
    match task {
        LeanTask::Counter => &[
            "back-btn",
            "task-title",
            "counter-value",
            "counter-increment",
        ],
        LeanTask::FlightBooker => &[
            "back-btn",
            "task-title",
            "flight-one-way",
            "flight-return",
            "flight-start",
            "flight-return-date",
            "flight-book",
        ],
        LeanTask::Timer => &[
            "back-btn",
            "task-title",
            "timer-elapsed",
            "timer-duration",
            "timer-duration-value",
            "timer-reset",
        ],
        LeanTask::Crud => &[
            "back-btn",
            "task-title",
            "crud-filter",
            "crud-list",
            "crud-name",
            "crud-surname",
            "crud-create",
            "crud-update",
            "crud-delete",
        ],
    }
}

fn boot(task: LeanTask, incremental: bool) -> Harness<DioxusDocument> {
    stand::boot(task, stand::options(incremental))
}

/// The node `id` names, asserted present.
#[track_caller]
fn node<'s>(snapshot: &'s Snapshot, id: &str) -> &'s SnapshotNode {
    snapshot
        .get(id)
        .unwrap_or_else(|| panic!("{id:?} is a snapshot node"))
}

/// `node`'s descendants in pre-order, `node` excluded.
fn descendants(node: &SnapshotNode) -> Vec<&SnapshotNode> {
    let mut out = Vec::new();
    let mut stack: Vec<&SnapshotNode> = node.children.iter().rev().collect();
    while let Some(next) = stack.pop() {
        out.push(next);
        stack.extend(next.children.iter().rev());
    }
    out
}

/// The tree's nodes by id.
fn by_id(tree: &TreeUpdate) -> HashMap<TreeNodeId, &Node> {
    tree.nodes.iter().map(|(id, node)| (*id, node)).collect()
}

/// The tree node carrying `author_id`.
#[track_caller]
fn tree_node<'t>(tree: &'t TreeUpdate, author_id: &str) -> (TreeNodeId, &'t Node) {
    let found: Vec<_> = tree
        .nodes
        .iter()
        .filter(|(_, node)| node.author_id() == Some(author_id))
        .collect();
    assert_eq!(found.len(), 1, "one tree node carries {author_id:?}");
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

#[test]
fn every_node_carries_the_five_fields() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let snapshot = boot(task, incremental).doc.snapshot();
            let nodes: Vec<_> = snapshot.nodes().collect();
            assert!(!nodes.is_empty(), "{task:?}: the snapshot has nodes");
            for node in nodes {
                assert!(!node.id.is_empty(), "{task:?}: a node has an id");
                let rect = node.bounds;
                for value in [rect.x, rect.y, rect.width, rect.height] {
                    assert!(value.is_finite(), "{task:?}: {:?} reads {rect:?}", node.id);
                }
                assert!(
                    rect.width >= 0.0 && rect.height >= 0.0,
                    "{task:?}: {:?} reads {rect:?}",
                    node.id
                );
                assert_ne!(node.role, Role::TextRun, "{task:?}: {:?}", node.id);
            }
        }
    }
}

#[test]
fn ids_are_the_accessibility_tree_ids() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let snapshot = harness.doc.snapshot();
            let tree = harness.doc.accessibility_tree();

            let ids: Vec<&str> = snapshot.nodes().map(|node| node.id.as_str()).collect();
            let distinct: HashSet<&str> = ids.iter().copied().collect();
            assert_eq!(distinct.len(), ids.len(), "{task:?}: ids are distinct");
            let carried: HashSet<&str> = tree
                .nodes
                .iter()
                .filter_map(|(_, node)| node.author_id())
                .collect();
            assert_eq!(distinct, carried, "{task:?}: the tree's author ids");

            for id in ids {
                let (tree_id, _) = tree_node(&tree, id);
                let element = NodeId::from_u64(tree_id.0);
                assert_eq!(
                    harness.doc.element_id(element).as_deref(),
                    Some(id),
                    "{task:?}: {id:?} is its element's id"
                );
                assert!(
                    !id.contains(&format!("{element:?}")) && !id.contains("NodeId"),
                    "{task:?}: {id:?} carries no NodeId token"
                );
            }
        }
    }
}

#[test]
fn role_and_name_match_the_accessibility_tree() {
    let mut checked = HashSet::new();
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let snapshot = harness.doc.snapshot();
            let tree = harness.doc.accessibility_tree();
            let nodes = by_id(&tree);
            for node in snapshot.nodes() {
                let (_, accessible) = tree_node(&tree, &node.id);
                assert_eq!(node.role, accessible.role(), "{task:?}: {:?}", node.id);
                assert_eq!(
                    node.name,
                    name(&nodes, accessible).trim(),
                    "{task:?}: {:?}",
                    node.id
                );
            }
            for (control, role) in controls(task) {
                let read = node(&snapshot, control);
                assert_eq!(read.role, *role, "{task:?}: {control:?}");
                assert!(!read.name.is_empty(), "{task:?}: {control:?} is named");
                if let Some((_, expected)) = INPUT_NAMES.iter().find(|(id, _)| id == control) {
                    assert_eq!(read.name, *expected, "{task:?}: {control:?}");
                }
                checked.insert(*control);
            }
        }
    }
    assert_eq!(checked.len(), 15, "the stand's 15 controls");
}

#[test]
fn tree_follows_the_task_shell() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let snapshot = boot(task, incremental).doc.snapshot();
            assert_eq!(snapshot.roots.len(), 1, "{task:?}: one root");
            assert_eq!(snapshot.roots[0].id, "/html:0", "{task:?}: the root");
            assert!(
                snapshot
                    .nodes()
                    .all(|node| !node.id.starts_with("/html:0/head")),
                "{task:?}: no node descends from <head>"
            );

            let shell = node(&snapshot, "task-shell");
            let header = node(&snapshot, "task-header");
            let body = node(&snapshot, "task-body");
            let shell_children: Vec<&str> = shell.children.iter().map(|n| n.id.as_str()).collect();
            assert_eq!(
                shell_children,
                ["task-header", "task-body"],
                "{task:?}: task-shell nests the header and the body"
            );
            let header_children: Vec<&str> =
                header.children.iter().map(|n| n.id.as_str()).collect();
            assert!(
                header_children.starts_with(&["back-btn", "task-title"]),
                "{task:?}: task-header holds {header_children:?}"
            );
            assert!(
                header.bounds.y < body.bounds.y,
                "{task:?}: the header {:?} sits above the body {:?}",
                header.bounds,
                body.bounds
            );

            if task == LeanTask::Crud {
                let list = node(&snapshot, "crud-list");
                let rows: Vec<&str> = descendants(list)
                    .into_iter()
                    .map(|n| n.id.as_str())
                    .filter(|id| id.contains("/div["))
                    .collect();
                assert_eq!(rows.len(), 3, "{task:?}: three rows in {rows:?}");
                for (row, key) in rows.iter().zip(["div[0]", "div[1]", "div[2]"]) {
                    assert!(row.ends_with(key), "{task:?}: {row:?} ends {key:?}");
                }
            }
        }
    }
}

#[test]
fn controls_lie_inside_the_viewport() {
    let (width, height) = (f64::from(VIEWPORT_WIDTH), f64::from(VIEWPORT_HEIGHT));
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let snapshot = harness.doc.snapshot();
            assert!(
                snapshot.get("flight-booked").is_none(),
                "{task:?}: nothing is booked at boot"
            );
            for id in rendered(task) {
                let rect = node(&snapshot, id).bounds;
                assert!(
                    rect.width > 0.0 && rect.height > 0.0,
                    "{task:?}: {id:?} reads {rect:?}"
                );
                assert!(
                    rect.x >= 0.0
                        && rect.y >= 0.0
                        && rect.x + rect.width <= width
                        && rect.y + rect.height <= height,
                    "{task:?}: {id:?} reads {rect:?} inside {width}x{height}"
                );
                let element = harness.node(&format!("#{id}"));
                assert_eq!(
                    harness.base().get_client_bounding_rect(element),
                    Some(rect),
                    "{task:?}: {id:?} is its element's bounding client rect"
                );
            }
        }
    }
}

#[test]
fn state_reads_the_engine() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let mut harness = boot(task, incremental);
            let snapshot = harness.doc.snapshot();
            assert!(
                snapshot.nodes().all(|node| !node.state.focused),
                "{task:?}: nothing is focused at boot"
            );
            for node in snapshot.nodes().filter(|node| node.role == Role::Button) {
                assert_eq!(node.state.checked, None, "{task:?}: {:?}", node.id);
                assert_eq!(node.state.value, None, "{task:?}: {:?}", node.id);
            }

            let moved = harness
                .base_mut()
                .focus_next_node()
                .expect("focus moves to the first control");
            let snapshot = harness.doc.snapshot();
            let focused: Vec<&str> = snapshot
                .nodes()
                .filter(|node| node.state.focused)
                .map(|node| node.id.as_str())
                .collect();
            assert_eq!(focused, ["back-btn"], "{task:?}: one focused node");
            assert_eq!(
                harness.doc.element_id(moved).as_deref(),
                Some("back-btn"),
                "{task:?}: the node focus moved to"
            );
        }

        if task == LeanTask::FlightBooker {
            for incremental in [false, true] {
                let harness = boot(task, incremental);
                let snapshot = harness.doc.snapshot();
                let return_date = node(&snapshot, "flight-return-date");
                assert_eq!(return_date.state.enabled, Some(false), "one-way at boot");
                let start = node(&snapshot, "flight-start");
                assert_eq!(start.state.enabled, Some(true));
                let element = harness.node("#flight-start");
                let editor = harness
                    .base()
                    .get_node(element)
                    .and_then(|n| n.element_data())
                    .and_then(|el| el.text_input_data())
                    .map(|data| data.editor.text().to_string())
                    .expect("flight-start is a text input");
                assert!(!editor.is_empty(), "the fixture fills the start date");
                assert_eq!(start.state.value.as_deref(), Some(editor.as_str()));
            }
        }
    }
}

#[test]
fn snapshot_is_deterministic() {
    for task in LeanTask::ALL {
        let mut modes = Vec::new();
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let first = harness.doc.snapshot();
            assert!(
                first.nodes().count() > 0,
                "{task:?}: the snapshot has nodes"
            );
            assert_eq!(first, harness.doc.snapshot(), "{task:?}: a second read");
            modes.push(first);
        }
        assert_eq!(modes[0], modes[1], "{task:?}: incremental false and true");
    }
}

#[test]
fn snapshot_follows_a_rerender() {
    for incremental in [false, true] {
        let mut counter = boot(LeanTask::Counter, incremental);
        assert_eq!(node(&counter.doc.snapshot(), "counter-value").name, "0");
        counter.click("#counter-increment");
        assert_eq!(node(&counter.doc.snapshot(), "counter-value").name, "1");

        let mut crud = boot(LeanTask::Crud, incremental);
        let row = |snapshot: &Snapshot| {
            descendants(node(snapshot, "crud-list"))
                .into_iter()
                .any(|n| n.id.ends_with("/div[3]"))
        };
        assert!(!row(&crud.doc.snapshot()), "no fourth row at boot");
        crud.click("#crud-create");
        assert!(
            row(&crud.doc.snapshot()),
            "Create added a row under crud-list"
        );
    }
}
