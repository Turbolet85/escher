//! The visual lint reads inside a canvas: a label an app's widget paints from Rust whose ink box
//! — its glyphs' outline bounds — runs past its region reads `truncated` on the canvas element,
//! once per label, `text` the ink box and `visible` its part inside the region: the content box,
//! cut by every clip the widget pushed around the run and by the element's clip region. A
//! planted page of six numbered canvases reads one literal table of findings in both layout
//! modes; the `lint` verb answers them keyed from its closed set, holding no glyph id, glyph
//! position or text; and Circle Drawer reads no finding at boot, after a click and at 12 and 100
//! circles, its glyph runs counted first so a clean reading is never vacuous. A failure names a
//! layout mode, a state or a row index only.

use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::Widget;
use blitz_dom::node::ComputedStyles;
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use escher_driver::{Bounds, CanvasCommand, Defect, Finding, Outcome, Reason, Session};
use peniko::kurbo::{Affine, Rect, Vec2};
use peniko::{Color, Fill};
use seven_guis::stand;

mod session_common;
use session_common::{CIRCLE_CANVAS, canvas, canvased, lint, reach};

/// The text every planted label draws and its size in pixels.
const TEXT: &str = "Wide label";
const FONT_SIZE: f32 = 20.0;

/// Where a planted widget pushes a clip: none, around its label, or pushed and popped before it.
#[derive(Clone, Copy, PartialEq)]
enum Clip {
    None,
    Around(f64),
    Before(f64),
}

/// One planted canvas: its id, its label's left edge and baseline in the content box, its clip,
/// and whether it sits inside a 100 px wide box that clips by `overflow`.
type Case = (&'static str, f64, f64, Clip, bool);

const CASES: [Case; 6] = [
    // 1. The label wholly inside the content box: no finding.
    ("fx-inside", 10.0, 30.0, Clip::None, false),
    // 2. The label starting at x 150 runs past the content box's right edge: truncated.
    ("fx-past-edge", 150.0, 30.0, Clip::None, false),
    // 3. A clip 100 px wide around the label at x 10, which runs past it inside the content
    //    box: truncated.
    ("fx-past-clip", 10.0, 30.0, Clip::Around(100.0), false),
    // 4. A clip 50 px wide pushed and popped before the label at x 10: no finding.
    ("fx-after-pop", 10.0, 30.0, Clip::Before(50.0), false),
    // 5. The baseline at y 10: every glyph origin inside the content box, the ascent past its
    //    top edge: truncated, read by the ink box alone.
    ("fx-ascent", 10.0, 10.0, Clip::None, false),
    // 6. Inside a 100 px wide box that clips, the label at x 90 inside the content box but
    //    running past the clipping box: clipped, then truncated. Its ink is 102 px wide, so a
    //    label starting past the clipping box would run past the content box as well.
    ("fx-in-clipped-box", 90.0, 30.0, Clip::None, true),
];

/// A widget shaping [`TEXT`] from a context of the bundled DejaVu Sans alone and drawing it at
/// its case's place, glyph origins on whole device px.
struct Labelled {
    fonts: parley::FontContext,
    layouts: parley::LayoutContext<()>,
    left: f64,
    baseline: f64,
    clip: Clip,
}

impl Labelled {
    fn draw(&mut self, scene: &mut anyrender::Scene, scale: f64) {
        let mut builder = self
            .layouts
            .ranged_builder(&mut self.fonts, TEXT, 1.0, true);
        builder.push_default(parley::StyleProperty::FontSize(FONT_SIZE));
        builder.push_default(parley::GenericFamily::SansSerif);
        let mut layout = builder.build(TEXT);
        layout.break_all_lines(None);
        for line in layout.lines() {
            for item in line.items() {
                if let parley::PositionedLayoutItem::GlyphRun(glyphs) = item {
                    let run = glyphs.run();
                    let baseline = glyphs.baseline();
                    scene.draw_glyphs(
                        run.font(),
                        run.font_size() * scale as f32,
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
                            x: (scale * (self.left + f64::from(g.x))).round() as f32,
                            y: (scale * (self.baseline + f64::from(g.y - baseline))).round() as f32,
                        }),
                        AntiAliasing::Enabled,
                    );
                }
            }
        }
    }
}

