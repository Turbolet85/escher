//! A single-choice `select` holds one chosen option and an agent can move it by keyboard alone,
//! in every document: the select lays out as a box showing its chosen option's text; its choice
//! is seeded from the markup — the option written `selected`, else its first option that is not
//! disabled — and re-seeded when an app re-renders `selected` onto another option; focused, its
//! arrows choose the next or previous option that is not disabled, Home and End the first and
//! last, and typed text the first such option whose label starts with it, compared without case,
//! the text kept for 1 s of the document's clock after each character; a click focuses it; each
//! change fires one `input` then one `change` event and marks the changed set, and a key or text
//! that changes nothing fires none and marks nothing; a disabled select takes no key and no
//! click; a click and Enter and Space open its list, a released Space doing nothing more, each
//! firing no event (`select_popup` holds the list's own reading). A `multiple` select lays out its
//! rows with boxes of their own: its arrows, Home and End choose a row alone, with Shift span from
//! the anchor and with Ctrl or Meta move the current row and change nothing, a Space flips the
//! current row, and typed text chooses its match alone; a plain click chooses a row alone, Ctrl or
//! Meta flips it and Shift spans to it, a disabled row never chosen; each change fires one `input`
//! then one `change`, `evt.values()` reading the set; `multiple` removed settles it to one option.
//! An `optgroup`'s options are in its select's list: a single select's keys and text reach them,
//! the chosen one shows alone with no box for its group, and a disabled group's rows are skipped.
//! In both layout modes.
//!
//! <https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element>

use std::time::Duration;

use ::keyboard_types::{Key, Modifiers, NamedKey};
use blitz_dom::{QualName, local_name, ns};
use blitz_test_harness::{Harness, mouse_pointer_event};
use blitz_traits::events::UiEvent;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;

mod fixtures;

/// The fixture: `fx-pick` with Banana written `selected` and a disabled Blackberry, in an app
/// that writes `selected` on the option its `input` event names, as a controlled select's app
/// does, counts the select's `input` and `change` events and re-seeds Cherry from a button; `fx-none`
/// with no option written `selected`; `fx-skip` whose first option is disabled; and `fx-off`, a
/// disabled select.
fn page() -> Element {
    let mut inputs = use_signal(|| 0u32);
    let mut changes = use_signal(|| 0u32);
    let mut seeded = use_signal(|| "b".to_string());
    rsx! {
        div { id: "fx-root",
            select {
                id: "fx-pick",
                oninput: move |event| {
                    *inputs.write() += 1;
                    seeded.set(event.value());
                },
                onchange: move |_| *changes.write() += 1,
                option { id: "fx-apple", value: "a", selected: seeded() == "a", "Apple" }
                option { id: "fx-avocado", value: "v", selected: seeded() == "v", "Avocado" }
                option { id: "fx-banana", value: "b", selected: seeded() == "b", "Banana" }
                option { id: "fx-blackberry", value: "k", disabled: true, "Blackberry" }
                option { id: "fx-blueberry", value: "u", selected: seeded() == "u", "Blueberry" }
                option { id: "fx-cherry", value: "c", selected: seeded() == "c", "Cherry" }
            }
            select { id: "fx-none",
                option { id: "fx-red", value: "r", "Red" }
                option { id: "fx-green", value: "g", "Green" }
            }
            select { id: "fx-skip",
                option { id: "fx-x", value: "x", disabled: true, "X" }
                option { id: "fx-y", value: "y", "Y" }
            }
            select { id: "fx-off", disabled: true,
                option { id: "fx-p", value: "p", "P" }
                option { id: "fx-q", value: "q", "Q" }
            }
            p { id: "fx-inputs", "{inputs}" }
            p { id: "fx-changes", "{changes}" }
            button { id: "fx-reseed", onclick: move |_| seeded.set("c".to_string()), "Reseed" }
        }
    }
}

type Page = Harness<DioxusDocument>;

/// The value the select `selector` reads: its chosen option's.
fn value(harness: &Page, selector: &str) -> Option<String> {
    let select = harness.node(selector);
    harness.base().select_value(select)
}

