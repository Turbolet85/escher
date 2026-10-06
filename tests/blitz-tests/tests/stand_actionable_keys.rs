//! Every element an agent can act on in the stand reads an author key:
//! `DioxusDocument::unkeyed_actionable` returns nothing on the four lean tasks and on Home, booted
//! headlessly in both layout modes, whatever state the screen is in. The three tasks outside
//! the lean four are measured with the same function and their counts pinned.

use std::collections::HashMap;

use accesskit::Role;
use blitz_dom::Document;
use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, UnkeyedActionable};
use seven_guis::stand::{self, LeanTask};
use seven_guis::tasks::timer::TimerTicks;

const CARDS: [&str; 7] = [
    "task-card-counter",
    "task-card-temp-converter",
    "task-card-flight-booker",
    "task-card-timer",
    "task-card-crud",
    "task-card-circle-drawer",
    "task-card-cells",
];

fn app_root() -> Element {
    use_hook(|| provide_context(TimerTicks::default()));
    seven_guis::app::app()
}

fn boot(task: LeanTask, incremental: bool) -> Harness<DioxusDocument> {
    stand::boot(task, stand::options(incremental))
}

fn boot_home(incremental: bool) -> Harness<DioxusDocument> {
    let harness = Harness::from_vdom(VirtualDom::new(app_root), stand::options(incremental));
    assert!(harness.query("#home").is_some(), "the app opens on Home");
    harness
}

/// How many elements are actionable, read off the same three readers the library uses:
/// focusability, the accessibility node's role (the roles the stand's controls carry) and the
/// listener mark.
fn actionable_count(harness: &Harness<DioxusDocument>) -> usize {
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
            let role = matches!(
                roles.get(&node_id.as_u64()),
                Some(Role::Button | Role::TextInput | Role::Slider)
            );
            node.is_focussable() || role || listener
        })
        .count()
}

/// The library check returns nothing on a screen that has actionable elements.
#[track_caller]
fn assert_all_keyed(harness: &Harness<DioxusDocument>, what: &str) {
    let found = harness.doc.unkeyed_actionable();
    let remedies: Vec<String> = found.iter().map(UnkeyedActionable::remedy).collect();
    assert!(found.is_empty(), "{what}: {remedies:#?}");
    assert!(
        actionable_count(harness) > 0,
        "{what}: the screen has actionable elements"
    );
}

#[test]
fn every_actionable_stand_element_reads_an_author_key() {
    for incremental in [false, true] {
        for task in LeanTask::ALL {
            let harness = boot(task, incremental);
            assert_all_keyed(&harness, &format!("{task:?} incremental={incremental}"));
        }

        let home = boot_home(incremental);
        let cards = home.query_all("#task-grid > .task-card");
        assert_eq!(cards.len(), CARDS.len(), "Home's task cards");
        for (card, key) in cards.into_iter().zip(CARDS) {
            assert_eq!(home.doc.element_id(card).as_deref(), Some(key));
        }
        assert!(
            actionable_count(&home) >= CARDS.len(),
            "every card is actionable"
        );
        assert_all_keyed(&home, &format!("Home incremental={incremental}"));
    }
}

#[test]
fn the_verdict_does_not_change_with_state() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let mut flight = boot(LeanTask::FlightBooker, incremental);
        assert!(flight.attr("#flight-book", "disabled").is_none());
        assert_all_keyed(&flight, &format!("flight, Book enabled {mode}"));
        flight.click("#flight-start");
        flight.type_text("x");
        assert!(flight.attr("#flight-book", "disabled").is_some());
        assert_all_keyed(&flight, &format!("flight, Book disabled {mode}"));

        let mut flight = boot(LeanTask::FlightBooker, incremental);
        flight.click("#flight-book");
        assert!(flight.query("#flight-booked").is_some(), "Book was pressed");
        assert_all_keyed(&flight, &format!("flight, booked {mode}"));

        let mut crud = boot(LeanTask::Crud, incremental);
        crud.click("#crud-create");
        assert_eq!(
            crud.query_all(".list > .list-item").len(),
            4,
            "Create added a row"
        );
        assert_all_keyed(&crud, &format!("crud, created {mode}"));
        crud.click("#crud-person-3");
        assert!(
            crud.attr("#crud-delete", "disabled").is_none(),
            "a row is selected"
        );
        assert_all_keyed(&crud, &format!("crud, selected {mode}"));
    }
}

/// The tasks outside the lean four are not keyed yet: their unkeyed actionable elements are
/// counted here so the number is re-measured on every run. Keying them is owed to the working
/// route's CARRY for the three non-lean tasks; when a task is keyed its count drops to 0 here.
#[test]
fn the_non_lean_tasks_are_measured() {
    const CARDS_AND_TITLES: [(&str, &str); 3] = [
        ("task-card-temp-converter", "Temperature Converter"),
        ("task-card-circle-drawer", "Circle Drawer"),
        ("task-card-cells", "Cells"),
    ];
    const UNKEYED: [(&str, usize); 3] = [
        ("Temperature Converter", 2),
        ("Circle Drawer", 3),
        ("Cells", 676),
    ];
    for incremental in [false, true] {
        let mut measured = Vec::new();
        for (card, title) in CARDS_AND_TITLES {
            let mut harness = boot_home(incremental);
            harness.click(&format!("#{card}"));
            assert_eq!(harness.text_content("#task-title"), title, "{card} opened");
            assert!(actionable_count(&harness) > 0, "{title}: has controls");
            measured.push((title, harness.doc.unkeyed_actionable().len()));
        }
        assert_eq!(measured, UNKEYED, "incremental={incremental}");
    }
}
