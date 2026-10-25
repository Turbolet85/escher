//! The stand is operable and readable by keyboard alone, proven headlessly on the four lean
//! tasks in both layout modes, with minimal fixtures beside them for the cases they lack.
//!
//! *Tab walk:* from a fresh boot, Tab walks each task's stated sequence and wraps to its first
//! control, and Shift+Tab walks it back and wraps to its last; at every step the engine's focused
//! node, the snapshot's one `focused` node and the accessibility tree's focus name the same
//! control. The flight booker with a return flight chosen by ArrowDown on its type select and
//! CRUD with a row selected by ArrowDown walk their longer stated sequences the same way, so
//! every stand control is reached by Tab in some state, and Enter and Space on the CRUD buttons
//! enabled then read as their click. *Activation:* Enter and Space on every focused stand button
//! leave the screen as a click on it does, a stated table of 16 rows; Space types into a text
//! input, Enter changes nothing there, and neither key moves the range input. *Choice:* the
//! flight type's select, focused by Tab, takes its arrows — ArrowDown chooses the return flight,
//! ArrowUp the one-way flight, each enabling or disabling the return date — while focus stays
//! on it, and Enter and Space change nothing there. On a fixture: Space toggles a checkbox
//! and a radio and Enter does not, Enter follows a link and Space does not, both toggle a
//! `details` through its `summary` and press an `input type="button"`, a button whose `disabled`
//! attribute appeared after it took focus takes neither, and a button that is itself a scrolled
//! box takes Enter as its own click. *Focus:* a click on a stand button focuses it and the next
//! Tab continues from it; a `disabled` attribute of any value — bare, `"false"`, `"true"` —
//! takes a button out of the walk, while a `tabindex` div carrying one stays in it, as HTML
//! disables form controls alone; the CRUD list takes focus and its arrows, Home and End move the
//! selection as a click on a row does, while the focus stays on the list. *Delete:* the Apple
//! delete binding deletes one character on every OS. *Inert:* Tab skips content under a truthy
//! `inert` and reaches content under a falsy one, whose attribute the bridge removed. Known
//! limit: a node that holds focus when it becomes inert keeps it until focus moves, as HTML's
//! focus fixup is not built — pinned by `inert_subtree`.
//!
//! A failure message carries a layout mode, a task and an index, never an id or what a screen
//! reads.

use ::keyboard_types::{Key, Modifiers, NamedKey};
use blitz_dom::{QualName, ScrollBehavior, local_name, ns};
use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand::LeanTask;

mod common;
mod fixtures;
use common::{ENABLED_TAB_ORDER, boot, controls, editor_text, focused, node, tab_order};

/// Each stand button in the four Tab sequences, and whether a click on it changes the screen
/// beyond its focus.
const BUTTONS: [(LeanTask, &str, bool); 8] = [
    (LeanTask::Counter, "back-btn", false),
    (LeanTask::Counter, "counter-increment", true),
    (LeanTask::FlightBooker, "back-btn", false),
    (LeanTask::FlightBooker, "flight-book", true),
    (LeanTask::Timer, "back-btn", false),
    (LeanTask::Timer, "timer-reset", false),
    (LeanTask::Crud, "back-btn", false),
    (LeanTask::Crud, "crud-create", true),
];

/// The stand's text inputs in the Tab sequences.
const TEXT_INPUTS: [(LeanTask, &str); 4] = [
    (LeanTask::FlightBooker, "flight-start"),
    (LeanTask::Crud, "crud-filter"),
    (LeanTask::Crud, "crud-name"),
    (LeanTask::Crud, "crud-surname"),
];

fn space() -> Key {
    Key::Character(" ".to_string())
}