/// The `input` and `change` events the app has counted.
fn counted(harness: &Page) -> (u32, u32) {
    let read = |selector: &str| {
        harness
            .text_content(selector)
            .trim()
            .parse()
            .unwrap_or(u32::MAX)
    };
    (read("#fx-inputs"), read("#fx-changes"))
}

/// Runs `act` on a drained changed set and reads the events it fired and whether it marked
/// the set.
fn measured(harness: &mut Page, act: impl FnOnce(&mut Page)) -> ((u32, u32), bool) {
    harness.base_mut().take_changed_nodes();
    let before = counted(harness);
    act(harness);
    let after = counted(harness);
    (
        (after.0 - before.0, after.1 - before.1),
        harness.base().has_changes(),
    )
}

/// A fixture booted with `fx-pick` focused by a click, the list the click opens closed again.
fn focused_pick(incremental: bool) -> Page {
    let mut harness = fixtures::boot(page, incremental);
    harness.click("#fx-pick");
    harness.press(Key::Named(NamedKey::Escape));
    let pick = harness.node("#fx-pick");
    assert_eq!(
        harness.focused(),
        Some(pick),
        "incremental {incremental}: a click focuses the select"
    );
    harness
}

#[test]
fn a_select_lays_out_as_a_box_holding_its_chosen_option() {
    for incremental in [false, true] {
        let harness = fixtures::boot(page, incremental);
        let doc = harness.base();
        let rect = |selector: &str| {
            let node = harness.node(selector);
            doc.get_client_bounding_rect(node)
        };
        let pick = rect("#fx-pick").expect("the select reads a client rect");
        let banana = rect("#fx-banana").expect("the chosen option reads a client rect");
        assert!(
            pick.width > 0.0 && pick.height > 0.0,
            "incremental {incremental}: the select's box is not empty"
        );
        assert!(
            banana.width > 0.0 && banana.height > 0.0,
            "incremental {incremental}: the chosen option's text lays out"
        );
        assert!(
            pick.x <= banana.x
                && banana.x + banana.width <= pick.x + pick.width
                && pick.y <= banana.y
                && banana.y + banana.height <= pick.y + pick.height,
            "incremental {incremental}: the chosen option lies inside the select's box"
        );
        for option in ["#fx-apple", "#fx-avocado", "#fx-cherry"] {
            let shown = rect(option).is_some_and(|rect| rect.width > 0.0);
            assert!(
                !shown,
                "incremental {incremental}: an unchosen option lays out nothing"
            );
        }
    }
}

#[test]
fn the_choice_is_seeded_from_the_markup() {
    // (select, the value it reads at boot)
    const SEEDED: [(&str, &str); 4] = [
        ("#fx-pick", "b"),
        ("#fx-none", "r"),
        ("#fx-skip", "y"),
        ("#fx-off", "p"),
    ];
    for incremental in [false, true] {
        let harness = fixtures::boot(page, incremental);
        for (row, (select, expected)) in SEEDED.into_iter().enumerate() {
            assert_eq!(
                value(&harness, select).as_deref(),
                Some(expected),
                "incremental {incremental}: row {row}"
            );
        }
    }
}

/// Keys pressed in order on the focused `fx-pick`, from Banana: the value each leaves and the
/// `input` and `change` events it fires.
const KEYS: [(Key, Modifiers, &str, u32); 14] = [
    (Key::Named(NamedKey::ArrowDown), Modifiers::empty(), "u", 1),
    (Key::Named(NamedKey::ArrowDown), Modifiers::empty(), "c", 1),
    (Key::Named(NamedKey::ArrowDown), Modifiers::empty(), "c", 0),
    (Key::Named(NamedKey::ArrowUp), Modifiers::empty(), "u", 1),
    (Key::Named(NamedKey::ArrowUp), Modifiers::empty(), "b", 1),
    (Key::Named(NamedKey::Home), Modifiers::empty(), "a", 1),
    (Key::Named(NamedKey::ArrowUp), Modifiers::empty(), "a", 0),
    (Key::Named(NamedKey::Home), Modifiers::empty(), "a", 0),
    (Key::Named(NamedKey::End), Modifiers::empty(), "c", 1),
    (Key::Named(NamedKey::End), Modifiers::empty(), "c", 0),
    (Key::Named(NamedKey::PageDown), Modifiers::empty(), "c", 0),
    (Key::Named(NamedKey::Backspace), Modifiers::empty(), "c", 0),
    (Key::Named(NamedKey::ArrowLeft), Modifiers::empty(), "c", 0),
    (Key::Character(String::new()), Modifiers::CONTROL, "c", 0),
];

