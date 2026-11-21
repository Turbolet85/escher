//! A focused control that no longer shows gives focus up after the next layout, as HTML's focus
//! fixup has it: `hidden` on it or a container, `display: none` inline or by a class, and
//! `visibility: hidden` or `collapse`. No `blur` or `focusout` is fired, as for a node that comes
//! under `inert`; the engine's focus, the snapshot's and the accessibility tree's agree on none,
//! the control is marked in the changed set, and a key no longer reaches it. A `display: contents`
//! control and one beside a hidden sibling keep focus. In both layout modes. It closes the stated
//! limit that let such a control keep focus and take keys while the tree's focus named the window.
//! Tab and Shift+Tab skip a control that does not show, so focus given up is not taken back by the
//! next Tab.
//!
//! <https://html.spec.whatwg.org/multipage/interaction.html#focus-fixup-rule>
//! <https://html.spec.whatwg.org/multipage/interaction.html#sequential-navigation-search-algorithm>

use ::keyboard_types::{Key, Modifiers, NamedKey};
use accesskit::NodeId as TreeNodeId;
use blitz_dom::{Document, QualName, local_name, ns};
use blitz_test_harness::Harness;
use blitz_traits::node_id::NodeId;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;

mod fixtures;

/// A box holding a counting button, a sibling button, and the counts of its clicks, blurs and
/// focusouts, under `sheet`.
fn page(sheet: &'static str) -> Element {
    let mut clicks = use_signal(|| 0u32);
    let mut blurs = use_signal(|| 0u32);
    let mut focusouts = use_signal(|| 0u32);
    rsx! {
        style { "{sheet}" }
        div { id: "fx-root",
            div { id: "fx-box",
                button {
                    id: "fx-control",
                    onclick: move |_| clicks += 1,
                    onblur: move |_| blurs += 1,
                    onfocusout: move |_| focusouts += 1,
                    "Control"
                }
                button { id: "fx-sibling", "Sibling" }
            }
            p { id: "fx-clicks", "{clicks}" }
            p { id: "fx-blurs", "{blurs}" }
            p { id: "fx-focusouts", "{focusouts}" }
        }
    }
}

fn focus_page() -> Element {
    page(".gone { display: none; }")
}

fn focus_within_page() -> Element {
    page("#fx-box:focus-within { display: none; }")
}

/// The engine's focused node with the root it reads when nothing is focused dropped, and the
/// focus the snapshot and the tree read.
#[track_caller]
fn focus_readings(harness: &Harness<DioxusDocument>) -> (Option<NodeId>, usize, TreeNodeId) {
    let root = harness.base().try_root_element().map(|root| root.id);
    (
        harness.focused().filter(|node| Some(*node) != root),
        harness
            .doc
            .snapshot()
            .nodes()
            .filter(|node| node.state.focused)
            .count(),
        harness.doc.accessibility_tree().focus,
    )
}

fn name(local: &str) -> QualName {
    match local {
        "hidden" => QualName::new(None, ns!(), local_name!("hidden")),
        "class" => QualName::new(None, ns!(), local_name!("class")),
        _ => QualName::new(None, ns!(), local_name!("style")),
    }
}

/// Boots the page with `#fx-control` focused, asserting the three readings name it.
#[track_caller]
fn boot_focused(incremental: bool, mode: &str) -> (Harness<DioxusDocument>, NodeId) {
    let mut harness = fixtures::boot(focus_page, incremental);
    let control = harness.node("#fx-control");
    harness.base_mut().set_focus_to(control);
    harness.pump();
    assert!(
        focus_readings(&harness) == (Some(control), 1, TreeNodeId(control.as_u64())),
        "{mode}: the fixture holds the control focused"
    );
    (harness, control)
}

fn set(harness: &mut Harness<DioxusDocument>, selector: &str, attr: &str, value: &str) {
    let node = harness.node(selector);
    harness
        .base_mut()
        .mutate()
        .set_attribute(node, name(attr), value);
}

