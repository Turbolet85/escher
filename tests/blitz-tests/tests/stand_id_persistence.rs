//! A lean stand task's stable element ids persist: every element keeps the id it read before a
//! re-render, a remounted task reads the same ids on fresh nodes, and a second process booting
//! the stand reads the same ids as the first, in both layout modes.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::process::Command;

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, NodeId};
use seven_guis::stand::{self, LeanTask};
use seven_guis::tasks::timer::TimerTicks;

const ROWS: &str = ".list > .list-item";
const ROW_KEY: &str = "crud-person-";
const CHILD_LINE: &str = "stand-id";

fn ids(harness: &Harness<DioxusDocument>) -> Vec<(NodeId, String)> {
    harness.doc.element_ids()
}

fn id_strings(ids: &[(NodeId, String)]) -> Vec<String> {
    ids.iter().map(|(_, id)| id.clone()).collect()
}

/// Every node present both before and after reads the same id; returns how many survived.
#[track_caller]
fn assert_survivors_keep_ids(
    before: &[(NodeId, String)],
    after: &[(NodeId, String)],
    what: &str,
) -> usize {
    let before: HashMap<NodeId, &String> = before.iter().map(|(n, id)| (*n, id)).collect();
    let mut survivors = 0;
    for (node, id) in after {
        if let Some(old) = before.get(node) {
            assert_eq!(*old, id, "{what}: a surviving element changed its id");
            survivors += 1;
        }
    }
    assert!(survivors > 0, "{what}: elements survived the re-render");
    survivors
}

/// Each CRUD row's text (`"{last}, {first}"`) with its id.
fn row_ids(harness: &Harness<DioxusDocument>) -> BTreeMap<String, String> {
    harness
        .query_all(ROWS)
        .into_iter()
        .map(|row| {
            let text = harness.base().get_node(row).unwrap().text_content();
            let id = harness.doc.element_id(row).expect("a row reads an id");
            (text, id)
        })
        .collect()
}

fn row_key(key: u64) -> String {
    format!("{ROW_KEY}{key}")
}

/// Run `rerender` on `harness`, asserting the DOM changed and every surviving element kept its id.
fn rerender(
    harness: &mut Harness<DioxusDocument>,
    what: &str,
    act: impl FnOnce(&mut Harness<DioxusDocument>),
) -> Vec<(NodeId, String)> {
    let before = ids(harness);
    let dom_before = harness.dom_string();
    act(harness);
    assert_ne!(harness.dom_string(), dom_before, "{what}: the DOM changed");
    let after = ids(harness);
    assert_survivors_keep_ids(&before, &after, what);
    after
}

#[test]
fn ids_hold_across_rerenders() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let mut counter = stand::boot(LeanTask::Counter, stand::options(incremental));
        rerender(&mut counter, &format!("counter {mode}"), |h| {
            h.click("#counter-increment");
            h.click("#counter-increment");
            assert_eq!(h.text_content("#counter-value"), "2");
        });

        let mut flight = stand::boot(LeanTask::FlightBooker, stand::options(incremental));
        let first = ids(&flight);
        assert!(flight.attr("#flight-return-date", "disabled").is_some());
        rerender(&mut flight, &format!("flight return {mode}"), |h| {
            h.click("#flight-return");
            assert!(h.attr("#flight-return-date", "disabled").is_none());
        });
        assert!(flight.query("#flight-booked").is_none());
        let booked = rerender(&mut flight, &format!("flight book {mode}"), |h| {
            h.click("#flight-book")
        });
        let inserted = flight.node("#flight-booked");
        assert_eq!(
            flight.doc.element_id(inserted).as_deref(),
            Some("flight-booked")
        );
        assert_eq!(
            assert_survivors_keep_ids(&first, &booked, &format!("flight {mode}")),
            first.len(),
            "flight {mode}: every element from before the toggle survived"
        );

        let (mut timer, ticks) = stand::boot_timer(stand::options(incremental));
        rerender(&mut timer, &format!("timer {mode}"), |h| {
            ticks.deliver(5);
            h.pump();
            assert_eq!(h.text_content("#timer-elapsed"), "Elapsed: 0.5s");
        });

        crud_rows_follow_their_person(incremental);
    }
}