#[test]
fn the_arrows_home_and_end_choose_an_enabled_option() {
    assert_eq!(KEYS.len(), 14);
    for incremental in [false, true] {
        let mut harness = focused_pick(incremental);
        for (row, (key, modifiers, expected, events)) in KEYS.iter().cloned().enumerate() {
            let key = match key {
                Key::Character(_) => Key::Character("a".to_string()),
                key => key,
            };
            let (fired, marked) =
                measured(&mut harness, |harness| harness.press_with(key, modifiers));
            assert_eq!(
                value(&harness, "#fx-pick").as_deref(),
                Some(expected),
                "incremental {incremental}: row {row}"
            );
            assert_eq!(
                fired,
                (events, events),
                "incremental {incremental}: row {row}"
            );
            assert_eq!(marked, events > 0, "incremental {incremental}: row {row}");
        }
    }
}

/// Text typed into the focused `fx-pick`, from Banana, each row a fresh 1 s after the last:
/// the value it leaves and the `input` and `change` events it fires.
const TYPED: [(&str, &str, u32); 7] = [
    ("c", "c", 1),
    ("av", "v", 2),
    ("BAN", "b", 1),
    ("bl", "u", 1),
    ("x", "u", 0),
    ("B", "b", 1),
    ("b", "b", 0),
];

#[test]
fn typed_text_chooses_the_first_enabled_option_its_label_starts() {
    assert_eq!(TYPED.len(), 7);
    for incremental in [false, true] {
        let mut harness = focused_pick(incremental);
        for (row, (text, expected, events)) in TYPED.into_iter().enumerate() {
            harness.advance_time(Duration::from_secs(1));
            let (fired, marked) = measured(&mut harness, |harness| harness.type_text(text));
            assert_eq!(
                value(&harness, "#fx-pick").as_deref(),
                Some(expected),
                "incremental {incremental}: row {row}"
            );
            assert_eq!(
                fired,
                (events, events),
                "incremental {incremental}: row {row}"
            );
            assert_eq!(marked, events > 0, "incremental {incremental}: row {row}");
        }
    }
}

#[test]
fn typed_text_is_kept_for_one_second_of_the_documents_clock() {
    for incremental in [false, true] {
        let mut harness = focused_pick(incremental);
        harness.type_text("a");
        assert_eq!(value(&harness, "#fx-pick").as_deref(), Some("a"));
        harness.advance_time(Duration::from_millis(999));
        harness.type_text("c");
        assert_eq!(
            value(&harness, "#fx-pick").as_deref(),
            Some("a"),
            "incremental {incremental}: inside the second the text reads `ac`, which nothing starts"
        );
        harness.advance_time(Duration::from_secs(1));
        harness.type_text("c");
        assert_eq!(
            value(&harness, "#fx-pick").as_deref(),
            Some("c"),
            "incremental {incremental}: after the second the text starts afresh"
        );
        harness.type_text("x");
        harness.click("#fx-pick");
        harness.press(Key::Named(NamedKey::Escape));
        harness.type_text("a");
        assert_eq!(
            value(&harness, "#fx-pick").as_deref(),
            Some("a"),
            "incremental {incremental}: a click starts the text afresh"
        );
    }
}

#[test]
fn a_disabled_select_takes_no_key_and_no_click() {
    for incremental in [false, true] {
        let mut harness = fixtures::boot(page, incremental);
        let off = harness.node("#fx-off");
        harness.click("#fx-off");
        assert_ne!(
            harness.focused(),
            Some(off),
            "incremental {incremental}: a click does not focus a disabled select"
        );
        harness.base_mut().set_focus_to(off);
        assert_eq!(
            harness.focused(),
            Some(off),
            "incremental {incremental}: the fixture holds the disabled select focused"
        );
        for key in [
            Key::Named(NamedKey::ArrowDown),
            Key::Named(NamedKey::End),
            Key::Character("q".to_string()),
        ] {
            harness.press(key);
            assert_eq!(
                value(&harness, "#fx-off").as_deref(),
                Some("p"),
                "incremental {incremental}: a disabled select takes no key"
            );
        }
    }
}

