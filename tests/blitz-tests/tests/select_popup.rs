//! A single-choice `select` opens a list the engine draws over the page, in every document: a
//! click, Enter and Space open it, its highlight on the chosen option; the arrows, Home, End and
//! typed text move the highlight over the enabled rows and choose nothing; Enter, Space, Tab and a
//! click on a row choose, one `input` then one `change`, and a choice of the chosen option fires
//! neither; Escape, a second click, a click outside — which goes on to what it landed on — and the
//! page scrolling close it with no event. A disabled, inert, hidden or singularly transformed
//! select opens nothing, and one made so with its list open closes with no panic. The tree reads
//! `expanded` on the select and no `aria-expanded`, its focus staying on the select. Boxes that
//! clip the select do not clip the list. Hostile markup keeps every box finite, and a `multiple`
//! select's `size` makes it that many rows tall. In both layout modes, through real dispatch. A
//! failure message carries the layout mode and a row index.
//!
//! <https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element>

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use ::keyboard_types::{Key, Modifiers, NamedKey};
use accesskit::{Node as TreeNode, NodeId as TreeId, Role};
use blitz_dom::{BoundingRect, DocumentConfig, NodeId, QualName, ns};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_test_harness::Harness;
use blitz_traits::shell::{ColorScheme, ShellProvider, Viewport};
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand;

mod fixtures;

type Page = Harness<DioxusDocument>;

