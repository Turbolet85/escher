//! The one statement of the tables and helpers the stand checks share: each lean task's
//! controls, input names and rendered ids, the stand's boot and the lookups over a snapshot, an
//! accessibility tree and a text input's editor. A check declares `mod common;` and reads what
//! it needs. This module holds no test.

#![allow(dead_code)]

use std::collections::HashMap;

use accesskit::{Node, NodeId as TreeNodeId, Role, TreeUpdate};
use blitz_test_harness::Harness;
use dioxus_native_dom::{DioxusDocument, Snapshot, SnapshotNode};
use seven_guis::stand::{self, LeanTask};

/// Each task's interactive controls with their HTML-AAM role, the shell's `back-btn` first.
pub fn controls(task: LeanTask) -> &'static [(&'static str, Role)] {
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
pub const INPUT_NAMES: [(&str, &str); 6] = [
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
pub fn rendered(task: LeanTask) -> &'static [&'static str] {
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

pub fn boot(task: LeanTask, incremental: bool) -> Harness<DioxusDocument> {
    stand::boot(task, stand::options(incremental))
}

/// The node `id` names, asserted present.
#[track_caller]
pub fn node<'s>(snapshot: &'s Snapshot, id: &str) -> &'s SnapshotNode {
    snapshot
        .get(id)
        .unwrap_or_else(|| panic!("{id:?} is a snapshot node"))
}

/// The tree's nodes by id.
pub fn by_id(tree: &TreeUpdate) -> HashMap<TreeNodeId, &Node> {
    tree.nodes.iter().map(|(id, node)| (*id, node)).collect()
}

/// The tree node carrying `author_id`.
#[track_caller]
pub fn tree_node<'t>(tree: &'t TreeUpdate, author_id: &str) -> (TreeNodeId, &'t Node) {
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
pub fn name(nodes: &HashMap<TreeNodeId, &Node>, node: &Node) -> String {
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

/// The text the engine's editor holds for the text input `id`.
#[track_caller]
pub fn editor_text(harness: &Harness<DioxusDocument>, id: &str) -> String {
    let element = harness.node(&format!("#{id}"));
    harness
        .base()
        .get_node(element)
        .and_then(|node| node.element_data())
        .and_then(|element| element.text_input_data())
        .map(|data| data.editor.text().to_string())
        .unwrap_or_else(|| panic!("{id:?} is a text input"))
}