#[test]
fn an_app_writing_selected_on_another_option_re_seeds_the_choice() {
    for incremental in [false, true] {
        let mut harness = focused_pick(incremental);
        harness.press(Key::Named(NamedKey::Home));
        assert_eq!(
            value(&harness, "#fx-pick").as_deref(),
            Some("a"),
            "incremental {incremental}: the fixture moved the choice by key"
        );
        let ((inputs, changes), _) = measured(&mut harness, |harness| harness.click("#fx-reseed"));
        assert_eq!(
            value(&harness, "#fx-pick").as_deref(),
            Some("c"),
            "incremental {incremental}: the re-rendered `selected` is the choice"
        );
        assert_eq!(
            (inputs, changes),
            (0, 0),
            "incremental {incremental}: a re-seed fires no event"
        );
        let doc = harness.base();
        let cherry = harness.node("#fx-cherry");
        assert!(
            doc.get_client_bounding_rect(cherry)
                .is_some_and(|rect| rect.width > 0.0),
            "incremental {incremental}: the re-seeded option is shown"
        );
    }
}

#[test]
fn enter_and_space_open_the_list_and_a_released_space_does_nothing_more() {
    for incremental in [false, true] {
        let mut harness = focused_pick(incremental);
        let pick = harness.node("#fx-pick");
        for (row, key) in [Key::Named(NamedKey::Enter), Key::Character(" ".to_string())]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                harness.base().open_select(),
                None,
                "incremental {incremental}: row {row}: the fixture's list is closed"
            );
            let (fired, _) = measured(&mut harness, |harness| harness.press(key));
            assert_eq!(
                fired,
                (0, 0),
                "incremental {incremental}: row {row} fires no event"
            );
            assert_eq!(
                harness.base().open_select(),
                Some(pick),
                "incremental {incremental}: row {row} leaves the list open"
            );
            let doc = harness.base();
            for option in ["#fx-apple", "#fx-avocado", "#fx-cherry"] {
                let node = harness.node(option);
                assert!(
                    !doc.get_client_bounding_rect(node)
                        .is_some_and(|rect| rect.width > 0.0),
                    "incremental {incremental}: row {row}: the list lays nothing out"
                );
            }
            drop(doc);
            harness.press(Key::Named(NamedKey::Escape));
        }
    }
}

/// A `multiple` select in an app that writes `selected` back from `evt.values()` and shows the
/// set, whose third row is disabled; and a disabled `multiple` select.
fn rows_page() -> Element {
    let mut inputs = use_signal(|| 0u32);
    let mut changes = use_signal(|| 0u32);
    let mut set = use_signal(Vec::<String>::new);
    let holds = move |value: &str| set.read().iter().any(|held| held == value);
    rsx! {
        div { id: "fx-root",
            select {
                id: "fx-rows",
                name: "fruit",
                multiple: true,
                oninput: move |event| {
                    *inputs.write() += 1;
                    set.set(
                        event
                            .values()
                            .into_iter()
                            .filter_map(|(_, value)| match value {
                                FormValue::Text(text) => Some(text),
                                FormValue::File(_) => None,
                            })
                            .collect(),
                    );
                },
                onchange: move |_| *changes.write() += 1,
                option { id: "fx-a", value: "a", selected: holds("a"), "Apple" }
                option { id: "fx-b", value: "b", selected: holds("b"), "Banana" }
                option { id: "fx-c", value: "c", disabled: true, "Cherry" }
                option { id: "fx-d", value: "d", selected: holds("d"), "Date" }
                option { id: "fx-e", value: "e", selected: holds("e"), "Elder Berry" }
            }
            select { id: "fx-rows-off", multiple: true, disabled: true,
                option { id: "fx-off-row", value: "x", "X" }
            }
            p { id: "fx-inputs", "{inputs}" }
            p { id: "fx-changes", "{changes}" }
            p { id: "fx-set", {set.read().join(",")} }
        }
    }
}

/// The set the select `selector` holds, in tree order.
fn values(harness: &Page, selector: &str) -> Vec<String> {
    harness.base().select_values(harness.node(selector))
}

