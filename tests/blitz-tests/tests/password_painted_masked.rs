//! A password input paints masked: whatever characters it holds, the frame shows one bullet per
//! character, so a typed password never leaves the engine as pixels. Painted through
//! `paint_scene` into the CPU renderer with the stand's bundled font, in both layout modes:
//! two equal-length password values paint the same frame, while the same two values in a text
//! input paint different frames (the check can see characters), the bullets are drawn, and the
//! caret paints when the input holds focus.

use anyrender::render_to_buffer;
use anyrender_vello_cpu::VelloCpuImageRenderer;
use blitz_html::HtmlDocument;
use blitz_paint::paint_scene;
use blitz_test_harness::{Harness, HarnessOptions};
use seven_guis::stand;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

/// Two values of the same length with no character in common.
const FIRST: &str = "abcdefgh";
const SECOND: &str = "STUVWXYZ";

fn harness(kind: &str, value: &str, incremental: bool) -> Harness<HtmlDocument> {
    let html = format!(
        r#"<html><body style="margin:0; background:#ffffff;">
            <input id="field" type="{kind}" value="{value}"
                style="position:absolute; left:20px; top:20px; width:240px; font-size:20px;">
        </body></html>"#
    );
    Harness::from_html_with(
        &html,
        HarnessOptions {
            width: WIDTH,
            height: HEIGHT,
            font_ctx: Some(stand::font_ctx()),
            incremental: Some(incremental),
            ..Default::default()
        },
    )
}

fn paint(harness: &mut Harness<HtmlDocument>) -> Vec<u8> {
    let mut doc = harness.base_mut();
    render_to_buffer::<VelloCpuImageRenderer, _>(
        |scene| paint_scene(scene, &mut doc, 1.0, WIDTH, HEIGHT, 0, 0),
        WIDTH,
        HEIGHT,
    )
}

/// The frame of an input of `kind` holding `value`, focused or not.
fn frame(kind: &str, value: &str, focused: bool, incremental: bool) -> Vec<u8> {
    let mut harness = harness(kind, value, incremental);
    if focused {
        harness.click("#field");
        assert!(
            harness.focused().is_some(),
            "mode incremental={incremental}: the click focuses the input"
        );
    }
    paint(&mut harness)
}

/// The RGBA bytes inside the input's border box.
fn field_pixels(kind: &str, value: &str, focused: bool, incremental: bool) -> Vec<u8> {
    let mut harness = harness(kind, value, incremental);
    if focused {
        harness.click("#field");
    }
    let rect = harness.layout_rect("#field");
    let frame = paint(&mut harness);
    let (x0, y0) = (rect.x as usize, rect.y as usize);
    let (x1, y1) = (
        ((rect.x + rect.width) as usize).min(WIDTH as usize),
        ((rect.y + rect.height) as usize).min(HEIGHT as usize),
    );
    let mut out = Vec::new();
    for y in y0..y1 {
        let row = y * WIDTH as usize * 4;
        out.extend_from_slice(&frame[row + x0 * 4..row + x1 * 4]);
    }
    out
}

#[test]
fn a_text_input_paints_its_characters() {
    for incremental in [false, true] {
        assert_ne!(
            frame("text", FIRST, false, incremental),
            frame("text", SECOND, false, incremental),
            "mode incremental={incremental}: two text values paint different frames"
        );
    }
}

#[test]
fn two_password_values_of_one_length_paint_the_same_frame() {
    for incremental in [false, true] {
        assert_eq!(
            frame("password", FIRST, false, incremental),
            frame("password", SECOND, false, incremental),
            "mode incremental={incremental}: two password values paint different frames"
        );
    }
}

#[test]
fn a_password_input_paints_its_bullets() {
    for incremental in [false, true] {
        assert_ne!(
            field_pixels("password", FIRST, false, incremental),
            field_pixels("password", "", false, incremental),
            "mode incremental={incremental}: a password value paints nothing in its box"
        );
    }
}

#[test]
fn a_focused_password_input_paints_its_caret() {
    for incremental in [false, true] {
        assert_eq!(
            frame("password", FIRST, true, incremental),
            frame("password", SECOND, true, incremental),
            "mode incremental={incremental}: two focused password values paint different frames"
        );
        assert_ne!(
            field_pixels("password", FIRST, true, incremental),
            field_pixels("password", FIRST, false, incremental),
            "mode incremental={incremental}: focus paints no caret in the box"
        );
    }
}