/// `fx-pick` with Banana written `selected` and a disabled Blackberry, in an app that writes
/// `selected` on the option its `input` event names, as a controlled select's app does, counts its
/// `input` and `change` events and logs its focus events; a grouped select; a select for each gate;
/// and a far button that counts its clicks.
fn page() -> Element {
    let mut inputs = use_signal(|| 0u32);
    let mut changes = use_signal(|| 0u32);
    let mut clicks = use_signal(|| 0u32);
    let mut log = use_signal(String::new);
    let mut seeded = use_signal(|| "b".to_string());
    rsx! {
        div { id: "fx-root", style: "padding-top: 60px; height: 1400px; position: relative;",
            label { id: "fx-label", r#for: "fx-pick", style: "display: block; width: 60px;", "Fruit" }
            select {
                id: "fx-pick",
                oninput: move |event| {
                    *inputs.write() += 1;
                    log.write().push_str("input ");
                    seeded.set(event.value());
                },
                onchange: move |_| {
                    *changes.write() += 1;
                    log.write().push_str("change ");
                },
                onblur: move |_| log.write().push_str("blur "),
                option { id: "fx-apple", value: "a", selected: seeded() == "a", "Apple" }
                option { id: "fx-avocado", value: "v", selected: seeded() == "v", "Avocado" }
                option { id: "fx-banana", value: "b", selected: seeded() == "b", "Banana" }
                option { id: "fx-blackberry", value: "k", disabled: true, "Blackberry" }
                option { id: "fx-blueberry", value: "u", selected: seeded() == "u", "Blueberry" }
                option { id: "fx-cherry", value: "c", selected: seeded() == "c", "Cherry" }
            }
            button { id: "fx-next", style: "display: block; margin-top: 300px;", "Next" }
            select { id: "fx-grouped",
                option { id: "fx-plain", value: "p", "Plain" }
                optgroup { id: "fx-fruit", label: "Fruit",
                    option { id: "fx-berry", value: "b", "Berry" }
                }
                optgroup { id: "fx-off-group", label: "Off", disabled: true,
                    option { id: "fx-gone", value: "g", "Gone" }
                }
                optgroup { id: "fx-unlabelled",
                    option { id: "fx-bare", value: "e", "Bare" }
                }
                option { id: "fx-hidden-row", value: "h", hidden: true, "Hidden" }
            }
            select { id: "fx-off", disabled: true,
                option { id: "fx-off-a", value: "a", "A" }
            }
            div { "inert": "true",
                select { id: "fx-inert",
                    option { id: "fx-inert-a", value: "a", "A" }
                }
            }
            select { id: "fx-hidden", hidden: true,
                option { id: "fx-hidden-a", value: "a", "A" }
            }
            div { style: "transform: scale(0);",
                select { id: "fx-flat",
                    option { id: "fx-flat-a", value: "a", "A" }
                }
            }
            select { id: "fx-empty" }
            button {
                id: "fx-far",
                style: "position: absolute; left: 500px; top: 10px;",
                onclick: move |_| *clicks.write() += 1,
                "Far"
            }
            p { id: "fx-inputs", "{inputs}" }
            p { id: "fx-changes", "{changes}" }
            p { id: "fx-clicks", "{clicks}" }
            p { id: "fx-log", "{log}" }
        }
    }
}

fn number(harness: &Page, selector: &str) -> u32 {
    harness
        .text_content(selector)
        .trim()
        .parse()
        .unwrap_or(u32::MAX)
}

fn counted(harness: &Page) -> (u32, u32) {
    (
        number(harness, "#fx-inputs"),
        number(harness, "#fx-changes"),
    )
}

fn value(harness: &Page, selector: &str) -> Option<String> {
    harness.base().select_value(harness.node(selector))
}

fn open(harness: &Page) -> Option<NodeId> {
    harness.base().open_select()
}

/// The highlighted row.
fn highlighted(harness: &Page) -> Option<NodeId> {
    let view = harness.base().select_list_view()?;
    view.rows
        .iter()
        .find(|row| row.highlighted)
        .map(|row| row.node)
}

/// The page point at the centre of a client rect.
fn point_of(harness: &Page, rect: BoundingRect) -> (f32, f32) {
    let scroll = harness.base().viewport_scroll();
    (
        (rect.x + rect.width / 2.0 + scroll.x) as f32,
        (rect.y + rect.height / 2.0 + scroll.y) as f32,
    )
}

/// The page point at the centre of the open list's row `selector`.
fn row_point(harness: &Page, selector: &str, at: &str) -> (f32, f32) {
    let rect = harness.base().select_list_bounds(harness.node(selector));
    let rect = rect.unwrap_or_else(|| panic!("{at}: the row reads a box"));
    point_of(harness, rect)
}

/// The page point at the centre of `selector`'s client rect.
fn client_point(harness: &Page, selector: &str, at: &str) -> (f32, f32) {
    let rect = harness
        .base()
        .get_client_bounding_rect(harness.node(selector));
    let rect = rect.unwrap_or_else(|| panic!("{at}: the element reads a client rect"));
    point_of(harness, rect)
}

fn click(harness: &mut Page, selector: &str, at: &str) {
    let (x, y) = client_point(harness, selector, at);
    harness.click_at(x, y);
}

fn click_row(harness: &mut Page, selector: &str, at: &str) {
    let (x, y) = row_point(harness, selector, at);
    harness.click_at(x, y);
}

fn named(key: NamedKey) -> Key {
    Key::Named(key)
}

fn space() -> Key {
    Key::Character(" ".to_string())
}

/// The ways a list opens: a click, and Enter and Space on the select a key press focuses.
const OPENERS: usize = 3;

fn open_by(harness: &mut Page, opener: usize, at: &str) {
    match opener {
        0 => click(harness, "#fx-pick", at),
        key => {
            let pick = harness.node("#fx-pick");
            harness.base_mut().set_focus_to(pick);
            harness.press(if key == 1 {
                named(NamedKey::Enter)
            } else {
                space()
            });
        }
    }
}

#[test]
fn a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing() {
    for incremental in [false, true] {
        for opener in 0..OPENERS {
            let at = format!("incremental {incremental}: opener {opener}");
            let mut harness = fixtures::boot(page, incremental);
            let pick = harness.node("#fx-pick");
            assert_eq!(open(&harness), None, "{at}: the fixture starts closed");
            harness.base_mut().take_changed_nodes();
            open_by(&mut harness, opener, &at);
            assert_eq!(open(&harness), Some(pick), "{at}: the list opens");
            assert_eq!(
                highlighted(&harness),
                Some(harness.node("#fx-banana")),
                "{at}: the highlight starts on the chosen option"
            );
            assert_eq!(counted(&harness), (0, 0), "{at}: opening fires nothing");
            assert!(
                harness.base().has_changes(),
                "{at}: opening marks the changed set"
            );
            assert_eq!(
                harness.focused(),
                Some(pick),
                "{at}: focus is on the select"
            );
        }
    }
}

/// Keys pressed in order from Banana with the list open, and the row each leaves highlighted.
fn moves() -> [(Key, &'static str); 11] {
    [
        (named(NamedKey::ArrowDown), "#fx-blueberry"),
        (named(NamedKey::ArrowDown), "#fx-cherry"),
        (named(NamedKey::ArrowDown), "#fx-cherry"),
        (named(NamedKey::ArrowUp), "#fx-blueberry"),
        (named(NamedKey::ArrowUp), "#fx-banana"),
        (named(NamedKey::Home), "#fx-apple"),
        (named(NamedKey::ArrowUp), "#fx-apple"),
        (named(NamedKey::End), "#fx-cherry"),
        (named(NamedKey::ArrowLeft), "#fx-cherry"),
        (named(NamedKey::Backspace), "#fx-cherry"),
        (Key::Character("b".to_string()), "#fx-banana"),
    ]
}

#[test]
fn the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing() {
    assert_eq!(moves().len(), 11);
    for incremental in [false, true] {
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-pick", "fixture");
        harness.base_mut().take_changed_nodes();
        for (row, (key, expected)) in moves().into_iter().enumerate() {
            let at = format!("incremental {incremental}: row {row}");
            harness.press(key);
            assert_eq!(
                highlighted(&harness),
                Some(harness.node(expected)),
                "{at}: the highlight"
            );
            assert_eq!(
                value(&harness, "#fx-pick").as_deref(),
                Some("b"),
                "{at}: the choice stands"
            );
            assert_eq!(counted(&harness), (0, 0), "{at}: a move fires nothing");
            assert!(!harness.base().has_changes(), "{at}: a move marks nothing");
        }
    }
}

/// The ways a row is chosen: Enter, Space, Tab and a click.
const CHOOSERS: usize = 4;

fn choose_by(harness: &mut Page, chooser: usize, at: &str) {
    match chooser {
        0 => harness.press(named(NamedKey::Enter)),
        1 => harness.press(space()),
        2 => harness.press(named(NamedKey::Tab)),
        _ => {
            let row = highlighted(harness);
            let row = row.unwrap_or_else(|| panic!("{at}: a row is highlighted"));
            let rect = harness.base().select_list_bounds(row);
            let rect = rect.unwrap_or_else(|| panic!("{at}: the row reads a box"));
            let (x, y) = point_of(harness, rect);
            harness.click_at(x, y);
        }
    }
}

#[test]
fn enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change() {
    for incremental in [false, true] {
        for chooser in 0..CHOOSERS {
            let at = format!("incremental {incremental}: chooser {chooser}");
            let mut harness = fixtures::boot(page, incremental);
            click(&mut harness, "#fx-pick", &at);
            harness.press(named(NamedKey::ArrowDown));
            harness.base_mut().take_changed_nodes();
            choose_by(&mut harness, chooser, &at);
            assert_eq!(
                value(&harness, "#fx-pick").as_deref(),
                Some("u"),
                "{at}: the select reads the row"
            );
            assert_eq!(counted(&harness), (1, 1), "{at}: one input, one change");
            assert_eq!(open(&harness), None, "{at}: the list closes");
            assert!(
                harness.base().has_changes(),
                "{at}: the change marks the set"
            );
            assert!(
                harness.text_content("#fx-log").starts_with("input change"),
                "{at}: input comes before change"
            );
            let expected_focus = if chooser == 2 { "#fx-next" } else { "#fx-pick" };
            assert_eq!(
                harness.focused(),
                Some(harness.node(expected_focus)),
                "{at}: Tab moves on and the rest keep focus"
            );
        }
    }
}

#[test]
fn tab_chooses_and_closes_before_focus_moves_either_way() {
    for incremental in [false, true] {
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-pick", "fixture");
        harness.press(named(NamedKey::End));
        harness.press(named(NamedKey::Tab));
        assert_eq!(
            harness.text_content("#fx-log").trim(),
            "input change blur",
            "incremental {incremental}: the choice's events come before the blur"
        );

        click(&mut harness, "#fx-pick", "fixture");
        harness.press(named(NamedKey::Home));
        harness.press_with(named(NamedKey::Tab), Modifiers::SHIFT);
        assert_eq!(
            value(&harness, "#fx-pick").as_deref(),
            Some("a"),
            "incremental {incremental}: Shift+Tab chooses too"
        );
        assert_ne!(
            harness.focused(),
            Some(harness.node("#fx-pick")),
            "incremental {incremental}: Shift+Tab moves on"
        );
        assert_eq!(
            open(&harness),
            None,
            "incremental {incremental}: and closes"
        );
    }
}

#[test]
fn a_choice_of_the_chosen_option_fires_nothing() {
    for incremental in [false, true] {
        for chooser in 0..CHOOSERS {
            let at = format!("incremental {incremental}: chooser {chooser}");
            let mut harness = fixtures::boot(page, incremental);
            click(&mut harness, "#fx-pick", &at);
            choose_by(&mut harness, chooser, &at);
            assert_eq!(counted(&harness), (0, 0), "{at}: nothing changed");
            assert_eq!(open(&harness), None, "{at}: the list closes");
        }
    }
}

#[test]
fn a_disabled_row_changes_nothing_and_leaves_the_list_open() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-pick", &at);
        click_row(&mut harness, "#fx-blackberry", &at);
        assert_eq!(counted(&harness), (0, 0), "{at}: nothing fires");
        assert_eq!(value(&harness, "#fx-pick").as_deref(), Some("b"), "{at}");
        assert!(open(&harness).is_some(), "{at}: the list stands");
    }
}

