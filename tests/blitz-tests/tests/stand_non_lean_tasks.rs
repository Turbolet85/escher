//! The tasks outside the lean four, reached through Home in process, in both layout modes:
//! how many elements on each of the seven tasks react to a click and state no role (measured
//! and pinned; the snapshot carries no field for it), whether Circle Drawer's dialog rule sizes
//! its range input, and that the dialog's two controls read their author keys while it is open.

use std::collections::HashMap;

use accesskit::Role;
use blitz_dom::Document;
use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand::{self, LeanTask};
use seven_guis::tasks::timer::TimerTicks;

mod common;
use common::boot;

fn app_root() -> Element {
    use_hook(|| provide_context(TimerTicks::default()));
    seven_guis::app::app()
}

/// Home booted, then the task behind `card` opened by a click on its card, asserted open.
fn open(card: &str, title: &str, incremental: bool) -> Harness<DioxusDocument> {
    let mut harness = Harness::from_vdom(VirtualDom::new(app_root), stand::options(incremental));
    assert!(harness.query("#home").is_some(), "the app opens on Home");
    // Home scrolls, and its last card lies below its box: a click lands only on a card that is
    // in view.
    let card_node = harness.node(&format!("#{card}"));
    harness.scroll_into_view(card_node);
    harness.click(&format!("#{card}"));
    assert_eq!(harness.text_content("#task-title"), title, "{card} opened");
    harness
}

/// The elements whose accessibility node is a generic container and which carry the listener
/// mark: they react to a click and state no role.
fn reacting_without_role(harness: &Harness<DioxusDocument>) -> usize {
    let roles: HashMap<u64, Role> = harness
        .doc
        .accessibility_tree()
        .nodes
        .iter()
        .map(|(id, node)| (id.0, node.role()))
        .collect();
    let doc = harness.base();
    harness
        .doc
        .element_ids()
        .into_iter()
        .filter(|(node_id, _)| {
            let node = doc.get_node(*node_id).expect("an element id names a node");
            let listener = node.element_data().is_some_and(|element| {
                element
                    .attrs
                    .iter()
                    .any(|attr| *attr.name.local == *"data-dioxus-id")
            });
            listener && roles.get(&node_id.as_u64()) == Some(&Role::GenericContainer)
        })
        .count()
}

const NON_LEAN: [(&str, &str); 3] = [
    ("task-card-temp-converter", "Temperature Converter"),
    ("task-card-circle-drawer", "Circle Drawer"),
    ("task-card-cells", "Cells"),
];

/// Circle Drawer opened, a circle added by a click at the canvas centre and the dialog opened
/// by a second click at the same point, read as a double click.
fn circle_drawer_with_dialog(incremental: bool) -> Harness<DioxusDocument> {
    let mut harness = open("task-card-circle-drawer", "Circle Drawer", incremental);
    let (x, y) = harness.center_of("#circle-canvas");
    harness.click_at(x, y);
    assert_eq!(
        harness.query_all(".circle").len(),
        1,
        "a click on the canvas adds a circle"
    );
    // A click within 500 ms of wall time and 2 px of the last one counts as a double click; a
    // third would count 3, so none follows.
    harness.click_at(x, y);
    assert!(
        harness.query("#circle-diameter").is_some(),
        "a double click opens the dialog"
    );
    harness
}

#[test]
fn elements_that_react_to_a_click_and_state_no_role_are_counted() {
    const MEASURED: [(&str, usize); 7] = [
        ("Counter", 0),
        ("Flight Booker", 0),
        ("Timer", 0),
        ("CRUD", 0),
        ("Temperature Converter", 0),
        ("Circle Drawer", 1),
        ("Cells", 676),
    ];
    for incremental in [false, true] {
        let mut measured = Vec::new();
        for task in LeanTask::ALL {
            let harness = boot(task, incremental);
            let title = harness.text_content("#task-title");
            measured.push((title, reacting_without_role(&harness)));
        }
        for (card, title) in NON_LEAN {
            let harness = open(card, title, incremental);
            measured.push((title.to_string(), reacting_without_role(&harness)));
        }
        let measured: Vec<(&str, usize)> = measured
            .iter()
            .map(|(title, count)| (title.as_str(), *count))
            .collect();
        assert_eq!(measured, MEASURED, "incremental={incremental}");
    }
}

#[test]
fn circle_drawers_dialog_rule_applies() {
    for incremental in [false, true] {
        let harness = circle_drawer_with_dialog(incremental);
        assert_eq!(
            harness.layout_rect("#circle-diameter").width,
            240.0,
            "incremental={incremental}"
        );
    }
}

#[test]
fn circle_drawers_open_dialog_reads_every_control_keyed() {
    for incremental in [false, true] {
        let harness = circle_drawer_with_dialog(incremental);
        assert!(
            harness.query("#circle-close").is_some(),
            "the dialog holds its Close button"
        );
        assert!(
            harness.doc.unkeyed_actionable().is_empty(),
            "incremental={incremental}"
        );
    }
}