impl Widget for Labelled {
    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        _width: u32,
        height: u32,
        scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        let clip = |width: f64| Rect::new(0.0, 0.0, scale * width, f64::from(height));
        match self.clip {
            Clip::None => self.draw(&mut scene, scale),
            Clip::Around(width) => {
                scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &clip(width));
                self.draw(&mut scene, scale);
                scene.pop_layer();
            }
            Clip::Before(width) => {
                scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &clip(width));
                scene.pop_layer();
                self.draw(&mut scene, scale);
            }
        }
        scene
    }
}

const CANVAS_STYLE: &str = "display: block; width: 200px; height: 40px;";

/// One planted canvas, the case at `index`.
#[component]
fn PlantedCanvas(index: usize) -> Element {
    let (id, left, baseline, clip, in_box) = CASES[index];
    let attr = use_hook(move || {
        CustomWidgetAttr::new(Labelled {
            fonts: blitz_dom::build_single_font_ctx(seven_guis::DEJAVU_SANS),
            layouts: parley::LayoutContext::new(),
            left,
            baseline,
            clip,
        })
    });
    let canvas = rsx! {
        object {
            id,
            role: "group",
            aria_label: "Planted label",
            style: CANVAS_STYLE,
            "data": attr,
        }
    };
    if in_box {
        rsx! {
            div { style: "width: 100px; overflow: hidden;", {canvas} }
        }
    } else {
        canvas
    }
}

/// The planted page: the six canvases stacked.
#[component]
fn PlantedPage() -> Element {
    rsx! {
        for index in 0..CASES.len() {
            PlantedCanvas { key: "{index}", index }
        }
    }
}

fn planted(incremental: bool) -> Session {
    Session::start("planted", move || {
        stand::boot_alone(PlantedPage, || (), stand::options(incremental))
    })
    .expect("planted is a label")
}

/// The glyph runs a canvas's reading holds, each by its origins box.
fn runs(session: &mut Session, id: &str, at: &str) -> Vec<Option<[f64; 4]>> {
    let reading = session
        .canvas(id)
        .unwrap_or_else(|_| panic!("{at}: the canvas reads"));
    reading
        .commands
        .iter()
        .filter_map(|command| match command {
            CanvasCommand::Glyphs { origins, .. } => Some(*origins),
            _ => None,
        })
        .collect()
}