/// What a list closes by with no choice, and whether the click reaches the far button.
fn close_by(harness: &mut Page, closer: usize, at: &str) {
    match closer {
        0 => harness.press(named(NamedKey::Escape)),
        1 => click(harness, "#fx-pick", at),
        2 => click(harness, "#fx-far", at),
        3 => click(harness, "#fx-label", at),
        _ => harness.scroll_into_view(harness.node("#fx-clicks")),
    }
}

#[test]
fn escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event() {
    // (closer, the far button's clicks it leaves)
    let rows = [(0, 0), (1, 0), (2, 1), (3, 0), (4, 0)];
    assert_eq!(rows.len(), 5);
    for incremental in [false, true] {
        for (closer, clicks) in rows {
            let at = format!("incremental {incremental}: closer {closer}");
            let mut harness = fixtures::boot(page, incremental);
            click(&mut harness, "#fx-pick", &at);
            harness.press(named(NamedKey::ArrowDown));
            assert!(open(&harness).is_some(), "{at}: the fixture's list opens");
            harness.base_mut().take_changed_nodes();
            close_by(&mut harness, closer, &at);
            assert_eq!(open(&harness), None, "{at}: the list closes");
            assert_eq!(counted(&harness), (0, 0), "{at}: no event fires");
            assert_eq!(value(&harness, "#fx-pick").as_deref(), Some("b"), "{at}");
            assert_eq!(
                number(&harness, "#fx-clicks"),
                clicks,
                "{at}: a click outside goes on to what it landed on"
            );
            assert!(harness.base().has_changes(), "{at}: a close marks the set");
        }
    }
}

