//! An inert subtree — an element carrying an `inert` attribute, read by presence, and everything
//! it holds — takes no focus, no hit and no accessibility node, in both layout modes:
//! Tab and Shift+Tab skip the buttons under it, a hit at one of them answers the element beneath,
//! and the tree the shell hands the platform adapter holds nothing of it. Removing the attribute
//! restores all three. A node that holds focus when it or an ancestor becomes inert, or moves
//! under an inert box, gives focus up at the mutator's flush, as HTML's focus fixup has it, with
//! no `blur` fired: the engine, the snapshot and the tree agree on no focus, and neither a key
//! nor typed text lands there. The tree names the window as its focus whenever the focused node
//! was not built, so it never names a node it does not hold. What an assistive technology reads
//! stays unmeasured: no adapter runs here.
//!
//! <https://html.spec.whatwg.org/multipage/interaction.html#the-inert-attribute>
//! <https://html.spec.whatwg.org/multipage/interaction.html#focus-fixup-rule>

use ::keyboard_types::{Key, Modifiers, NamedKey};
use accesskit::NodeId as TreeNodeId;
use blitz_dom::{Document, QualName, local_name, ns};
use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::node_id::NodeId;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;

mod fixtures;

/// Two buttons inside an inert box between two outside it, the box lying over an empty block.
const FIXTURE: &str = r#"<html><body style="margin:0">
    <button id="before" style="display:block; width:100px; height:30px">Before</button>
    <div id="beneath" style="height:100px"></div>
    <div id="shield" inert style="position:absolute; left:0; top:30px; width:400px; height:100px">
        <button id="in-a" style="display:block; width:100px; height:30px">A</button>
        <button id="in-b" style="display:block; width:100px; height:30px">B</button>
    </div>
    <button id="after" style="display:block; width:100px; height:30px">After</button>
</body></html>"#;

fn boot(incremental: bool) -> Harness<HtmlDocument> {
    Harness::from_html_with(
        FIXTURE,
        HarnessOptions {
            incremental: Some(incremental),
            ..Default::default()
        },
    )
}

fn inert() -> QualName {
    QualName::new(None, ns!(), local_name!("inert"))
}

fn tab(harness: &mut Harness<HtmlDocument>) -> Option<NodeId> {
    harness.press(Key::Named(NamedKey::Tab));
    harness.focused()
}

fn shift_tab(harness: &mut Harness<HtmlDocument>) -> Option<NodeId> {
    harness.press_with(Key::Named(NamedKey::Tab), Modifiers::SHIFT);
    harness.focused()
}

/// Whether the accessibility tree holds a node for `node`.
fn in_tree(harness: &Harness<HtmlDocument>, node: NodeId) -> bool {
    harness
        .base()
        .build_accessibility_tree()
        .nodes
        .iter()
        .any(|(id, _)| *id == TreeNodeId(node.as_u64()))
}

/// The element a hit at the centre of `selector`'s box answers.
fn hit_at_centre(harness: &Harness<HtmlDocument>, selector: &str) -> Option<NodeId> {
    let (x, y) = harness.center_of(selector);
    harness.hit(x, y).map(|hit| hit.node_id)
}

#[test]
fn tab_and_shift_tab_skip_an_inert_subtree() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot(incremental);
        let [before, after] = ["#before", "#after"].map(|selector| harness.node(selector));
        assert!(
            harness.attr("#shield", "inert").is_some(),
            "{mode}: the fixture's box is inert"
        );
        assert!(tab(&mut harness) == Some(before), "{mode}: the first Tab");
        assert!(
            tab(&mut harness) == Some(after),
            "{mode}: Tab skips the inert buttons"
        );
        assert!(tab(&mut harness) == Some(before), "{mode}: Tab wraps");
        assert!(
            shift_tab(&mut harness) == Some(after),
            "{mode}: Shift+Tab wraps"
        );
        assert!(
            shift_tab(&mut harness) == Some(before),
            "{mode}: Shift+Tab skips the inert buttons back"
        );
    }
}

