use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::node::ComputedStyles;
use blitz_dom::{BoundingRect, VirtualNodeDeclaration, VirtualNodeRole};
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
    /// The id the app gave the circle when a click added it, from 1, never reused.
    id: u64,
}

/// The width of the `.cd-canvas` border: a click is read against the canvas's border box, and
/// the widget paints in its content box.
const CANVAS_BORDER: f64 = 2.0;

/// The width of the `.cd-canvas` content box, in CSS px: its 700 px border box less the border
/// on either side. A circle's region is cut to it.
const CANVAS_WIDTH: f64 = 696.0;

/// The height of the `.cd-canvas` content box, in CSS px: its 400 px border box less the border.
const CANVAS_HEIGHT: f64 = 396.0;

/// The circles as the canvas's `data-circles` attribute carries them: `x,y,d,id` per circle, its
/// centre in the canvas's content box, its diameter and its id, `;` between circles.
fn circles_text(circles: &[Circle]) -> String {
    circles
        .iter()
        .map(|c| format!("{},{},{},{}", c.x, c.y, c.r * 2.0, c.id))
        .collect::<Vec<_>>()
        .join(";")
}

/// One circle the canvas paints: its centre and diameter, in CSS px of its content box, and its
/// id.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Painted {
    x: f64,
    y: f64,
    d: f64,
    id: u64,
}

/// Reads a `data-circles` value: per circle three finite numbers and an id, a whole number from
/// 1; an entry that is not one is skipped, so malformed text reads as no circle.
fn parse_circles(text: &str) -> Vec<Painted> {
    text.split(';')
        .filter_map(|entry| {
            let mut parts = entry.split(',').map(str::trim);
            let (Some(x), Some(y), Some(d), Some(id), None) = (
                parts.next(),
                parts.next(),
                parts.next(),
                parts.next(),
                parts.next(),
            ) else {
                return None;
            };
            let (Ok(x), Ok(y), Ok(d), Ok(id)) = (
                x.parse::<f64>(),
                y.parse::<f64>(),
                d.parse::<f64>(),
                id.parse::<u64>(),
            ) else {
                return None;
            };
            ([x, y, d].iter().all(|n| n.is_finite()) && id >= 1).then_some(Painted { x, y, d, id })
        })
        .collect()
}

/// Reads a `data-selected` or `data-adjusting` value: a circle's id, a whole number from 1, else
/// none.
fn parse_id(text: &str) -> Option<u64> {
    text.trim().parse::<u64>().ok().filter(|id| *id >= 1)
}

/// The region a circle's virtual node declares: its bounding square cut to the content box, or
/// none where the cut leaves it empty.
fn circle_region(c: &Painted) -> Option<BoundingRect> {
    let r = c.d / 2.0;
    let (left, top) = ((c.x - r).max(0.0), (c.y - r).max(0.0));
    let (right, bottom) = ((c.x + r).min(CANVAS_WIDTH), (c.y + r).min(CANVAS_HEIGHT));
    (right > left && bottom > top).then_some(BoundingRect {
        x: left,
        y: top,
        width: right - left,
        height: bottom - top,
    })
}

type Fonts = Arc<Mutex<parley::FontContext>>;

/// The drawing area: paints the app's circles and each one's diameter at its centre, the
/// diameter's glyphs shaped from the font context the document hands it, the selected circle
/// filled darker.
#[derive(Default)]
struct CircleCanvas {
    circles: Vec<Painted>,
    /// The selected circle's id, from `data-selected`.
    selected: Option<u64>,
    /// The id of the circle the open diameter dialog adjusts, from `data-adjusting`.
    adjusting: Option<u64>,
    fonts: Option<Fonts>,
    layouts: parley::LayoutContext<()>,
}

const FILL: Color = Color::from_rgba8(58, 92, 229, 38);
const SELECTED_FILL: Color = Color::from_rgba8(58, 92, 229, 96);
const STROKE: Color = Color::from_rgb8(0x3a, 0x5c, 0xe5);
const LABEL: Color = Color::from_rgb8(0x1a, 0x1a, 0x1a);
const LABEL_SIZE: f32 = 12.0;
const RING: f64 = 2.0;