/// One finding as the table holds it: the id, the defect, the element's bounds, then the
/// reason's boxes — a truncation's text and visible part, a clip's visible part — each as x, y,
/// width and height.
type Row = (&'static str, Defect, [f64; 4], [f64; 4], Option<[f64; 4]>);

/// How many findings the planted page reads: cases 2, 3 and 5 one each, case 6 two.
const ROWS: usize = 5;

/// The planted page's findings, measured in the engine.
const TABLE: [Row; ROWS] = [
    (
        "fx-past-edge",
        Defect::Truncated,
        [8.0, 48.0, 200.0, 40.0],
        [158.6640625, 62.8046875, 102.017578125, 15.478515625],
        Some([158.6640625, 62.8046875, 49.3359375, 15.478515625]),
    ),
    (
        "fx-past-clip",
        Defect::Truncated,
        [8.0, 88.0, 200.0, 40.0],
        [18.6640625, 102.8046875, 102.017578125, 15.478515625],
        Some([18.6640625, 102.8046875, 89.3359375, 15.478515625]),
    ),
    (
        "fx-ascent",
        Defect::Truncated,
        [8.0, 168.0, 200.0, 40.0],
        [18.6640625, 162.8046875, 102.017578125, 15.478515625],
        Some([18.6640625, 168.0, 102.017578125, 10.283203125]),
    ),
    (
        "fx-in-clipped-box",
        Defect::Clipped,
        [8.0, 208.0, 200.0, 40.0],
        [8.0, 208.0, 100.0, 40.0],
        None,
    ),
    (
        "fx-in-clipped-box",
        Defect::Truncated,
        [8.0, 208.0, 200.0, 40.0],
        [98.6640625, 222.8046875, 102.017578125, 15.478515625],
        Some([98.6640625, 222.8046875, 9.3359375, 15.478515625]),
    ),
];

/// The glyph runs Circle Drawer's canvas holds at each state, measured in the engine: one label
/// per circle.
const STAND_RUNS: [(&str, usize); 4] = [("boot", 0), ("click", 1), ("12", 12), ("100", 100)];

/// The keys a lint answer's line may hold: the answer's, a finding's and a truncation's or a
/// clip's reason's.
const KEYS: [&str; 6] = ["findings", "id", "defect", "bounds", "text", "visible"];

fn edges(bounds: Bounds) -> [f64; 4] {
    [bounds.x, bounds.y, bounds.width, bounds.height]
}

/// A finding as a row of the table; `None` for a reason the planted page never reads.
fn row(finding: &Finding) -> Option<(String, Defect, [f64; 4], [f64; 4], Option<[f64; 4]>)> {
    let (first, second) = match &finding.reason {
        Reason::Truncated { text, visible } => (edges(*text), visible.map(edges)),
        Reason::Clipped { visible } => (edges(*visible), None),
        _ => return None,
    };
    Some((
        finding.id.clone(),
        finding.defect,
        edges(finding.bounds),
        first,
        second,
    ))
}

/// The keys a JSON line holds, each read before its colon.
fn keys(line: &str) -> Vec<&str> {
    let pieces: Vec<&str> = line.split("\":").collect();
    pieces[..pieces.len().saturating_sub(1)]
        .iter()
        .map(|piece| piece.rsplit('"').next().unwrap_or(""))
        .collect()
}

#[test]
fn a_label_past_its_region_reads_truncated_on_the_planted_page() {
    assert!(TABLE.len() == ROWS, "the table holds its stated row count");
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = planted(incremental);
        let screen = session.harness().doc.snapshot();
        for (index, (id, ..)) in CASES.iter().enumerate() {
            let at = format!("{at} case {index}");
            assert!(
                screen.nodes().any(|node| node.id == *id),
                "{at}: the canvas is a snapshot node"
            );
            let held = runs(&mut session, id, &at);
            assert!(held.len() == 1, "{at}: the canvas holds one glyph run");
        }
        let ascent = runs(&mut session, "fx-ascent", &at);
        assert!(
            ascent[0].is_some_and(|[x0, y0, x1, y1]| x0 >= 0.0
                && y0 >= 0.0
                && x1 <= 200.0
                && y1 <= 40.0),
            "{at}: every glyph origin of the ascent case lies inside its content box"
        );

        let findings = session
            .lint()
            .unwrap_or_else(|_| panic!("{at}: the lint reads"));
        assert!(
            findings.len() == ROWS,
            "{at}: the page reads the table's row count"
        );
        for (index, finding) in findings.iter().enumerate() {
            let (id, defect, bounds, first, second) = TABLE[index];
            assert!(
                row(finding) == Some((id.to_string(), defect, bounds, first, second)),
                "{at} row {index}: the finding reads its row"
            );
        }
    }
}

#[test]
fn the_lint_verb_answers_the_canvas_findings() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = planted(incremental);
        let read = session
            .lint()
            .unwrap_or_else(|_| panic!("{at}: the lint reads"));
        let Ok(outcome @ Outcome::Linted { .. }) = session.run(&lint()) else {
            panic!("{at}: the verb answers a lint");
        };
        let Outcome::Linted { findings } = &outcome else {
            panic!("{at}: the verb answers a lint");
        };
        assert!(
            *findings == read,
            "{at}: the verb answers the reader's findings"
        );
        let line = outcome.to_json();
        assert!(
            line.matches("\"defect\":").count() == ROWS,
            "{at}: the line holds every finding"
        );
        let read_keys = keys(&line);
        assert!(
            read_keys.contains(&"text") && read_keys.iter().all(|key| KEYS.contains(key)),
            "{at}: the line holds keys of the closed set alone"
        );
        assert!(!line.contains("Wide"), "{at}: the line holds no label text");
    }
}

#[test]
fn circle_drawer_reads_no_canvas_finding_at_every_state() {
    for incremental in [false, true] {
        for (state, expected) in STAND_RUNS {
            let at = format!("incremental={incremental} {state}");
            let mut session = reach(state, incremental);
            let Some(Outcome::CanvasSummary { kinds, .. }) =
                canvased(&mut session, &canvas(CIRCLE_CANVAS))
            else {
                panic!("{at}: the summary reads");
            };
            assert!(
                kinds[6] == expected,
                "{at}: the canvas holds its glyph runs"
            );
            let findings = session
                .lint()
                .unwrap_or_else(|_| panic!("{at}: the lint reads"));
            assert!(findings.is_empty(), "{at}: the lint reads no finding");
        }
    }
}
