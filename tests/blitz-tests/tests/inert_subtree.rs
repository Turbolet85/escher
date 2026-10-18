//! An inert subtree — an element carrying an `inert` attribute, read by presence, and everything
//! it holds — takes no focus, no hit and no accessibility node, in both layout modes:
//! Tab and Shift+Tab skip the buttons under it, a hit at one of them answers the element beneath,
//! and the tree the shell hands the platform adapter holds nothing of it. Removing the attribute
//! restores all three. Known limit: HTML's focus fixup is not built, so a node that holds focus
//! when it becomes inert keeps it until focus moves — pinned by the fourth test.
//!
//! <https://html.spec.whatwg.org/multipage/interaction.html#the-inert-attribute>

use ::keyboard_types::{Key, Modifiers, NamedKey};
use accesskit::NodeId as TreeNodeId;
use blitz_dom::{QualName, local_name, ns};
use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::node_id::NodeId;

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

        // The limit, as measured: the focused button keeps focus when its box becomes inert,
        // and the next Tab leaves the box.
        assert!(
            shift_tab(&mut harness) == Some(in_b),
            "{mode}: Shift+Tab reaches the second button"
        );
        harness
            .base_mut()
            .mutate()
            .set_attribute(shield, inert(), "");
        harness.pump();
        assert!(
            harness.focused() == Some(in_b),
            "{mode}: the focused button keeps focus when its box becomes inert"
        );
        assert!(
            tab(&mut harness) == Some(after),
            "{mode}: the next Tab leaves the inert box"
        );
    }
}
