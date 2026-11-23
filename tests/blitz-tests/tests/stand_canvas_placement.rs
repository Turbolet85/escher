//! A widget painted from Rust is placed at its content box. blitz-dom's `Widget::paint` is handed
//! the element's content-box size, and the scene it paints lands at the content-box origin inside
//! the element's own transform — a padded, bordered `<object>` paints its content box alone, the
//! same box under `scale(2)` paints where the transform carries it, and a widget with no padding
//! or border paints its whole box as before — read in a harness shot of a component booted alone
//! through the stand, in both layout modes. Guards upstream's
//! <https://github.com/DioxusLabs/blitz/issues/1578>: the paint was handed the border-box size and
//! its scene was appended at the content origin in output axes, outside the element's transform.
//! A failure message carries a layout mode, a pixel's coordinates and what it read only.

use std::sync::{Arc, Mutex};

use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::Widget;
use blitz_dom::node::ComputedStyles;
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use escher_driver::Session;
use peniko::kurbo::{Affine, Rect};
use peniko::{Color, Fill};
use seven_guis::stand;

/// The id of the element the widget paints.
const CANVAS: &str = "placed-canvas";

/// The widget's painted colour as a shot reads it, measured.
const RED: [u8; 4] = [255, 0, 0, 255];

/// The colour a shot composites over, measured.
const WHITE: [u8; 4] = [255, 255, 255, 255];

/// A content-box object of 100 × 60 with 12 px of padding and a 3 px border: its border box
/// stands at (8, 8)–(138, 98) past the body's margin, its content box at (23, 23)–(123, 83).
const PADDED: &str = "display: block; box-sizing: content-box; width: 100px; height: 60px; \
                      padding: 12px; border: 3px solid black;";

/// The padded object scaled by 2 about its top left corner: its content box lands at
/// (38, 38)–(238, 158).
const SCALED: &str = "display: block; box-sizing: content-box; width: 100px; height: 60px; \
                      padding: 12px; border: 3px solid black; transform-origin: 0 0; \
                      transform: scale(2);";

/// A 200 × 100 object with no padding or border, at (8, 8)–(208, 108).
const PLAIN: &str = "display: block; width: 200px; height: 100px;";

/// The size the widget's last paint was handed.
type Handed = Arc<Mutex<Option<(u32, u32)>>>;

/// A widget that fills the size it is handed with one colour and records that size.
struct ContentFill(Handed);

impl Widget for ContentFill {
    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        width: u32,
        height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        *self.0.lock().expect("the cell is not poisoned") = Some((width, height));
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

#[derive(Props, Clone)]
struct CanvasProps {
    style: &'static str,
    handed: Handed,
}

impl PartialEq for CanvasProps {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style && Arc::ptr_eq(&self.handed, &other.handed)
    }
}

/// One `<object>` carrying the widget, styled as the case gives.
fn placed_canvas(props: CanvasProps) -> Element {
    let handed = props.handed.clone();
    let attr = use_hook(move || CustomWidgetAttr::new(ContentFill(handed)));
    rsx! {
        object { id: CANVAS, style: props.style, "data": attr }
    }
}

/// A session on one styled canvas, booted alone through the stand, and the cell its widget
/// records into.
fn canvas_session(style: &'static str, incremental: bool) -> (Session, Handed) {
    let handed: Handed = Arc::new(Mutex::new(None));
    let given = handed.clone();
    let session = Session::start("canvas", move || {
        stand::boot_alone(
            placed_canvas,
            move || CanvasProps {
                style,
                handed: given.clone(),
            },
            stand::options(incremental),
        )
    })
    .expect("canvas is a label");
    (session, handed)
}

fn pixel(rgba: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
    let at = ((y * width + x) * 4) as usize;
    [rgba[at], rgba[at + 1], rgba[at + 2], rgba[at + 3]]
}

/// Shoot the session, check the size its widget's paint was handed, then each pixel against the
/// colour it must read.
#[track_caller]
fn assert_paint(
    session: &mut Session,
    handed: &Handed,
    at: &str,
    size: (u32, u32),
    expected: &[((u32, u32), [u8; 4])],
) {
    let raster = session
        .harness_mut()
        .screenshot()
        .expect("the stand paints");
    let read = *handed.lock().expect("the cell is not poisoned");
    assert!(
        read == Some(size),
        "{at}: the paint is handed {size:?}, read {read:?}"
    );
    for &((x, y), colour) in expected {
        let read = pixel(&raster.rgba, raster.width, x, y);
        assert!(
            read == colour,
            "{at}: ({x}, {y}) reads {colour:?}, read {read:?}"
        );
    }
}

#[test]
fn a_padded_widget_paints_its_content_box() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let (mut session, handed) = canvas_session(PADDED, incremental);

        let screen = session.harness().doc.snapshot();
        let Some(node) = screen.get(CANVAS) else {
            panic!("{at}: the canvas is on the screen");
        };
        let b = node.bounds;
        assert!(
            (b.x, b.y, b.width, b.height) == (8.0, 8.0, 130.0, 90.0),
            "{at}: the canvas's border box stands at its measured bounds"
        );

        assert_paint(
            &mut session,
            &handed,
            &at,
            (100, 60),
            &[
                ((50, 50), RED),
                ((120, 80), RED),
                ((125, 50), WHITE),
                ((145, 50), WHITE),
                ((50, 105), WHITE),
            ],
        );
    }
}

#[test]
fn a_transformed_widget_paints_inside_its_transform() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let (mut session, handed) = canvas_session(SCALED, incremental);
        assert_paint(
            &mut session,
            &handed,
            &at,
            (100, 60),
            &[
                ((40, 40), RED),
                ((236, 100), RED),
                ((26, 26), WHITE),
                ((34, 34), WHITE),
                ((245, 100), WHITE),
                ((255, 100), WHITE),
            ],
        );
    }
}

#[test]
fn a_widget_with_no_padding_paints_its_whole_box() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let (mut session, handed) = canvas_session(PLAIN, incremental);
        assert_paint(
            &mut session,
            &handed,
            &at,
            (200, 100),
            &[((58, 58), RED), ((207, 107), RED), ((209, 109), WHITE)],
        );
    }
}
