//! Where a click inside a filled password field places its caret: by the bullets paint draws for
//! the field's characters, never by the layout of the characters themselves, so for two values of
//! equal length the same click lands at the same index and the caret stands where paint draws
//! it. On a fixture with the stand's options and its bundled font, in both layout modes, a field
//! filled by typing eight narrow characters and one filled with eight wide ones are each clicked
//! at the same offset inside the content box, and the two indexes read equal; a second click at
//! the same point selects the whole value. Before the fix (measured 2026-10-14) the click placed
//! the caret by the real characters' widths: index 5 after eight narrow characters, 2 after eight
//! wide. A failure message carries the layout mode and a case index, never the typed text.

use blitz_html::HtmlDocument;
use blitz_test_harness::Harness;
use seven_guis::stand;

const PAGE: &str = r#"<html><body style="margin:0">
    <input id="field" type="password"
        style="position:absolute; left:20px; top:20px; width:240px; padding:4px 8px; font-size:20px">
</body></html>"#;

/// How far into the content box the click lands, in CSS pixels.
const CLICK_OFFSET: f32 = 30.0;

/// The two fillings, eight characters each: narrow, then wide.
const FILLINGS: [&str; 2] = ["iiiiiiii", "WWWWWWWW"];

/// The selection's anchor and focus in the field's editor, in characters, and the width of the
/// editor's layout of the real characters.
fn selection(harness: &Harness<HtmlDocument>) -> (usize, usize, f32) {
    let field = harness.node("#field");
    let doc = harness.base();
    let data = doc
        .get_node(field)
        .and_then(|node| node.element_data())
        .and_then(|element| element.text_input_data())
        .expect("the field is a text input");
    let text = data.editor.text().to_string();
    let chars = |index: usize| text.get(..index).map_or(0, |before| before.chars().count());
    let selection = data.editor.raw_selection();
    let width = data
        .editor
        .try_layout()
        .map_or(0.0, |layout| layout.width());
    (
        chars(selection.anchor().index()),
        chars(selection.focus().index()),
        width,
    )
}

/// A fresh field holding `filling`, typed after a click, and the page point `CLICK_OFFSET` into
/// its content box, at its middle.
fn filled(filling: &str, incremental: bool, at: &str) -> (Harness<HtmlDocument>, (f32, f32)) {
    let mut harness = Harness::from_html_with(PAGE, stand::options(incremental));
    harness.click("#field");
    assert!(
        harness.focused() == Some(harness.node("#field")),
        "{at}: the click focuses the field"
    );
    harness.type_text(filling);
    let (_, typed_to, extent) = selection(&harness);
    assert!(
        typed_to == 8 && extent > 0.0,
        "{at}: the field holds eight typed characters laid out with a width"
    );

    let field = harness.node("#field");
    let rect = harness.layout_rect_of(field);
    let (left, layout) = {
        let doc = harness.base();
        let layout = doc
            .get_node(field)
            .map(|node| *node.final_layout())
            .expect("the field is laid out");
        (rect.x + layout.border.left + layout.padding.left, layout)
    };
    assert!(
        CLICK_OFFSET < extent && layout.content_box_width() > extent,
        "{at}: the offset lies inside the characters, and the characters inside the box"
    );
    (harness, (left + CLICK_OFFSET, rect.y + rect.height / 2.0))
}

#[test]
fn a_click_at_one_offset_places_the_caret_by_the_bullets() {
    for incremental in [false, true] {
        let mut measured = Vec::new();
        for (case, filling) in FILLINGS.into_iter().enumerate() {
            let at = format!("incremental={incremental}, case {case}");
            let (mut harness, (x, y)) = filled(filling, incremental, &at);
            harness.click_at(x, y);
            let (anchor, focus, _) = selection(&harness);
            assert!(
                anchor == focus && (1..8).contains(&focus),
                "{at}: the click places a caret inside the value"
            );
            measured.push(focus);
        }
        assert!(
            measured[0] == measured[1],
            "incremental={incremental}: the same click lands at the same index for two values of \
             equal length"
        );
    }
}

#[test]
fn a_double_click_selects_the_whole_value() {
    for incremental in [false, true] {
        for (case, filling) in FILLINGS.into_iter().enumerate() {
            let at = format!("incremental={incremental}, case {case}");
            let (mut harness, (x, y)) = filled(filling, incremental, &at);
            harness.click_at(x, y);
            harness.click_at(x, y);
            let (anchor, focus, _) = selection(&harness);
            assert!(
                (anchor.min(focus), anchor.max(focus)) == (0, 8),
                "{at}: the second click selects every character"
            );
        }
    }
}
