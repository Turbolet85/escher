//! The state a snapshot node reads is the state of its control, proven through real input on
//! the stand, booted headlessly in both layout modes: a disabled control reads not enabled
//! until the app enables it, typed text reads back as the value, and exactly the focused
//! control reads focused after a click, a Tab press and Shift+Tab. The cases the lean tasks
//! lack are proven on a minimal fixture booted with the stand's options: a checkbox and a
//! radio read checked after a click, and a password input reads a fixed mask, never its text.

use accesskit::{Node, Role};
use blitz_dom::Document;
use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, MASKED_VALUE, UnkeyedActionable};
use keyboard_types::{Key, Modifiers};
use seven_guis::stand::{self, LeanTask};

mod common;
use common::{boot, by_id, editor_text, name, node};

/// What [`focused`] reads when no control has focus.
const NOTHING: [&str; 0] = [];

/// The text typed into the fixture's password input: synthetic, and no part of any name.
const SECRET: &str = "synthetic-pw-7Qz";

/// What `id` reads as enabled, asserted to agree with the presence of its `disabled`
/// attribute.
#[track_caller]
fn enabled(harness: &Harness<DioxusDocument>, id: &str) -> Option<bool> {
    let read = node(&harness.doc.snapshot(), id).state.enabled;
    let disabled = harness.attr(&format!("#{id}"), "disabled").is_some();
    assert_eq!(
        read,
        Some(!disabled),
        "{id:?} follows its disabled attribute"
    );
    read
}

/// What `id` reads as checked.
#[track_caller]
fn checked(harness: &Harness<DioxusDocument>, id: &str) -> Option<bool> {
    node(&harness.doc.snapshot(), id).state.checked
}

/// Whether `id` reads exactly `expected` as its value.
#[track_caller]
fn reads_value(harness: &Harness<DioxusDocument>, id: &str, expected: Option<&str>) -> bool {
    node(&harness.doc.snapshot(), id).state.value.as_deref() == expected
}

/// The ids of the snapshot nodes reading focused, asserted to be the element the
/// accessibility tree reports as its focus: none when that is the `Window`.
#[track_caller]
fn focused(harness: &Harness<DioxusDocument>) -> Vec<String> {
    let snapshot = harness.doc.snapshot();
    let focused: Vec<String> = snapshot
        .nodes()
        .filter(|node| node.state.focused)
        .map(|node| node.id.clone())
        .collect();
    let tree = harness.doc.accessibility_tree();
    let tree_focus: Vec<String> = tree
        .nodes
        .iter()
        .filter(|(id, _)| *id == tree.focus)
        .filter_map(|(_, node)| node.author_id())
        .map(str::to_string)
        .collect();
    assert_eq!(focused, tree_focus, "the accessibility tree's focus");
    focused
}

/// Markup a Dioxus document holds without the bridge writing it: it is parsed, so a `disabled`
/// attribute keeps the value it was written with.
fn parsed_markup() -> Element {
    rsx! {
        div {
            id: "fx-parsed",
            dangerous_inner_html: r#"<button id="fx-falsy" disabled="false">A</button><button id="fx-bare" disabled>B</button>"#,
        }
    }
}