#[test]
fn a_focused_control_that_stops_showing_gives_focus_up_with_no_blur() {
    const ROWS: [(&str, &str, &str); 6] = [
        ("#fx-control", "hidden", ""),
        ("#fx-box", "hidden", ""),
        ("#fx-control", "style", "display:none"),
        ("#fx-box", "class", "gone"),
        ("#fx-box", "style", "visibility:hidden"),
        ("#fx-control", "style", "visibility:collapse"),
    ];
    assert_eq!(ROWS.len(), 6);
    let window = TreeNodeId(u64::MAX);
    for incremental in [false, true] {
        for (row, (selector, attr, value)) in ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let (mut harness, control) = boot_focused(incremental, &mode);

            harness.base_mut().take_changed_nodes();
            set(&mut harness, selector, attr, value);
            harness.pump();
            assert!(
                harness.base_mut().take_changed_nodes().contains(&control),
                "{mode}: the lost focus marks the control in the changed set"
            );
            assert!(
                focus_readings(&harness) == (None, 0, window),
                "{mode}: the engine, the snapshot and the tree agree on no focus"
            );
            assert!(
                harness.text_content("#fx-blurs") == "0"
                    && harness.text_content("#fx-focusouts") == "0",
                "{mode}: no blur or focusout is fired"
            );

            harness.press(Key::Named(NamedKey::Enter));
            assert!(
                harness.text_content("#fx-clicks") == "0",
                "{mode}: Enter clicks nothing"
            );
        }
    }
}

#[test]
fn a_display_contents_control_and_one_beside_a_hidden_sibling_keep_focus() {
    const ROWS: [(&str, &str, &str); 2] = [
        ("#fx-control", "style", "display:contents"),
        ("#fx-sibling", "hidden", ""),
    ];
    assert_eq!(ROWS.len(), 2);
    for incremental in [false, true] {
        for (row, (selector, attr, value)) in ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let (mut harness, control) = boot_focused(incremental, &mode);
            set(&mut harness, selector, attr, value);
            harness.pump();
            assert!(
                focus_readings(&harness) == (Some(control), 1, TreeNodeId(control.as_u64())),
                "{mode}: the control keeps focus"
            );
        }
    }
}

#[test]
fn a_focus_within_rule_that_hides_its_holder_shows_it_again_once_focus_goes() {
    let window = TreeNodeId(u64::MAX);
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(focus_within_page, incremental);
        assert!(
            harness.layout_rect("#fx-box").height > 0.0,
            "{mode}: the box shows before focus"
        );
        let control = harness.node("#fx-control");
        harness.base_mut().set_focus_to(control);
        harness.pump();
        assert!(
            focus_readings(&harness) == (None, 0, window),
            "{mode}: the box hidden by its rule, focus is given up"
        );
        // The given-up focus restyles at the next resolve, so the box shows one pump later.
        harness.pump();
        assert!(
            harness.layout_rect("#fx-box").height > 0.0,
            "{mode}: the box shows again"
        );
    }
}

#[test]
fn tab_and_shift_tab_skip_a_control_that_does_not_show() {
    const ROWS: [(&str, &str); 3] = [
        ("hidden", ""),
        ("style", "display:none"),
        ("style", "visibility:hidden"),
    ];
    assert_eq!(ROWS.len(), 3);
    for incremental in [false, true] {
        for (row, (attr, value)) in ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let mut harness = fixtures::boot(focus_page, incremental);
            let [control, sibling] = ["#fx-control", "#fx-sibling"].map(|s| harness.node(s));
            harness.press(Key::Named(NamedKey::Tab));
            assert!(
                harness.focused() == Some(control),
                "{mode}: the first Tab reaches the shown control"
            );

            set(&mut harness, "#fx-control", attr, value);
            harness.pump();
            harness.press(Key::Named(NamedKey::Tab));
            assert!(
                harness.focused() == Some(sibling),
                "{mode}: Tab skips the control that does not show"
            );
            harness.press_with(Key::Named(NamedKey::Tab), Modifiers::SHIFT);
            // The sibling is the one stop left, so Shift+Tab wraps back onto it.
            assert!(
                harness.focused() == Some(sibling),
                "{mode}: Shift+Tab skips it back"
            );
        }
    }
}
