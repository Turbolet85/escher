//! The accessibility tree reads three ARIA states from an element's markup, by value:
//! `aria-selected` as its node's `selected`, `aria-pressed` as its `toggled` and `aria-invalid`
//! as its `invalid`.
//!
//! The builder read `aria-label` and `aria-hidden` and no state, so a control that stated
//! itself selected, pressed or invalid read the same as one that did not. On parsed pages at a
//! fixed 800 × 600 viewport, in both layout modes: each value of an attribute's grammar reads
//! the property stated for it; a value is compared trimmed, exactly and in lower case, so
//! `TRUE` is a value of none of the three; an `aria-invalid` value outside its grammar reads
//! invalid, as the specification rules; a value of several thousand characters builds the tree
//! like any other and is nowhere in it; an element whose role does not support an attribute
//! reads as one whose role does; each attribute sets its own property alone; and `aria-checked`
//! sets none of the three.
//!
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-selected>
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-pressed>
//! <https://www.w3.org/TR/wai-aria-1.2/#aria-invalid>

use accesskit::{Invalid, NodeId, Role, Toggled, TreeUpdate};
use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

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

#[test]
fn aria_checked_sets_none_of_the_three() {
    let supporting = [BUTTON, OPTION, INPUT];
    let values = [Some("true"), Some("false"), Some("mixed")];
    for incremental in [false, true] {
        for (element, supporting) in supporting.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: element {element}");
            let doc = page(&carriers("aria-checked", supporting, &values), incremental);
            let tree = doc.build_accessibility_tree();
            for row in 0..values.len() {
                let (role, on_supporting) = read(&doc, &tree, &format!("s{row}"));
                assert_eq!(role, supporting.2, "{mode}: row {row}, the fixture's role");
                assert_eq!(on_supporting, NOTHING, "{mode}: row {row}");
                let (_, on_plain) = read(&doc, &tree, &format!("p{row}"));
                assert_eq!(on_plain, NOTHING, "{mode}: row {row} on the plain element");
            }
        }

        let html = r#"<html><body><input id="box" type="checkbox" checked aria-checked="true"></body></html>"#;
        let doc = page(html, incremental);
        let tree = doc.build_accessibility_tree();
        let (role, states) = read(&doc, &tree, "box");
        assert_eq!(
            role,
            Role::CheckBox,
            "incremental={incremental}: the fixture's checkbox"
        );
        assert_eq!(
            states, NOTHING,
            "incremental={incremental}: a native checkbox"
        );
    }
}
