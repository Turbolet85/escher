//! A document reads its own time — the overlay scrollbar fade, double-click detection and the
//! smooth scroll — from the clock its config hands it, and from nothing else. On `HtmlDocument`s
//! built with a `VirtualClock` in `DocumentConfig`, the stand's font and viewport, in both layout
//! modes and with no sleep: after a scroll a scroller's thumb reads fully opaque through 500 ms
//! of clock, part-faded at 600 ms and gone at 700 ms, when the document stops animating; two
//! clicks on one point with the clock unmoved select a text input's word, while 500 ms of clock
//! between them leave its caret; and a smooth scroll stands still however often the document
//! resolves until the clock moves, and ends at its target after 300 ms of it. A finger pan's
//! moves and its release are stamped by the clock: a pan whose moves the clock spaced flings on
//! release, one the clock held through, or released 100 ms of clock after its last move, does
//! not; and the fling stands still however often the document resolves until the clock moves.
//! A document built with no clock keeps the wall clock: its thumb reads fully opaque right
//! after a scroll. A failure message carries the layout mode, a case and a step index.

use std::sync::Arc;
use std::time::Duration;

use blitz_dom::{Clock, DocumentConfig, NodeId, ScrollBehavior, VirtualClock};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_test_harness::Harness;
use blitz_traits::shell::Viewport;
use seven_guis::stand;

const PAGE: &str = r#"<html><body style="margin:0">
    <div id="scroller" style="width:200px; height:100px; overflow-y:auto;">
        <div style="height:1000px; background:#0000ff;"></div>
    </div>
    <input id="field" value="alpha beta"
        style="position:absolute; left:20px; top:200px; width:240px; padding:4px 8px; font-size:20px">
</body></html>"#;

/// The stand's viewport and font, in `incremental`'s layout mode, reading `clock` when given.
fn config(incremental: bool, clock: Option<&VirtualClock>) -> DocumentConfig {
    let options = stand::options(incremental);
    DocumentConfig {
        viewport: Some(Viewport::new(
            options.width,
            options.height,
            options.scale,
            options.color_scheme,
        )),
        html_parser_provider: Some(Arc::new(HtmlProvider) as _),
        font_ctx: options.font_ctx,
        incremental: Some(incremental),
        clock: clock.map(|clock| Arc::new(clock.clone()) as Arc<dyn Clock>),
        ..Default::default()
    }
}

fn document(incremental: bool, clock: Option<&VirtualClock>) -> HtmlDocument {
    let mut doc = HtmlDocument::from_html(PAGE, config(incremental, clock));
    doc.resolve(0.0);
    doc
}

fn scroller(doc: &HtmlDocument) -> NodeId {
    doc.query_selector("#scroller")
        .expect("the selector parses")
        .expect("the page has a scroller")
}

fn offset_y(doc: &HtmlDocument, node: NodeId) -> f64 {
    doc.get_node(node)
        .expect("the node is live")
        .scroll_offset()
        .y
}

/// What a thumb reads at a step of the fade.
#[derive(Clone, Copy)]
enum Thumb {
    /// Fully opaque, with the fade still due.
    Opaque,
    /// Part faded.
    Fading,
    /// Gone, and nothing animates.
    Gone,
}

#[test]
fn the_scrollbar_fade_moves_only_with_the_clock() {
    // The clock's total after each advance, and what the thumb reads then.
    let rows = [
        (0, Thumb::Opaque),
        (500, Thumb::Opaque),
        (600, Thumb::Fading),
        (700, Thumb::Gone),
    ];
    assert_eq!(rows.len(), 4);
    for incremental in [false, true] {
        let clock = VirtualClock::new();
        let mut doc = document(incremental, Some(&clock));
        let node = scroller(&doc);
        let at = format!("incremental={incremental}: fade");
        assert!(doc.scrollbar_opacity(node) == 0.0, "{at}: no thumb at rest");
        doc.scroll_by(node, 0.0, 50.0, ScrollBehavior::Instant);
        assert!(
            offset_y(&doc, node) == 50.0,
            "{at}: the scroll moves the box"
        );

        for (step, (total_ms, thumb)) in rows.into_iter().enumerate() {
            clock.advance(Duration::from_millis(total_ms) - clock.now());
            doc.resolve(0.0);
            let opacity = doc.scrollbar_opacity(node);
            let reads = match thumb {
                Thumb::Opaque => opacity == 1.0 && doc.is_animating(),
                Thumb::Fading => opacity > 0.0 && opacity < 1.0 && doc.is_animating(),
                Thumb::Gone => opacity == 0.0 && !doc.is_animating(),
            };
            assert!(
                reads,
                "{at}: step {step}: the thumb reads as the row states"
            );
        }
        doc.resolve(0.0);
        assert!(
            doc.scrollbar_opacity(node) == 0.0 && !doc.is_animating(),
            "{at}: the finished fade stays finished"
        );
    }
}

/// The selection's anchor and focus in the field's editor, in bytes.
fn selection(harness: &Harness<HtmlDocument>) -> (usize, usize) {
    let field = harness.node("#field");
    let doc = harness.base();
    let data = doc
        .get_node(field)
        .and_then(|node| node.element_data())
        .and_then(|element| element.text_input_data())
        .expect("the field is a text input");
    let selection = data.editor.raw_selection();
    (selection.anchor().index(), selection.focus().index())
}

/// The field's editor text.
fn field_text(harness: &Harness<HtmlDocument>) -> String {
    let field = harness.node("#field");
    let doc = harness.base();
    doc.get_node(field)
        .and_then(|node| node.element_data())
        .and_then(|element| element.text_input_data())
        .map(|data| data.editor.text().to_string())
        .unwrap_or_default()
}

