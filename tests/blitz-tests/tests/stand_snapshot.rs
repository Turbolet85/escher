//! The snapshot of each lean stand task, booted headlessly in both layout modes: every node
//! is an element the accessibility tree keeps, carrying its stable element id, the role and
//! name of its accessibility node, its state as the engine reads it and its bounding client
//! rect; the tree follows the TaskShell; and the snapshot is deterministic and follows a
//! re-render.

use std::collections::HashSet;

use accesskit::Role;
use blitz_dom::Document;
use blitz_traits::node_id::NodeId;
use dioxus_native_dom::{Snapshot, SnapshotNode};
use seven_guis::stand::{LeanTask, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

mod common;
use common::{INPUT_NAMES, boot, by_id, controls, name, node, rendered, tree_node};

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
                    .filter(|id| id.starts_with("crud-person-"))
                    .collect();
                assert_eq!(
                    rows,
                    ["crud-person-0", "crud-person-1", "crud-person-2"],
                    "{task:?}: three rows under crud-list"
                );
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
                .any(|n| n.id == "crud-person-3")
        };
        assert!(!row(&crud.doc.snapshot()), "no fourth row at boot");
        crud.click("#crud-create");
        assert!(
            row(&crud.doc.snapshot()),
            "Create added a row under crud-list"
        );
    }
}