/// A `rows_page` booted with `fx-rows` focused by Tab, nothing chosen.
fn focused_rows(incremental: bool) -> Page {
    let mut harness = fixtures::boot(rows_page, incremental);
    harness.press(Key::Named(NamedKey::Tab));
    let rows = harness.node("#fx-rows");
    assert_eq!(
        harness.focused(),
        Some(rows),
        "incremental {incremental}: Tab focuses the select"
    );
    assert!(
        values(&harness, "#fx-rows").is_empty(),
        "incremental {incremental}: the fixture chooses nothing"
    );
    harness
}

#[test]
fn a_multiple_select_lays_out_its_rows_with_boxes_of_their_own() {
    const ROWS: [&str; 5] = ["#fx-a", "#fx-b", "#fx-c", "#fx-d", "#fx-e"];
    assert_eq!(ROWS.len(), 5);
    for incremental in [false, true] {
        let harness = fixtures::boot(rows_page, incremental);
        let doc = harness.base();
        let rect = |selector: &str| {
            doc.get_client_bounding_rect(harness.node(selector))
                .expect("the element reads a client rect")
        };
        let list = rect("#fx-rows");
        let mut below = list.y;
        for (row, selector) in ROWS.into_iter().enumerate() {
            let read = rect(selector);
            assert!(
                read.width > 0.0 && read.height > 0.0,
                "incremental {incremental}: row {row} has a box of its own"
            );
            assert!(
                read.y >= below
                    && read.x >= list.x
                    && read.x + read.width <= list.x + list.width
                    && read.y + read.height <= list.y + list.height,
                "incremental {incremental}: row {row} lies below the last, inside the select"
            );
            below = read.y + read.height;
        }
    }
}

/// Keys pressed in order on `fx-rows` from no row: the set each leaves and the events it fires.
fn row_keys() -> [(Key, Modifiers, &'static [&'static str], u32); 17] {
    let (none, shift, ctrl, meta) = (
        Modifiers::empty(),
        Modifiers::SHIFT,
        Modifiers::CONTROL,
        Modifiers::META,
    );
    let (down, up) = (
        Key::Named(NamedKey::ArrowDown),
        Key::Named(NamedKey::ArrowUp),
    );
    let (home, end) = (Key::Named(NamedKey::Home), Key::Named(NamedKey::End));
    let space = Key::Character(" ".to_string());
    [
        (down.clone(), none, &["a"], 1),
        (down.clone(), none, &["b"], 1),
        (down.clone(), none, &["d"], 1),
        (up.clone(), shift, &["b", "d"], 1),
        (down.clone(), shift, &["d"], 1),
        (down.clone(), ctrl, &["d"], 0),
        (space.clone(), none, &["d", "e"], 1),
        (up.clone(), meta, &["d", "e"], 0),
        (space, none, &["e"], 1),
        (home.clone(), none, &["a"], 1),
        (home.clone(), none, &["a"], 0),
        (end, none, &["e"], 1),
        (home, shift, &["a", "b", "d", "e"], 1),
        (Key::Named(NamedKey::Enter), none, &["a", "b", "d", "e"], 0),
        (
            Key::Named(NamedKey::ArrowLeft),
            none,
            &["a", "b", "d", "e"],
            0,
        ),
        (
            Key::Character("a".to_string()),
            ctrl,
            &["a", "b", "d", "e"],
            0,
        ),
        (down, none, &["b"], 1),
    ]
}

#[test]
fn the_keys_of_a_multiple_select_choose_span_and_flip_its_rows() {
    let rows = row_keys();
    assert_eq!(rows.len(), 17);
    for incremental in [false, true] {
        let mut harness = focused_rows(incremental);
        for (row, (key, modifiers, expected, events)) in row_keys().into_iter().enumerate() {
            let (fired, marked) =
                measured(&mut harness, |harness| harness.press_with(key, modifiers));
            assert_eq!(
                values(&harness, "#fx-rows"),
                expected,
                "incremental {incremental}: row {row}"
            );
            assert_eq!(
                fired,
                (events, events),
                "incremental {incremental}: row {row}"
            );
            assert_eq!(marked, events > 0, "incremental {incremental}: row {row}");
            assert_eq!(
                harness.text_content("#fx-set"),
                expected.join(","),
                "incremental {incremental}: row {row}: `evt.values()` reads the set"
            );
            let doc = harness.base();
            assert_eq!(
                doc.select_value(harness.node("#fx-rows")).as_deref(),
                expected.first().copied(),
                "incremental {incremental}: row {row}: the value is the first chosen"
            );
        }
    }
}

