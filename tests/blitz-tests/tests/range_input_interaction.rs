//! A range input's value moves by its default actions, in every document: with the input focused
//! the right and up arrows step it up by its `step`, the left and down arrows step it down, and
//! Home and End take it to its minimum and maximum; a click sets it by where it lands along the
//! content box and focuses the input; every change fires one `input` event and a move that
//! changes nothing fires none. The value is the input's own current value, apart from its `value`
//! attribute: a write of that attribute re-seeds it, a write of `max` re-sanitizes it, a removal
//! of `value` re-seeds it and a removal of `step`, `min` or `max` re-sanitizes it, a change of
//! `type` away from `range` clears it and a change back re-seeds it, and any markup number reads
//! finite, inside `[min, max]` and on the step grid.
//!
//! <https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range)>

use blitz_dom::{LocalName, NodeId, QualName, ns};
use blitz_html::HtmlDocument;
use blitz_test_harness::{
    Harness, HarnessOptions, Key, Modifiers, NamedKey, key_event, mouse_pointer_event,
};
use blitz_traits::events::{KeyState, UiEvent};

const STEP_TWO: &str = r#"min="0" max="10" step="2" value="4""#;

fn page(attributes: &str) -> String {
    format!(
        r#"<html><body style="margin:0"><input id="range" type="range" {attributes}
            style="width:200px; padding:4px 20px; border:2px solid black"></body></html>"#
    )
}

/// A harness over `attributes`' range input, asserted to hold its current value.
fn open(attributes: &str, incremental: bool) -> (Harness<HtmlDocument>, NodeId) {
    let harness = Harness::from_html_with(
        &page(attributes),
        HarnessOptions {
            incremental: Some(incremental),
            ..Default::default()
        },
    );
    let range = harness.node("#range");
    assert!(
        harness
            .base()
            .get_node(range)
            .and_then(|node| node.element_data())
            .and_then(|element| element.range_input_data())
            .is_some(),
        "incremental {incremental}: the fixture holds a range input with its value seeded"
    );
    (harness, range)
}

fn value(harness: &Harness<HtmlDocument>, range: NodeId) -> Option<String> {
    harness
        .base()
        .get_node(range)?
        .element_data()?
        .range_input_value()
}

fn focus(harness: &mut Harness<HtmlDocument>, range: NodeId) {
    harness.base_mut().set_focus_to(range);
}

/// Presses `key` and reads how many `input` events it fired.
fn press(harness: &mut Harness<HtmlDocument>, key: Key) -> usize {
    let events = harness.dispatch_recorded([
        UiEvent::KeyDown(key_event(
            key.clone(),
            KeyState::Pressed,
            Modifiers::default(),
        )),
        UiEvent::KeyUp(key_event(key, KeyState::Released, Modifiers::default())),
    ]);
    harness.pump();
    inputs(&events)
}

/// Clicks at `(x, y)` and reads how many `input` events it fired.
fn click(harness: &mut Harness<HtmlDocument>, x: f32, y: f32) -> usize {
    let events = harness.dispatch_recorded([
        UiEvent::PointerDown(mouse_pointer_event(x, y)),
        UiEvent::PointerUp(mouse_pointer_event(x, y)),
    ]);
    harness.pump();
    inputs(&events)
}

fn inputs(events: &[String]) -> usize {
    events.iter().filter(|name| *name == "input").count()
}

/// The page point at `fraction` of the way along the input's content box, and its vertical
/// centre.
fn along(harness: &Harness<HtmlDocument>, range: NodeId, fraction: f32) -> (f32, f32) {
    let rect = harness.layout_rect_of(range);
    let doc = harness.base();
    let layout = doc.get_node(range).map(|node| *node.final_layout());
    let layout = layout.expect("the range input is laid out");
    let left = rect.x + layout.border.left + layout.padding.left;
    (
        left + fraction * layout.content_box_width(),
        rect.y + rect.height / 2.0,
    )
}

/// Keys pressed in order on the step-two input, focused: the value each reads and the `input`
/// events each fires.
const KEYS: [(Key, &str, usize); 7] = [
    (Key::Named(NamedKey::ArrowRight), "6", 1),
    (Key::Named(NamedKey::ArrowUp), "8", 1),
    (Key::Named(NamedKey::ArrowLeft), "6", 1),
    (Key::Named(NamedKey::ArrowDown), "4", 1),
    (Key::Named(NamedKey::End), "10", 1),
    (Key::Named(NamedKey::Home), "0", 1),
    (Key::Named(NamedKey::Home), "0", 0),
];