#[test]
fn a_labels_click_focuses_the_select_and_opens_nothing() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-label", &at);
        assert_eq!(
            harness.focused(),
            Some(harness.node("#fx-pick")),
            "{at}: the label focuses its select"
        );
        assert_eq!(open(&harness), None, "{at}: and opens nothing");
    }
}

#[test]
fn a_disabled_inert_hidden_singular_or_empty_select_opens_nothing() {
    let rows = [
        "#fx-off",
        "#fx-inert",
        "#fx-hidden",
        "#fx-flat",
        "#fx-empty",
    ];
    assert_eq!(rows.len(), 5);
    for incremental in [false, true] {
        for (row, selector) in rows.into_iter().enumerate() {
            let at = format!("incremental {incremental}: row {row}");
            let mut harness = fixtures::boot(page, incremental);
            let select = harness.node(selector);
            let rect = harness.base().get_client_bounding_rect(select);
            if let Some(rect) = rect {
                let (x, y) = point_of(&harness, rect);
                harness.click_at(x, y);
            }
            assert_eq!(open(&harness), None, "{at}: a click opens nothing");
            harness.base_mut().set_focus_to(select);
            for key in [named(NamedKey::Enter), space(), named(NamedKey::ArrowDown)] {
                harness.press(key);
                assert_eq!(open(&harness), None, "{at}: a key opens nothing");
            }
            assert_eq!(counted(&harness), (0, 0), "{at}: nothing fires");
        }
    }
}

fn attribute(name: &str) -> QualName {
    QualName::new(None, ns!(), name.into())
}

#[test]
fn a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes() {
    let rows = 5;
    for incremental in [false, true] {
        for row in 0..rows {
            let at = format!("incremental {incremental}: row {row}");
            let mut harness = fixtures::boot(page, incremental);
            let pick = harness.node("#fx-pick");
            let root = harness.node("#fx-root");
            click(&mut harness, "#fx-pick", &at);
            assert_eq!(open(&harness), Some(pick), "{at}: the fixture's list opens");
            {
                let mut doc = harness.base_mut();
                let mut mutator = doc.mutate();
                match row {
                    0 => mutator.set_attribute(pick, attribute("inert"), ""),
                    1 => mutator.set_attribute(pick, attribute("disabled"), ""),
                    2 => mutator.set_attribute(pick, attribute("hidden"), ""),
                    3 => mutator.set_attribute(root, attribute("hidden"), ""),
                    _ => {
                        mutator.remove_and_drop_node(pick);
                    }
                }
            }
            harness.pump();
            assert_eq!(open(&harness), None, "{at}: the list closes");
            assert!(
                harness.base().select_list_view().is_none(),
                "{at}: paint reads no list"
            );
            let (x, y) = (100.0, 120.0);
            assert!(
                harness.hit(x, y).is_none_or(|hit| hit.node_id != pick),
                "{at}: a hit reads what stands there"
            );
            // A key with focus under a newly hidden ancestor reads the anonymous block that hiding
            // dropped, a panic of the event path with no list open too; it is not this check's.
            if row != 3 {
                harness.press(named(NamedKey::ArrowDown));
            }
        }
    }
}

#[test]
fn an_option_dropped_with_the_list_open_is_no_row() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-pick", &at);
        let cherry = harness.node("#fx-cherry");
        harness.press(named(NamedKey::End));
        assert_eq!(highlighted(&harness), Some(cherry), "{at}: the fixture");
        let before = harness
            .base()
            .select_list_view()
            .map(|view| view.rows.len());
        harness.base_mut().mutate().remove_and_drop_node(cherry);
        // Read with no resolve between: the rows are derived, so a dropped node is none of them.
        let after = harness
            .base()
            .select_list_view()
            .map(|view| view.rows.len());
        assert_eq!(after, before.map(|rows| rows - 1), "{at}: its row is gone");
        assert_eq!(
            highlighted(&harness),
            None,
            "{at}: the highlight goes with it"
        );
        harness.pump();
        harness.press(named(NamedKey::Enter));
        assert_eq!(counted(&harness), (0, 0), "{at}: Enter chooses nothing");
        assert_eq!(value(&harness, "#fx-pick").as_deref(), Some("b"), "{at}");
    }
}

/// The accessibility node of `selector`.
fn tree_node(harness: &Page, selector: &str, at: &str) -> TreeNode {
    let id = TreeId(harness.node(selector).as_u64());
    let tree = harness.base().build_accessibility_tree();
    tree.nodes
        .into_iter()
        .find(|(node, _)| *node == id)
        .map(|(_, node)| node)
        .unwrap_or_else(|| panic!("{at}: the tree holds the node"))
}