/// The two activation keys, each with the name a failure message gives it.
fn activation_keys() -> [(&'static str, Key); 2] {
    [("Enter", Key::Named(NamedKey::Enter)), ("Space", space())]
}

/// The ids of the controls holding focus, read three ways that must agree: the stable id of the
/// engine's focused node, the snapshot's `focused` nodes and the accessibility tree's focus.
/// The engine reads the root element as focused when nothing is.
#[track_caller]
fn holder(harness: &Harness<DioxusDocument>) -> Vec<String> {
    let read = focused(harness);
    let root = harness.base().try_root_element().map(|root| root.id);
    let by_engine: Vec<String> = harness
        .focused()
        .filter(|node| Some(*node) != root)
        .and_then(|node| harness.doc.element_id(node))
        .into_iter()
        .collect();
    assert!(
        read == by_engine,
        "the engine's focused node is the snapshot's focused node"
    );
    read
}

/// Tabs from a fresh boot to `id`, asserted to hold focus then.
#[track_caller]
fn tab_to(harness: &mut Harness<DioxusDocument>, task: LeanTask, id: &str, mode: &str) {
    let position = tab_order(task)
        .iter()
        .position(|stop| *stop == id)
        .unwrap_or_else(|| panic!("{mode}: {task:?}: the control is in the stated sequence"));
    for _ in 0..=position {
        harness.press(Key::Named(NamedKey::Tab));
    }
    assert!(
        holder(harness) == [id],
        "{mode}: {task:?}: Tab {position} reaches the control"
    );
}

/// Tabs from where focus is until `id` holds it, at most `bound` presses.
#[track_caller]
fn tab_until(harness: &mut Harness<DioxusDocument>, id: &str, bound: usize, what: &str) {
    for _ in 0..bound {
        harness.press(Key::Named(NamedKey::Tab));
        if holder(harness) == [id] {
            return;
        }
    }
    panic!("{what}: Tab reaches the control");
}

fn text(harness: &Harness<DioxusDocument>) -> String {
    harness.doc.snapshot().to_text()
}

#[test]
fn tab_and_shift_tab_walk_the_stated_sequence_and_wrap() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        for task in LeanTask::ALL {
            let sequence = tab_order(task);
            let n = sequence.len();
            let mut harness = boot(task, incremental);
            assert!(
                holder(&harness).is_empty(),
                "{mode}: {task:?}: nothing is focused at boot"
            );
            for index in 0..=n {
                harness.press(Key::Named(NamedKey::Tab));
                assert!(
                    holder(&harness) == [sequence[index % n]],
                    "{mode}: {task:?}: Tab {index}"
                );
            }
            for index in 1..=n + 1 {
                harness.press_with(Key::Named(NamedKey::Tab), Modifiers::SHIFT);
                assert!(
                    holder(&harness) == [sequence[(n - index % n) % n]],
                    "{mode}: {task:?}: Shift+Tab {index}"
                );
            }
        }
    }
}

#[test]
fn enter_and_space_on_a_focused_button_read_as_its_click() {
    let derived: Vec<(LeanTask, &str)> = LeanTask::ALL
        .into_iter()
        .flat_map(|task| {
            tab_order(task)
                .iter()
                .filter(move |stop| {
                    controls(task)
                        .iter()
                        .any(|(id, role)| id == *stop && *role == accesskit::Role::Button)
                })
                .map(move |stop| (task, *stop))
        })
        .collect();
    let stated: Vec<(LeanTask, &str)> = BUTTONS.iter().map(|(task, id, _)| (*task, *id)).collect();
    assert!(
        derived == stated,
        "the table states every button of the Tab sequences"
    );

    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut rows = 0;
        let mut changing = 0;
        for (index, (task, id, changes)) in BUTTONS.into_iter().enumerate() {
            for (key_name, key) in activation_keys() {
                rows += 1;
                let mut by_key = boot(task, incremental);
                tab_to(&mut by_key, task, id, &mode);
                by_key.press(key);
                let keyed = text(&by_key);

                let mut by_click = boot(task, incremental);
                tab_to(&mut by_click, task, id, &mode);
                let focus_only = text(&by_click);
                by_click.click(&format!("#{id}"));
                let clicked = text(&by_click);

                assert!(
                    keyed == clicked,
                    "{mode}: {task:?}: row {index}, {key_name}: the key reads as the click"
                );
                assert!(
                    (clicked != focus_only) == changes,
                    "{mode}: {task:?}: row {index}: whether the click changes the screen \
                     beyond focus reads as stated"
                );
                if changes {
                    changing += 1;
                }
            }
        }
        assert_eq!(rows, 16, "{mode}: the stated rows");
        assert!(changing >= 1, "{mode}: a row changes the screen");
    }
}