#[test]
fn the_arrows_home_and_end_step_a_focused_range_input() {
    assert_eq!(KEYS.len(), 7);
    for incremental in [false, true] {
        let (mut harness, range) = open(STEP_TWO, incremental);
        assert_eq!(value(&harness, range).as_deref(), Some("4"));
        focus(&mut harness, range);
        for (row, (key, read, fired)) in KEYS.into_iter().enumerate() {
            let counted = press(&mut harness, key);
            assert_eq!(
                value(&harness, range).as_deref(),
                Some(read),
                "incremental {incremental}, row {row}"
            );
            assert_eq!(counted, fired, "incremental {incremental}, row {row}");
            assert_eq!(
                harness.focused(),
                Some(range),
                "incremental {incremental}, row {row}"
            );
        }
    }
}

/// Clicks on the step-two input, in order: the fraction of its content box each lands at (`None`
/// for a point inside its left padding) and the value it maps to, sanitized onto the grid.
const CLICKS: [(Option<f32>, &str); 5] = [
    (Some(0.0), "0"),
    (Some(0.25), "2"),
    (Some(0.5), "6"),
    (Some(1.0), "10"),
    (None, "0"),
];

#[test]
fn a_click_sets_the_value_by_position_and_focuses() {
    assert_eq!(CLICKS.len(), 5);
    for incremental in [false, true] {
        let (mut harness, range) = open(STEP_TWO, incremental);
        assert_ne!(harness.focused(), Some(range), "incremental {incremental}");
        for (row, (fraction, read)) in CLICKS.into_iter().enumerate() {
            let (x, y) = match fraction {
                Some(fraction) => along(&harness, range, fraction),
                None => {
                    let (start, y) = along(&harness, range, 0.0);
                    (start - 10.0, y)
                }
            };
            let counted = click(&mut harness, x, y);
            assert_eq!(
                value(&harness, range).as_deref(),
                Some(read),
                "incremental {incremental}, row {row}"
            );
            assert_eq!(counted, 1, "incremental {incremental}, row {row}");
            assert_eq!(
                harness.focused(),
                Some(range),
                "incremental {incremental}, row {row}"
            );
        }
    }
}