#[test]
fn typed_text_chooses_its_row_alone_and_a_space_joins_it_while_it_stands() {
    for incremental in [false, true] {
        let mut harness = focused_rows(incremental);
        harness.press_with(Key::Named(NamedKey::Home), Modifiers::SHIFT);
        harness.press_with(Key::Named(NamedKey::End), Modifiers::SHIFT);
        assert_eq!(
            values(&harness, "#fx-rows"),
            ["a", "b", "d", "e"],
            "incremental {incremental}: the fixture spans the rows"
        );
        harness.type_text("elder b");
        assert_eq!(
            values(&harness, "#fx-rows"),
            ["e"],
            "incremental {incremental}: the text, its space within it, chooses its match alone"
        );
        harness.advance_time(Duration::from_secs(1));
        harness.press(Key::Character(" ".to_string()));
        assert!(
            values(&harness, "#fx-rows").is_empty(),
            "incremental {incremental}: with no text standing a Space flips the current row"
        );
        harness.advance_time(Duration::from_secs(1));
        harness.type_text("c");
        assert!(
            values(&harness, "#fx-rows").is_empty(),
            "incremental {incremental}: the disabled row is not chosen by its text"
        );
    }
}

/// Clicks the row `selector` with `mods` held, then lets a click on that point read as a new one.
fn click_row(harness: &mut Page, selector: &str, mods: Modifiers) {
    let (x, y) = harness.center_of(selector);
    let mut event = mouse_pointer_event(x, y);
    event.mods = mods;
    harness.dispatch(UiEvent::PointerDown(event.clone()));
    harness.dispatch(UiEvent::PointerUp(event));
    harness.pump();
    harness.advance_time(Duration::from_millis(500));
}

#[test]
fn a_click_on_a_row_chooses_flips_or_spans_it() {
    let rows: [(&str, Modifiers, &[&str], u32); 10] = [
        ("#fx-a", Modifiers::empty(), &["a"], 1),
        ("#fx-d", Modifiers::empty(), &["d"], 1),
        ("#fx-b", Modifiers::CONTROL, &["b", "d"], 1),
        ("#fx-e", Modifiers::META, &["b", "d", "e"], 1),
        ("#fx-e", Modifiers::CONTROL, &["b", "d"], 1),
        ("#fx-a", Modifiers::SHIFT, &["a", "b", "d", "e"], 1),
        ("#fx-c", Modifiers::empty(), &["a", "b", "d", "e"], 0),
        ("#fx-d", Modifiers::empty(), &["d"], 1),
        ("#fx-d", Modifiers::empty(), &["d"], 0),
        ("#fx-e", Modifiers::SHIFT, &["d", "e"], 1),
    ];
    assert_eq!(rows.len(), 10);
    for incremental in [false, true] {
        let mut harness = fixtures::boot(rows_page, incremental);
        let select = harness.node("#fx-rows");
        for (row, (selector, mods, expected, events)) in rows.into_iter().enumerate() {
            let (fired, _) = measured(&mut harness, |harness| {
                click_row(harness, selector, mods);
            });
            assert_eq!(
                values(&harness, "#fx-rows"),
                expected,
                "incremental {incremental}: row {row}"
            );
            assert_eq!(
                fired,
                (events, events),
                "incremental {incremental}: row {row}"
            );
            assert_eq!(
                harness.focused(),
                Some(select),
                "incremental {incremental}: row {row}: a click focuses the select"
            );
        }
    }
}

#[test]
fn a_disabled_multiple_select_takes_no_click_and_no_key() {
    for incremental in [false, true] {
        let mut harness = fixtures::boot(rows_page, incremental);
        let off = harness.node("#fx-rows-off");
        click_row(&mut harness, "#fx-off-row", Modifiers::empty());
        assert!(
            values(&harness, "#fx-rows-off").is_empty(),
            "incremental {incremental}: its row takes no click"
        );
        assert_ne!(
            harness.focused(),
            Some(off),
            "incremental {incremental}: a click does not focus it"
        );
        harness.base_mut().set_focus_to(off);
        harness.press(Key::Named(NamedKey::ArrowDown));
        assert!(
            values(&harness, "#fx-rows-off").is_empty(),
            "incremental {incremental}: it takes no key"
        );
    }
}