fn crud_rows_follow_their_person(incremental: bool) {
    let mode = format!("incremental={incremental}");
    let fixture = BTreeMap::from([
        ("Emil, Hans".to_string(), row_key(0)),
        ("Mustermann, Max".to_string(), row_key(1)),
        ("Tisch, Roman".to_string(), row_key(2)),
    ]);

    let mut crud = stand::boot(LeanTask::Crud, stand::options(incremental));
    assert_eq!(row_ids(&crud), fixture, "crud {mode}: fixture rows");
    rerender(&mut crud, &format!("crud filter {mode}"), |h| {
        h.click("#crud-filter");
        h.type_text("M");
    });
    assert_eq!(
        row_ids(&crud),
        BTreeMap::from([("Mustermann, Max".to_string(), row_key(1))]),
        "crud filter {mode}: the filtered row keeps its id"
    );

    let mut crud = stand::boot(LeanTask::Crud, stand::options(incremental));
    rerender(&mut crud, &format!("crud create {mode}"), |h| {
        h.click("#crud-name");
        h.type_text("Ada");
        h.click("#crud-surname");
        h.type_text("Lovelace");
        h.click("#crud-create");
    });
    let mut created = fixture.clone();
    created.insert("Lovelace, Ada".to_string(), row_key(3));
    assert_eq!(
        row_ids(&crud),
        created,
        "crud create {mode}: a new person reads a fresh id, the others keep theirs"
    );

    rerender(&mut crud, &format!("crud select {mode}"), |h| {
        h.click(&format!("{ROWS}:nth-child(1)"))
    });
    assert_eq!(row_ids(&crud), created, "crud select {mode}");
    assert!(crud.attr("#crud-delete", "disabled").is_none());
    assert_eq!(crud.attr("#crud-name", "value").as_deref(), Some("Hans"));
    rerender(&mut crud, &format!("crud delete {mode}"), |h| {
        h.click("#crud-delete")
    });
    let after_delete = row_ids(&crud);
    assert_eq!(
        after_delete,
        BTreeMap::from([
            ("Mustermann, Max".to_string(), row_key(1)),
            ("Tisch, Roman".to_string(), row_key(2)),
            ("Lovelace, Ada".to_string(), row_key(3)),
        ]),
        "crud delete {mode}: the remaining people keep their ids"
    );
    assert!(!after_delete.values().any(|id| *id == row_key(0)));
}

fn app_root() -> Element {
    use_hook(|| provide_context(TimerTicks::default()));
    seven_guis::app::app()
}

/// The document skeleton `DioxusDocument` builds outside the VirtualDom; it outlives any remount.
const SKELETON: [&str; 4] = ["/html:0", "/html:0/head:0", "/html:0/body:0", "main"];

/// Mount `task` from the app root through its Home card, leave it and mount it again: it reads
/// the stand's ids both times, on fresh nodes the second time.
fn a_remounted_task_reads_the_same_ids(incremental: bool, task: LeanTask, nth: usize) {
    let what = format!("{task:?} incremental={incremental}");
    let card = format!("#task-grid > .task-card:nth-child({nth})");
    let mut harness = Harness::from_vdom(VirtualDom::new(app_root), stand::options(incremental));

    harness.click(&card);
    assert!(harness.query("#task-shell").is_some(), "{what}: mounted");
    let before = ids(&harness);
    let stand_ids = id_strings(&ids(&stand::boot(task, stand::options(incremental))));
    assert_eq!(id_strings(&before), stand_ids, "{what}: app root = stand");

    harness.click("#back-btn");
    assert!(harness.query("#task-shell").is_none(), "{what}: unmounted");
    assert!(harness.query("#home").is_some(), "{what}: home");

    harness.click(&card);
    let after = ids(&harness);
    assert_eq!(id_strings(&after), id_strings(&before), "{what}: same ids");

    only_the_skeleton_outlives_the_remount(&harness, &before, &after, &what);
}