#[test]
fn a_disabled_control_reads_disabled_until_the_app_enables_it() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let mut flight = boot(LeanTask::FlightBooker, incremental);
        assert_eq!(
            node(&flight.doc.snapshot(), "task-title").state.enabled,
            None,
            "{mode}: a heading cannot be disabled"
        );
        assert_eq!(
            enabled(&flight, "flight-return-date"),
            Some(false),
            "{mode}"
        );
        assert_eq!(enabled(&flight, "flight-book"), Some(true), "{mode}");
        flight.click("#flight-return");
        assert_eq!(
            enabled(&flight, "flight-return-date"),
            Some(true),
            "{mode}: a return flight enables the return date"
        );
        assert_eq!(enabled(&flight, "flight-book"), Some(true), "{mode}");
        flight.click("#flight-return-date");
        flight.type_text("x");
        assert_eq!(
            enabled(&flight, "flight-book"),
            Some(false),
            "{mode}: an invalid return date disables Book"
        );
        flight.click("#flight-one-way");
        assert_eq!(
            enabled(&flight, "flight-book"),
            Some(true),
            "{mode}: a one-way flight enables Book again"
        );
        assert_eq!(
            enabled(&flight, "flight-return-date"),
            Some(false),
            "{mode}: a one-way flight disables the return date again"
        );
        flight.click("#flight-start");
        flight.type_text("x");
        assert_eq!(
            enabled(&flight, "flight-book"),
            Some(false),
            "{mode}: an invalid start date disables Book"
        );

        let mut crud = boot(LeanTask::Crud, incremental);
        assert_eq!(
            node(&crud.doc.snapshot(), "task-title").state.enabled,
            None,
            "{mode}: a heading cannot be disabled"
        );
        assert_eq!(enabled(&crud, "crud-update"), Some(false), "{mode}");
        assert_eq!(enabled(&crud, "crud-delete"), Some(false), "{mode}");
        assert_eq!(enabled(&crud, "crud-create"), Some(true), "{mode}");
        crud.click("#crud-person-0");
        assert_eq!(
            enabled(&crud, "crud-update"),
            Some(true),
            "{mode}: a selection enables Update"
        );
        assert_eq!(
            enabled(&crud, "crud-delete"),
            Some(true),
            "{mode}: a selection enables Delete"
        );
        assert_eq!(enabled(&crud, "crud-create"), Some(true), "{mode}");

        let parsed =
            Harness::from_vdom(VirtualDom::new(parsed_markup), stand::options(incremental));
        for id in ["fx-falsy", "fx-bare"] {
            let element = parsed.node(&format!("#{id}"));
            assert!(
                parsed
                    .base()
                    .get_node(element)
                    .is_some_and(|node| node.is_focussable()),
                "{mode}: the focusability reader takes {id:?} as enabled"
            );
            assert_eq!(
                enabled(&parsed, id),
                Some(false),
                "{mode}: a present disabled attribute reads not enabled"
            );
        }
    }
}

#[test]
fn typed_text_reads_back_as_the_value() {
    const TYPED: [(&str, &str); 3] = [
        ("crud-name", "Ada"),
        ("crud-surname", "Lovelace"),
        ("crud-filter", "Mu"),
    ];
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let mut crud = boot(LeanTask::Crud, incremental);
        for (id, _) in TYPED {
            assert!(
                reads_value(&crud, id, Some("")),
                "{mode}: {id:?} is empty at boot"
            );
        }
        for (id, text) in TYPED {
            crud.click(&format!("#{id}"));
            crud.type_text(text);
            assert!(
                editor_text(&crud, id) == text,
                "{mode}: the editor of {id:?} holds what was typed"
            );
            assert!(
                reads_value(&crud, id, Some(text)),
                "{mode}: {id:?} reads back what was typed"
            );
        }
        for (id, text) in TYPED {
            assert!(
                reads_value(&crud, id, Some(text)),
                "{mode}: {id:?} still reads what was typed"
            );
        }
        assert!(
            reads_value(&crud, "crud-create", None),
            "{mode}: a button reads no value"
        );

        let mut flight = boot(LeanTask::FlightBooker, incremental);
        let before = editor_text(&flight, "flight-start");
        assert!(!before.is_empty(), "{mode}: the task fills the start date");
        assert!(
            reads_value(&flight, "flight-start", Some(&before)),
            "{mode}: flight-start reads its text at boot"
        );
        assert!(!before.contains('x'), "{mode}: the start date holds no x");
        flight.click("#flight-start");
        flight.type_text("x");
        let after = node(&flight.doc.snapshot(), "flight-start")
            .state
            .value
            .clone()
            .unwrap_or_default();
        assert!(
            after.chars().count() == before.chars().count() + 1 && after.contains('x'),
            "{mode}: flight-start reads one more character, the typed one"
        );
        assert!(
            after == editor_text(&flight, "flight-start"),
            "{mode}: flight-start reads its editor's text"
        );
    }
}

