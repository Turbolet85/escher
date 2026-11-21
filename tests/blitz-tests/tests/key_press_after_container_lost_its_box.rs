//! A key press with focus on a control whose anonymous block was dropped panicked: an ancestor
//! that stops generating a box drops the blocks it held and collects no children, so the control
//! kept a `layout_parent` naming a dropped block, and `absolute_position`, read for every event's
//! target, indexed it. Any page could reach it, by `hidden`, by `display: none` or by CSS alone
//! with no script. `select_popup`'s row for a select whose ancestor gains `hidden` exposed it;
//! upstream's tree panics the same way at `e42ee1f25`'s `node/node.rs:1740`. Since HTML's focus
//! fixup runs after each layout, the control gives focus up once its box goes; a key reaches it
//! only when it is focused again with no layout between (`set_focus_to`, which refuses no hidden
//! node), and then still acts on it with no panic, as it does on a control in no anonymous block.
//! Under a `:focus-within` rule the box goes as focus arrives, so there the fixup itself is the
//! witness: focus is given up and the container shows again at the next pump. In both layout
//! modes, through real dispatch. A failure message carries the layout mode and the row. The file
//! is also an `a11y` leg witness of SC 2.4.3: the engine's focus and the accessibility tree's
//! agree once a focused control no longer shows.

use ::keyboard_types::{Key, NamedKey};
use accesskit::NodeId as TreeNodeId;
use blitz_dom::{Document, NodeId, QualName, local_name, ns};
use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, HarnessOptions};

const BUTTON: &str = r#"<button id="control">Go</button>"#;
const SELECT: &str =
    r#"<select id="control"><option value="a">A</option><option value="b">B</option></select>"#;
const HIDDEN_ON_FOCUS: &str = "#root:focus-within { display: none; }";

/// How the control's ancestor stops generating a box.
#[derive(Clone, Copy, Debug)]
enum Lost {
    Hidden,
    StyleDisplayNone,
    /// By the page's own sheet as the control takes focus, with no mutation and no script.
    FocusWithin,
}

const WAYS: [Lost; 3] = [Lost::Hidden, Lost::StyleDisplayNone, Lost::FocusWithin];

/// The control beside a block, which puts it in an anonymous block.
fn in_anonymous_block(control: &str, sheet: &str) -> String {
    format!(
        r#"<html><head><style>{sheet}</style></head><body style="margin:0">
        <div id="root">{control}<div style="height:20px"></div></div>
        </body></html>"#
    )
}

/// The control in its anonymous block under `#root`, and a button `#after` past `#root`.
fn in_anonymous_block_then_after(control: &str, sheet: &str) -> String {
    format!(
        r#"<html><head><style>{sheet}</style></head><body style="margin:0">
        <div id="root">{control}<div style="height:20px"></div></div>
        <button id="after">After</button>
        </body></html>"#
    )
}

/// The accessibility tree's focus, read through the trait.
fn tree_focus(harness: &Harness<HtmlDocument>) -> TreeNodeId {
    let doc: &dyn Document = &harness.doc;
    doc.accessibility_tree().focus
}

/// The control as a block of its own, under no anonymous block.
fn in_no_anonymous_block(control: &str) -> String {
    format!(
        r#"<html><head><style>#control {{ display: block; }}</style></head><body style="margin:0">
        <div id="root">{control}</div>
        </body></html>"#
    )
}

fn boot(page: &str, incremental: bool) -> Harness<HtmlDocument> {
    Harness::from_html_with(
        page,
        HarnessOptions {
            incremental: Some(incremental),
            ..Default::default()
        },
    )
}

fn arrow_down(harness: &mut Harness<HtmlDocument>) {
    harness.press(Key::Named(NamedKey::ArrowDown));
}