#[test]
fn a_hit_passes_through_an_inert_subtree() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = boot(incremental);
        let [beneath, in_a, in_b] =
            ["#beneath", "#in-a", "#in-b"].map(|selector| harness.node(selector));
        let (_, y) = harness.center_of("#in-a");
        let beneath_rect = harness.layout_rect("#beneath");
        assert!(
            y > beneath_rect.y && y < beneath_rect.y + beneath_rect.height,
            "{mode}: the inert button lies over the block beneath"
        );
        for (index, (selector, button)) in
            [("#in-a", in_a), ("#in-b", in_b)].into_iter().enumerate()
        {
            let hit = hit_at_centre(&harness, selector);
            assert!(
                hit != Some(button),
                "{mode}: button {index}: the hit does not answer the inert button"
            );
            assert!(
                hit == Some(beneath),
                "{mode}: button {index}: the hit answers the element beneath"
            );
        }
    }
}

#[test]
fn the_tree_holds_no_node_for_an_inert_subtree() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = boot(incremental);
        for selector in ["#before", "#after", "#beneath"] {
            assert!(
                in_tree(&harness, harness.node(selector)),
                "{mode}: the tree keeps the elements outside the box"
            );
        }
        for (index, selector) in ["#shield", "#in-a", "#in-b"].into_iter().enumerate() {
            assert!(
                !in_tree(&harness, harness.node(selector)),
                "{mode}: node {index} of the inert subtree is not in the tree"
            );
        }
    }
}

#[test]
fn removing_inert_restores_focus_hit_and_tree() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot(incremental);
        let [shield, before, in_a, in_b, after] =
            ["#shield", "#before", "#in-a", "#in-b", "#after"].map(|s| harness.node(s));
        harness.base_mut().mutate().clear_attribute(shield, inert());
        harness.pump();
        assert!(
            harness.attr("#shield", "inert").is_none(),
            "{mode}: the attribute is removed"
        );

        let walk: Vec<_> = (0..4).map(|_| tab(&mut harness)).collect();
        assert!(
            walk == [Some(before), Some(in_a), Some(in_b), Some(after)],
            "{mode}: Tab reaches the buttons in the box"
        );
        assert!(
            hit_at_centre(&harness, "#in-a") == Some(in_a),
            "{mode}: a hit answers the button"
        );
        for (index, node) in [shield, in_a, in_b].into_iter().enumerate() {
            assert!(
                in_tree(&harness, node),
                "{mode}: node {index} of the box is in the tree"
            );
        }
    }
}

/// Two buttons around a box holding a counting button and a text input, a box with no `inert`
/// yet.
fn focus_page() -> Element {
    let mut clicks = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            button { id: "fx-first", "First" }
            div { id: "fx-box",
                button { id: "fx-in-button", onclick: move |_| clicks += 1, "In" }
                input { id: "fx-in-text" }
            }
            button { id: "fx-last", "Last" }
            div { id: "fx-shelter", "inert": "true" }
            p { id: "fx-clicks", "{clicks}" }
        }
    }
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

/// The ways a focused node comes under `inert`: on an ancestor, on the node itself and by moving
/// under a box that already carries it.
#[derive(Clone, Copy)]
enum MadeInert {
    Ancestor,
    Itself,
    Moved,
}