#[test]
fn the_range_input_reads_the_apps_duration() {
    for incremental in [false, true] {
        let timer = boot(LeanTask::Timer, incremental);
        assert_eq!(
            node(&timer.doc.snapshot(), "timer-duration").role,
            Role::Slider,
            "incremental={incremental}: the duration control is a range input"
        );
        let attribute = timer.attr("#timer-duration", "value");
        assert!(
            attribute.as_deref() == Some("15"),
            "incremental={incremental}: the app writes its duration as the value attribute"
        );
        assert!(
            reads_value(&timer, "timer-duration", Some("15")),
            "incremental={incremental}: timer-duration reads the app's duration"
        );
    }
}

#[test]
fn focus_reads_on_exactly_the_focused_control() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let mut crud = boot(LeanTask::Crud, incremental);
        assert!(
            crud.doc.snapshot().nodes().count() > 0,
            "{mode}: the snapshot has nodes"
        );
        assert_eq!(
            focused(&crud),
            NOTHING,
            "{mode}: nothing is focused at boot"
        );
        crud.click("#crud-name");
        assert_eq!(focused(&crud), ["crud-name"], "{mode}: after a click");
        crud.press(Key::Tab);
        assert_eq!(focused(&crud), ["crud-surname"], "{mode}: after Tab");
        crud.press_with(Key::Tab, Modifiers::SHIFT);
        assert_eq!(focused(&crud), ["crud-name"], "{mode}: after Shift+Tab");
        crud.click("#task-title");
        assert_eq!(
            focused(&crud),
            NOTHING,
            "{mode}: a click on a heading clears focus"
        );

        let mut fresh = boot(LeanTask::Crud, incremental);
        fresh.press(Key::Tab);
        assert_eq!(
            focused(&fresh),
            ["back-btn"],
            "{mode}: the first Tab on a fresh boot"
        );
    }
}