#[test]
fn the_tree_reads_the_list_open_on_the_select_its_focus_staying_there() {
    // (the step, whether the select then reads expanded)
    let rows: [(usize, bool); 4] = [(0, true), (1, false), (0, true), (2, false)];
    assert_eq!(rows.len(), 4);
    for incremental in [false, true] {
        let mut harness = fixtures::boot(page, incremental);
        let at = format!("incremental {incremental}");
        assert_eq!(
            tree_node(&harness, "#fx-pick", &at).is_expanded(),
            Some(false),
            "{at}: a closed select reads not expanded"
        );
        for (row, (step, expanded)) in rows.into_iter().enumerate() {
            let at = format!("{at}: row {row}");
            harness.base_mut().take_changed_nodes();
            match step {
                0 => click(&mut harness, "#fx-pick", &at),
                1 => harness.press(named(NamedKey::Escape)),
                _ => harness.press(named(NamedKey::Enter)),
            }
            assert!(harness.base().has_changes(), "{at}: the step marks the set");
            let select = tree_node(&harness, "#fx-pick", &at);
            assert_eq!(select.is_expanded(), Some(expanded), "{at}: expanded");
            assert_eq!(select.role(), Role::ComboBox, "{at}: role");
            let selected: Vec<_> = [
                "#fx-apple",
                "#fx-avocado",
                "#fx-banana",
                "#fx-blackberry",
                "#fx-blueberry",
                "#fx-cherry",
            ]
            .into_iter()
            .filter(|option| {
                let node = tree_node(&harness, option, &at);
                assert_eq!(node.role(), Role::ListBoxOption, "{at}: an option's role");
                node.is_selected() == Some(true)
            })
            .collect();
            assert_eq!(selected, ["#fx-banana"], "{at}: exactly the chosen option");
            let tree = harness.base().build_accessibility_tree();
            assert_eq!(
                tree.focus,
                TreeId(harness.node("#fx-pick").as_u64()),
                "{at}: the tree's focus stays on the select"
            );
        }
    }
}

/// A select whose author attributes must steer nothing: `open` and `aria-expanded`.
fn stated_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-stated", aria_expanded: "true", "open": "true",
                option { id: "fx-stated-a", value: "a", "A" }
            }
            button { id: "fx-toggle", aria_expanded: "true", "Toggle" }
        }
    }
}

#[test]
fn an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(stated_page, incremental);
        assert_eq!(
            tree_node(&harness, "#fx-stated", &at).is_expanded(),
            Some(false),
            "{at}: a select reads no aria-expanded"
        );
        assert_eq!(
            tree_node(&harness, "#fx-toggle", &at).is_expanded(),
            Some(true),
            "{at}: another element still reads it"
        );
        assert!(open(&harness).is_none(), "{at}: open opens nothing");
        click(&mut harness, "#fx-stated", &at);
        assert!(open(&harness).is_some(), "{at}: the fixture's list opens");
        assert!(
            harness.query("select:open").is_none(),
            "{at}: :open does not match a select"
        );
    }
}

/// A select inside an `overflow: hidden` box, one inside a `contain: paint` box, and a button a row
/// lies over.
fn clipped_page() -> Element {
    rsx! {
        div { id: "fx-root",
            div { id: "fx-clip", style: "width: 200px; height: 40px; overflow: hidden;",
                select { id: "fx-in-clip",
                    option { id: "fx-clip-a", value: "a", "Alpha" }
                    option { id: "fx-clip-b", value: "b", "Beta" }
                    option { id: "fx-clip-c", value: "c", "Gamma" }
                }
            }
            div { id: "fx-paint", style: "width: 200px; height: 40px; contain: paint;",
                select { id: "fx-in-paint",
                    option { id: "fx-paint-a", value: "a", "Alpha" }
                    option { id: "fx-paint-b", value: "b", "Beta" }
                    option { id: "fx-paint-c", value: "c", "Gamma" }
                }
            }
            button { id: "fx-beside", style: "position: absolute; left: 400px; top: 60px;", "Beside" }
        }
    }
}

#[test]
fn a_box_that_clips_the_select_clips_no_row_and_a_point_outside_reads_what_shows() {
    let rows = [
        ("#fx-in-clip", "#fx-clip-c", "#fx-clip"),
        ("#fx-in-paint", "#fx-paint-c", "#fx-paint"),
    ];
    assert_eq!(rows.len(), 2);
    for incremental in [false, true] {
        for (row, (select, option, clip)) in rows.into_iter().enumerate() {
            let at = format!("incremental {incremental}: row {row}");
            let mut harness = fixtures::boot(clipped_page, incremental);
            click(&mut harness, select, &at);
            assert!(open(&harness).is_some(), "{at}: the list opens");
            let (x, y) = row_point(&harness, option, &at);
            let boxed = harness.base().get_client_bounding_rect(harness.node(clip));
            let boxed = boxed.unwrap_or_else(|| panic!("{at}: the box reads a client rect"));
            assert!(
                f64::from(y) > boxed.y + boxed.height,
                "{at}: the fixture's row lies past the box that clips the select"
            );
            assert_eq!(
                harness.hit(x, y).map(|hit| hit.node_id),
                Some(harness.node(option)),
                "{at}: the row is hit where it paints"
            );
            let (beside_x, beside_y) = client_point(&harness, "#fx-beside", &at);
            assert_eq!(
                harness.base().element_from_point(beside_x, beside_y),
                Some(harness.node("#fx-beside")),
                "{at}: a point the list does not cover reads what paint shows"
            );
            harness.click_at(x, y);
            assert_eq!(
                value(&harness, select).as_deref(),
                Some("c"),
                "{at}: the click chooses it"
            );
        }
    }
}

