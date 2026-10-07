//! Every element of each lean stand task, booted headlessly in both layout modes, reads exactly
//! one stable element id, by the grammar's four tiers: an element the author keyed with an HTML
//! `id` reads that key (a CRUD row included); an unkeyed element under a keyed element of its own
//! component reads an anchored path, `{key}//{segment}`; every other element a component renders
//! reads its component path; and an element outside every component reads its document path.

use std::collections::HashSet;

use blitz_dom::local_name;
use blitz_test_harness::Harness;
use dioxus_native_dom::{DioxusDocument, NodeId};
use seven_guis::stand::{self, LeanTask};

const CHROME_KEYS: [&str; 7] = [
    "main",
    "task-shell",
    "task-header",
    "back-btn",
    "task-title",
    "task-header-spacer",
    "task-body",
];

const PINNED_PATHS: [&str; 4] = [
    "/html:0",
    "/html:0/head:0",
    "/html:0/body:0",
    "TaskShell/style:0",
];

fn task_keys(task: LeanTask) -> &'static [&'static str] {
    match task {
        LeanTask::Counter => &["counter-value", "counter-increment"],
        LeanTask::FlightBooker => &[
            "flight-one-way",
            "flight-return",
            "flight-start",
            "flight-return-date",
            "flight-book",
            "flight-booked",
        ],
        LeanTask::Timer => &[
            "timer-progress",
            "timer-elapsed",
            "timer-duration",
            "timer-duration-value",
            "timer-reset",
        ],
        LeanTask::Crud => &[
            "crud-filter",
            "crud-list",
            "crud-person-0",
            "crud-person-1",
            "crud-person-2",
            "crud-name",
            "crud-surname",
            "crud-create",
            "crud-update",
            "crud-delete",
        ],
    }
}

fn component(task: LeanTask) -> &'static str {
    match task {
        LeanTask::Counter => "Counter",
        LeanTask::FlightBooker => "FlightBooker",
        LeanTask::Timer => "Timer",
        LeanTask::Crud => "Crud",
    }
}

/// Boot `task`; the flight booker books its default one-way date so `#flight-booked` mounts.
fn boot(task: LeanTask, incremental: bool) -> Harness<DioxusDocument> {
    let mut harness = stand::boot(task, stand::options(incremental));
    if task == LeanTask::FlightBooker {
        harness.click("#flight-book");
        assert!(
            harness.query("#flight-booked").is_some(),
            "Book was pressed"
        );
    }
    harness
}

/// Every element node reachable from the document root, in pre-order, read straight off the DOM.
fn dom_elements(harness: &Harness<DioxusDocument>) -> Vec<NodeId> {
    let doc = harness.base();
    let mut elements = Vec::new();
    let mut stack = vec![doc.root_node().id];
    while let Some(id) = stack.pop() {
        let node = doc.get_node(id).expect("a reachable node resolves");
        if node.is_element() {
            elements.push(id);
        }
        stack.extend(node.children.iter().rev());
    }
    elements
}

/// Each element with its HTML `id` value, when that value satisfies the author-key rule:
/// non-empty, no `/`, and not claimed by an earlier element.
fn author_keys(harness: &Harness<DioxusDocument>) -> Vec<(NodeId, Option<String>)> {
    let mut claimed = HashSet::new();
    dom_elements(harness)
        .into_iter()
        .map(|node| {
            let value = harness
                .base()
                .get_node(node)
                .and_then(|n| n.attr(local_name!("id")).map(str::to_string));
            let key = value
                .filter(|v| !v.is_empty() && !v.contains('/'))
                .filter(|v| claimed.insert(v.clone()));
            (node, key)
        })
        .collect()
}

#[track_caller]
fn assert_contains(ids: &[String], expected: &str, task: LeanTask) {
    assert!(
        ids.iter().any(|id| id == expected),
        "{task:?}: expected id {expected:?} in {ids:?}"
    );
}

#[test]
fn every_element_reads_exactly_one_unique_id() {
    for task in LeanTask::ALL {
        let mut per_mode = Vec::new();
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let elements = dom_elements(&harness);
            assert!(!elements.is_empty(), "{task:?}: the stand has elements");

            let ids = harness.doc.element_ids();
            let nodes: Vec<NodeId> = ids.iter().map(|(node, _)| *node).collect();
            assert_eq!(nodes, elements, "{task:?}: one id per element, in order");

            let mut seen = HashSet::new();
            for (node, id) in &ids {
                assert_eq!(harness.doc.element_id(*node).as_ref(), Some(id), "{task:?}");
                assert!(seen.insert(id.clone()), "{task:?}: id {id:?} read twice");
            }
            per_mode.push(ids.into_iter().map(|(_, id)| id).collect::<Vec<_>>());
        }
        assert_eq!(per_mode[0], per_mode[1], "{task:?}: both layout modes");
    }
}

