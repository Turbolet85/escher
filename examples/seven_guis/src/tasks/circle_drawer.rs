use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::node::ComputedStyles;
use dioxus_native::prelude::*;
use dioxus_native::{CustomWidgetAttr, Widget};
use peniko::kurbo::{Affine, Circle as Disc, Rect, Stroke, Vec2};
use peniko::{Color, Fill};

use super::{use_graph_signal, writes};
use std::rc::Rc;
use std::sync::{Arc, Mutex, PoisonError};

#[derive(Clone, PartialEq)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

/// The width of the `.cd-canvas` border: a click is read against the canvas's border box, and
/// the widget paints in its content box.
const CANVAS_BORDER: f64 = 2.0;

/// The circles as the canvas's `data-circles` attribute carries them: `x,y,d` per circle, its
/// centre in the canvas's content box and its diameter, `;` between circles.
fn circles_text(circles: &[Circle]) -> String {
    circles
        .iter()
        .map(|c| format!("{},{},{}", c.x, c.y, c.r * 2.0))
        .collect::<Vec<_>>()
        .join(";")
}

/// One circle the canvas paints: its centre and diameter, in CSS px of its content box.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Painted {
    x: f64,
    y: f64,
    d: f64,
}

/// Reads a `data-circles` value: a triple of finite numbers per circle; a triple that is not
/// one is skipped, so malformed text reads as no circle.
fn parse_circles(text: &str) -> Vec<Painted> {
    text.split(';')
        .filter_map(|triple| {
            let mut parts = triple.split(',').map(|n| n.trim().parse::<f64>().ok());
            let (Some(Some(x)), Some(Some(y)), Some(Some(d)), None) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            else {
                return None;
            };
            [x, y, d]
                .iter()
                .all(|n| n.is_finite())
                .then_some(Painted { x, y, d })
        })
        .collect()
}

type Fonts = Arc<Mutex<parley::FontContext>>;

/// The drawing area: paints the app's circles and each one's diameter at its centre, the
/// diameter's glyphs shaped from the font context the document hands it.
#[derive(Default)]
struct CircleCanvas {
    circles: Vec<Painted>,
    fonts: Option<Fonts>,
    layouts: parley::LayoutContext<()>,
}

const FILL: Color = Color::from_rgba8(58, 92, 229, 38);
const STROKE: Color = Color::from_rgb8(0x3a, 0x5c, 0xe5);
const LABEL: Color = Color::from_rgb8(0x1a, 0x1a, 0x1a);
const LABEL_SIZE: f32 = 12.0;
const RING: f64 = 2.0;

impl CircleCanvas {
    /// Draws `text` centred on `centre`, `transform` taking CSS px to the scene.
    fn draw_label(
        &mut self,
        scene: &mut anyrender::Scene,
        fonts: &mut parley::FontContext,
        transform: Affine,
        centre: (f64, f64),
        text: &str,
    ) {
        let mut builder = self.layouts.ranged_builder(fonts, text, 1.0, true);
        builder.push_default(parley::StyleProperty::FontSize(LABEL_SIZE));
        builder.push_default(parley::GenericFamily::SansSerif);
        let mut layout = builder.build(text);
        layout.break_all_lines(None);
        let at = transform
            * Affine::translate((
                centre.0 - f64::from(layout.width()) / 2.0,
                centre.1 - f64::from(layout.height()) / 2.0,
            ));
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
                        &anyrender::Paint::from(LABEL),
                        1.0,
                        at,
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
    }
}

impl Widget for CircleCanvas {
    fn attribute_changed(&mut self, name: &str, _old: Option<&str>, new_value: Option<&str>) {
        if name == "data-circles" {
            self.circles = new_value.map(parse_circles).unwrap_or_default();
        }
    }