#[test]
fn two_clicks_are_a_double_click_by_the_clock_alone() {
    // The milliseconds of clock between the two clicks, and whether they select a word.
    let rows = [(0, true), (499, true), (500, false)];
    assert_eq!(rows.len(), 3);
    for incremental in [false, true] {
        for (case, (between_ms, selects)) in rows.into_iter().enumerate() {
            let at = format!("incremental={incremental}: double click: case {case}");
            let clock = VirtualClock::new();
            let mut harness = Harness::wrap(document(incremental, Some(&clock)));
            harness.pump();
            assert!(
                field_text(&harness) == "alpha beta",
                "{at}: the field holds two words"
            );
            let field = harness.node("#field");
            let rect = harness
                .base()
                .get_client_bounding_rect(field)
                .expect("the field is laid out");
            let (x, y) = (rect.x as f32 + 20.0, (rect.y + rect.height / 2.0) as f32);

            harness.click_at(x, y);
            let (anchor, focus) = selection(&harness);
            assert!(anchor == focus, "{at}: the first click places a caret");
            clock.advance(Duration::from_millis(between_ms));
            harness.click_at(x, y);
            let (anchor, focus) = selection(&harness);
            assert!(
                (anchor != focus) == selects,
                "{at}: the second click selects a word exactly when it lands inside 500 ms of \
                 clock"
            );
        }
    }
}

#[test]
fn a_smooth_scroll_stands_still_until_the_clock_moves() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: smooth scroll");
        let clock = VirtualClock::new();
        let mut doc = document(incremental, Some(&clock));
        let node = scroller(&doc);
        doc.scroll_to(node, 0.0, 200.0, ScrollBehavior::Smooth);
        for step in 0..5 {
            doc.resolve(0.0);
            assert!(
                doc.is_animating() && offset_y(&doc, node) == 0.0,
                "{at}: step {step}: the scroll is under way and has not moved"
            );
        }
        clock.advance(Duration::from_millis(299));
        doc.resolve(0.0);
        let midway = offset_y(&doc, node);
        assert!(
            midway > 0.0 && midway < 200.0,
            "{at}: 299 ms of clock leave the box short of its target"
        );
        clock.advance(Duration::from_millis(1));
        doc.resolve(0.0);
        assert!(
            offset_y(&doc, node) == 200.0,
            "{at}: 300 ms of clock end the scroll at its target"
        );
        doc.resolve(0.0);
        assert!(
            offset_y(&doc, node) == 200.0,
            "{at}: the ended scroll stays at its target"
        );
    }
}

/// The scroller's offset in a harness's document.
fn harness_offset_y(harness: &Harness<HtmlDocument>) -> f64 {
    let node = harness.node("#scroller");
    harness
        .base()
        .get_node(node)
        .expect("the node is live")
        .scroll_offset()
        .y
}

#[test]
fn a_finger_pan_and_its_fling_move_only_with_the_clock() {
    // The clock between the pan's moves, the clock between its last move and the release, and
    // whether the release flings: a fling reads the pan's speed off the moves' clock stamps and
    // takes none from a pan more than 100 ms of clock old.
    let rows = [(10, 0, true), (0, 0, false), (10, 100, false)];
    assert_eq!(rows.len(), 3);
    for incremental in [false, true] {
        for (case, (between_ms, before_release_ms, flings)) in rows.into_iter().enumerate() {
            let at = format!("incremental={incremental}: pan: case {case}");
            let clock = VirtualClock::new();
            let mut harness = Harness::wrap(document(incremental, Some(&clock)));
            harness.pump();
            assert!(
                harness_offset_y(&harness) == 0.0,
                "{at}: the box starts unscrolled"
            );

            // A finger drawn up the box: the first move past the threshold starts the pan, and
            // each later one scrolls by its own distance.
            harness.touch_down(0, 100.0, 90.0);
            for y in [80.0, 60.0, 40.0] {
                clock.advance(Duration::from_millis(between_ms));
                harness.touch_move(0, 100.0, y);
            }
            assert!(
                harness_offset_y(&harness) == 40.0,
                "{at}: the pan moves the box by the finger's distance"
            );
            clock.advance(Duration::from_millis(before_release_ms));
            harness.touch_up(0, 100.0, 40.0);
            for step in 0..5 {
                harness.pump();
                assert!(
                    harness_offset_y(&harness) == 40.0,
                    "{at}: step {step}: with the clock held the box stands still"
                );
            }
            clock.advance(Duration::from_millis(16));
            harness.pump();
            let after = harness_offset_y(&harness);
            assert!(
                (after > 40.0) == flings,
                "{at}: 16 ms of clock move the box exactly when the release flung"
            );
            if flings {
                clock.advance(Duration::from_millis(16));
                harness.pump();
                assert!(
                    harness_offset_y(&harness) > after,
                    "{at}: the fling goes on as the clock moves"
                );
            }
        }
    }
}

#[test]
fn a_document_built_with_no_clock_fades_by_the_wall_clock() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: wall clock");
        let mut doc = document(incremental, None);
        let node = scroller(&doc);
        doc.scroll_by(node, 0.0, 50.0, ScrollBehavior::Instant);
        assert!(
            offset_y(&doc, node) == 50.0,
            "{at}: the scroll moves the box"
        );
        assert!(
            doc.scrollbar_opacity(node) == 1.0 && doc.is_animating(),
            "{at}: the thumb is fully opaque right after the scroll"
        );
    }
}