/// Brings `task` from a fresh boot into the state `ENABLED_TAB_ORDER` states for it, by keyboard
/// alone — ArrowDown on `flight-type`, ArrowDown on the CRUD list — asserted to have enabled the
/// controls disabled at boot. Focus is left where the last key was pressed.
#[track_caller]
fn enable_by_keyboard(harness: &mut Harness<DioxusDocument>, task: LeanTask, mode: &str) {
    let (stop, key, enabled): (&str, Key, &[&str]) = match task {
        LeanTask::FlightBooker => (
            "flight-type",
            Key::Named(NamedKey::ArrowDown),
            &["flight-return-date"],
        ),
        LeanTask::Crud => (
            "crud-list",
            Key::Named(NamedKey::ArrowDown),
            &["crud-update", "crud-delete"],
        ),
        _ => panic!("{mode}: {task:?}: no control of the task is disabled at boot"),
    };
    let snapshot = harness.doc.snapshot();
    for (index, id) in enabled.iter().enumerate() {
        assert!(
            node(&snapshot, id).state.enabled == Some(false),
            "{mode}: {task:?}: control {index} is disabled at boot"
        );
    }
    tab_to(harness, task, stop, mode);
    harness.press(key);
    let snapshot = harness.doc.snapshot();
    for (index, id) in enabled.iter().enumerate() {
        assert!(
            node(&snapshot, id).state.enabled == Some(true),
            "{mode}: {task:?}: control {index} is enabled by the key"
        );
    }
}

#[test]
fn the_flight_type_select_moves_its_choice_by_its_arrows() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot(LeanTask::FlightBooker, incremental);
        tab_to(&mut harness, LeanTask::FlightBooker, "flight-type", &mode);
        // Each key, then the select's value, the return date's enabled reading and the
        // options' selected readings it leaves.
        let rows = [
            (
                Key::Named(NamedKey::ArrowDown),
                "return",
                true,
                [false, true],
            ),
            (
                Key::Named(NamedKey::ArrowDown),
                "return",
                true,
                [false, true],
            ),
            (Key::Named(NamedKey::Enter), "return", true, [false, true]),
            (Key::Character(" ".into()), "return", true, [false, true]),
            (
                Key::Named(NamedKey::ArrowUp),
                "one-way",
                false,
                [true, false],
            ),
            (
                Key::Named(NamedKey::ArrowUp),
                "one-way",
                false,
                [true, false],
            ),
        ];
        assert_eq!(rows.len(), 6);
        for (index, (key, value, enabled, options)) in rows.into_iter().enumerate() {
            harness.press(key);
            let snapshot = harness.doc.snapshot();
            assert!(
                node(&snapshot, "flight-type").state.value.as_deref() == Some(value),
                "{mode}: key {index}: the select reads the stated choice"
            );
            assert!(
                node(&snapshot, "flight-return-date").state.enabled == Some(enabled),
                "{mode}: key {index}: the return date follows the choice"
            );
            assert!(
                ["flight-type-one-way", "flight-type-return"]
                    .map(|id| node(&snapshot, id).state.selected)
                    == options.map(Some),
                "{mode}: key {index}: exactly the chosen option reads selected"
            );
            assert!(
                holder(&harness) == ["flight-type"],
                "{mode}: key {index}: focus stays on the select"
            );
        }
    }
}

#[test]
fn the_enabled_states_extend_the_walk_in_order() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        for (task, sequence) in ENABLED_TAB_ORDER {
            let n = sequence.len();
            let mut harness = boot(task, incremental);
            enable_by_keyboard(&mut harness, task, &mode);
            let holding = holder(&harness);
            let mut at = sequence
                .iter()
                .position(|stop| holding == [*stop])
                .unwrap_or_else(|| panic!("{mode}: {task:?}: focus is on a stated stop"));
            for index in 1..=n + 1 {
                harness.press(Key::Named(NamedKey::Tab));
                at = (at + 1) % n;
                assert!(
                    holder(&harness) == [sequence[at]],
                    "{mode}: {task:?}: Tab {index}"
                );
            }
            for index in 1..=n + 1 {
                harness.press_with(Key::Named(NamedKey::Tab), Modifiers::SHIFT);
                at = (at + n - 1) % n;
                assert!(
                    holder(&harness) == [sequence[at]],
                    "{mode}: {task:?}: Shift+Tab {index}"
                );
            }
        }
    }
}