    fn fonts(&mut self, fonts: &Fonts) {
        self.fonts = Some(Arc::clone(fonts));
    }

    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        width: u32,
        height: u32,
        scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        let transform = Affine::scale(scale);
        scene.push_clip_layer(
            Fill::NonZero,
            Affine::IDENTITY,
            &Rect::new(0.0, 0.0, f64::from(width), f64::from(height)),
        );
        for c in &self.circles {
            let disc = Disc::new((c.x, c.y), c.d / 2.0);
            scene.fill(
                Fill::NonZero,
                transform,
                FILL,
                None,
                &disc,
                AntiAliasing::Enabled,
            );
            let ring = Disc::new((c.x, c.y), (c.d / 2.0 - RING / 2.0).max(0.0));
            scene.stroke(
                &Stroke::new(RING),
                transform,
                STROKE,
                None,
                &ring,
                AntiAliasing::Enabled,
            );
        }
        if let Some(fonts) = self.fonts.clone() {
            let mut fonts = fonts.lock().unwrap_or_else(PoisonError::into_inner);
            for c in self.circles.clone() {
                let text = format!("{}", c.d.round());
                self.draw_label(&mut scene, &mut fonts, transform, (c.x, c.y), &text);
            }
        }
        scene.pop_layer();
        scene
    }
}

#[component]
pub fn CircleDrawer() -> Element {
    // The canvas's click is async, and its task runs inside the click's own dispatch.
    const HISTORY: &[dioxus_native::DeclaredWrite] = &[
        writes("circle-canvas", "click"),
        writes("circle-undo", "click"),
        writes("circle-redo", "click"),
        writes("circle-diameter", "input"),
    ];
    let mut circles: Signal<Vec<Circle>> = use_graph_signal("circles", HISTORY, Vec::new);
    // undo stack: each entry is a full snapshot of circles vec
    let mut undo_stack: Signal<Vec<Vec<Circle>>> =
        use_graph_signal("undo_stack", HISTORY, Vec::new);
    let mut redo_stack: Signal<Vec<Vec<Circle>>> =
        use_graph_signal("redo_stack", HISTORY, Vec::new);
    // index of circle being resized, if dialog is open
    let mut dialog_idx: Signal<Option<usize>> = use_graph_signal(
        "dialog_idx",
        const {
            &[
                writes("circle-canvas", "dblclick"),
                writes("circle-close", "click"),
            ]
        },
        || None,
    );
    let mut dialog_r: Signal<f64> = use_graph_signal(
        "dialog_r",
        const {
            &[
                writes("circle-canvas", "dblclick"),
                writes("circle-diameter", "input"),
            ]
        },
        || 20.0,
    );
    // Captured at mount; rect is read at click time because Blitz fires
    // `mounted` before layout, and `element_coordinates()` is unimplemented.
    let mut canvas_handle: Signal<Option<Rc<MountedData>>> =
        use_graph_signal("canvas_handle", &[], || None);
    let canvas = use_hook(|| CustomWidgetAttr::new(CircleCanvas::default()));

    rsx! {
        div { class: "cd-root",
            style { {CSS} }
            div { class: "cd-toolbar",
                button {
                    id: "circle-undo",
                    disabled: undo_stack().is_empty(),
                    onclick: move |_| {
                        if let Some(prev) = undo_stack.write().pop() {
                            redo_stack.write().push(circles());
                            circles.set(prev);
                        }
                    },
                    "Undo"
                }
                button {
                    id: "circle-redo",
                    disabled: redo_stack().is_empty(),
                    onclick: move |_| {
                        if let Some(next) = redo_stack.write().pop() {
                            undo_stack.write().push(circles());
                            circles.set(next);
                        }
                    },
                    "Redo"
                }
            }
            // Canvas area
            object {
                id: "circle-canvas",
                class: "cd-canvas",
                role: "group",
                aria_label: "Drawing area",
                "data": canvas,
                "data-circles": circles_text(&circles()),
                onmounted: move |evt: MountedEvent| {
                    canvas_handle.set(Some(evt.data()));
                },
                onclick: move |evt| async move {
                    let raw = evt.page_coordinates();
                    let (ox, oy) = match canvas_handle() {
                        Some(h) => h.get_client_rect().await
                            .map(|r| (r.origin.x, r.origin.y))
                            .unwrap_or((0.0, 0.0)),
                        None => (0.0, 0.0),
                    };
                    let x = raw.x - ox - CANVAS_BORDER;
                    let y = raw.y - oy - CANVAS_BORDER;
                    undo_stack.write().push(circles());
                    redo_stack.write().clear();
                    circles.write().push(Circle { x, y, r: 20.0 });
                },
                ondoubleclick: move |evt| async move {
                    evt.stop_propagation();
                    let raw = evt.page_coordinates();
                    let (ox, oy) = match canvas_handle() {
                        Some(h) => h.get_client_rect().await
                            .map(|r| (r.origin.x, r.origin.y))
                            .unwrap_or((0.0, 0.0)),
                        None => (0.0, 0.0),
                    };
                    let cx = raw.x - ox - CANVAS_BORDER;
                    let cy = raw.y - oy - CANVAS_BORDER;
                    let nearest = circles().iter().enumerate().min_by(|(_, a), (_, b)| {
                        let da = (a.x - cx).powi(2) + (a.y - cy).powi(2);
                        let db = (b.x - cx).powi(2) + (b.y - cy).powi(2);
                        da.total_cmp(&db)
                    }).map(|(i, c)| (i, c.r));
                    if let Some((i, r)) = nearest {
                        dialog_idx.set(Some(i));
                        dialog_r.set(r);
                    }
                },
            }
            // Diameter adjustment dialog
            if let Some(idx) = dialog_idx() {
                div { class: "cd-dialog",
                    p { "Adjust diameter of circle #{idx}" }
                    input {
                        id: "circle-diameter",
                        r#type: "range",
                        min: "5",
                        max: "100",
                        value: "{dialog_r() * 2.0}",
                        oninput: move |evt| {
                            if let Ok(d) = evt.value().parse::<f64>() {
                                let d = d.clamp(5.0, 100.0);
                                let new_r = d / 2.0;
                                dialog_r.set(new_r);
                                undo_stack.write().push(circles());
                                redo_stack.write().clear();
                                if let Some(c) = circles.write().get_mut(idx) {
                                    c.r = new_r;
                                }
                            }
                        }
                    }
                    p { "Diameter: {(dialog_r() * 2.0) as u32}px" }
                    button {
                        id: "circle-close",
                        onclick: move |_| dialog_idx.set(None),
                        "Close"
                    }
                }
            }
        }
    }
}

