//! The accessibility tree of each lean stand task, booted headlessly in both layout modes,
//! carries every element's stable element id as the AccessKit `author_id` of that element's
//! node, and no id on any other node; every interactive control has a role and a non-empty
//! accessible name; the ids hold across a re-render; and the Tab order is unchanged.
//!
//! Every node of an interactive role reads a non-empty name, the CRUD list its stated one. The
//! tree the shell hands the platform adapter — `Document::accessibility_tree` called through
//! `&dyn Document`, as `AccessibilityState::update_tree` calls it — is the override's tree, its
//! every `author_id` its node's stable element id and none an engine or framework handle; and a
//! focus move and a rewritten text each leave the changed set non-empty, the shell's condition
//! for rebuilding that tree, with the tree then reading them. The newer checks' failure messages
//! carry a layout mode, a task and an index, never an id or what a screen reads.

use std::collections::{HashMap, HashSet};

use accesskit::{NodeId as TreeNodeId, Role, TreeUpdate};
use blitz_dom::Document;
use blitz_test_harness::Harness;
use dioxus_native_dom::DioxusDocument;
use keyboard_types::{Key, NamedKey};
use seven_guis::stand::LeanTask;

mod common;
use common::{INPUT_NAMES, REGION_NAMES, boot, by_id, controls, name, tab_order};

/// The roles an element takes input in, each of whose nodes must read a name.
const INTERACTIVE_ROLES: [Role; 9] = [
    Role::Button,
    Role::TextInput,
    Role::Slider,
    Role::ListBox,
    Role::ListBoxOption,
    Role::CheckBox,
    Role::RadioButton,
    Role::ComboBox,
    Role::Link,
];

/// The tree the shell builds for `harness`'s document, through the trait object it holds.
fn shell_tree(harness: &Harness<DioxusDocument>) -> TreeUpdate {
    let document: &dyn Document = &harness.doc;
    document.accessibility_tree()
}

fn mode(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
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
                let (_, node) = common::tree_node(&tree, control);
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
    assert_eq!(checked.len(), 14, "the stand's 14 controls");
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
        let (created, _) = common::tree_node(&tree, "crud-person-3");
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

#[test]
fn every_interactive_role_element_reads_a_name() {
    for incremental in [false, true] {
        let mode = mode(incremental);
        let mut named = HashSet::new();
        for task in LeanTask::ALL {
            let harness = boot(task, incremental);
            let tree = harness.doc.accessibility_tree();
            let nodes = by_id(&tree);
            let mut interactive = 0;
            for (index, (_, node)) in tree.nodes.iter().enumerate() {
                if !INTERACTIVE_ROLES.contains(&node.role()) {
                    continue;
                }
                interactive += 1;
                assert!(
                    !name(&nodes, node).trim().is_empty(),
                    "{mode}, {task:?}, tree node {index}: an interactive node is named"
                );
                named.extend(node.author_id().map(str::to_string));
            }
            assert!(
                interactive >= controls(task).len(),
                "{mode}, {task:?}: every control is among the interactive nodes"
            );

            let snapshot = harness.doc.snapshot();
            for (index, (id, expected)) in REGION_NAMES.iter().enumerate() {
                let Some(read) = snapshot.get(id) else {
                    continue;
                };
                let (_, node) = common::tree_node(&tree, id);
                assert!(
                    node.role() == Role::ListBox,
                    "{mode}, {task:?}, region {index}: the region is a list box"
                );
                assert!(
                    name(&nodes, node).trim() == *expected,
                    "{mode}, {task:?}, region {index}: the tree reads the stated name"
                );
                assert!(
                    read.name == *expected,
                    "{mode}, {task:?}, region {index}: the snapshot reads the stated name"
                );
            }
        }
        for (index, (id, _)) in REGION_NAMES.iter().enumerate() {
            assert!(
                named.contains(*id),
                "{mode}, region {index}: the region is among the named interactive nodes"
            );
        }
    }
}

#[test]
fn the_tree_the_shell_hands_the_adapter_carries_stable_ids_only() {
    for incremental in [false, true] {
        let mode = mode(incremental);
        for task in LeanTask::ALL {
            let harness = boot(task, incremental);
            let handed = shell_tree(&harness);
            let overridden = harness.doc.accessibility_tree();
            assert!(
                handed.focus == overridden.focus && handed.tree == overridden.tree,
                "{mode}, {task:?}: the handed tree has the override's root and focus"
            );
            assert!(
                by_id(&handed) == by_id(&overridden)
                    && handed.nodes.len() == overridden.nodes.len(),
                "{mode}, {task:?}: the handed tree is the override's, node for node"
            );

            let mut carried = 0;
            for (index, (tree_id, node)) in handed.nodes.iter().enumerate() {
                let Some(author_id) = node.author_id() else {
                    continue;
                };
                carried += 1;
                let element = blitz_traits::node_id::NodeId::from_u64(tree_id.0);
                assert!(
                    harness.doc.element_id(element).as_deref() == Some(author_id),
                    "{mode}, {task:?}, tree node {index}: the author id is the stable element id"
                );
                assert!(
                    !author_id.is_empty()
                        && !author_id.chars().all(|c| c.is_ascii_digit())
                        && !author_id.contains("0x"),
                    "{mode}, {task:?}, tree node {index}: the author id is no handle"
                );
            }
            assert!(
                carried >= controls(task).len(),
                "{mode}, {task:?}: the handed tree carries ids"
            );
        }
    }
}

/// The stable id of the element the tree reports as its focus, if any.
fn tree_focus(tree: &TreeUpdate) -> Option<String> {
    tree.nodes
        .iter()
        .find(|(id, _)| *id == tree.focus)
        .and_then(|(_, node)| node.author_id())
        .map(str::to_string)
}

#[test]
fn a_focus_move_and_a_rewritten_name_reach_the_tree_the_shell_builds() {
    for incremental in [false, true] {
        let mode = mode(incremental);
        let mut counter = boot(LeanTask::Counter, incremental);
        let sequence = tab_order(LeanTask::Counter);

        for (index, stop) in sequence.iter().enumerate() {
            counter.base_mut().take_changed_nodes();
            assert!(
                !counter.base().has_changes(),
                "{mode}, Counter, Tab {index}: the drained set is empty"
            );
            counter.press(Key::Named(NamedKey::Tab));
            assert!(
                counter.base().has_changes(),
                "{mode}, Counter, Tab {index}: the focus move marks the changed set"
            );
            assert!(
                tree_focus(&shell_tree(&counter)).as_deref() == Some(*stop),
                "{mode}, Counter, Tab {index}: the shell's tree reads the new focus"
            );
        }

        let value = |harness: &Harness<DioxusDocument>| {
            let tree = shell_tree(harness);
            let nodes = by_id(&tree);
            let (_, node) = common::tree_node(&tree, "counter-value");
            name(&nodes, node)
        };
        let before = value(&counter);
        counter.base_mut().take_changed_nodes();
        assert!(
            !counter.base().has_changes(),
            "{mode}, Counter: the drained set is empty before Enter"
        );
        counter.press(Key::Named(NamedKey::Enter));
        assert!(
            counter.base().has_changes(),
            "{mode}, Counter: the rewritten count marks the changed set"
        );
        let after = value(&counter);
        assert!(
            before.trim() == "0" && after.trim() == "1",
            "{mode}, Counter: the shell's tree reads the new count"
        );
    }
}