/// The CRUD buttons enabled once a row is selected. Each is read after a character was typed
/// into the name field, so that each one's click changes the screen.
const ENABLED_BUTTONS: [(LeanTask, &str); 2] = [
    (LeanTask::Crud, "crud-update"),
    (LeanTask::Crud, "crud-delete"),
];

/// Selects the first CRUD row and types one character into the name field, by keyboard alone,
/// then Tabs to `id`.
#[track_caller]
fn crud_ready_at(harness: &mut Harness<DioxusDocument>, id: &str, mode: &str, index: usize) {
    enable_by_keyboard(harness, LeanTask::Crud, mode);
    harness.press(Key::Named(NamedKey::Tab));
    assert!(
        holder(harness) == ["crud-name"],
        "{mode}: Crud: row {index}: Tab reaches the name field"
    );
    harness.type_text("x");
    tab_until(harness, id, 8, &format!("{mode}: Crud: row {index}"));
}

#[test]
fn enter_and_space_on_the_controls_enabled_later_read_as_their_click() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut rows = 0;
        for (index, (task, id)) in ENABLED_BUTTONS.into_iter().enumerate() {
            for (key_name, key) in activation_keys() {
                rows += 1;
                let mut by_key = boot(task, incremental);
                crud_ready_at(&mut by_key, id, &mode, index);
                by_key.press(key);
                let keyed = text(&by_key);

                let mut by_click = boot(task, incremental);
                crud_ready_at(&mut by_click, id, &mode, index);
                let focus_only = text(&by_click);
                by_click.click(&format!("#{id}"));
                let clicked = text(&by_click);

                assert!(
                    clicked != focus_only,
                    "{mode}: {task:?}: row {index}: the click changes the screen beyond focus"
                );
                assert!(
                    keyed == clicked,
                    "{mode}: {task:?}: row {index}, {key_name}: the key reads as the click"
                );
            }
        }
        assert_eq!(rows, 4, "{mode}: the stated rows");
    }
}

#[test]
fn every_control_is_reached_by_tab() {
    for task in LeanTask::ALL {
        let enabled: &[&str] = ENABLED_TAB_ORDER
            .iter()
            .find(|(stated, _)| *stated == task)
            .map_or(&[], |(_, sequence)| sequence);
        let unreached: Vec<usize> = controls(task)
            .iter()
            .enumerate()
            .filter(|(_, (id, _))| !tab_order(task).contains(id) && !enabled.contains(id))
            .map(|(index, _)| index)
            .collect();
        assert!(
            unreached.is_empty(),
            "{task:?}: controls {unreached:?} lie in no stated Tab sequence"
        );
    }
}

#[test]
fn keys_that_do_not_activate_type_or_change_nothing() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut rows = 0;
        for (index, (task, id)) in TEXT_INPUTS.into_iter().enumerate() {
            rows += 1;
            let mut spaced = boot(task, incremental);
            tab_to(&mut spaced, task, id, &mode);
            let before = editor_text(&spaced, id);
            spaced.press(space());
            let after = editor_text(&spaced, id);
            let spaces = |s: &str| s.chars().filter(|c| *c == ' ').count();
            assert!(
                after.chars().count() == before.chars().count() + 1
                    && spaces(&after) == spaces(&before) + 1,
                "{mode}: {task:?}: row {index}: Space types one space"
            );

            rows += 1;
            let mut entered = boot(task, incremental);
            tab_to(&mut entered, task, id, &mode);
            let (screen, held) = (text(&entered), editor_text(&entered, id));
            entered.press(Key::Named(NamedKey::Enter));
            assert!(
                text(&entered) == screen && editor_text(&entered, id) == held,
                "{mode}: {task:?}: row {index}: Enter changes nothing"
            );
        }
        for (key_name, key) in activation_keys() {
            rows += 1;
            let mut timer = boot(LeanTask::Timer, incremental);
            tab_to(&mut timer, LeanTask::Timer, "timer-duration", &mode);
            let screen = text(&timer);
            timer.press(key);
            assert!(
                text(&timer) == screen,
                "{mode}: Timer: {key_name} on the range input changes nothing"
            );
        }
        assert_eq!(rows, 10, "{mode}: the stated rows");
    }
}