/// Degenerate attributes and how they read: `min`, `max`, the step (`None` for `any`) and the
/// step base.
#[allow(clippy::type_complexity)]
const DEGENERATE: [(&str, (f64, f64, Option<f64>, f64)); 7] = [
    (r#"step="0""#, (0.0, 100.0, Some(1.0), 0.0)),
    (r#"step="-1""#, (0.0, 100.0, Some(1.0), 0.0)),
    (r#"step="any""#, (0.0, 100.0, None, 0.0)),
    (r#"min="10" max="5""#, (10.0, 10.0, Some(1.0), 10.0)),
    (r#"value="abc""#, (0.0, 100.0, Some(1.0), 0.0)),
    (r#"value="1e308""#, (0.0, 100.0, Some(1.0), 1e308)),
    (r#"min="NaN""#, (0.0, 100.0, Some(1.0), 0.0)),
];

#[test]
fn degenerate_attributes_keep_the_value_finite_bounded_and_on_the_grid() {
    assert_eq!(DEGENERATE.len(), 7);
    for incremental in [false, true] {
        for (row, (attributes, (min, max, step, base))) in DEGENERATE.into_iter().enumerate() {
            let (mut harness, range) = open(attributes, incremental);
            focus(&mut harness, range);
            press(&mut harness, Key::Named(NamedKey::ArrowRight));
            let (x, y) = along(&harness, range, 0.5);
            click(&mut harness, x, y);

            let read = value(&harness, range)
                .and_then(|text| text.parse::<f64>().ok())
                .unwrap_or(f64::NAN);
            assert!(read.is_finite(), "incremental {incremental}, row {row}");
            assert!(
                min <= read && read <= max,
                "incremental {incremental}, row {row}"
            );
            if let Some(step) = step {
                let index = read / step - base / step;
                assert!(
                    (index - index.round()).abs() < 1e-6,
                    "incremental {incremental}, row {row}"
                );
            }
        }
    }
}

#[test]
fn a_decimal_step_reads_its_decimals() {
    for incremental in [false, true] {
        let (mut harness, range) = open(r#"min="0" max="1" step="0.1" value="0""#, incremental);
        focus(&mut harness, range);
        for _ in 0..3 {
            press(&mut harness, Key::Named(NamedKey::ArrowRight));
        }
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("0.3"),
            "incremental {incremental}"
        );
    }
}

#[test]
fn a_disabled_range_input_takes_neither_keys_nor_a_click() {
    for incremental in [false, true] {
        let (mut harness, range) = open(&format!("{STEP_TWO} disabled"), incremental);
        focus(&mut harness, range);
        let keyed = press(&mut harness, Key::Named(NamedKey::ArrowRight));
        assert_eq!(keyed, 0, "incremental {incremental}");
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("4"),
            "incremental {incremental}"
        );

        let (x, y) = along(&harness, range, 1.0);
        let clicked = click(&mut harness, x, y);
        assert_eq!(clicked, 0, "incremental {incremental}");
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("4"),
            "incremental {incremental}"
        );
    }
}

fn attribute(name: &str) -> QualName {
    QualName::new(None, ns!(), LocalName::from(name))
}

#[test]
fn attribute_writes_reseed_and_resanitize_the_value() {
    for incremental in [false, true] {
        let (mut harness, range) = open(STEP_TWO, incremental);
        focus(&mut harness, range);
        press(&mut harness, Key::Named(NamedKey::ArrowRight));
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("6"),
            "incremental {incremental}"
        );

        harness
            .base_mut()
            .mutate()
            .set_attribute(range, attribute("value"), "7");
        harness.pump();
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("8"),
            "incremental {incremental}: a value write re-seeds, sanitized"
        );

        harness
            .base_mut()
            .mutate()
            .set_attribute(range, attribute("max"), "5");
        harness.pump();
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("4"),
            "incremental {incremental}: a max write below the value clamps it"
        );
    }
}

#[test]
fn attribute_removals_reseed_and_resanitize_the_value() {
    for incremental in [false, true] {
        let (mut harness, range) = open(r#"min="1" max="9" step="1.5" value="4""#, incremental);
        focus(&mut harness, range);
        press(&mut harness, Key::Named(NamedKey::ArrowRight));
        press(&mut harness, Key::Named(NamedKey::ArrowRight));
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("7"),
            "incremental {incremental}: two keys moved the value off its attribute"
        );

        harness
            .base_mut()
            .mutate()
            .clear_attribute(range, attribute("value"));
        harness.pump();
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("5.5"),
            "incremental {incremental}: a value removal re-seeds the default, on the grid"
        );

        harness
            .base_mut()
            .mutate()
            .clear_attribute(range, attribute("step"));
        harness.pump();
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("6"),
            "incremental {incremental}: a step removal re-sanitizes onto the default step"
        );

        harness
            .base_mut()
            .mutate()
            .clear_attribute(range, attribute("min"));
        harness.pump();
        press(&mut harness, Key::Named(NamedKey::Home));
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("0"),
            "incremental {incremental}: after a min removal Home reads the default minimum"
        );

        harness
            .base_mut()
            .mutate()
            .clear_attribute(range, attribute("max"));
        harness.pump();
        press(&mut harness, Key::Named(NamedKey::End));
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("100"),
            "incremental {incremental}: after a max removal End reads the default maximum"
        );
    }
}

#[test]
fn a_type_change_clears_the_range_data_and_a_change_back_reseeds_it() {
    for incremental in [false, true] {
        let (mut harness, range) = open(STEP_TWO, incremental);
        focus(&mut harness, range);
        press(&mut harness, Key::Named(NamedKey::ArrowRight));
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("6"),
            "incremental {incremental}: a key moved the value off its attribute"
        );

        harness
            .base_mut()
            .mutate()
            .set_attribute(range, attribute("type"), "text");
        harness.pump();
        let holds_range = |harness: &Harness<HtmlDocument>| {
            harness
                .base()
                .get_node(range)
                .and_then(|node| node.element_data())
                .is_some_and(|element| element.range_input_data().is_some())
        };
        assert!(
            !holds_range(&harness),
            "incremental {incremental}: a text input holds no range data"
        );

        harness
            .base_mut()
            .mutate()
            .set_attribute(range, attribute("type"), "range");
        harness.pump();
        assert!(
            holds_range(&harness),
            "incremental {incremental}: a range input again holds its range data"
        );
        assert_eq!(
            value(&harness, range).as_deref(),
            Some("4"),
            "incremental {incremental}: the change back re-seeds from the value attribute"
        );
    }
}
