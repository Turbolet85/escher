//! What an app paints from Rust shows in a headless shot. A widget painted through blitz-dom's
//! `Widget` trait, handed to an `<object>` whose `data` carries a `CustomWidgetAttr` and booted
//! alone through the stand, reads its painted pixels in the harness's own shot and in the
//! driver's `screenshot`, whole and cropped to the element, in both layout modes and in every
//! build selection — a per-package build as much as the workspace's. The shots are taken in
//! process only. No check here skips, sleeps or reads a clock. A failure message carries a layout
//! mode, a row and the pixel it read only.

use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::Widget;
use blitz_dom::node::ComputedStyles;
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use escher_driver::Session;
use peniko::kurbo::{Affine, Rect};
use peniko::{Color, Fill};
use seven_guis::stand;

mod session_common;
use session_common::{screenshot, screenshot_of, shot};

/// The stand's pinned viewport, in pixels.
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

/// The id of the element the widget paints.
const CANVAS: &str = "painted-canvas";

/// The widget's painted colour as a shot reads it, measured.
const RED: [u8; 4] = [255, 0, 0, 255];

/// The colour a shot composites over, measured.
const WHITE: [u8; 4] = [255, 255, 255, 255];

/// A widget that fills its whole box with one colour.
struct RedFill;

impl Widget for RedFill {
    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        width: u32,
        height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgb8(255, 0, 0),
            None,
            &Rect::new(0.0, 0.0, f64::from(width), f64::from(height)),
            AntiAliasing::default(),
        );
        scene
    }
}

/// One `<object>` carrying the widget, with no background of its own.
#[component]
fn PaintedCanvas() -> Element {
    let attr = use_hook(|| CustomWidgetAttr::new(RedFill));
    rsx! {
        object { id: CANVAS, style: "display: block; width: 200px; height: 100px;", "data": attr }
    }
}

/// A session on the painted canvas, booted alone through the stand.
fn canvas_session(incremental: bool) -> Session {
    Session::start("canvas", || {
        stand::boot_alone(PaintedCanvas, || (), stand::options(incremental))
    })
    .expect("canvas is a label")
}

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

#[test]
fn a_widget_painted_from_rust_shows_in_a_harness_shot() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = canvas_session(incremental);

        let screen = session.harness().doc.snapshot();
        let Some(node) = screen.get(CANVAS) else {
            panic!("{at}: the canvas is on the screen");
        };
        let b = node.bounds;
        assert!(
            (b.x, b.y, b.width, b.height) == (8.0, 8.0, 200.0, 100.0),
            "{at}: the canvas stands at its measured bounds"
        );

        let raster = session
            .harness_mut()
            .screenshot()
            .expect("the stand paints");
        let inside = pixel(&raster.rgba, raster.width, 58, 58);
        assert!(
            inside == RED,
            "{at}: row 58 reads the widget's paint inside the canvas, read {inside:?}"
        );
        assert!(
            pixel(&raster.rgba, raster.width, 400, 400) == WHITE,
            "{at}: row 400 reads the page outside the canvas"
        );
    }
}

#[test]
fn a_driver_shot_carries_the_widget_whole_and_cropped() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = canvas_session(incremental);

        let Some(whole) = shot(&mut session, &screenshot()) else {
            panic!("{at}: the whole screenshot is accepted");
        };
        let (width, height, rgba) = decode(&whole.png);
        assert!(
            (whole.width, whole.height, width, height) == (WIDTH, HEIGHT, WIDTH, HEIGHT),
            "{at}: the whole shot is the viewport"
        );
        let raster = session
            .harness_mut()
            .screenshot()
            .expect("the stand paints");
        assert!(
            raster.rgba == rgba,
            "{at}: the whole shot decodes to the harness's own raster"
        );

        let Some(element) = shot(&mut session, &screenshot_of(CANVAS)) else {
            panic!("{at}: the element shot is accepted");
        };
        let (width, height, rgba) = decode(&element.png);
        assert!(
            (width, height) == (200, 100),
            "{at}: the element shot is the canvas's box"
        );
        for (index, px) in rgba.chunks(4).enumerate() {
            assert!(
                px == RED,
                "{at}: row {} reads the widget's paint in the element shot",
                index / 200
            );
        }
    }
}