/// The controls the lean tasks lack, each with a count of the clicks it took where it has one.
fn activation_fixture() -> Element {
    let mut followed = use_signal(|| 0u32);
    let mut pressed = use_signal(|| 0u32);
    let mut later = use_signal(|| 0u32);
    let mut scrolled = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            input { id: "fx-check", r#type: "checkbox", "aria-label": "Check" }
            input { id: "fx-radio", r#type: "radio", "aria-label": "Radio" }
            a { id: "fx-link", href: "#fx-root", onclick: move |_| followed += 1, "Link" }
            details { id: "fx-details",
                summary { id: "fx-summary", "More" }
                p { "Body" }
            }
            input {
                id: "fx-input-button",
                r#type: "button",
                value: "Go",
                onclick: move |_| pressed += 1,
            }
            button { id: "fx-later", onclick: move |_| later += 1, "Later" }
            button {
                id: "fx-scrolled",
                style: "display: block; width: 120px; height: 40px; overflow: scroll;",
                onclick: move |_| scrolled += 1,
                div { style: "width: 400px; height: 400px;", "Inside" }
            }
            p { id: "fx-followed", "{followed}" }
            p { id: "fx-pressed", "{pressed}" }
            p { id: "fx-later-count", "{later}" }
            p { id: "fx-scrolled-count", "{scrolled}" }
        }
    }
}

#[test]
fn the_fixture_controls_take_the_keys_html_gives_them() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let checked = |harness: &Harness<DioxusDocument>, id: &str| {
            node(&harness.doc.snapshot(), id).state.checked
        };

        for (what, id) in [("checkbox", "fx-check"), ("radio", "fx-radio")] {
            let mut harness = fixtures::boot(activation_fixture, incremental);
            tab_until(&mut harness, id, 8, &format!("{mode}: the {what}"));
            assert!(
                checked(&harness, id) == Some(false),
                "{mode}: the {what} is unchecked first"
            );
            harness.press(Key::Named(NamedKey::Enter));
            assert!(
                checked(&harness, id) == Some(false),
                "{mode}: Enter leaves the {what} unchecked"
            );
            harness.press(space());
            assert!(
                checked(&harness, id) == Some(true),
                "{mode}: Space checks the {what}"
            );
        }

        let mut harness = fixtures::boot(activation_fixture, incremental);
        tab_until(&mut harness, "fx-link", 8, &format!("{mode}: the link"));
        assert!(
            harness.text_content("#fx-followed") == "0",
            "{mode}: the link is not followed first"
        );
        harness.press(space());
        assert!(
            harness.text_content("#fx-followed") == "0",
            "{mode}: Space does not follow the link"
        );
        harness.press(Key::Named(NamedKey::Enter));
        assert!(
            harness.text_content("#fx-followed") == "1",
            "{mode}: Enter follows the link"
        );

        let mut harness = fixtures::boot(activation_fixture, incremental);
        tab_until(
            &mut harness,
            "fx-summary",
            8,
            &format!("{mode}: the summary"),
        );
        let open = |harness: &Harness<DioxusDocument>| harness.attr("#fx-details", "open");
        assert!(
            open(&harness).is_none(),
            "{mode}: the details is closed first"
        );
        harness.press(Key::Named(NamedKey::Enter));
        assert!(open(&harness).is_some(), "{mode}: Enter opens the details");
        harness.press(space());
        assert!(open(&harness).is_none(), "{mode}: Space closes the details");

        let mut harness = fixtures::boot(activation_fixture, incremental);
        tab_until(
            &mut harness,
            "fx-input-button",
            8,
            &format!("{mode}: the input button"),
        );
        harness.press(Key::Named(NamedKey::Enter));
        assert!(
            harness.text_content("#fx-pressed") == "1",
            "{mode}: Enter presses the input button"
        );
        harness.press(space());
        assert!(
            harness.text_content("#fx-pressed") == "2",
            "{mode}: Space presses the input button"
        );
    }
}