/// Of the nodes read `before` the remount only the skeleton is among those read `after` it:
/// every other node is fresh, and a dropped node reads no id and holds no focus.
fn only_the_skeleton_outlives_the_remount(
    harness: &Harness<DioxusDocument>,
    before: &[(NodeId, String)],
    after: &[(NodeId, String)],
    what: &str,
) {
    let before_nodes: HashSet<NodeId> = before.iter().map(|(n, _)| *n).collect();
    let mut kept = Vec::new();
    let mut fresh = 0;
    for (node, id) in after {
        if before_nodes.contains(node) {
            kept.push(id.as_str());
        } else {
            fresh += 1;
        }
    }
    assert_eq!(
        kept, SKELETON,
        "{what}: only the skeleton outlived the remount"
    );
    assert_eq!(fresh, after.len() - SKELETON.len());
    assert!(fresh > 0, "{what}: the remount minted fresh nodes");

    let dropped: Vec<NodeId> = before
        .iter()
        .map(|(n, _)| *n)
        .filter(|n| !after.iter().any(|(m, _)| m == n))
        .collect();
    assert_eq!(dropped.len(), fresh);
    for node in &dropped {
        assert_eq!(harness.doc.element_id(*node), None, "{what}: stale node");
    }
    if let Some(focused) = harness.focused() {
        assert!(
            !dropped.contains(&focused),
            "{what}: focus on a dropped node"
        );
    }
}

#[test]
fn ids_hold_across_a_remount() {
    let cards = [
        (LeanTask::Counter, 1),
        (LeanTask::FlightBooker, 3),
        (LeanTask::Timer, 4),
        (LeanTask::Crud, 5),
    ];
    for incremental in [false, true] {
        for (task, nth) in cards {
            a_remounted_task_reads_the_same_ids(incremental, task, nth);
        }
    }
}

/// Every lean task's ids at boot, keyed `"{task:?} {incremental}"`; the timer gets no ticks.
fn boot_ids() -> BTreeMap<String, Vec<String>> {
    let mut lists = BTreeMap::new();
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let harness = if task == LeanTask::Timer {
                stand::boot_timer(stand::options(incremental)).0
            } else {
                stand::boot(task, stand::options(incremental))
            };
            lists.insert(
                format!("{task:?} {incremental}"),
                id_strings(&ids(&harness)),
            );
        }
    }
    lists
}

#[test]
fn ids_hold_across_a_fresh_process() {
    let parent = boot_ids();
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "ids_in_a_child_process",
            "--nocapture",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "child failed\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mut child_pid = None;
    let mut child: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in stdout.lines() {
        let mut parts = line.splitn(4, ' ');
        match (parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some("pid"), Some(pid), None, None) => child_pid = pid.parse::<u32>().ok(),
            (Some(CHILD_LINE), Some(task), Some(incremental), Some(id)) => child
                .entry(format!("{task} {incremental}"))
                .or_default()
                .push(id.to_string()),
            _ => {}
        }
    }

    let child_pid = child_pid.expect("the child printed its pid");
    assert_ne!(child_pid, std::process::id(), "a distinct process");
    assert_eq!(child.len(), LeanTask::ALL.len() * 2, "every task and mode");
    assert!(child.values().all(|ids| !ids.is_empty()));
    assert_eq!(child, parent, "the child reads the parent's ids");
}

#[test]
#[ignore = "spawned as a child process by ids_hold_across_a_fresh_process"]
fn ids_in_a_child_process() {
    println!("pid {}", std::process::id());
    for (key, ids) in boot_ids() {
        for id in ids {
            println!("{CHILD_LINE} {key} {id}");
        }
    }
}
