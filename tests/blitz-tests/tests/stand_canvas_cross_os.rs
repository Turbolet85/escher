//! A canvas-only shot reads the same driver answer on every OS the tests run on. Three widgets
//! painted from Rust — a solid fill, an antialiased circle and stroke at fractional coordinates,
//! and glyphs shaped from the document's own font context — are each booted alone through the
//! stand and shot through the driver's `screenshot`, the whole screen and an element shot cropped
//! to the canvas, in both layout modes. The digest the driver keeps of each answer,
//! `answer_digest`, the PNG inside it, is held equal to one committed table, so a run on linux,
//! windows and macOS reading the one table is the cross-OS reading. A shot holding page text is
//! outside the table: each fixture is first read to paint a canvas alone. A second table holds
//! the stand's Circle Drawer: its canvas's element shot after one centre click, a circle and its
//! diameter's glyphs, while its whole screen holds page text and is never shot here. A failure
//! names the fixture, the form and the layout mode only.

use std::sync::{Arc, Mutex};

use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::Widget;
use blitz_dom::node::ComputedStyles;
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use escher_driver::{Call, Cause, Outcome, Session, answer_digest};
use peniko::kurbo::{Affine, Circle, Line, Rect, Stroke, Vec2};
use peniko::{Color, Fill};
use seven_guis::stand;

mod session_common;
use session_common::{act, click, lint, linted, screenshot, screenshot_of};

/// One reading: the fixture, the form of the shot (`whole` or `element`), the layout mode and
/// the digest of the answer.
type Row = (&'static str, &'static str, bool, u64);

/// How many readings the table holds: three fixtures, two forms, two layout modes.
const ROWS: usize = 12;

/// Every reading of the three fixtures.
const TABLE: [Row; ROWS] = [
    ("fill", "whole", false, 0x557c4c668471d182),
    ("fill", "element", false, 0xcf444a7cbab12f83),
    ("fill", "whole", true, 0x557c4c668471d182),
    ("fill", "element", true, 0xcf444a7cbab12f83),
    ("edge", "whole", false, 0x31410c265f852b74),
    ("edge", "element", false, 0x5a7f3f38c6d5cd8b),
    ("edge", "whole", true, 0x31410c265f852b74),
    ("edge", "element", true, 0x5a7f3f38c6d5cd8b),
    ("text", "whole", false, 0xb2a7df659708ebd9),
    ("text", "element", false, 0xe9e590af56fa6500),
    ("text", "whole", true, 0xb2a7df659708ebd9),
    ("text", "element", true, 0xe9e590af56fa6500),
];

/// How many readings the sample's table holds: one element shot, two layout modes.
const SAMPLE_ROWS: usize = 2;

/// Circle Drawer's canvas element shot after one click at its centre: a circle and its
/// diameter painted by the widget.
const SAMPLE: [Row; SAMPLE_ROWS] = [
    ("circle-drawer", "element", false, 0x5d5664142f9a5201),
    ("circle-drawer", "element", true, 0x5d5664142f9a5201),
];

const FIXTURES: [&str; 3] = ["fill", "edge", "text"];

const FORMS: [&str; 2] = ["whole", "element"];

/// The box every fixture's canvas stands in.
const BOX: &str = "display: block; width: 200px; height: 100px;";

/// A widget that fills its whole box with one colour.
struct SolidFill;

impl Widget for SolidFill {
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

/// A widget painting an antialiased circle at a fractional centre and a stroked diagonal.
struct Edges;

impl Widget for Edges {
    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        _width: u32,
        _height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgb8(40, 90, 200),
            None,
            &Circle::new((100.4, 50.2), 37.3),
            AntiAliasing::Enabled,
        );
        scene.stroke(
            &Stroke::new(1.5),
            Affine::IDENTITY,
            Color::from_rgb8(0, 0, 0),
            None,
            &Line::new((3.3, 96.7), (196.1, 4.9)),
            AntiAliasing::Enabled,
        );
        scene
    }
}

type Fonts = Arc<Mutex<parley::FontContext>>;

/// A widget shaping «Hg» from the font context the document hands it.
struct DocumentGlyphs {
    fonts: Option<Fonts>,
    layouts: parley::LayoutContext<()>,
}

impl Widget for DocumentGlyphs {
    fn fonts(&mut self, fonts: &Fonts) {
        self.fonts = Some(Arc::clone(fonts));
    }

    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        _width: u32,
        _height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        let Some(fonts) = self.fonts.clone() else {
            return scene;
        };
        let mut fonts = fonts.lock().expect("the font context is not poisoned");
        let mut builder = self.layouts.ranged_builder(&mut fonts, "Hg", 1.0, true);
        builder.push_default(parley::StyleProperty::FontSize(40.0));
        builder.push_default(parley::GenericFamily::SansSerif);
        let mut layout = builder.build("Hg");
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
}

#[component]
fn FillCanvas() -> Element {
    let attr = use_hook(|| CustomWidgetAttr::new(SolidFill));
    rsx! {
        object { id: "canvas-fill", style: BOX, "data": attr }
    }
}

#[component]
fn EdgeCanvas() -> Element {
    let attr = use_hook(|| CustomWidgetAttr::new(Edges));
    rsx! {
        object { id: "canvas-edge", style: BOX, "data": attr }
    }
}