#[test]
fn a_pointer_at_a_rows_centre_lands_on_it_with_the_page_scrolled_or_not() {
    let scrolled_by = [0.0, -30.0];
    assert_eq!(scrolled_by.len(), 2);
    for incremental in [false, true] {
        for (row, delta) in scrolled_by.into_iter().enumerate() {
            let at = format!("incremental {incremental}: row {row}");
            let mut harness = fixtures::boot(page, incremental);
            if delta != 0.0 {
                harness.wheel_at(700.0, 500.0, 0.0, delta);
                assert!(
                    harness.base().viewport_scroll().y > 0.0,
                    "{at}: the fixture scrolled the page"
                );
            }
            click(&mut harness, "#fx-pick", &at);
            assert!(open(&harness).is_some(), "{at}: the list opens");
            let (x, y) = row_point(&harness, "#fx-apple", &at);
            assert_eq!(
                harness.hit(x, y).map(|hit| hit.node_id),
                Some(harness.node("#fx-apple")),
                "{at}: the hit reads the row"
            );
            harness.click_at(x, y);
            assert_eq!(
                value(&harness, "#fx-pick").as_deref(),
                Some("a"),
                "{at}: the click chooses it"
            );
        }
    }
}

#[test]
fn the_pointer_moves_the_highlight_to_an_enabled_row_and_a_later_key_stands() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-pick", &at);
        let (x, y) = row_point(&harness, "#fx-apple", &at);
        harness.move_mouse_to(x, y);
        assert_eq!(
            highlighted(&harness),
            Some(harness.node("#fx-apple")),
            "{at}: the pointer moves the highlight"
        );
        let (x, y) = row_point(&harness, "#fx-blackberry", &at);
        harness.move_mouse_to(x, y);
        assert_eq!(
            highlighted(&harness),
            Some(harness.node("#fx-apple")),
            "{at}: a disabled row takes no highlight"
        );
        let (x, y) = row_point(&harness, "#fx-apple", &at);
        harness.move_mouse_to(x, y);
        harness.press(named(NamedKey::End));
        harness.pump();
        assert_eq!(
            highlighted(&harness),
            Some(harness.node("#fx-cherry")),
            "{at}: a hover refresh does not undo a key's move"
        );
    }
}

#[test]
fn a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-pick", &at);
        let from = row_point(&harness, "#fx-apple", &at);
        let to = row_point(&harness, "#fx-cherry", &at);
        harness.mouse_down_at(from.0, from.1);
        assert!(
            harness.base().get_selected_text().is_none(),
            "{at}: a press on a row selects no text"
        );
        harness.drag(from, to, 4);
        assert_eq!(
            value(&harness, "#fx-pick").as_deref(),
            Some("c"),
            "{at}: the release chooses the row under it"
        );
    }
}

#[test]
fn groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        let grouped = harness.node("#fx-grouped");
        harness.base_mut().set_focus_to(grouped);
        harness.press(named(NamedKey::Enter));
        assert_eq!(
            open(&harness),
            Some(grouped),
            "{at}: the fixture's list opens"
        );
        let view = harness.base().select_list_view();
        let view = view.unwrap_or_else(|| panic!("{at}: paint reads the list"));
        // The hidden option is no row, and a group with no label has no header.
        let labels: Vec<&str> = view.rows.iter().map(|row| row.label.as_str()).collect();
        assert_eq!(
            labels,
            ["Plain", "Fruit", "Berry", "Off", "Gone", "Bare"],
            "{at}: the rows"
        );
        assert!(
            view.rows[2].text_x > view.rows[1].text_x,
            "{at}: an option under a header is indented past it"
        );
        assert!(
            view.rows[4].disabled,
            "{at}: a disabled group's option reads so"
        );
        click_row(&mut harness, "#fx-fruit", &at);
        assert_eq!(
            open(&harness),
            Some(grouped),
            "{at}: a header leaves it open"
        );
        assert_eq!(value(&harness, "#fx-grouped").as_deref(), Some("p"), "{at}");
        harness.press(named(NamedKey::End));
        assert_eq!(
            highlighted(&harness),
            Some(harness.node("#fx-bare")),
            "{at}: a key reaches a group's option and skips no enabled row"
        );
        harness.press(named(NamedKey::ArrowUp));
        assert_eq!(
            highlighted(&harness),
            Some(harness.node("#fx-berry")),
            "{at}: a key skips a disabled group's option and its header"
        );
    }
}

/// A select of 40 rows, past the list's limit.
fn long_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-long",
                for row in 0..40 {
                    option { key: "{row}", id: "fx-row-{row}", value: "{row}", "Row {row}" }
                }
            }
        }
    }
}

