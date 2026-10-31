//! The accessibility tree reads ARIA states from an element's markup, by value: `aria-selected`
//! as its node's `selected`, `aria-invalid` as its `invalid`, and as its `toggled` the
//! `aria-checked` of an element whose role is `checkbox`, `switch`, `radio`, `menuitemcheckbox`
//! or `menuitemradio` and the `aria-pressed` of every other element.
//!
//! The builder read `aria-label` and `aria-hidden` and no state, so a control that stated
//! itself selected, pressed or invalid read the same as one that did not. On parsed pages at a
//! fixed 800 × 600 viewport, in both layout modes: each value of an attribute's grammar reads
//! the property stated for it; a value is compared trimmed, exactly and in lower case, so
//! `TRUE` is a value of none of them; an `aria-invalid` value outside its grammar reads
//! invalid, as the specification rules; a value of several thousand characters builds the tree
//! like any other and is nowhere in it; an element whose role does not support `aria-selected`,
//! `aria-pressed` or `aria-invalid` reads it as one whose role does; each attribute sets its own
//! property alone; and AccessKit's one `toggled` property takes `aria-checked` on the five
//! checkable roles alone, where `aria-pressed` sets nothing, and `aria-pressed` everywhere else,
//! where `aria-checked` sets nothing — but for a native checkbox or radio `input`, whose
//! `aria-checked` and `aria-pressed` set nothing whatever its role, and which states its own
//! checkedness as `toggled` where its role is checkable, and nothing under any other role: a
//! click and a Space move the tree, and the snapshot's `checked` and `pressed` read as they did.
//! <https://www.w3.org/TR/html-aria/#docconformance>
//! <https://www.w3.org/TR/html-aam-1.0/#att-checked>
//!
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-selected>
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-pressed>
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-checked>
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-invalid>

use ::keyboard_types::Key;
use accesskit::{Invalid, NodeId, Role, Toggled, TreeUpdate};
use blitz_dom::{Document, DocumentConfig};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use dioxus::prelude::*;
use std::sync::Arc;

mod fixtures;

/// What a node reads for the three states: its `selected`, its `toggled` and its `invalid`.
type States = (Option<bool>, Option<Toggled>, Option<Invalid>);

/// A node that states nothing.
const NOTHING: States = (None, None, None);

/// A word in no attribute's grammar.
const UNRECOGNIZED: &str = "zqx-no-such-value";

/// The part a value of several thousand characters repeats.
const LONG_PART: &str = "long-value-";

/// An element whose role supports an attribute: its opening tag without the `>`, its closing
/// tag and the role it reads.
type Supporting = (&'static str, &'static str, Role);

const BUTTON: Supporting = ("<button", "</button>", Role::Button);
const OPTION: Supporting = (r#"<div role="option""#, "</div>", Role::ListBoxOption);
const INPUT: Supporting = ("<input", "", Role::TextInput);

fn page(html: &str, incremental: bool) -> HtmlDocument {
    let mut doc = HtmlDocument::from_html(
        html,
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            ..Default::default()
        },
    );
    doc.set_incremental_layout(incremental);
    doc.resolve(0.0);
    doc
}