#[test]
fn a_button_disabled_after_it_took_focus_takes_no_key() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(activation_fixture, incremental);
        tab_until(&mut harness, "fx-later", 8, &format!("{mode}: the button"));
        let button = harness.node("#fx-later");
        harness.base_mut().mutate().set_attribute(
            button,
            QualName::new(None, ns!(), local_name!("disabled")),
            "",
        );
        harness.pump();
        assert!(
            harness.attr("#fx-later", "disabled").is_some() && harness.focused() == Some(button),
            "{mode}: the button is disabled and still holds focus"
        );
        for (key_name, key) in activation_keys() {
            harness.press(key);
            assert!(
                harness.text_content("#fx-later-count") == "0",
                "{mode}: {key_name} dispatches no click"
            );
        }
    }
}

#[test]
fn a_button_that_is_itself_scrolled_takes_enter_as_its_own_click() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(activation_fixture, incremental);
        tab_until(
            &mut harness,
            "fx-scrolled",
            8,
            &format!("{mode}: the button"),
        );
        let button = harness.node("#fx-scrolled");
        harness
            .base_mut()
            .scroll_to(button, 0.0, 100.0, ScrollBehavior::Instant);
        harness.pump();
        let offset = harness
            .base()
            .get_node(button)
            .map(|node| node.scroll_offset().y);
        assert!(
            offset.is_some_and(|y| y != 0.0),
            "{mode}: the button is scrolled"
        );
        harness.press(Key::Named(NamedKey::Enter));
        assert!(
            harness.text_content("#fx-scrolled-count") == "1",
            "{mode}: Enter fires the scrolled button's own click"
        );
    }
}

#[test]
fn a_click_focuses_a_stand_button_and_tab_continues_from_it() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        for (index, (task, id, _)) in BUTTONS.into_iter().enumerate() {
            let sequence = tab_order(task);
            let position = sequence
                .iter()
                .position(|stop| *stop == id)
                .unwrap_or_else(|| panic!("{mode}: {task:?}: row {index} is in the sequence"));
            let mut harness = boot(task, incremental);
            assert!(
                holder(&harness).is_empty(),
                "{mode}: {task:?}: row {index}: nothing is focused at boot"
            );
            harness.click(&format!("#{id}"));
            assert!(
                holder(&harness) == [id],
                "{mode}: {task:?}: row {index}: the click focuses the button"
            );
            harness.press(Key::Named(NamedKey::Tab));
            assert!(
                holder(&harness) == [sequence[(position + 1) % sequence.len()]],
                "{mode}: {task:?}: row {index}: Tab continues from the button"
            );
        }
    }
}

/// Parsed markup, so each `disabled` attribute keeps the value it was written with.
fn disabled_markup() -> Element {
    rsx! {
        div {
            id: "fx-parsed",
            dangerous_inner_html: r#"<button id="fx-a">A</button><button id="fx-b" disabled="">B</button><button id="fx-c" disabled="false">C</button><div id="fx-div" disabled tabindex="0">Div</div><button id="fx-d" disabled="true">D</button><button id="fx-e">E</button>"#,
        }
    }
}

#[test]
fn a_disabled_attribute_skips_a_form_control_and_not_a_div() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(disabled_markup, incremental);
        let stated = [
            ("#fx-b", "disabled", ""),
            ("#fx-c", "disabled", "false"),
            ("#fx-div", "disabled", ""),
            ("#fx-div", "tabindex", "0"),
            ("#fx-d", "disabled", "true"),
        ];
        for (index, (selector, attr, value)) in stated.into_iter().enumerate() {
            assert!(
                harness.attr(selector, attr).as_deref() == Some(value),
                "{mode}: attribute {index} is written as stated"
            );
        }
        harness.press(Key::Named(NamedKey::Tab));
        assert!(holder(&harness) == ["fx-a"], "{mode}: the first Tab");
        harness.press(Key::Named(NamedKey::Tab));
        assert!(
            holder(&harness) == ["fx-div"],
            "{mode}: Tab skips the disabled buttons and reaches the div"
        );
        harness.press(Key::Named(NamedKey::Tab));
        assert!(
            holder(&harness) == ["fx-e"],
            "{mode}: Tab skips the last disabled button"
        );
        harness.press_with(Key::Named(NamedKey::Tab), Modifiers::SHIFT);
        assert!(
            holder(&harness) == ["fx-div"],
            "{mode}: Shift+Tab skips the disabled button back to the div"
        );
        harness.press_with(Key::Named(NamedKey::Tab), Modifiers::SHIFT);
        assert!(
            holder(&harness) == ["fx-a"],
            "{mode}: Shift+Tab skips the disabled buttons back"
        );
    }
}