#[test]
fn a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(long_page, incremental);
        click(&mut harness, "#fx-long", &at);
        let view = harness.base().select_list_view();
        let view = view.unwrap_or_else(|| panic!("{at}: paint reads the list"));
        assert_eq!(
            view.rows.len(),
            20,
            "{at}: twenty rows reach the scrollport"
        );
        assert!(view.thumb.is_some(), "{at}: a thumb is read");
        harness.press(named(NamedKey::End));
        let last = harness
            .base()
            .select_list_bounds(harness.node("#fx-row-39"));
        let last = last.unwrap_or_else(|| panic!("{at}: the row reads a box"));
        let port = harness
            .base()
            .select_list_view()
            .map(|view| view.scrollport);
        let port = port.unwrap_or_else(|| panic!("{at}: paint reads the list"));
        assert!(
            last.y >= port.y && last.y + last.height <= port.y + port.height + 0.01,
            "{at}: a key brings its row in"
        );
        let (x, y) = point_of(&harness, port);
        harness.wheel_at(x, y, 0.0, 60.0);
        let moved = harness
            .base()
            .select_list_bounds(harness.node("#fx-row-39"));
        assert!(
            moved.is_some_and(|moved| moved.y > last.y),
            "{at}: the wheel over the list scrolls its rows"
        );
        assert!(open(&harness).is_some(), "{at}: and leaves it open");
        harness.wheel_at(790.0, 590.0, 0.0, -10.0);
        assert_eq!(open(&harness), None, "{at}: a wheel elsewhere closes it");
    }
}

/// Hostile markup: `size` non-numeric, `0` and oversized, 2,000 options and a 5,500-character label.
fn hostile_html() -> String {
    let options: String = (0..2000)
        .map(|row| format!(r#"<option value="{row}">Row {row}</option>"#))
        .collect();
    let label = "x".repeat(5500);
    format!(
        r#"<html><body>
        <select id="many">{options}</select>
        <select id="wide"><option>{label}</option><option>Short</option></select>
        <select id="word" size="many"><option>A</option></select>
        <select id="zero" multiple size="0"><option>A</option><option>B</option></select>
        <select id="huge" multiple size="4294967296"><option>A</option></select>
        <select id="vast" multiple size="70000"><option>A</option></select>
        <select id="bad" multiple size="-3"><option>A</option></select>
        </body></html>"#
    )
}

fn finite(rect: BoundingRect) -> bool {
    [rect.x, rect.y, rect.width, rect.height]
        .iter()
        .all(|edge| edge.is_finite())
}

#[test]
fn hostile_markup_keeps_every_box_finite_and_panics_nowhere() {
    let html = hostile_html();
    let opened = ["#many", "#wide", "#word"];
    let others = ["#zero", "#huge", "#vast", "#bad"];
    assert_eq!(opened.len() + others.len(), 7);
    for incremental in [false, true] {
        let mut harness =
            Harness::<HtmlDocument>::from_html_with(&html, stand::options(incremental));
        for (row, selector) in opened.into_iter().enumerate() {
            let at = format!("incremental {incremental}: row {row}");
            let select = harness.node(selector);
            harness.base_mut().set_focus_to(select);
            harness.press(named(NamedKey::Enter));
            assert_eq!(harness.base().open_select(), Some(select), "{at}: it opens");
            let view = harness.base().select_list_view();
            let view = view.unwrap_or_else(|| panic!("{at}: paint reads the list"));
            assert!(finite(view.bounds), "{at}: the list is finite");
            assert!(view.bounds.width <= 800.0, "{at}: within the viewport");
            assert!(view.rows.len() <= 20, "{at}: at most twenty rows are read");
            assert!(
                view.rows.iter().all(|row| finite(row.bounds)),
                "{at}: every row is finite"
            );
            harness.press(named(NamedKey::End));
            harness.press(named(NamedKey::Escape));
        }
        for (row, selector) in others.into_iter().enumerate() {
            let at = format!("incremental {incremental}: other {row}");
            let select = harness.node(selector);
            harness.base_mut().set_focus_to(select);
            harness.press(named(NamedKey::ArrowDown));
            let rect = harness.base().get_client_bounding_rect(select);
            assert!(rect.is_some_and(finite), "{at}: its box is finite");
            assert_eq!(harness.base().open_select(), None, "{at}: it opens no list");
        }
    }
}

/// `multiple` selects of six rows: one with `size=3`, one with no `size`.
fn sized_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-sized", multiple: true, "size": "3",
                for row in 0..6 {
                    option { key: "{row}", id: "fx-s-{row}", value: "{row}", "Row {row}" }
                }
            }
            select { id: "fx-all", multiple: true,
                for row in 0..6 {
                    option { key: "{row}", id: "fx-a-{row}", value: "{row}", "Row {row}" }
                }
            }
        }
    }
}

#[test]
fn size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(sized_page, incremental);
        let rect = |harness: &Page, selector: &str| {
            harness
                .base()
                .get_client_bounding_rect(harness.node(selector))
                .unwrap_or_else(|| panic!("{at}: the element reads a client rect"))
        };
        let box_rect = rect(&harness, "#fx-sized");
        let row = rect(&harness, "#fx-s-0");
        let chrome = 6.0; // the default sheet's 2px padding and 1px border, each side.
        let rows = (box_rect.height - chrome) / row.height;
        assert!(
            (rows - 3.0).abs() < 0.2,
            "{at}: the box holds three rows, reading {rows}"
        );
        let all = rect(&harness, "#fx-all");
        let row_all = rect(&harness, "#fx-a-0");
        assert!(
            (all.height - chrome) / row_all.height > 5.5,
            "{at}: with no size every row shows"
        );
        let sized = harness.node("#fx-sized");
        harness.base_mut().set_focus_to(sized);
        for _ in 0..4 {
            harness.press(named(NamedKey::ArrowDown));
        }
        let fourth = rect(&harness, "#fx-s-3");
        let box_rect = rect(&harness, "#fx-sized");
        assert!(
            fourth.y + fourth.height <= box_rect.y + box_rect.height + 1.0,
            "{at}: a key brings the fourth row in"
        );
    }
}

