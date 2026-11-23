//! Text a widget paints from Rust is shaped from the document's own fonts. blitz-paint hands
//! each widget the document's font context through `Widget::fonts` before it paints, so a widget
//! shaping «Hg» through parley from that context draws the stand's bundled DejaVu Sans — the same
//! frame, byte for byte, as a widget shaping the same text from its own context of that one font.
//! Read on a component booted alone through the stand, in both layout modes. The comparison is
//! between two widgets' shots on one host, never a pixel count, so no host's rasterization is
//! pinned. A failure message carries a layout mode only.

use std::sync::{Arc, Mutex};

use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::node::ComputedStyles;
use blitz_dom::{Document as _, Widget};
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use escher_driver::Session;
use peniko::kurbo::{Affine, Vec2};
use peniko::{Color, Fill};
use seven_guis::stand;

/// The id of the element the widget paints.
const CANVAS: &str = "text-canvas";

/// The canvas's box past the body's margin: (8, 8)–(208, 108).
const LEFT: u32 = 8;
const TOP: u32 = 8;
const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

/// The text the widgets draw and its size in pixels.
const TEXT: &str = "Hg";
const FONT_SIZE: f32 = 40.0;

type Fonts = Arc<Mutex<parley::FontContext>>;

/// Shape [`TEXT`] from `fonts` and draw its glyph runs in black at the scene's origin.
fn draw_text(
    fonts: &mut parley::FontContext,
    layouts: &mut parley::LayoutContext<()>,
) -> anyrender::Scene {
    let mut scene = anyrender::Scene::new();
    let mut builder = layouts.ranged_builder(fonts, TEXT, 1.0, true);
    builder.push_default(parley::StyleProperty::FontSize(FONT_SIZE));
    builder.push_default(parley::GenericFamily::SansSerif);
    let mut layout = builder.build(TEXT);
    layout.break_all_lines(None);
    for line in layout.lines() {
        for item in line.items() {
            if let parley::PositionedLayoutItem::GlyphRun(glyphs) = item {
                let run = glyphs.run();
                scene.draw_glyphs(
                    run.font(),
                    run.font_size(),
                    true,
                    &[],
                    Vec2::default(),
                    Fill::NonZero,
                    &anyrender::Paint::from(Color::from_rgb8(0, 0, 0)),
                    1.0,
                    Affine::IDENTITY,
                    None,
                    glyphs.positioned_glyphs().map(|g| anyrender::Glyph {
                        id: g.id as _,
                        x: g.x,
                        y: g.y,
                    }),
                    AntiAliasing::Enabled,
                );
            }
        }
    }
    scene
}

/// A widget shaping its text from the font context it was handed, and keeping that context for
/// the test to read; handed none, it draws nothing.
struct DocumentText {
    handed: Arc<Mutex<Option<Fonts>>>,
    layouts: parley::LayoutContext<()>,
}

impl Widget for DocumentText {
    fn fonts(&mut self, fonts: &Fonts) {
        *self.handed.lock().expect("the cell is not poisoned") = Some(Arc::clone(fonts));
    }

    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        _width: u32,
        _height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        let handed = self
            .handed
            .lock()
            .expect("the cell is not poisoned")
            .clone();
        let Some(fonts) = handed else {
            return anyrender::Scene::new();
        };
        let mut fonts = fonts.lock().expect("the font context is not poisoned");
        draw_text(&mut fonts, &mut self.layouts)
    }
}

/// A widget shaping its text from a context of its own holding the stand's bundled font alone.
struct BundledText {
    fonts: parley::FontContext,
    layouts: parley::LayoutContext<()>,
}

impl Widget for BundledText {
    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        _width: u32,
        _height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        draw_text(&mut self.fonts, &mut self.layouts)
    }
}

#[derive(Props, Clone)]
struct DocumentProps {
    handed: Arc<Mutex<Option<Fonts>>>,
}

impl PartialEq for DocumentProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.handed, &other.handed)
    }
}

/// One `<object>` carrying the widget that shapes from the document's fonts.
fn document_canvas(props: DocumentProps) -> Element {
    let handed = props.handed.clone();
    let attr = use_hook(move || {
        CustomWidgetAttr::new(DocumentText {
            handed,
            layouts: parley::LayoutContext::new(),
        })
    });
    rsx! {
        object { id: CANVAS, style: "display: block; width: 200px; height: 100px;", "data": attr }
    }
}

/// One `<object>` carrying the widget that shapes from its own bundled font.
#[component]
fn BundledCanvas() -> Element {
    let attr = use_hook(|| {
        CustomWidgetAttr::new(BundledText {
            fonts: blitz_dom::build_single_font_ctx(seven_guis::DEJAVU_SANS),
            layouts: parley::LayoutContext::new(),
        })
    });
    rsx! {
        object { id: CANVAS, style: "display: block; width: 200px; height: 100px;", "data": attr }
    }
}

/// The canvas's region of a shot, row by row.
fn canvas_region(rgba: &[u8], width: u32) -> Vec<u8> {
    let mut region = Vec::with_capacity((WIDTH * HEIGHT * 4) as usize);
    for y in TOP..TOP + HEIGHT {
        let start = ((y * width + LEFT) * 4) as usize;
        region.extend_from_slice(&rgba[start..start + (WIDTH * 4) as usize]);
    }
    region
}

#[test]
fn canvas_text_is_shaped_from_the_documents_own_fonts() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");

        let handed: Arc<Mutex<Option<Fonts>>> = Arc::new(Mutex::new(None));
        let given = handed.clone();
        let mut document = Session::start("canvas", move || {
            stand::boot_alone(
                document_canvas,
                move || DocumentProps {
                    handed: given.clone(),
                },
                stand::options(incremental),
            )
        })
        .expect("canvas is a label");
        let shot = document
            .harness_mut()
            .screenshot()
            .expect("the stand paints");
        let fonts = handed.lock().expect("the cell is not poisoned").clone();
        let Some(fonts) = fonts else {
            panic!("{at}: the widget is handed a font context before it paints");
        };
        assert!(
            Arc::ptr_eq(&fonts, &document.harness().doc.inner().font_ctx()),
            "{at}: the context the widget is handed is the document's own"
        );
        let from_document = canvas_region(&shot.rgba, shot.width);
        assert!(
            from_document
                .chunks(4)
                .any(|px| px[0] <= 128 && px[1] <= 128 && px[2] <= 128),
            "{at}: the canvas holds a dark pixel of the text"
        );

        let mut bundled = Session::start("canvas", move || {
            stand::boot_alone(BundledCanvas, || (), stand::options(incremental))
        })
        .expect("canvas is a label");
        let shot = bundled
            .harness_mut()
            .screenshot()
            .expect("the stand paints");
        assert!(
            from_document == canvas_region(&shot.rgba, shot.width),
            "{at}: the canvas text is the bundled font the document's text is"
        );
    }
}