#[component]
fn TextCanvas() -> Element {
    let attr = use_hook(|| {
        CustomWidgetAttr::new(DocumentGlyphs {
            fonts: None,
            layouts: parley::LayoutContext::new(),
        })
    });
    rsx! {
        object { id: "canvas-text", style: BOX, "data": attr }
    }
}

/// The id of the canvas `fixture` paints.
fn canvas_id(fixture: &str) -> &'static str {
    match fixture {
        "fill" => "canvas-fill",
        "edge" => "canvas-edge",
        "text" => "canvas-text",
        _ => panic!("a fixture's name"),
    }
}

/// A session on `fixture`, booted alone through the stand.
fn session_on(fixture: &str, incremental: bool) -> Session {
    let options = stand::options(incremental);
    match fixture {
        "fill" => Session::start("canvas", || stand::boot_alone(FillCanvas, || (), options)),
        "edge" => Session::start("canvas", || stand::boot_alone(EdgeCanvas, || (), options)),
        "text" => Session::start("canvas", || stand::boot_alone(TextCanvas, || (), options)),
        _ => panic!("a fixture's name"),
    }
    .expect("canvas is a label")
}

/// The driver call that shoots `fixture` in `form`.
fn shot_call(fixture: &str, form: &str) -> Call {
    match form {
        "whole" => screenshot(),
        _ => screenshot_of(canvas_id(fixture)),
    }
}

/// Shoots `fixture` in each form and each layout mode and holds every answer's digest to the
/// table's row.
fn reads_its_table(fixture: &str) {
    for incremental in [false, true] {
        let mut session = session_on(fixture, incremental);
        for form in FORMS {
            let at = format!("{fixture} {form} incremental={incremental}");
            let Some(row) = TABLE
                .iter()
                .find(|row| row.0 == fixture && row.1 == form && row.2 == incremental)
            else {
                panic!("{at}: the table holds the reading");
            };
            let answer = session.run(&shot_call(fixture, form));
            assert!(
                matches!(answer, Ok(Outcome::Shot { .. })),
                "{at}: the shot is accepted"
            );
            assert!(
                answer_digest(&answer) == row.3,
                "{at}: the reading is the table's"
            );
        }
    }
}

#[test]
fn the_table_counts_its_rows_and_covers_every_fixture() {
    assert!(TABLE.len() == ROWS, "the table holds its stated row count");
    for fixture in FIXTURES {
        for form in FORMS {
            for incremental in [false, true] {
                let held = TABLE
                    .iter()
                    .filter(|row| row.0 == fixture && row.1 == form && row.2 == incremental)
                    .count();
                assert!(
                    held == 1,
                    "{fixture} {form} incremental={incremental}: the table holds one reading"
                );
            }
        }
    }
}

#[test]
fn each_fixture_paints_a_canvas_alone() {
    for fixture in FIXTURES {
        for incremental in [false, true] {
            let at = format!("{fixture} incremental={incremental}");
            let mut session = session_on(fixture, incremental);
            assert!(
                session.run(&lint()).err().map(|refusal| refusal.cause())
                    != Some(Cause::FontsUnavailable),
                "{at}: the held app can paint text"
            );
            let screen = session.harness().doc.snapshot();
            let Some(node) = screen.get(canvas_id(fixture)) else {
                panic!("{at}: the canvas is on the screen");
            };
            let b = node.bounds;
            assert!(
                (b.x, b.y, b.width, b.height) == (8.0, 8.0, 200.0, 100.0),
                "{at}: the canvas stands at its measured bounds"
            );
            assert!(
                screen.nodes().all(|node| node.name.is_empty()),
                "{at}: no node on the screen holds a name"
            );
            assert!(
                linted(&mut session, &lint()).is_some_and(|findings| findings.is_empty()),
                "{at}: the lint reads no finding"
            );
        }
    }
}

#[test]
fn a_solid_fill_reads_its_table() {
    reads_its_table("fill");
}

#[test]
fn an_antialiased_edge_reads_its_table() {
    reads_its_table("edge");
}

#[test]
fn canvas_glyphs_read_their_table() {
    reads_its_table("text");
}

#[test]
fn circle_drawers_canvas_reads_its_digest() {
    assert!(
        SAMPLE.len() == SAMPLE_ROWS,
        "the sample's table holds its stated row count"
    );
    for incremental in [false, true] {
        let at = format!("circle-drawer element incremental={incremental}");
        let Some(row) = SAMPLE.iter().find(|row| row.2 == incremental) else {
            panic!("{at}: the table holds the reading");
        };
        let mut session = Session::start("circle-drawer", move || {
            stand::boot_circle_drawer(stand::options(incremental))
        })
        .expect("circle-drawer is a label");
        assert!(
            act(&mut session, &click("circle-canvas")).is_some(),
            "{at}: the click is accepted"
        );
        let answer = session.run(&screenshot_of("circle-canvas"));
        assert!(
            matches!(answer, Ok(Outcome::Shot { .. })),
            "{at}: the shot is accepted"
        );
        assert!(
            answer_digest(&answer) == row.3,
            "{at}: the reading is the table's"
        );
    }
}