#[test]
fn multiple_removed_settles_the_set_to_one_option() {
    for incremental in [false, true] {
        let mut harness = focused_rows(incremental);
        harness.press_with(Key::Named(NamedKey::End), Modifiers::SHIFT);
        harness.press_with(Key::Named(NamedKey::Home), Modifiers::SHIFT);
        assert_eq!(
            values(&harness, "#fx-rows"),
            ["a", "b", "d", "e"],
            "incremental {incremental}: the fixture spans the rows"
        );
        let rows = harness.node("#fx-rows");
        harness
            .base_mut()
            .mutate()
            .clear_attribute(rows, QualName::new(None, ns!(), local_name!("multiple")));
        harness.pump();
        assert_eq!(
            values(&harness, "#fx-rows"),
            ["e"],
            "incremental {incremental}: the last chosen option stays"
        );
    }
}

#[test]
fn a_chosen_row_and_a_plain_one_meet_their_ratio_at_the_user_agents_pair() {
    for incremental in [false, true] {
        let mut harness = focused_rows(incremental);
        harness.press(Key::Named(NamedKey::Home));
        let read = |harness: &Page, selector: &str| {
            harness
                .text_contrast(harness.node(selector))
                .expect("the row's text is measured")
        };
        let chosen = read(&harness, "#fx-a");
        let plain = read(&harness, "#fx-b");
        assert!(
            chosen.ratio >= chosen.required_ratio() && plain.ratio >= plain.required_ratio(),
            "incremental {incremental}: both rows meet their ratios"
        );
        assert!(
            (chosen.ratio - 12.56).abs() < 0.01,
            "incremental {incremental}: the chosen row reads the user agent's pair"
        );
    }
}

/// A `multiple` select whose markup chooses its second row.
fn seeded_page() -> Element {
    rsx! {
        select { id: "fx-seeded", multiple: true,
            option { id: "fx-s-a", value: "a", "A" }
            option { id: "fx-s-b", value: "b", selected: true, "B" }
            option { id: "fx-s-c", value: "c", "C" }
        }
    }
}

#[test]
fn a_set_the_markup_seeds_is_stepped_and_spanned_from_its_last_chosen_row() {
    for incremental in [false, true] {
        let mut harness = fixtures::boot(seeded_page, incremental);
        assert_eq!(
            values(&harness, "#fx-seeded"),
            ["b"],
            "incremental {incremental}: the markup seeds the set"
        );
        let seeded = harness.node("#fx-seeded");
        harness.base_mut().set_focus_to(seeded);
        harness.press_with(Key::Named(NamedKey::ArrowDown), Modifiers::SHIFT);
        assert_eq!(
            values(&harness, "#fx-seeded"),
            ["b", "c"],
            "incremental {incremental}: the first span reaches from the chosen row"
        );
    }
}

/// A single select holding a plain option, a group, a disabled group and a last option; and a
/// `multiple` select holding a group.
fn groups_page() -> Element {
    let mut inputs = use_signal(|| 0u32);
    let mut changes = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            select {
                id: "fx-grouped",
                oninput: move |_| *inputs.write() += 1,
                onchange: move |_| *changes.write() += 1,
                option { id: "fx-plain", value: "p", "Plain" }
                optgroup { id: "fx-fruit", label: "Fruit",
                    option { id: "fx-apple", value: "a", "Apple" }
                    option { id: "fx-berry", value: "b", "Berry" }
                }
                optgroup { id: "fx-off-group", label: "Off", disabled: true,
                    option { id: "fx-gone", value: "g", "Gone" }
                }
                option { id: "fx-last", value: "l", "Last" }
            }
            select { id: "fx-rows-grouped", multiple: true,
                optgroup { id: "fx-row-group", label: "Rows",
                    option { id: "fx-row-a", value: "a", "A" }
                    option { id: "fx-row-b", value: "b", "B" }
                }
                optgroup { id: "fx-row-off", label: "Off", disabled: true,
                    option { id: "fx-row-c", value: "c", "C" }
                }
            }
            p { id: "fx-inputs", "{inputs}" }
            p { id: "fx-changes", "{changes}" }
        }
    }
}