impl CircleCanvas {
    /// Draws `text` centred on `centre`, in CSS px, `scale` device px to the CSS px.
    ///
    /// Each glyph origin is placed on a whole device pixel and drawn with no transform: the CPU
    /// renderer snaps glyph origins one way on Apple hosts (CoreGraphics' grid) and another
    /// elsewhere (Skia's), and only a whole-pixel origin lands on the same pixel under both, so
    /// the canvas's text paints the same bytes on every host.
    fn draw_label(
        &mut self,
        scene: &mut anyrender::Scene,
        fonts: &mut parley::FontContext,
        scale: f64,
        centre: (f64, f64),
        text: &str,
    ) {
        let mut builder = self.layouts.ranged_builder(fonts, text, 1.0, true);
        builder.push_default(parley::StyleProperty::FontSize(LABEL_SIZE));
        builder.push_default(parley::GenericFamily::SansSerif);
        let mut layout = builder.build(text);
        layout.break_all_lines(None);
        let origin = (
            centre.0 - f64::from(layout.width()) / 2.0,
            centre.1 - f64::from(layout.height()) / 2.0,
        );
        for line in layout.lines() {
            for item in line.items() {
                if let parley::PositionedLayoutItem::GlyphRun(glyphs) = item {
                    let run = glyphs.run();
                    scene.draw_glyphs(
                        run.font(),
                        run.font_size() * scale as f32,
                        true,
                        &[],
                        Vec2::default(),
                        Fill::NonZero,
                        &anyrender::Paint::from(LABEL),
                        1.0,
                        Affine::IDENTITY,
                        None,
                        glyphs.positioned_glyphs().map(|g| anyrender::Glyph {
                            id: g.id as _,
                            x: (scale * (origin.0 + f64::from(g.x))).round() as f32,
                            y: (scale * (origin.1 + f64::from(g.y))).round() as f32,
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
        match name {
            "data-circles" => self.circles = new_value.map(parse_circles).unwrap_or_default(),
            "data-selected" => self.selected = new_value.and_then(parse_id),
            "data-adjusting" => self.adjusting = new_value.and_then(parse_id),
            _ => {}
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
            let fill = if self.selected == Some(c.id) {
                SELECTED_FILL
            } else {
                FILL
            };
            scene.fill(
                Fill::NonZero,
                transform,
                fill,
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
                self.draw_label(&mut scene, &mut fonts, scale, (c.x, c.y), &text);
            }
        }
        scene.pop_layer();
        scene
    }

    fn virtual_nodes(&self) -> Vec<VirtualNodeDeclaration> {
        self.circles
            .iter()
            .filter_map(|c| {
                circle_region(c).map(|region| VirtualNodeDeclaration {
                    key: format!("c{}", c.id),
                    role: VirtualNodeRole::Image,
                    name: format!("Circle {}, {} px", c.id, c.d.round()),
                    region,
                    disabled: self.adjusting.is_some_and(|a| a != c.id),
                    selected: Some(self.selected == Some(c.id)),
                })
            })
            .collect()
    }
}

#[component]
pub fn CircleDrawer() -> Element {
    // The canvas's click is async, and its task runs inside the click's own dispatch.
    const SELECTION: &[dioxus_native::DeclaredWrite] = &[
        writes("circle-canvas", "click"),
        writes("circle-canvas", "keydown"),
    ];
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
    // the id the next circle a click adds takes
    let mut next_id: Signal<u64> = use_graph_signal(
        "next_id",
        const { &[writes("circle-canvas", "click")] },
        || 1,
    );
    // the id of the selected circle, if any
    let mut selected: Signal<Option<u64>> = use_graph_signal("selected", SELECTION, || None);
    // index of circle being resized, if dialog is open
    let mut dialog_idx: Signal<Option<usize>> = use_graph_signal(
        "dialog_idx",
        const {
            &[
                writes("circle-canvas", "dblclick"),
                writes("circle-canvas", "keydown"),
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
                writes("circle-canvas", "keydown"),
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
                tabindex: "0",
                "data": canvas,
                "data-circles": circles_text(&circles()),
                "data-selected": selected().map(|id| id.to_string()).unwrap_or_default(),
                "data-adjusting": dialog_idx()
                    .and_then(|i| circles().get(i).map(|c| c.id.to_string()))
                    .unwrap_or_default(),
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
                    let hit = circles()
                        .iter()
                        .rev()
                        .find(|c| (c.x - x).powi(2) + (c.y - y).powi(2) <= c.r * c.r)
                        .map(|c| c.id);
                    if let Some(id) = hit {
                        selected.set(Some(id));
                    } else {
                        undo_stack.write().push(circles());
                        redo_stack.write().clear();
                        let id = next_id();
                        next_id.set(id + 1);
                        circles.write().push(Circle { x, y, r: 20.0, id });
                    }
                },
                onkeydown: move |evt| {
                    if dialog_idx().is_some() {
                        return;
                    }
                    let all = circles();
                    let at = selected().and_then(|id| all.iter().position(|c| c.id == id));
                    let last = all.len().saturating_sub(1);
                    let to = match (evt.key(), at) {
                        (Key::ArrowLeft | Key::ArrowUp, Some(i)) => i.saturating_sub(1),
                        (Key::ArrowRight | Key::ArrowDown, Some(i)) => (i + 1).min(last),
                        (Key::ArrowLeft | Key::ArrowUp | Key::ArrowRight | Key::ArrowDown, None) => 0,
                        (Key::Enter, Some(i)) => {
                            if let Some(c) = all.get(i) {
                                dialog_idx.set(Some(i));
                                dialog_r.set(c.r);
                            }
                            return;
                        }
                        _ => return,
                    };
                    if let Some(c) = all.get(to) {
                        selected.set(Some(c.id));
                    }
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
    flex-shrink: 0;
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
        let rows: [(&str, usize); 11] = [
            ("", 0),
            ("x", 0),
            ("1,2", 0),
            ("1,2,40", 0),
            ("1,2,40,0", 0),
            ("1,2,40,-1", 0),
            ("1,2,40,1.5", 0),
            ("1,2,NaN,1", 0),
            ("1,2,inf,1", 0),
            ("1,2,40,1,5", 0),
            ("10,20,40,1;bad;30,40,50;30,40,50,2", 2),
        ];
        assert!(rows.len() == 11, "the table's row count");
        for (index, (text, read)) in rows.into_iter().enumerate() {
            let mut canvas = CircleCanvas::default();
            canvas.attribute_changed("data-circles", None, Some(text));
            assert!(canvas.circles.len() == read, "row {index}");
        }
    }

    #[test]
    fn the_selected_and_adjusting_texts_read_an_id_or_none() {
        let rows: [(&str, Option<u64>); 9] = [
            ("", None),
            ("x", None),
            ("0", None),
            ("-1", None),
            ("1.5", None),
            ("1,2", None),
            ("1", Some(1)),
            (" 7 ", Some(7)),
            ("42", Some(42)),
        ];
        assert!(rows.len() == 9, "the table's row count");
        for (index, (text, read)) in rows.into_iter().enumerate() {
            let mut canvas = CircleCanvas::default();
            canvas.attribute_changed("data-selected", None, Some(text));
            canvas.attribute_changed("data-adjusting", None, Some(text));
            assert!(
                canvas.selected == read && canvas.adjusting == read,
                "row {index}"
            );
        }
    }

    #[test]
    fn each_circle_declares_its_selected_and_disabled_state() {
        // (selected text, adjusting text, each circle's (selected, disabled))
        type Row = (&'static str, &'static str, [(bool, bool); 3]);
        let rows: [Row; 4] = [
            ("", "", [(false, false); 3]),
            ("2", "", [(false, false), (true, false), (false, false)]),
            ("2", "2", [(false, true), (true, false), (false, true)]),
            ("", "3", [(false, true), (false, true), (false, false)]),
        ];
        assert!(rows.len() == 4, "the table's row count");
        for (index, (selected, adjusting, expected)) in rows.into_iter().enumerate() {
            let mut canvas = CircleCanvas::default();
            canvas.attribute_changed(
                "data-circles",
                None,
                Some("100,100,40,1;200,100,40,2;300,100,40,3"),
            );
            canvas.attribute_changed("data-selected", None, Some(selected));
            canvas.attribute_changed("data-adjusting", None, Some(adjusting));
            let read: Vec<(Option<bool>, bool)> = canvas
                .virtual_nodes()
                .iter()
                .map(|n| (n.selected, n.disabled))
                .collect();
            let expected: Vec<(Option<bool>, bool)> = expected
                .iter()
                .map(|(selected, disabled)| (Some(*selected), *disabled))
                .collect();
            assert!(read == expected, "row {index}");
        }
    }
}