/// The block holding `control` when that block is anonymous.
fn anonymous_block_of(harness: &Harness<HtmlDocument>, control: NodeId) -> Option<NodeId> {
    let doc = harness.base();
    let block = doc.get_node(control)?.layout_parent.get()?;
    doc.get_node(block)?.is_anonymous().then_some(block)
}

/// Takes the control's ancestor's box away by attribute, then lays out.
fn take_box_away(harness: &mut Harness<HtmlDocument>, way: Lost) {
    let root = harness.node("#root");
    {
        let mut doc = harness.base_mut();
        let mut mutator = doc.mutate();
        match way {
            Lost::Hidden => {
                mutator.set_attribute(root, QualName::new(None, ns!(), local_name!("hidden")), "")
            }
            Lost::StyleDisplayNone => mutator.set_attribute(
                root,
                QualName::new(None, ns!(), local_name!("style")),
                "display:none",
            ),
            Lost::FocusWithin => {}
        }
    }
    harness.pump();
}

/// The element the harness reads as focused when nothing is.
fn root_element(harness: &Harness<HtmlDocument>) -> Option<NodeId> {
    harness.base().try_root_element().map(|root| root.id)
}

/// Boots the control in its anonymous block, focuses it by Tab and takes its ancestor's box away,
/// asserting each condition the key rests on: focus given up at the layout, the block dropped.
/// Returns the harness and the control focused again with no layout since, or `None` under a
/// `:focus-within` rule, whose witness is the fixup itself, asserted here.
#[track_caller]
fn control_past_its_block(
    control: &str,
    way: Lost,
    incremental: bool,
    at: &str,
) -> Option<(Harness<HtmlDocument>, NodeId)> {
    let sheet = match way {
        Lost::FocusWithin => HIDDEN_ON_FOCUS,
        Lost::Hidden | Lost::StyleDisplayNone => "",
    };
    let mut harness = boot(&in_anonymous_block(control, sheet), incremental);
    let control = harness.node("#control");
    let block = anonymous_block_of(&harness, control);
    let block = block.unwrap_or_else(|| panic!("{at}: the control sits in an anonymous block"));

    harness.press(Key::Named(NamedKey::Tab));
    if matches!(way, Lost::FocusWithin) {
        // The box goes as focus arrives, so Tab's own layout gives focus up.
        assert_eq!(
            harness.focused(),
            root_element(&harness),
            "{at}: focus is given up"
        );
        assert!(
            harness.base().get_node(block).is_none(),
            "{at}: the anonymous block is dropped"
        );
        take_box_away(&mut harness, way);
        assert!(
            harness.layout_rect("#root").height > 0.0,
            "{at}: the container shows again once focus is gone"
        );
        return None;
    }
    assert_eq!(harness.focused(), Some(control), "{at}: Tab focuses it");
    take_box_away(&mut harness, way);

    assert_eq!(
        harness.focused(),
        root_element(&harness),
        "{at}: focus is given up"
    );
    assert!(
        harness.base().get_node(block).is_none(),
        "{at}: the anonymous block is dropped"
    );
    assert!(
        harness.base_mut().set_focus_to(control),
        "{at}: the control is focused again"
    );
    Some((harness, control))
}

#[test]
fn a_key_on_a_button_whose_anonymous_block_was_dropped_does_not_panic() {
    for incremental in [false, true] {
        for way in WAYS {
            let at = format!("incremental {incremental}: {way:?}");
            let Some((mut harness, _)) = control_past_its_block(BUTTON, way, incremental, &at)
            else {
                continue;
            };

            arrow_down(&mut harness);

            assert_eq!(
                harness.focused(),
                root_element(&harness),
                "{at}: the key's layout gives focus up again"
            );
        }
    }
}