/// The controls the lean tasks lack: a checkbox, two radios sharing a name and a password
/// input, each with an author id and an accessible name.
fn fixture() -> Element {
    rsx! {
        div {
            input { id: "fx-agree", r#type: "checkbox" }
            label { r#for: "fx-agree", "Agree" }
        }
        div {
            input {
                id: "fx-plan-a",
                r#type: "radio",
                name: "fx-plan",
                "aria-label": "Plan A",
            }
            input {
                id: "fx-plan-b",
                r#type: "radio",
                name: "fx-plan",
                "aria-label": "Plan B",
            }
        }
        div {
            label { r#for: "fx-secret", "Passphrase" }
            input { id: "fx-secret", r#type: "password" }
        }
    }
}

const FIXTURE_CONTROLS: [(&str, Role); 4] = [
    ("fx-agree", Role::CheckBox),
    ("fx-plan-a", Role::RadioButton),
    ("fx-plan-b", Role::RadioButton),
    ("fx-secret", Role::PasswordInput),
];

fn boot_fixture(incremental: bool) -> Harness<DioxusDocument> {
    Harness::from_vdom(VirtualDom::new(fixture), stand::options(incremental))
}

#[test]
fn the_fixture_controls_are_keyed_named_and_typed() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = boot_fixture(incremental);
        let snapshot = harness.doc.snapshot();
        let tree = harness.doc.accessibility_tree();
        let nodes = by_id(&tree);
        for (id, role) in FIXTURE_CONTROLS {
            let read = node(&snapshot, id);
            assert_eq!(read.role, role, "{mode}: {id:?}");
            assert!(!read.name.is_empty(), "{mode}: {id:?} is named");
            let accessible: Vec<&&Node> = nodes
                .values()
                .filter(|node| node.author_id() == Some(id))
                .collect();
            assert_eq!(accessible.len(), 1, "{mode}: one tree node carries {id:?}");
            assert_eq!(
                read.name,
                name(&nodes, accessible[0]).trim(),
                "{mode}: {id:?} reads its accessibility node's name"
            );
            assert!(
                read.bounds.width > 0.0 && read.bounds.height > 0.0,
                "{mode}: {id:?} reads {:?}",
                read.bounds
            );
        }
        let unkeyed: Vec<String> = harness
            .doc
            .unkeyed_actionable()
            .iter()
            .map(UnkeyedActionable::remedy)
            .collect();
        assert!(unkeyed.is_empty(), "{mode}: {unkeyed:#?}");
    }
}

#[test]
fn a_checkbox_and_a_radio_read_checked_after_a_click() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot_fixture(incremental);

        assert_eq!(
            checked(&harness, "fx-agree"),
            Some(false),
            "{mode}: at boot"
        );
        assert_eq!(focused(&harness), NOTHING, "{mode}: nothing is focused");
        harness.click("#fx-agree");
        assert_eq!(checked(&harness, "fx-agree"), Some(true), "{mode}: clicked");
        assert_eq!(
            focused(&harness),
            ["fx-agree"],
            "{mode}: a click focuses the checkbox"
        );
        harness.click("#fx-agree");
        assert_eq!(
            checked(&harness, "fx-agree"),
            Some(false),
            "{mode}: clicked again"
        );

        assert_eq!(
            checked(&harness, "fx-plan-a"),
            Some(false),
            "{mode}: at boot"
        );
        assert_eq!(
            checked(&harness, "fx-plan-b"),
            Some(false),
            "{mode}: at boot"
        );
        harness.click("#fx-plan-b");
        assert_eq!(
            checked(&harness, "fx-plan-b"),
            Some(true),
            "{mode}: B picked"
        );
        assert_eq!(
            checked(&harness, "fx-plan-a"),
            Some(false),
            "{mode}: B picked"
        );
        harness.click("#fx-plan-a");
        assert_eq!(
            checked(&harness, "fx-plan-a"),
            Some(true),
            "{mode}: A picked"
        );
        assert_eq!(
            checked(&harness, "fx-plan-b"),
            Some(false),
            "{mode}: A picked"
        );
        assert_eq!(
            checked(&harness, "fx-secret"),
            None,
            "{mode}: a text-entry input reads no checked"
        );
    }
}

#[test]
fn a_typed_password_never_appears_in_the_snapshot() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot_fixture(incremental);
        assert!(
            reads_value(&harness, "fx-secret", Some("")),
            "{mode}: an empty password reads empty"
        );

        harness.click("#fx-secret");
        harness.type_text(SECRET);
        assert!(
            editor_text(&harness, "fx-secret") == SECRET,
            "{mode}: the password input holds the typed text"
        );

        assert!(
            reads_value(&harness, "fx-secret", Some(MASKED_VALUE)),
            "{mode}: a password holding text reads the mask"
        );
        let snapshot = harness.doc.snapshot();
        for node in snapshot.nodes() {
            let carried = [
                Some(node.id.as_str()),
                Some(node.name.as_str()),
                node.state.value.as_deref(),
            ];
            assert!(
                carried
                    .into_iter()
                    .flatten()
                    .all(|field| !field.contains(SECRET)),
                "{mode}: a field of a snapshot node holds the typed text"
            );
        }
        let tree = harness.doc.accessibility_tree();
        assert!(tree.nodes.len() > 1, "{mode}: the tree has nodes");
        for (_, node) in &tree.nodes {
            let carried = [node.label(), node.value()];
            assert!(
                carried
                    .into_iter()
                    .flatten()
                    .all(|field| !field.contains(SECRET)),
                "{mode}: a label or value of an accessibility node holds the typed text"
            );
        }
    }
}
