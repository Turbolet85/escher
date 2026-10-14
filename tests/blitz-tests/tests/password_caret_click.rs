//! Where a click inside a filled password field places its caret, measured: the click places the
//! caret by the layout of the field's real characters, while paint draws one bullet per
//! character and the caret at the bullet of its index — so for two values of equal length the
//! same click can land at a different index. On a fixture with the stand's options and its
//! bundled font, in both layout modes, a field filled by typing eight narrow characters and one
//! filled with eight wide ones are each clicked at the same offset inside the content box. The
//! two indexes are pinned as measured (2026-10-14); a fix that places the caret by the bullets
//! turns this check red by design, and it is restated then. A failure message carries the
//! layout mode and a case index, never the typed text.

use blitz_html::HtmlDocument;
use blitz_test_harness::Harness;
use seven_guis::stand;

const PAGE: &str = r#"<html><body style="margin:0">
    <input id="field" type="password"
        style="position:absolute; left:20px; top:20px; width:240px; padding:4px 8px; font-size:20px">
</body></html>"#;

/// How far into the content box the click lands, in CSS pixels.
const CLICK_OFFSET: f32 = 30.0;

/// The two fillings, eight characters each: narrow, then wide. Each with the caret index the
/// click places, in characters, as measured.
const CASES: [(&str, usize); 2] = [("iiiiiiii", 5), ("WWWWWWWW", 2)];

/// The caret's index in the field's editor, in characters, and the width of the editor's layout
/// of the real characters.
fn caret(harness: &Harness<HtmlDocument>) -> (usize, f32) {
    let field = harness.node("#field");
    let doc = harness.base();
    let data = doc
        .get_node(field)
        .and_then(|node| node.element_data())
        .and_then(|element| element.text_input_data())
        .expect("the field is a text input");
    let text = data.editor.text().to_string();
    let index = data.editor.raw_selection().focus().index();
    let chars = text.get(..index).map_or(0, |before| before.chars().count());
    let width = data
        .editor
        .try_layout()
        .map_or(0.0, |layout| layout.width());
    (chars, width)
}

#[test]
fn a_click_at_one_offset_places_the_caret_by_the_real_characters() {
    for incremental in [false, true] {
        let mut measured = Vec::new();
        for (case, (filling, _)) in CASES.into_iter().enumerate() {
            let at = format!("incremental={incremental}, case {case}");
            let mut harness = Harness::from_html_with(PAGE, stand::options(incremental));
            harness.click("#field");
            assert!(
                harness.focused() == Some(harness.node("#field")),
                "{at}: the click focuses the field"
            );
            harness.type_text(filling);
            let (typed_to, extent) = caret(&harness);
            assert!(
                typed_to == 8 && extent > 0.0,
                "{at}: the field holds eight typed characters laid out with a width"
            );

            let field = harness.node("#field");
            let rect = harness.layout_rect_of(field);
            let (left, padding) = {
                let doc = harness.base();
                let layout = doc
                    .get_node(field)
                    .map(|node| *node.final_layout())
                    .expect("the field is laid out");
                (rect.x + layout.border.left + layout.padding.left, layout)
            };
            assert!(
                CLICK_OFFSET < extent && padding.content_box_width() > extent,
                "{at}: the offset lies inside the characters, and the characters inside the box"
            );
            harness.click_at(left + CLICK_OFFSET, rect.y + rect.height / 2.0);
            let (placed, _) = caret(&harness);
            measured.push(placed);
        }
        let pinned: Vec<usize> = CASES.iter().map(|(_, index)| *index).collect();
        assert!(
            measured == pinned,
            "incremental={incremental}: each click places the caret at its measured index"
        );
        assert!(
            measured[0] != measured[1],
            "incremental={incremental}: the same click lands at different indexes for two \
             values of equal length"
        );
    }
}