#[test]
fn author_keyed_elements_read_their_key() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = boot(task, incremental);
            let mut read = Vec::new();
            for (node, key) in author_keys(&harness) {
                let id = harness
                    .doc
                    .element_id(node)
                    .expect("an element reads an id");
                if let Some(key) = key {
                    assert_eq!(id, key, "{task:?}: a keyed element reads its key");
                }
                read.push(id);
            }
            for key in CHROME_KEYS.iter().chain(task_keys(task)) {
                assert_contains(&read, key, task);
            }
        }
    }
}

/// `task` in one layout mode: every unkeyed element reads a path, the pinned ones among them.
fn unkeyed_elements_read_paths(task: LeanTask, incremental: bool) {
    let harness = boot(task, incremental);
    let mut read = Vec::new();
    for (node, key) in author_keys(&harness) {
        let id = harness
            .doc
            .element_id(node)
            .expect("an element reads an id");
        if key.is_none() {
            assert!(id.contains('/'), "{task:?}: unkeyed element reads {id:?}");
        }
        read.push(id);
    }

    for path in PINNED_PATHS {
        assert_contains(&read, path, task);
    }
    task_body_reads_component_paths(&harness, task);
}

/// Every element under `#task-body` reads one of `task`'s keys or a path under its component.
fn task_body_reads_component_paths(harness: &Harness<DioxusDocument>, task: LeanTask) {
    let prefix = format!("TaskShell/{}/", component(task));
    let body = harness.node("#task-body");
    let doc = harness.base();
    let mut stack = doc.get_node(body).unwrap().children.clone();
    let mut under_body = 0;
    while let Some(id) = stack.pop() {
        let node = doc.get_node(id).unwrap();
        stack.extend(node.children.iter().copied());
        if !node.is_element() {
            continue;
        }
        under_body += 1;
        let element_id = harness.doc.element_id(id).unwrap();
        let keyed = task_keys(task).contains(&element_id.as_str());
        assert!(
            keyed || element_id.starts_with(&prefix),
            "{task:?}: {element_id:?} under #task-body"
        );
    }
    assert!(
        under_body > 0,
        "{task:?}: the task renders under #task-body"
    );
}

/// The Counter's unkeyed wrappers read their component paths, and a CRUD row reads its key.
fn counter_wrappers_and_a_crud_row(incremental: bool) {
    let counter = boot(LeanTask::Counter, incremental);
    let ids: Vec<String> = counter
        .doc
        .element_ids()
        .into_iter()
        .map(|(_, id)| id)
        .collect();
    assert_contains(&ids, "TaskShell/Counter/div:0", LeanTask::Counter);
    assert_contains(&ids, "TaskShell/Counter/div:0/div:0", LeanTask::Counter);

    let crud = boot(LeanTask::Crud, incremental);
    let first_row = crud.query_all(".list > .list-item")[0];
    assert_eq!(
        crud.doc.element_id(first_row).as_deref(),
        Some("crud-person-0")
    );
}

#[test]
fn unkeyed_elements_read_their_component_path() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            unkeyed_elements_read_paths(task, incremental);
        }
    }

    for incremental in [false, true] {
        counter_wrappers_and_a_crud_row(incremental);
    }
}

#[test]
fn text_and_removed_nodes_read_no_id() {
    for incremental in [false, true] {
        let mut harness = boot(LeanTask::Crud, incremental);
        let rows_before = harness.query_all(".list > .list-item");
        assert_eq!(rows_before.len(), 3, "the fixture starts with three rows");

        let text = harness.base().get_node(rows_before[0]).unwrap().children[0];
        assert!(harness.base().get_node(text).unwrap().is_text_node());
        assert_eq!(harness.doc.element_id(text), None);

        harness.click(".list > .list-item");
        assert!(
            harness.attr("#crud-delete", "disabled").is_none(),
            "a row is selected"
        );
        harness.click("#crud-delete");
        harness.pump();

        let rows_after = harness.query_all(".list > .list-item");
        assert_eq!(rows_after.len(), 2);
        let removed: Vec<NodeId> = rows_before
            .into_iter()
            .filter(|row| !rows_after.contains(row))
            .collect();
        assert_eq!(removed.len(), 1, "one row node left the list");
        assert_eq!(harness.doc.element_id(removed[0]), None);
        assert!(
            harness
                .doc
                .element_ids()
                .iter()
                .all(|(node, _)| *node != removed[0])
        );
    }
}
