//! The driver takes a screenshot with no display: the whole screen of the held instance, or one
//! element by its stable id, as a PNG of what the engine paints. On the stand, in both layout
//! modes: each lean task's whole screen at boot is an 800 × 600 PNG that decodes to the
//! harness's own raster of that state, text paints inside the task title, and the page reads its
//! background colour; CRUD's list is cropped to its bounds rounded outward, 369 × 420, the same
//! pixels as that rectangle of the whole screen. An element shot is refused `not-found` for an
//! id never read, `stale` for one read earlier and since gone, and `off-screen` for a row past
//! the list's box until a `scroll` brings it into view, each leaving the instance unchanged; an
//! app booted with a font context that holds no family is refused `fonts-unavailable`. A
//! password typed through the driver never leaves as pixels: two sentinels of one length give
//! byte-identical PNGs, where a text input's differ. A shot changes nothing: no change is marked,
//! the focus reads as before and the diff across it is empty, a focused text input included. No
//! check here skips, sleeps or reads a clock. A failure message carries a layout mode, a task and
//! an index only.

use blitz_test_harness::HarnessOptions;
use dioxus::prelude::*;
use escher_driver::{Cause, Session};
use seven_guis::stand::{self, LeanTask};

mod fixtures;
mod session_common;
use session_common::{
    act, click, focused, hold, refused_unchanged, screenshot, screenshot_of, scroll, shot, slug,
    type_into,
};

/// The stand's pinned viewport, in pixels.
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

/// The page background, design-system §Color Palette (Surface Scale).
const PAGE: [u8; 4] = [0xf5, 0xf5, 0xf5, 0xff];

/// How many driver clicks on Create put the last row's centre past the CRUD list's box.
const CREATES: usize = 12;

/// An id no element of the stand carries.
const NOBODY: &str = "screenshot-names-no-element";

/// What the checks type into the fixture's inputs: synthetic, of one length, no character
/// shared.
const SENTINELS: [&str; 2] = ["Shot-Sentinel-aqzx", "SHOT_SENTINEL_BWYV"];

/// The pixels a PNG decodes to: its width, its height and its RGBA bytes.
#[track_caller]
fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<u8>) {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder.read_info().expect("the shot is a PNG");
    let mut buf = vec![0; reader.output_buffer_size().expect("a bounded image")];
    let info = reader.next_frame(&mut buf).expect("the PNG holds a frame");
    assert!(info.color_type == png::ColorType::Rgba && info.bit_depth == png::BitDepth::Eight);
    buf.truncate(info.buffer_size());
    (info.width, info.height, buf)
}

fn pixel(rgba: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
    let at = ((y * width + x) * 4) as usize;
    [rgba[at], rgba[at + 1], rgba[at + 2], rgba[at + 3]]
}

/// The `width` × `height` pixels at (`x`, `y`) of a frame `frame_width` pixels wide.
fn crop(rgba: &[u8], frame_width: u32, (x, y, width, height): (u32, u32, u32, u32)) -> Vec<u8> {
    let mut out = Vec::new();
    for row in y..y + height {
        let start = ((row * frame_width + x) * 4) as usize;
        out.extend_from_slice(&rgba[start..start + (width * 4) as usize]);
    }
    out
}

/// The bounds a snapshot of `session` reads for `id`.
fn bounds(session: &Session, id: &str) -> (f64, f64, f64, f64) {
    let screen = session.harness().doc.snapshot();
    let node = screen.get(id).expect("the element is on the screen");
    let b = node.bounds;
    (b.x, b.y, b.width, b.height)
}

#[test]
fn the_whole_screen_is_a_png_of_the_harness_raster_and_text_paints() {
    for incremental in [false, true] {
        for (index, task) in LeanTask::ALL.into_iter().enumerate() {
            let at = format!("incremental={incremental}: task {index}");
            let (mut session, _ticks) = hold(task, incremental);
            let Some(shot) = shot(&mut session, &screenshot()) else {
                panic!("{at}: the screenshot is accepted");
            };
            assert!(
                (shot.width, shot.height) == (WIDTH, HEIGHT),
                "{at}: the shot is the viewport"
            );
            let (width, height, rgba) = decode(&shot.png);
            assert!(
                (width, height) == (WIDTH, HEIGHT),
                "{at}: the PNG is the viewport"
            );
            let raster = session
                .harness_mut()
                .screenshot()
                .expect("the stand paints text");
            assert!(
                raster.rgba == rgba,
                "{at}: the PNG decodes to the harness's own raster of that state"
            );

            let (x, y, w, h) = bounds(&session, "task-title");
            let (x0, y0) = (x.ceil() as u32, y.ceil() as u32);
            let (x1, y1) = ((x + w).floor() as u32, (y + h).floor() as u32);
            let corner = pixel(&rgba, width, x0, y0);
            let painted = (y0..y1)
                .flat_map(|py| (x0..x1).map(move |px| (px, py)))
                .filter(|(px, py)| pixel(&rgba, width, *px, *py) != corner)
                .count();
            assert!(painted > 0, "{at}: text paints inside the task title");

            assert!(
                pixel(&rgba, width, WIDTH - 4, HEIGHT - 4) == PAGE,
                "{at}: the page reads its background colour"
            );
        }
    }
}