#[test]
fn the_crud_list_takes_focus_and_its_keys_move_the_selection() {
    const ROWS: [&str; 3] = ["crud-person-0", "crud-person-1", "crud-person-2"];
    let fields = |harness: &Harness<DioxusDocument>| {
        let snapshot = harness.doc.snapshot();
        ["crud-name", "crud-surname"].map(|id| node(&snapshot, id).state.value.clone())
    };
    let selected = |harness: &Harness<DioxusDocument>| {
        let snapshot = harness.doc.snapshot();
        ROWS.map(|id| node(&snapshot, id).state.selected == Some(true))
    };
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let mut by_click = boot(LeanTask::Crud, incremental);
        let clicked: Vec<_> = ROWS
            .iter()
            .map(|id| {
                by_click.click(&format!("#{id}"));
                fields(&by_click)
            })
            .collect();

        let mut crud = boot(LeanTask::Crud, incremental);
        for _ in 0..3 {
            crud.press(Key::Named(NamedKey::Tab));
        }
        assert!(
            holder(&crud) == ["crud-list"],
            "{mode}: the third Tab focuses the list"
        );
        assert!(
            selected(&crud) == [false; 3],
            "{mode}: no row is selected first"
        );
        let steps = [
            (Key::Named(NamedKey::ArrowDown), 0),
            (Key::Named(NamedKey::ArrowDown), 1),
            (Key::Named(NamedKey::ArrowUp), 0),
            (Key::Named(NamedKey::End), 2),
            (Key::Named(NamedKey::Home), 0),
        ];
        for (index, (key, row)) in steps.into_iter().enumerate() {
            crud.press(key);
            let mut expected = [false; 3];
            expected[row] = true;
            assert!(
                selected(&crud) == expected,
                "{mode}: key {index} selects the stated row alone"
            );
            assert!(
                fields(&crud) == clicked[row],
                "{mode}: key {index} fills the fields as a click on the row does"
            );
            assert!(
                holder(&crud) == ["crud-list"],
                "{mode}: key {index} leaves the focus on the list"
            );
        }
    }
}

#[test]
fn the_apple_delete_binding_deletes_on_every_os() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut crud = boot(LeanTask::Crud, incremental);
        crud.click("#crud-name");
        crud.type_text("Ada");
        assert!(
            editor_text(&crud, "crud-name") == "Ada",
            "{mode}: the input holds three characters"
        );
        crud.apple_keybinding("deleteBackward:");
        assert!(
            editor_text(&crud, "crud-name") == "Ad",
            "{mode}: the binding deletes one"
        );
    }
}

/// A button under a truthy `inert` and one under a falsy one.
fn inert_fixture() -> Element {
    let on = use_signal(|| true);
    let off = use_signal(|| false);
    rsx! {
        div { id: "fx-inert-on", "inert": on(),
            button { id: "fx-under-inert", "On" }
        }
        div { id: "fx-inert-off", "inert": off(),
            button { id: "fx-under-falsy", "Off" }
        }
    }
}

#[test]
fn an_inert_subtree_is_skipped_by_tab() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(inert_fixture, incremental);
        assert!(
            harness.attr("#fx-inert-on", "inert").is_some()
                && harness.attr("#fx-inert-off", "inert").is_none(),
            "{mode}: a truthy inert is written and a falsy one is not"
        );
        harness.press(Key::Named(NamedKey::Tab));
        assert!(
            holder(&harness) == ["fx-under-falsy"],
            "{mode}: Tab skips the button under a truthy inert and reaches the one under a falsy \
             inert"
        );
        harness.press(Key::Named(NamedKey::Tab));
        assert!(
            holder(&harness) == ["fx-under-falsy"],
            "{mode}: Tab wraps to the one button outside the inert subtree"
        );
        harness.press_with(Key::Named(NamedKey::Tab), Modifiers::SHIFT);
        assert!(
            holder(&harness) == ["fx-under-falsy"],
            "{mode}: Shift+Tab skips the button under a truthy inert"
        );
    }
}