#[derive(Default)]
struct CountingShell {
    redraws: AtomicUsize,
}

impl ShellProvider for CountingShell {
    fn request_redraw(&self) {
        self.redraws.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn opening_moving_scrolling_and_closing_each_ask_a_redraw() {
    let html = r#"<html><body><select id="pick">
        <option>A</option><option>B</option><option>C</option>
    </select></body></html>"#;
    for incremental in [false, true] {
        let shell = Arc::new(CountingShell::default());
        let mut doc = HtmlDocument::from_html(
            html,
            DocumentConfig {
                viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
                html_parser_provider: Some(Arc::new(HtmlProvider) as _),
                shell_provider: Some(shell.clone() as _),
                ..Default::default()
            },
        );
        doc.set_incremental_layout(incremental);
        doc.resolve(0.0);
        let mut harness = Harness::wrap(doc);
        harness.pump();
        let pick = harness.node("#pick");
        harness.base_mut().set_focus_to(pick);
        harness.pump();
        // (the key, the state it leaves) in order: open, move, close.
        let rows = [
            (named(NamedKey::Enter), true),
            (named(NamedKey::ArrowDown), true),
            (named(NamedKey::Escape), false),
        ];
        assert_eq!(rows.len(), 3);
        for (row, (key, stands)) in rows.into_iter().enumerate() {
            let at = format!("incremental {incremental}: row {row}");
            shell.redraws.swap(0, Ordering::Relaxed);
            harness.press(key);
            assert!(
                shell.redraws.load(Ordering::Relaxed) > 0,
                "{at}: the step asks a redraw"
            );
            assert_eq!(
                harness.base().open_select().is_some(),
                stands,
                "{at}: the list's state"
            );
        }
    }
}

/// A select whose `:focus` rule moves its box, as a thicker focus border does.
fn focus_moved_page() -> Element {
    rsx! {
        div { id: "fx-root",
            style { "#fx-moved {{ border: 1px solid #999; }} #fx-moved:focus {{ border: 4px solid #333; }}" }
            select { id: "fx-moved",
                option { id: "fx-moved-a", value: "a", "Alpha" }
                option { id: "fx-moved-b", value: "b", "Beta" }
            }
        }
    }
}

#[test]
fn a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(focus_moved_page, incremental);
        let moved = harness.node("#fx-moved");
        let before = harness.base().get_client_bounding_rect(moved);
        click(&mut harness, "#fx-moved", &at);
        let after = harness.base().get_client_bounding_rect(moved);
        assert_ne!(
            before, after,
            "{at}: the fixture's focus rule moves the box"
        );
        assert_eq!(open(&harness), Some(moved), "{at}: the list stands");
        harness.pump();
        assert_eq!(open(&harness), Some(moved), "{at}: and goes on standing");
    }
}

/// A select whose chosen option is disabled.
fn chosen_disabled_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-pinned",
                option { id: "fx-pinned-a", value: "a", selected: true, disabled: true, "Alpha" }
                option { id: "fx-pinned-b", value: "b", "Beta" }
            }
        }
    }
}

#[test]
fn a_click_in_the_selects_box_on_a_disabled_chosen_option_closes_as_on_any_other() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(chosen_disabled_page, incremental);
        click(&mut harness, "#fx-pinned", &at);
        assert!(open(&harness).is_some(), "{at}: the fixture's list opens");
        let (x, y) = client_point(&harness, "#fx-pinned-a", &at);
        assert_eq!(
            harness.base().element_from_point(x, y),
            Some(harness.node("#fx-pinned-a")),
            "{at}: the fixture's point hits the chosen option in the select's box"
        );
        harness.click_at(x, y);
        assert_eq!(
            open(&harness),
            None,
            "{at}: the box takes it as a second click"
        );

        click(&mut harness, "#fx-pinned", &at);
        click_row(&mut harness, "#fx-pinned-a", &at);
        assert!(
            open(&harness).is_some(),
            "{at}: the same option's row, disabled, leaves the list open"
        );
    }
}

#[test]
fn a_click_on_the_chosen_text_in_the_selects_own_box_closes_and_chooses_nothing() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        click(&mut harness, "#fx-pick", &at);
        let banana = harness.node("#fx-banana");
        harness.press(named(NamedKey::Home));
        // The chosen option shows in the select's box as well as in its row, so a click there hits
        // an option of the open list and must not read as a choice of its row.
        let (x, y) = client_point(&harness, "#fx-banana", &at);
        assert_eq!(
            harness.base().element_from_point(x, y),
            Some(banana),
            "{at}: the fixture's point hits the chosen option in the select's box"
        );
        harness.click_at(x, y);
        assert_eq!(open(&harness), None, "{at}: the click closes the list");
        assert_eq!(value(&harness, "#fx-pick").as_deref(), Some("b"), "{at}");
        assert_eq!(counted(&harness), (0, 0), "{at}: no event fires");
    }
}