/// A page holding, per value, the supporting element under the id `s{row}` and a plain `div`
/// under `p{row}`, each carrying `attribute` with that value, or no attribute for `None`.
fn carriers(attribute: &str, supporting: Supporting, values: &[Option<&str>]) -> String {
    let (open, close, _) = supporting;
    let rows: String = values
        .iter()
        .enumerate()
        .map(|(row, value)| {
            let stated = value
                .map(|value| format!(r#" {attribute}="{value}""#))
                .unwrap_or_default();
            format!(r#"{open} id="s{row}"{stated}>{close}<div id="p{row}"{stated}></div>"#)
        })
        .collect();
    format!("<html><body>{rows}</body></html>")
}

/// The role and the three states the element `id` reads in `tree`.
#[track_caller]
fn read(doc: &HtmlDocument, tree: &TreeUpdate, id: &str) -> (Role, States) {
    let node_id = doc
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("the page holds {id:?}"));
    let node = tree
        .nodes
        .iter()
        .find(|(tree_id, _)| *tree_id == NodeId(node_id.as_u64()))
        .map(|(_, node)| node)
        .unwrap_or_else(|| panic!("the tree keeps {id:?}"));
    (
        node.role(),
        (node.is_selected(), node.toggled(), node.invalid()),
    )
}

/// Asserts, in both layout modes, that every row's value of `attribute` reads the states stated
/// for it, on the supporting element and on the plain `div` alike, and that the tree holds no
/// value as it was written.
#[track_caller]
fn assert_rows(attribute: &str, supporting: Supporting, rows: &[(Option<&str>, States)]) {
    let values: Vec<Option<&str>> = rows.iter().map(|(value, _)| *value).collect();
    let html = carriers(attribute, supporting, &values);
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: {attribute}");
        let doc = page(&html, incremental);
        let tree = doc.build_accessibility_tree();
        for (row, (_, stated)) in rows.iter().enumerate() {
            let (role, on_supporting) = read(&doc, &tree, &format!("s{row}"));
            assert_eq!(role, supporting.2, "{mode}: row {row}, the fixture's role");
            let (role, on_plain) = read(&doc, &tree, &format!("p{row}"));
            assert_eq!(
                role,
                Role::GenericContainer,
                "{mode}: row {row}, the fixture's plain element"
            );
            assert_eq!(
                on_supporting, *stated,
                "{mode}: row {row} on the supporting element"
            );
            assert_eq!(on_plain, *stated, "{mode}: row {row} on the plain element");
        }
        let written = format!("{tree:?}");
        assert!(
            !written.contains(UNRECOGNIZED) && !written.contains(LONG_PART),
            "{mode}: the tree holds a value as it was written"
        );
    }
}

#[test]
fn aria_selected_reads_true_and_false_and_nothing_else() {
    let long = LONG_PART.repeat(500);
    let selected = |selected| (Some(selected), None, None);
    let rows = [
        (Some("true"), selected(true)),
        (Some("false"), selected(false)),
        (None, NOTHING),
        (Some(""), NOTHING),
        (Some("undefined"), NOTHING),
        (Some("TRUE"), NOTHING),
        (Some(UNRECOGNIZED), NOTHING),
        (Some(" true "), selected(true)),
        (Some(long.as_str()), NOTHING),
    ];
    assert_eq!(rows.len(), 9);
    assert_rows("aria-selected", OPTION, &rows);
}

#[test]
fn aria_pressed_reads_its_three_values_and_nothing_else() {
    let long = LONG_PART.repeat(500);
    let toggled = |toggled| (None, Some(toggled), None);
    let rows = [
        (Some("true"), toggled(Toggled::True)),
        (Some("false"), toggled(Toggled::False)),
        (Some("mixed"), toggled(Toggled::Mixed)),
        (None, NOTHING),
        (Some(""), NOTHING),
        (Some("undefined"), NOTHING),
        (Some("TRUE"), NOTHING),
        (Some(UNRECOGNIZED), NOTHING),
        (Some(" mixed "), toggled(Toggled::Mixed)),
        (Some(long.as_str()), NOTHING),
    ];
    assert_eq!(rows.len(), 10);
    assert_rows("aria-pressed", BUTTON, &rows);
}

#[test]
fn aria_invalid_reads_its_values_and_an_unrecognized_one_as_true() {
    let long = LONG_PART.repeat(500);
    let invalid = |invalid| (None, None, Some(invalid));
    let rows = [
        (Some("true"), invalid(Invalid::True)),
        (Some("grammar"), invalid(Invalid::Grammar)),
        (Some("spelling"), invalid(Invalid::Spelling)),
        (Some(UNRECOGNIZED), invalid(Invalid::True)),
        (Some("TRUE"), invalid(Invalid::True)),
        (None, NOTHING),
        (Some(""), NOTHING),
        (Some("false"), NOTHING),
        (Some("  "), NOTHING),
        (Some(" grammar "), invalid(Invalid::Grammar)),
        (Some(long.as_str()), invalid(Invalid::True)),
    ];
    assert_eq!(rows.len(), 11);
    assert_rows("aria-invalid", INPUT, &rows);
}

/// The five roles whose toggle is their `aria-checked`, each as an element stating it.
const CHECKABLE: [Supporting; 5] = [
    (r#"<div role="checkbox""#, "</div>", Role::CheckBox),
    (r#"<div role="switch""#, "</div>", Role::Switch),
    (r#"<div role="radio""#, "</div>", Role::RadioButton),
    (
        r#"<div role="menuitemcheckbox""#,
        "</div>",
        Role::MenuItemCheckBox,
    ),
    (
        r#"<div role="menuitemradio""#,
        "</div>",
        Role::MenuItemRadio,
    ),
];

/// Asserts, in both layout modes, that on each element of `supporting` every row's value of
/// `attribute` reads the states stated for it, and the plain `div` beside it the states stated
/// for a plain element: a row is its value, then the two readings.
#[track_caller]
fn assert_rows_on_roles(
    attribute: &str,
    supporting: &[Supporting],
    rows: &[(Option<&str>, States, States)],
) {
    let values: Vec<Option<&str>> = rows.iter().map(|(value, _, _)| *value).collect();
    for incremental in [false, true] {
        for (element, supporting) in supporting.iter().enumerate() {
            let mode = format!("incremental={incremental}: {attribute}: element {element}");
            let doc = page(&carriers(attribute, *supporting, &values), incremental);
            let tree = doc.build_accessibility_tree();
            for (row, (_, stated, on_a_plain)) in rows.iter().enumerate() {
                let (role, on_supporting) = read(&doc, &tree, &format!("s{row}"));
                assert_eq!(role, supporting.2, "{mode}: row {row}, the fixture's role");
                assert_eq!(on_supporting, *stated, "{mode}: row {row}");
                let (_, on_plain) = read(&doc, &tree, &format!("p{row}"));
                assert_eq!(
                    on_plain, *on_a_plain,
                    "{mode}: row {row} on the plain element"
                );
            }
            let written = format!("{tree:?}");
            assert!(
                !written.contains(UNRECOGNIZED) && !written.contains(LONG_PART),
                "{mode}: the tree holds a value as it was written"
            );
        }
    }
}

#[test]
fn aria_checked_reads_its_three_values_on_a_checkable_role_alone() {
    let toggled = |toggled| (None, Some(toggled), None);
    let rows = [
        (Some("true"), toggled(Toggled::True), NOTHING),
        (Some("false"), toggled(Toggled::False), NOTHING),
        (Some("mixed"), toggled(Toggled::Mixed), NOTHING),
        (None, NOTHING, NOTHING),
        (Some("TRUE"), NOTHING, NOTHING),
        (Some(UNRECOGNIZED), NOTHING, NOTHING),
        (Some(" mixed "), toggled(Toggled::Mixed), NOTHING),
    ];
    assert_eq!(rows.len(), 7);
    assert_eq!(CHECKABLE.len(), 5);
    assert_rows_on_roles("aria-checked", &CHECKABLE, &rows);

    let unread: Vec<_> = rows
        .iter()
        .map(|(value, _, _)| (*value, NOTHING, NOTHING))
        .collect();
    assert_rows_on_roles("aria-checked", &[BUTTON, OPTION, INPUT], &unread);

    // A native checkbox or radio states its own checkedness: its author `aria-checked` and
    // `aria-pressed`, each here saying the opposite, write nothing, and under a role that is not
    // checkable nothing is written at all.
    let natives = [
        (
            r#"type="checkbox""#,
            false,
            Role::CheckBox,
            Some(Toggled::False),
        ),
        (
            r#"type="checkbox""#,
            true,
            Role::CheckBox,
            Some(Toggled::True),
        ),
        (
            r#"type="radio""#,
            false,
            Role::RadioButton,
            Some(Toggled::False),
        ),
        (
            r#"type="radio""#,
            true,
            Role::RadioButton,
            Some(Toggled::True),
        ),
        (
            r#"type="checkbox" role="switch""#,
            false,
            Role::Switch,
            Some(Toggled::False),
        ),
        (
            r#"type="checkbox" role="switch""#,
            true,
            Role::Switch,
            Some(Toggled::True),
        ),
        (r#"type="radio" role="button""#, false, Role::Button, None),
        (r#"type="radio" role="button""#, true, Role::Button, None),
    ];
    let attributes = ["aria-checked", "aria-pressed"];
    assert_eq!((natives.len(), attributes.len()), (8, 2));
    for incremental in [false, true] {
        for (row, (typed, checked, role_read, toggled)) in natives.into_iter().enumerate() {
            for attribute in attributes {
                let mode = format!("incremental={incremental}: native input {row}: {attribute}");
                let (state, stated) = if checked {
                    ("checked", "false")
                } else {
                    ("", "true")
                };
                let html = format!(
                    r#"<html><body><input id="box" {typed} {state} {attribute}="{stated}"></body></html>"#
                );
                let doc = page(&html, incremental);
                let holds = doc
                    .get_element_by_id("box")
                    .and_then(|id| doc.get_node(id))
                    .and_then(|node| node.element_data())
                    .is_some_and(|element| element.checkbox_input_checked() == Some(checked));
                assert!(holds, "{mode}: the fixture's input holds its stated state");
                let dynamic: &dyn Document = &doc;
                let tree = dynamic.accessibility_tree();
                let (role, states) = read(&doc, &tree, "box");
                assert_eq!(role, role_read, "{mode}: the fixture's role");
                assert_eq!(
                    states,
                    (None, toggled, None),
                    "{mode}: the input's own state is the toggle"
                );
            }
        }
    }
}

/// A checkbox, a radio and a checkbox whose `role` names no checkable role.
fn natives_page() -> Element {
    rsx! {
        div { id: "fx-root",
            input { id: "fx-check", r#type: "checkbox" }
            input { id: "fx-radio", r#type: "radio" }
            input { id: "fx-button", r#type: "checkbox", role: "button" }
        }
    }
}

/// What `id` reads: the toggle in the tree the shell builds, and the snapshot's `checked` and
/// `pressed`.
#[track_caller]
fn native_readings(
    harness: &blitz_test_harness::Harness<dioxus_native_dom::DioxusDocument>,
    id: &str,
) -> (Option<Toggled>, Option<bool>, Option<bool>) {
    let doc: &dyn Document = &harness.doc;
    let tree = doc.accessibility_tree();
    let node = NodeId(harness.node(&format!("#{id}")).as_u64());
    let toggled = tree
        .nodes
        .iter()
        .find(|(found, _)| *found == node)
        .and_then(|(_, node)| node.toggled());
    let snapshot = harness.doc.snapshot();
    let state = &snapshot
        .get(id)
        .expect("the fixture's input is a node")
        .state;
    (toggled, state.checked, state.pressed)
}

#[test]
fn a_click_and_a_space_move_a_native_control_in_the_tree() {
    let reads = |toggled, checked| (Some(toggled), Some(checked), None);
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(natives_page, incremental);
        assert_eq!(
            native_readings(&harness, "fx-check"),
            reads(Toggled::False, false),
            "{mode}: at boot"
        );
        harness.click("#fx-check");
        assert_eq!(
            native_readings(&harness, "fx-check"),
            reads(Toggled::True, true),
            "{mode}: a click checks it"
        );
        harness.press(Key::Character(" ".to_string()));
        assert_eq!(
            native_readings(&harness, "fx-check"),
            reads(Toggled::False, false),
            "{mode}: a Space unchecks it"
        );
        assert_eq!(
            native_readings(&harness, "fx-radio"),
            reads(Toggled::False, false),
            "{mode}: the radio at boot"
        );
        harness.click("#fx-radio");
        assert_eq!(
            native_readings(&harness, "fx-radio"),
            reads(Toggled::True, true),
            "{mode}: a click checks the radio"
        );
        harness.click("#fx-button");
        assert_eq!(
            native_readings(&harness, "fx-button"),
            (None, Some(true), None),
            "{mode}: under another role the tree states nothing and `pressed` reads nothing"
        );
    }
}

#[test]
fn aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere() {
    let long = LONG_PART.repeat(500);
    assert_eq!(long.chars().count(), 5_500);
    let pressed = |toggled| (None, Some(toggled), None);
    let rows = [
        (Some("true"), NOTHING, pressed(Toggled::True)),
        (Some("false"), NOTHING, pressed(Toggled::False)),
        (Some("mixed"), NOTHING, pressed(Toggled::Mixed)),
    ];
    assert_eq!(rows.len(), 3);
    assert_rows_on_roles("aria-pressed", &CHECKABLE, &rows);

    let rows = [(Some(long.as_str()), NOTHING, NOTHING)];
    assert_rows_on_roles("aria-checked", &CHECKABLE, &rows);
}
