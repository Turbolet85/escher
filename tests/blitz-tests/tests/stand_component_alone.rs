//! A component of the stand rendered alone, with the props its caller gives, through
//! `stand::boot_alone` at the stand's options, read against the same component inside its app, in
//! both layout modes. `TaskShell` given the stand's own props (title `Counter`, a no-op Back and
//! `Counter {}` as its children) reads the stand's Counter boot node for node — id, role, name,
//! state and bounds within 1/64 px — and the same UI graph. `Counter` alone reads every in-app
//! node of its subtree with the shell's `TaskShell/` segment gone from its positional ids, the
//! author-keyed ids unchanged, roles, names and states equal and its content-sized boxes at their
//! in-app sizes; its graph state `count` names `Counter` as its component and its one reader; a
//! click on its button moves its value as on the stand. The isolated document reads the pinned
//! 800 × 600 viewport with a font that paints text, and the Timer alone reads its ticks from the
//! root context the stand's root provides. A failure message names a layout mode and a row only.

use blitz_dom::BoundingRect;
use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, SnapshotNode};
use seven_guis::app::{TaskShell, TaskShellProps};
use seven_guis::stand::{self, LeanTask};
use seven_guis::tasks::counter::Counter;
use seven_guis::tasks::timer::{Timer, TimerTicks};

use blitz_test_harness::Harness;

/// How far apart two bounds' edges may read, inclusive: 1/64 px, the layout's own snapping.
const EDGE_TOLERANCE: f64 = 1.0 / 64.0;

/// The in-app Counter subtree's nodes, in pre-order: the in-app id, the id alone, and the size
/// alone where the box sizes by its content (measured at `tests/blitz-tests/checks/counter-cli.json`
/// line 3), `None` where it sizes by its container.
const COUNTER_ROWS: [(&str, &str, Option<(f64, f64)>); 4] = [
    ("TaskShell/Counter/div:0", "Counter/div:0", None),
    ("TaskShell/Counter/div:0/div:0", "Counter/div:0/div:0", None),
    ("counter-value", "counter-value", Some((60.0, 28.0))),
    (
        "counter-increment",
        "counter-increment",
        Some((95.515625, 39.0)),
    ),
];

fn boot_shell_alone(incremental: bool) -> Harness<DioxusDocument> {
    stand::boot_alone(
        TaskShell,
        || TaskShellProps {
            title: "Counter",
            on_back: EventHandler::new(|_| {}),
            children: rsx! {
                Counter {}
            },
        },
        stand::options(incremental),
    )
}

fn pre_order(node: &SnapshotNode) -> Vec<&SnapshotNode> {
    let mut nodes = vec![node];
    for child in &node.children {
        nodes.extend(pre_order(child));
    }
    nodes
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() <= EDGE_TOLERANCE
}

fn same_bounds(a: &BoundingRect, b: &BoundingRect) -> bool {
    near(a.x, b.x) && near(a.y, b.y) && near(a.width, b.width) && near(a.height, b.height)
}

#[test]
fn the_shell_given_the_stands_props_reads_the_stands_counter_node_for_node() {
    for incremental in [false, true] {
        let alone = boot_shell_alone(incremental);
        let in_app = stand::boot(LeanTask::Counter, stand::options(incremental));
        let (alone_snapshot, app_snapshot) = (alone.doc.snapshot(), in_app.doc.snapshot());
        let alone_nodes: Vec<_> = alone_snapshot.nodes().collect();
        let app_nodes: Vec<_> = app_snapshot.nodes().collect();
        assert!(
            app_nodes
                .iter()
                .any(|node| node.id == "TaskShell/Counter/div:0"),
            "incremental={incremental}: the stand's Counter is in its shell"
        );
        assert!(
            alone_nodes.len() == app_nodes.len(),
            "incremental={incremental}: one node alone per node in the app"
        );
        for (row, (a, b)) in alone_nodes.iter().zip(&app_nodes).enumerate() {
            assert!(
                a.id == b.id && a.role == b.role && a.name == b.name && a.state == b.state,
                "incremental={incremental}: row {row} reads its id, role, name and state"
            );
            assert!(
                same_bounds(&a.bounds, &b.bounds),
                "incremental={incremental}: row {row} reads its bounds"
            );
        }
        assert!(
            alone.doc.graph() == in_app.doc.graph(),
            "incremental={incremental}: the UI graph reads as the stand's"
        );
    }
}

