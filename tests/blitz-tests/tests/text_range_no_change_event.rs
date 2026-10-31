//! A stated limit, pinned: a text input and a range input fire `input` and no `change`, in every
//! document. Typing into a text input, a range's key and its pointerdown each fire `input` alone,
//! where HTML fires `change` when the edit is committed. On a Dioxus page booted with the stand's
//! options, in both layout modes. A failure message carries the layout mode and a row.
//!
//! <https://html.spec.whatwg.org/multipage/input.html#common-input-element-events>

use ::keyboard_types::{Key, NamedKey};
use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;

mod fixtures;

/// A text input and a range input, counting what each fires.
fn page() -> Element {
    let mut inputs = use_signal(|| 0u32);
    let mut changes = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            input {
                id: "fx-text",
                oninput: move |_| *inputs.write() += 1,
                onchange: move |_| *changes.write() += 1,
            }
            input {
                id: "fx-range",
                r#type: "range",
                min: "0",
                max: "100",
                value: "50",
                style: "display: block; width: 200px;",
                oninput: move |_| *inputs.write() += 1,
                onchange: move |_| *changes.write() += 1,
            }
            p { id: "fx-inputs", "{inputs}" }
            p { id: "fx-changes", "{changes}" }
        }
    }
}

type Page = Harness<DioxusDocument>;

/// The `input` and `change` events `act` fires.
fn fired(harness: &mut Page, act: impl FnOnce(&mut Page)) -> (u32, u32) {
    let read = |harness: &Page, selector: &str| {
        harness
            .text_content(selector)
            .trim()
            .parse::<u32>()
            .unwrap_or(u32::MAX)
    };
    let before = (read(harness, "#fx-inputs"), read(harness, "#fx-changes"));
    act(harness);
    let after = (read(harness, "#fx-inputs"), read(harness, "#fx-changes"));
    (after.0 - before.0, after.1 - before.1)
}

#[test]
fn typing_in_a_text_input_fires_input_and_no_change() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(page, incremental);
        harness.click("#fx-text");
        assert!(
            fired(&mut harness, |harness| harness.type_text("abc")) == (3, 0),
            "{mode}: each character fires input, and none change"
        );
        assert!(
            fired(&mut harness, |harness| {
                harness.press(Key::Named(NamedKey::Tab));
            }) == (0, 0),
            "{mode}: leaving the input fires no change"
        );
    }
}

#[test]
fn a_ranges_key_and_pointerdown_fire_input_and_no_change() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = fixtures::boot(page, incremental);
        let rect = harness.layout_rect("#fx-range");
        let (x, y) = (rect.x + rect.width * 0.9, rect.y + rect.height / 2.0);
        assert!(
            fired(&mut harness, |harness| harness.mouse_down_at(x, y)) == (1, 0),
            "{mode}: a pointerdown fires input alone"
        );
        harness.mouse_up_at(x, y);
        assert!(
            fired(&mut harness, |harness| {
                harness.press(Key::Named(NamedKey::ArrowLeft));
            }) == (1, 0),
            "{mode}: a key fires input alone"
        );
    }
}
