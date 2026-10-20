//! A single-choice `select` holds one chosen option and an agent can move it by keyboard alone,
//! in every document: the select lays out as a box showing its chosen option's text; its choice
//! is seeded from the markup — the option written `selected`, else its first option that is not
//! disabled — and re-seeded when an app re-renders `selected` onto another option; focused, its
//! arrows choose the next or previous option that is not disabled, Home and End the first and
//! last, and typed text the first such option whose label starts with it, compared without case,
//! the text kept for 1 s of the document's clock after each character; a click focuses it; each
//! change fires one `input` then one `change` event and marks the changed set, and a key or text
//! that changes nothing fires none and marks nothing; a disabled select takes no key and no
//! click. In both layout modes.
//!
//! <https://html.spec.whatwg.org/multipage/form-elements.html#the-select-element>

use std::time::Duration;

use ::keyboard_types::{Key, Modifiers, NamedKey};
use blitz_test_harness::Harness;
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

/// A fixture booted with `fx-pick` focused by a click.
fn focused_pick(incremental: bool) -> Page {
    let mut harness = fixtures::boot(page, incremental);
    harness.click("#fx-pick");
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
    (Key::Named(NamedKey::Enter), Modifiers::empty(), "c", 0),
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