const CSS: &str = r#"
.cd-root {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    height: 100%;
    background-color: #f5f5f5;
    font-family: sans-serif;
    padding: 16px;
    box-sizing: border-box;
}

.cd-toolbar {
    display: flex;
    flex-direction: row;
    gap: 12px;
    margin-bottom: 12px;
}

.cd-toolbar button {
    font-size: 14px;
    font-weight: 500;
    color: #ffffff;
    background-color: #3a5ce5;
    border: none;
    border-radius: 6px;
    padding: 8px 20px;
    cursor: pointer;
}

.cd-toolbar button:hover {
    background-color: #2a4cd3;
}

.cd-toolbar button:disabled {
    background-color: #a0a0a0;
    cursor: default;
}

.cd-canvas {
    display: block;
    position: relative;
    width: 700px;
    height: 400px;
    background-color: #ffffff;
    border: 2px solid #c0c0c0;
    border-radius: 4px;
    overflow: hidden;
    cursor: crosshair;
}

.cd-dialog {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    margin-top: 16px;
    background: #ffffff;
    border: 1px solid #d0d0d0;
    border-radius: 8px;
    padding: 20px 32px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.10);
    min-width: 320px;
}

.cd-dialog p {
    margin: 0;
    font-size: 14px;
    color: #333333;
}

.cd-dialog input[type="range"] {
    width: 240px;
}

.cd-dialog button {
    font-size: 14px;
    font-weight: 500;
    color: #ffffff;
    background-color: #3a5ce5;
    border: none;
    border-radius: 6px;
    padding: 8px 20px;
    cursor: pointer;
}

.cd-dialog button:hover {
    background-color: #2a4cd3;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_circle_text_reads_as_no_circle() {
        let rows: [(&str, usize); 6] = [
            ("", 0),
            ("x", 0),
            ("1,2", 0),
            ("1,2,NaN", 0),
            ("1,2,inf", 0),
            ("10,20,40;bad;30,40,50", 2),
        ];
        for (index, (text, read)) in rows.into_iter().enumerate() {
            let mut canvas = CircleCanvas::default();
            canvas.attribute_changed("data-circles", None, Some(text));
            assert!(canvas.circles.len() == read, "row {index}");
        }
    }
}