#[test]
fn the_counter_alone_reads_its_in_app_subtree_without_the_shell() {
    for incremental in [false, true] {
        let alone = stand::boot_alone(Counter, || (), stand::options(incremental));
        let in_app = stand::boot(LeanTask::Counter, stand::options(incremental));
        let (alone_snapshot, app_snapshot) = (alone.doc.snapshot(), in_app.doc.snapshot());
        let app_root = app_snapshot
            .get(COUNTER_ROWS[0].0)
            .expect("the stand's Counter subtree is mounted");
        let alone_root = alone_snapshot
            .get(COUNTER_ROWS[0].1)
            .expect("the Counter alone is mounted");
        let app_nodes = pre_order(app_root);
        let alone_nodes = pre_order(alone_root);
        assert_eq!(COUNTER_ROWS.len(), 4);
        assert!(
            app_nodes.len() == COUNTER_ROWS.len() && alone_nodes.len() == COUNTER_ROWS.len(),
            "incremental={incremental}: the subtree holds one node per row"
        );
        for (row, (in_app_id, alone_id, size)) in COUNTER_ROWS.into_iter().enumerate() {
            let (a, b) = (alone_nodes[row], app_nodes[row]);
            assert!(
                b.id == in_app_id && a.id == alone_id,
                "incremental={incremental}: row {row} reads its ids"
            );
            assert!(
                a.role == b.role && a.name == b.name && a.state == b.state,
                "incremental={incremental}: row {row} reads its role, name and state"
            );
            if let Some((width, height)) = size {
                assert!(
                    near(a.bounds.width, width)
                        && near(a.bounds.height, height)
                        && near(b.bounds.width, width)
                        && near(b.bounds.height, height),
                    "incremental={incremental}: row {row} reads its in-app size"
                );
            }
        }
        let graph = alone.doc.graph();
        let count = graph
            .states
            .iter()
            .find(|state| state.name == "count")
            .expect("the Counter registers count");
        assert!(
            count.component.as_deref() == Some("Counter") && count.readers == ["Counter"],
            "incremental={incremental}: count names Counter as its component and its one reader"
        );
    }
}

#[test]
fn a_click_alone_moves_the_counter_as_on_the_stand() {
    for incremental in [false, true] {
        let mut alone = stand::boot_alone(Counter, || (), stand::options(incremental));
        let value = |harness: &Harness<DioxusDocument>| {
            harness
                .doc
                .snapshot()
                .get("counter-value")
                .map(|node| node.name.clone())
        };
        assert!(
            value(&alone).as_deref() == Some("0"),
            "incremental={incremental}: the value reads 0 at boot"
        );
        alone.click("#counter-increment");
        assert!(
            value(&alone).as_deref() == Some("1"),
            "incremental={incremental}: the value reads 1 after a click"
        );
    }
}

#[test]
fn the_isolated_document_keeps_the_stands_viewport_and_font() {
    for incremental in [false, true] {
        let alone = stand::boot_alone(Counter, || (), stand::options(incremental));
        let viewport = alone.base().get_viewport();
        assert!(
            viewport.window_size == (800, 600) && viewport.hidpi_scale == 1.0,
            "incremental={incremental}: the document reads the 800 x 600 viewport at scale 1"
        );
        assert!(
            alone.base().can_paint_text(),
            "incremental={incremental}: the bundled font paints text"
        );
    }
}

#[test]
fn the_timer_alone_reads_its_ticks_from_the_root_context() {
    for incremental in [false, true] {
        let mut alone = stand::boot_alone(Timer, || (), stand::options(incremental));
        assert!(
            alone.text_content("#timer-elapsed") == "Elapsed: 0.0s",
            "incremental={incremental}: the Timer alone reads no time at boot"
        );
        let ticks = alone
            .doc
            .vdom
            .runtime()
            .has_context::<TimerTicks>(ScopeId::ROOT)
            .expect("the root provides the Timer's ticks");
        ticks.deliver(3);
        alone.pump();
        assert!(
            alone.text_content("#timer-elapsed") == "Elapsed: 0.3s",
            "incremental={incremental}: three ticks read 0.3s"
        );
    }
}