#[test]
fn an_element_shot_is_its_bounds_rounded_outward() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        assert!(
            bounds(&session, "crud-list") == (24.0, 112.796875, 369.0, 419.203125),
            "{at}: the list stands where the layout reads it"
        );
        let whole = shot(&mut session, &screenshot()).expect("the screenshot is accepted");
        let Some(list) = shot(&mut session, &screenshot_of("crud-list")) else {
            panic!("{at}: the element shot is accepted");
        };
        assert!(
            (list.width, list.height) == (369, 420),
            "{at}: the shot is the list's bounds rounded outward"
        );
        let (_, _, whole) = decode(&whole.png);
        let (width, height, part) = decode(&list.png);
        assert!((width, height) == (369, 420), "{at}: the PNG is that size");
        assert!(
            part == crop(&whole, WIDTH, (24, 112, 369, 420)),
            "{at}: the shot is that rectangle of the whole screen"
        );
    }
}

#[test]
fn an_element_shot_is_refused_as_every_id_addressed_call_is() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);

        assert!(
            refused_unchanged(&mut session, &screenshot_of(NOBODY))
                == (Some(Cause::NotFound), true),
            "{at}: an id never read is refused not-found, with nothing changed"
        );

        for index in 0..CREATES {
            assert!(
                act(&mut session, &click("crud-create")).is_some(),
                "{at}: create {index} runs"
            );
        }
        let last = format!("crud-person-{}", 2 + CREATES);
        assert!(
            refused_unchanged(&mut session, &screenshot_of(&last))
                == (Some(Cause::OffScreen), true),
            "{at}: the last row past the list's box is refused off-screen, with nothing changed"
        );
        assert!(
            act(&mut session, &scroll(&last))
                .is_some_and(|scrolled| scrolled.in_view == Some(true)),
            "{at}: the scroll brings the last row into view"
        );
        assert!(
            shot(&mut session, &screenshot_of(&last))
                .is_some_and(|row| row.width > 0 && row.height > 0),
            "{at}: the last row in view is accepted"
        );

        assert!(
            act(&mut session, &click(&last)).is_some()
                && act(&mut session, &click("crud-delete")).is_some(),
            "{at}: the last row is selected and deleted"
        );
        assert!(
            refused_unchanged(&mut session, &screenshot_of(&last)) == (Some(Cause::Stale), true),
            "{at}: a row read earlier and since gone is refused stale, with nothing changed"
        );
    }
}

#[test]
fn an_app_with_no_usable_font_is_refused_and_nothing_changes() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: Crud");
        let harness = stand::boot(
            LeanTask::Crud,
            HarnessOptions {
                font_ctx: Some(blitz_dom::build_single_font_ctx(&[0u8; 16])),
                ..stand::options(incremental)
            },
        );
        let mut session = Session::start(slug(LeanTask::Crud), || harness)
            .expect("a lean task's slug is a label");
        for (index, call) in [screenshot(), screenshot_of("crud-list")]
            .iter()
            .enumerate()
        {
            assert!(
                refused_unchanged(&mut session, call) == (Some(Cause::FontsUnavailable), true),
                "{at}: shot {index} is refused fonts-unavailable, with nothing changed"
            );
        }
    }
}

/// A password input, empty, beside the stand.
fn password_fixture() -> Element {
    rsx! {
        div { id: "fx-root",
            input { id: "fx-input", r#type: "password", aria_label: "Field" }
        }
    }
}

/// A text input, empty, beside the stand.
fn text_fixture() -> Element {
    rsx! {
        div { id: "fx-root",
            input { id: "fx-input", r#type: "text", aria_label: "Field" }
        }
    }
}

/// The shot of `fixture` after the driver typed `text` into its input.
fn typed_shot(fixture: fn() -> Element, text: &str, incremental: bool) -> Vec<u8> {
    let mut session = fixtures::hold("typed", fixture, incremental);
    assert!(
        act(&mut session, &type_into("fx-input", text)).is_some(),
        "the type runs"
    );
    shot(&mut session, &screenshot())
        .expect("the screenshot is accepted")
        .png
}

#[test]
fn a_typed_password_never_leaves_as_pixels() {
    assert!(SENTINELS[0].len() == SENTINELS[1].len());
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: fixture");
        let [first, second] = SENTINELS.map(|text| typed_shot(password_fixture, text, incremental));
        assert!(
            first == second,
            "{at}: two passwords of one length give the same PNG"
        );
        let [first, second] = SENTINELS.map(|text| typed_shot(text_fixture, text, incremental));
        assert!(first != second, "{at}: two texts give different PNGs");
    }
}

#[test]
fn a_shot_changes_nothing() {
    for incremental in [false, true] {
        for (index, focus) in [None, Some("crud-name")].into_iter().enumerate() {
            let at = format!("incremental={incremental}: Crud: case {index}");
            let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
            if let Some(id) = focus {
                assert!(
                    act(&mut session, &click(id)).is_some(),
                    "{at}: the click runs"
                );
            }
            let _ = session.harness_mut().base_mut().take_changed_nodes();
            let before = (session.harness().doc.snapshot(), focused(&session));
            assert!(
                focus.is_none() || before.1.0.len() == 1,
                "{at}: the input reads focused"
            );
            for call in [screenshot(), screenshot_of("crud-list")] {
                assert!(shot(&mut session, &call).is_some(), "{at}: the shot runs");
            }
            assert!(
                !session.harness().base().has_changes(),
                "{at}: a shot marks no change"
            );
            let after = (session.harness().doc.snapshot(), focused(&session));
            assert!(before.1 == after.1, "{at}: the focus reads as before");
            assert!(
                before.0.diff(&after.0).is_empty(),
                "{at}: the diff across the shots is empty"
            );
        }
    }
}