#[test]
fn a_groups_options_are_chosen_by_a_single_selects_keys_and_shown_alone() {
    let keys = [
        (Key::Named(NamedKey::ArrowDown), "a", 1),
        (Key::Named(NamedKey::ArrowDown), "b", 1),
        (Key::Named(NamedKey::ArrowDown), "l", 1),
        (Key::Named(NamedKey::ArrowDown), "l", 0),
        (Key::Named(NamedKey::ArrowUp), "b", 1),
        (Key::Named(NamedKey::Home), "p", 1),
        (Key::Named(NamedKey::End), "l", 1),
    ];
    assert_eq!(keys.len(), 7);
    for incremental in [false, true] {
        let mut harness = fixtures::boot(groups_page, incremental);
        harness.click("#fx-grouped");
        harness.press(Key::Named(NamedKey::Escape));
        assert_eq!(
            value(&harness, "#fx-grouped").as_deref(),
            Some("p"),
            "incremental {incremental}: the first enabled option is seeded"
        );
        for (row, (key, expected, events)) in keys.iter().cloned().enumerate() {
            let (fired, _) = measured(&mut harness, |harness| harness.press(key));
            assert_eq!(
                value(&harness, "#fx-grouped").as_deref(),
                Some(expected),
                "incremental {incremental}: row {row}"
            );
            assert_eq!(
                fired,
                (events, events),
                "incremental {incremental}: row {row}"
            );
        }

        harness.advance_time(Duration::from_secs(1));
        harness.type_text("be");
        assert_eq!(
            value(&harness, "#fx-grouped").as_deref(),
            Some("b"),
            "incremental {incremental}: typed text reaches a group's option"
        );
        harness.advance_time(Duration::from_secs(1));
        harness.type_text("g");
        assert_eq!(
            value(&harness, "#fx-grouped").as_deref(),
            Some("b"),
            "incremental {incremental}: a disabled group's option is not chosen by its text"
        );

        let doc = harness.base();
        let shown = |selector: &str| {
            doc.get_client_bounding_rect(harness.node(selector))
                .is_some_and(|rect| rect.width > 0.0)
        };
        assert!(
            shown("#fx-berry"),
            "incremental {incremental}: the chosen option shows"
        );
        for (row, selector) in [
            "#fx-fruit",
            "#fx-apple",
            "#fx-plain",
            "#fx-gone",
            "#fx-last",
        ]
        .into_iter()
        .enumerate()
        {
            assert!(
                !shown(selector),
                "incremental {incremental}: row {row} lays out nothing beside it"
            );
        }
    }
}

#[test]
fn a_multiple_selects_groups_lay_out_and_a_disabled_groups_rows_are_not_chosen() {
    for incremental in [false, true] {
        let mut harness = fixtures::boot(groups_page, incremental);
        click_row(&mut harness, "#fx-row-b", Modifiers::empty());
        assert_eq!(
            values(&harness, "#fx-rows-grouped"),
            ["b"],
            "incremental {incremental}: a group's row takes a click"
        );
        click_row(&mut harness, "#fx-row-c", Modifiers::empty());
        assert_eq!(
            values(&harness, "#fx-rows-grouped"),
            ["b"],
            "incremental {incremental}: a disabled group's row takes none"
        );
        harness.press(Key::Named(NamedKey::End));
        assert_eq!(
            values(&harness, "#fx-rows-grouped"),
            ["b"],
            "incremental {incremental}: End stops at the last row that is enabled"
        );
        harness.press_with(Key::Named(NamedKey::ArrowUp), Modifiers::SHIFT);
        assert_eq!(
            values(&harness, "#fx-rows-grouped"),
            ["a", "b"],
            "incremental {incremental}: a span reaches across a group's rows"
        );
        let doc = harness.base();
        let group = doc
            .get_client_bounding_rect(harness.node("#fx-row-group"))
            .expect("the group reads a client rect");
        assert!(
            group.width > 0.0 && group.height > 0.0,
            "incremental {incremental}: a `multiple` select's group has a box"
        );
    }
}