#[test]
fn a_key_on_a_select_whose_anonymous_block_was_dropped_still_moves_its_choice() {
    for incremental in [false, true] {
        for way in WAYS {
            let at = format!("incremental {incremental}: {way:?}");
            let Some((mut harness, control)) =
                control_past_its_block(SELECT, way, incremental, &at)
            else {
                continue;
            };
            let before = harness.base().select_value(control);
            assert_eq!(
                before.as_deref(),
                Some("a"),
                "{at}: the first option stands"
            );

            arrow_down(&mut harness);

            let after = harness.base().select_value(control);
            assert_eq!(
                after.as_deref(),
                Some("b"),
                "{at}: the key moves the choice"
            );
        }
    }
}

#[test]
fn a_key_on_a_control_in_no_anonymous_block_reads_the_same() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = boot(&in_no_anonymous_block(SELECT), incremental);
        let control = harness.node("#control");
        assert_eq!(
            anonymous_block_of(&harness, control),
            None,
            "{at}: no anonymous block holds it"
        );
        harness.press(Key::Named(NamedKey::Tab));
        assert_eq!(harness.focused(), Some(control), "{at}: Tab focuses it");
        take_box_away(&mut harness, Lost::Hidden);
        assert_eq!(
            harness.focused(),
            root_element(&harness),
            "{at}: focus is given up"
        );
        assert!(
            harness.base_mut().set_focus_to(control),
            "{at}: the control is focused again"
        );

        arrow_down(&mut harness);

        let after = harness.base().select_value(control);
        assert_eq!(
            after.as_deref(),
            Some("b"),
            "{at}: the key moves the choice"
        );
    }
}

#[test]
fn a_focused_control_that_no_longer_shows_gives_focus_up_the_tree_naming_the_window() {
    let window = TreeNodeId(u64::MAX);
    for incremental in [false, true] {
        for way in WAYS {
            let at = format!("incremental {incremental}: {way:?}");
            let sheet = match way {
                Lost::FocusWithin => HIDDEN_ON_FOCUS,
                Lost::Hidden | Lost::StyleDisplayNone => "",
            };
            let mut harness = boot(&in_anonymous_block_then_after(BUTTON, sheet), incremental);
            let control = harness.node("#control");
            assert!(
                anonymous_block_of(&harness, control).is_some(),
                "{at}: the control sits in an anonymous block"
            );
            harness.press(Key::Named(NamedKey::Tab));
            // Under `:focus-within` Tab's own layout already gives focus up.
            if !matches!(way, Lost::FocusWithin) {
                assert_eq!(harness.focused(), Some(control), "{at}: Tab focuses it");
            }
            // Under `:focus-within` the box goes as focus arrives, so no shown reading exists.
            if !matches!(way, Lost::FocusWithin) {
                assert!(
                    tree_focus(&harness) == TreeNodeId(control.as_u64()),
                    "{at}: the tree's focus is the control while it shows"
                );
            }

            take_box_away(&mut harness, way);

            assert_eq!(
                harness.focused(),
                root_element(&harness),
                "{at}: focus is given up"
            );
            let doc: &dyn Document = &harness.doc;
            let tree = doc.accessibility_tree();
            // Under `:focus-within` the container shows again once focus is gone.
            let left_out = !matches!(way, Lost::FocusWithin);
            assert!(
                left_out
                    != tree
                        .nodes
                        .iter()
                        .any(|(id, _)| *id == TreeNodeId(control.as_u64())),
                "{at}: the control is in the tree only where it shows"
            );
            assert!(
                tree.focus == window && tree.nodes.iter().any(|(id, _)| *id == tree.focus),
                "{at}: the tree's focus is the window, a node it holds"
            );

            harness.press(Key::Named(NamedKey::Tab));
            let after = harness.node("#after");
            // A stated limit: the shown control takes Tab, and its own `:focus-within` rule then
            // hides it, so the fixup gives focus up again.
            let expected = match way {
                Lost::FocusWithin => root_element(&harness),
                Lost::Hidden | Lost::StyleDisplayNone => Some(after),
            };
            assert_eq!(
                harness.focused(),
                expected,
                "{at}: the next Tab skips what does not show"
            );
        }
    }
}
