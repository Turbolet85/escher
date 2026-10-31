//! The engine fires `change` on a checkbox and a radio, in every document: a click and a Space on
//! a checkbox each fire one `input` then one `change`; a radio's first click fires both and a
//! click on it already checked fires `input` alone; a label's click reads as its control's. The
//! snapshot's `checked` and the changed set read as they did. On a Dioxus page booted with the
//! stand's options, in both layout modes. A failure message carries the layout mode and a row.
//!
//! <https://html.spec.whatwg.org/multipage/input.html#the-input-element:event-change>

use ::keyboard_types::Key;
use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;

mod fixtures;

/// Each event an app hears, in order, as one word per event.
fn page() -> Element {
    let mut heard = use_signal(Vec::<&'static str>::new);
    rsx! {
        div { id: "fx-root",
            input {
                id: "fx-check",
                r#type: "checkbox",
                oninput: move |_| heard.write().push("input"),
                onchange: move |_| heard.write().push("change"),
            }
            input {
                id: "fx-radio",
                r#type: "radio",
                name: "set",
                oninput: move |_| heard.write().push("input"),
                onchange: move |_| heard.write().push("change"),
            }
            label { id: "fx-label", r#for: "fx-labelled", style: "display: block;", "Labelled" }
            input {
                id: "fx-labelled",
                r#type: "checkbox",
                oninput: move |_| heard.write().push("input"),
                onchange: move |_| heard.write().push("change"),
            }
            p { id: "fx-heard", {heard.read().join(",")} }
        }
    }
}

type Page = Harness<DioxusDocument>;

/// What the app heard from `act` alone.
fn heard_from(harness: &mut Page, act: impl FnOnce(&mut Page)) -> String {
    let before = harness.text_content("#fx-heard");
    act(harness);
    let after = harness.text_content("#fx-heard");
    after
        .strip_prefix(&before)
        .unwrap_or(&after)
        .trim_start_matches(',')
        .to_string()
}

fn checked(harness: &Page, id: &str) -> Option<bool> {
    harness
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.checked)
}

#[test]
fn a_click_and_a_space_on_a_checkbox_each_fire_input_then_change() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(page, incremental);
        harness.base_mut().take_changed_nodes();
        assert!(
            heard_from(&mut harness, |harness| harness.click("#fx-check")) == "input,change",
            "{mode}: a click fires one input then one change"
        );
        assert!(
            checked(&harness, "fx-check") == Some(true),
            "{mode}: the click checks it"
        );
        let check = harness.node("#fx-check");
        assert!(
            harness.base_mut().take_changed_nodes().contains(&check),
            "{mode}: the click marks the control"
        );
        assert!(
            heard_from(&mut harness, |harness| {
                harness.press(Key::Character(" ".to_string()));
            }) == "input,change",
            "{mode}: a Space fires one input then one change"
        );
        assert!(
            checked(&harness, "fx-check") == Some(false),
            "{mode}: the Space unchecks it"
        );
    }
}

#[test]
fn a_radios_change_fires_on_an_actual_change_alone() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(page, incremental);
        assert!(
            heard_from(&mut harness, |harness| harness.click("#fx-radio")) == "input,change",
            "{mode}: the first click fires input then change"
        );
        assert!(
            checked(&harness, "fx-radio") == Some(true),
            "{mode}: the click checks it"
        );
        harness.advance_time(std::time::Duration::from_millis(500));
        assert!(
            heard_from(&mut harness, |harness| harness.click("#fx-radio")) == "input",
            "{mode}: a click on it already checked fires input alone"
        );
    }
}

#[test]
fn a_labels_click_reads_as_its_controls() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(page, incremental);
        assert_eq!(
            heard_from(&mut harness, |harness| harness.click("#fx-label")),
            "input,change",
            "{mode}: the label's click fires the control's input then change"
        );
        assert!(
            checked(&harness, "fx-labelled") == Some(true),
            "{mode}: the label's click checks the control"
        );
    }
}