#[test]
fn a_node_made_inert_loses_focus_and_takes_no_key() {
    const ROWS: [(&str, MadeInert); 5] = [
        ("#fx-in-button", MadeInert::Ancestor),
        ("#fx-in-text", MadeInert::Ancestor),
        ("#fx-in-button", MadeInert::Itself),
        ("#fx-in-text", MadeInert::Itself),
        ("#fx-in-button", MadeInert::Moved),
    ];
    assert_eq!(ROWS.len(), 5);
    let window = TreeNodeId(u64::MAX);
    for incremental in [false, true] {
        for (row, (focused, how)) in ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let mut harness = fixtures::boot(focus_page, incremental);
            let target = harness.node(focused);
            harness.base_mut().set_focus_to(target);
            harness.pump();
            assert!(
                focus_readings(&harness) == (Some(target), 1, TreeNodeId(target.as_u64())),
                "{mode}: the fixture holds the node focused"
            );

            harness.base_mut().take_changed_nodes();
            match how {
                MadeInert::Ancestor => {
                    let boxed = harness.node("#fx-box");
                    harness
                        .base_mut()
                        .mutate()
                        .set_attribute(boxed, inert(), "");
                }
                MadeInert::Itself => {
                    harness
                        .base_mut()
                        .mutate()
                        .set_attribute(target, inert(), "");
                }
                MadeInert::Moved => {
                    let shelter = harness.node("#fx-shelter");
                    harness
                        .base_mut()
                        .mutate()
                        .append_children(shelter, &[target]);
                }
            }
            harness.pump();
            assert!(
                harness.base_mut().take_changed_nodes().contains(&target),
                "{mode}: the lost focus marks the node in the changed set"
            );
            assert!(
                focus_readings(&harness) == (None, 0, window),
                "{mode}: the engine, the snapshot and the tree agree on no focus"
            );

            harness.press(Key::Named(NamedKey::Enter));
            harness.type_text("x");
            assert!(
                harness.text_content("#fx-clicks") == "0",
                "{mode}: Enter clicks nothing"
            );
            let typed = harness
                .base()
                .get_node(harness.node("#fx-in-text"))
                .and_then(|node| node.element_data())
                .and_then(|element| element.text_input_data())
                .map(|data| data.editor.text().to_string());
            assert!(
                typed.as_deref() == Some(""),
                "{mode}: a typed character lands nowhere"
            );

            let first = harness.node("#fx-first");
            harness.press(Key::Named(NamedKey::Tab));
            assert!(
                harness.focused() == Some(first),
                "{mode}: the next Tab lands on the first stop"
            );
        }
    }
}

/// A button shown beside three left out of the tree, by `display: none`, by `hidden` and under a
/// `display: none` box.
const LEFT_OUT: &str = r#"<html><body>
    <button id="shown">Shown</button>
    <button id="by-display" style="display:none">A</button>
    <button id="by-hidden" hidden>B</button>
    <div style="display:none"><button id="under-display">C</button></div>
</body></html>"#;

#[test]
fn the_tree_focus_falls_back_to_the_window_when_the_focused_node_is_not_built() {
    const ROWS: [&str; 3] = ["#by-display", "#by-hidden", "#under-display"];
    assert_eq!(ROWS.len(), 3);
    let window = TreeNodeId(u64::MAX);
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = Harness::from_html_with(
            LEFT_OUT,
            HarnessOptions {
                incremental: Some(incremental),
                ..Default::default()
            },
        );
        for (row, selector) in ROWS.into_iter().enumerate() {
            let node = harness.node(selector);
            harness.base_mut().set_focus_to(node);
            harness.pump();
            let doc: &dyn Document = &harness.doc;
            let tree = doc.accessibility_tree();
            assert!(
                !tree
                    .nodes
                    .iter()
                    .any(|(id, _)| *id == TreeNodeId(node.as_u64())),
                "{mode}: row {row}: the focused node is left out of the tree"
            );
            assert!(
                tree.focus == window && tree.nodes.iter().any(|(id, _)| *id == tree.focus),
                "{mode}: row {row}: the tree's focus is the window, a node it holds"
            );
        }

        let shown = harness.node("#shown");
        harness.base_mut().set_focus_to(shown);
        harness.pump();
        let doc: &dyn Document = &harness.doc;
        assert!(
            doc.accessibility_tree().focus == TreeNodeId(shown.as_u64()),
            "{mode}: a focused node the tree holds is its focus"
        );
    }
}
