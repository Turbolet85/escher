//! The one statement of what the checks beside the stand share: a fixture page booted with the
//! stand's own options, such a page held in a driver session, the fixture pages more than one
//! check reads — a canvas page painted from Rust and a canvas declaring virtual nodes among
//! them — and a network stand-in that holds every request until the check answers it. A
//! check declares `mod fixtures;` and reads what it needs. This module holds no test.

#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use anyrender::{AntiAliasing, PaintScene as _};
use blitz_dom::Widget;
use blitz_dom::node::ComputedStyles;
use blitz_test_harness::Harness;
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request};
use dioxus::prelude::*;
use dioxus_native_dom::{CustomWidgetAttr, DioxusDocument};
use escher_driver::{Fixture, Session};
use peniko::kurbo::{Affine, Circle, Rect, Stroke, Vec2};
use peniko::{Color, Fill};
use seven_guis::stand;

/// A fixture booted as the stand boots a task: pinned viewport, bundled font, offline.
pub fn boot(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
    Harness::from_vdom(VirtualDom::new(app), stand::options(incremental))
}

/// A session started under `label` on a fixture booted as the stand boots a task.
pub fn hold(label: &str, app: fn() -> Element, incremental: bool) -> Session {
    Session::start(label, || boot(app, incremental)).expect("the label is one of the closed set")
}

/// A session started under `label` on a fixture booted as the stand boots a task, brought to
/// `fixture`'s state through the driver's validated run path.
pub fn hold_in(label: &str, app: fn() -> Element, fixture: &Fixture, incremental: bool) -> Session {
    let mut session = hold(label, app, incremental);
    // A fixture a check states for its own page must reach; one that does not is the check's
    // defect, and the failure names it.
    session
        .reach(fixture)
        .expect("the check's fixture state is reached");
    session
}

/// A password input labelled `Passphrase` by a `<label>`, the control the lean tasks lack.
pub fn password_labelled() -> Element {
    rsx! {
        div { id: "fx-root",
            label { r#for: "fx-secret", "Passphrase" }
            input { id: "fx-secret", r#type: "password" }
        }
    }
}

/// A password input named `Secret` by `aria-label`, empty.
pub fn password_named() -> Element {
    rsx! {
        div { id: "fx-root",
            input { id: "fx-secret", r#type: "password", aria_label: "Secret" }
        }
    }
}

/// One button under one positioned overlay with a `z-index` above it: the `covered` case the
/// stand lacks.
pub fn covered() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 600px; height: 400px;",
            button {
                id: "fx-under",
                style: "position: absolute; left: 20px; top: 20px; width: 120px; height: 40px;",
                "Under"
            }
            div {
                id: "fx-cover",
                style: "position: absolute; left: 0; top: 0; width: 200px; height: 140px; z-index: 5; background: #cccccc;",
            }
        }
    }
}

/// The buttons on the fixture page.
pub const ITEMS: usize = 400;

/// 400 keyed buttons, each counting its clicks into one output, then a password input.
pub fn many_items() -> Element {
    let mut clicks = use_signal(|| 0u32);
    rsx! {
        div { id: "items",
            output { id: "clicks", "{clicks}" }
            for i in 1..=ITEMS {
                button {
                    key: "{i}",
                    id: format!("item-{i:03}"),
                    style: "display: block;",
                    onclick: move |_| clicks += 1,
                    "Item {i:03}"
                }
            }
            input { id: "secret", r#type: "password", aria_label: "Secret" }
        }
    }
}

/// The text the small canvas shapes into glyphs: no answer may hold it.
pub const CANVAS_TEXT: &str = "QZX";

/// How many solid fills the large canvas paints: its full dump passes the budget.
pub const LARGE_FILLS: usize = 200;

type Fonts = Arc<Mutex<parley::FontContext>>;

/// A widget painting a clip layer over its box, a solid fill, a stroked circle and the glyphs
/// of [`CANVAS_TEXT`] shaped from the document's own font context, then the pop.
struct SmallCanvas {
    fonts: Option<Fonts>,
    layouts: parley::LayoutContext<()>,
}

impl Widget for SmallCanvas {
    fn fonts(&mut self, fonts: &Fonts) {
        self.fonts = Some(Arc::clone(fonts));
    }

    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        width: u32,
        height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        let bounds = Rect::new(0.0, 0.0, f64::from(width), f64::from(height));
        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &bounds);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgb8(240, 240, 240),
            None,
            &Rect::new(10.0, 10.0, 60.0, 40.0),
            AntiAliasing::Enabled,
        );
        scene.stroke(
            &Stroke::new(2.0),
            Affine::IDENTITY,
            Color::from_rgb8(20, 20, 20),
            None,
            &Circle::new((100.0, 50.0), 30.0),
            AntiAliasing::Enabled,
        );
        if let Some(fonts) = self.fonts.clone() {
            let mut fonts = fonts.lock().expect("the font context is not poisoned");
            let mut builder = self
                .layouts
                .ranged_builder(&mut fonts, CANVAS_TEXT, 1.0, true);
            builder.push_default(parley::StyleProperty::FontSize(16.0));
            builder.push_default(parley::GenericFamily::SansSerif);
            let mut layout = builder.build(CANVAS_TEXT);
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
                            Affine::translate((150.0, 20.0)),
                            None,
                            glyphs.positioned_glyphs().map(|g| anyrender::Glyph {
                                id: g.id as _,
                                x: g.x.round(),
                                y: g.y.round(),
                            }),
                            AntiAliasing::Enabled,
                        );
                    }
                }
            }
        }
        scene.pop_layer();
        scene
    }
}

/// A widget painting [`LARGE_FILLS`] solid fills.
struct LargeCanvas;

impl Widget for LargeCanvas {
    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        _width: u32,
        _height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        let mut scene = anyrender::Scene::new();
        for index in 0..LARGE_FILLS {
            let x = (index % 20) as f64 * 10.0;
            let y = (index / 20) as f64 * 10.0;
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                Color::from_rgb8((index % 256) as u8, 100, 200),
                None,
                &Rect::new(x, y, x + 8.0, y + 8.0),
                AntiAliasing::Enabled,
            );
        }
        scene
    }
}

/// Two keyed canvases painted from Rust and a keyed button: `small-canvas`, a clip layer, a
/// fill, a stroke and a glyph run of [`CANVAS_TEXT`] from the document's fonts; `large-canvas`,
/// [`LARGE_FILLS`] fills; `fx-button`, no canvas.
pub fn canvas_commands() -> Element {
    let small = use_hook(|| {
        CustomWidgetAttr::new(SmallCanvas {
            fonts: None,
            layouts: parley::LayoutContext::new(),
        })
    });
    let large = use_hook(|| CustomWidgetAttr::new(LargeCanvas));
    rsx! {
        div { id: "fx-root",
            object {
                id: "small-canvas",
                style: "display: block; width: 200px; height: 100px;",
                "data": small,
            }
            object {
                id: "large-canvas",
                style: "display: block; width: 200px; height: 100px;",
                "data": large,
            }
            button { id: "fx-button", "Go" }
        }
    }
}

/// The declaring canvas's five virtual node ids under `full`, in declaration order.
pub const VIRTUAL_IDS: [&str; 5] = [
    "vn-canvas///a",
    "vn-canvas///e",
    "vn-canvas///b",
    "vn-canvas///d",
    "vn-canvas///c",
];

/// The names the declaring canvas declares, none of them text the page holds elsewhere: its five
/// nodes' under `full`, in declaration order, then `a`'s once clicked.
pub const VIRTUAL_NAMES: [&str; 6] = [
    "Planted alpha",
    "Planted echo",
    "Planted beta",
    "Planted delta",
    "Planted gamma",
    "Planted alpha pressed",
];

/// Which list the declaring canvas answers, from its `data-set` attribute.
#[derive(Clone, Copy, PartialEq)]
enum VirtualSet {
    /// All five nodes.
    Full,
    /// `a` alone.
    Less,
}

/// A widget declaring virtual nodes from its `data-set` attribute, `less` declaring `a` alone and
/// any other value all five; a pointer up inside `a`'s region renames it.
struct VirtualCanvas {
    set: VirtualSet,
    pressed: bool,
}

/// One declaration of the declaring canvas.
fn virtual_declared(
    key: &str,
    role: accesskit::Role,
    name: &str,
    (x, y, width, height): (f64, f64, f64, f64),
    disabled: bool,
) -> blitz_dom::VirtualNodeDeclaration {
    blitz_dom::VirtualNodeDeclaration {
        key: key.to_string(),
        role,
        name: name.to_string(),
        region: blitz_dom::BoundingRect {
            x,
            y,
            width,
            height,
        },
        disabled,
    }
}

/// `a`'s region in the canvas's content box.
const VIRTUAL_A: (f64, f64, f64, f64) = (0.0, 0.0, 60.0, 30.0);

impl Widget for VirtualCanvas {
    fn attribute_changed(&mut self, name: &str, _old_value: Option<&str>, new_value: Option<&str>) {
        if name == "data-set" {
            self.set = match new_value {
                Some("less") => VirtualSet::Less,
                _ => VirtualSet::Full,
            };
        }
    }

    fn handle_event(&mut self, event: &blitz_traits::events::UiEvent) {
        if let blitz_traits::events::UiEvent::PointerUp(event) = event {
            let (x, y) = (
                f64::from(event.coords.client_x),
                f64::from(event.coords.client_y),
            );
            let (left, top, width, height) = VIRTUAL_A;
            if x >= left && x < left + width && y >= top && y < top + height {
                self.pressed = true;
            }
        }
    }

    fn virtual_nodes(&self) -> Vec<blitz_dom::VirtualNodeDeclaration> {
        use accesskit::Role;
        let alpha = if self.pressed {
            VIRTUAL_NAMES[5]
        } else {
            VIRTUAL_NAMES[0]
        };
        let a = virtual_declared("a", Role::Button, alpha, VIRTUAL_A, false);
        if self.set == VirtualSet::Less {
            return vec![a];
        }
        vec![
            a,
            virtual_declared(
                "e",
                Role::Button,
                VIRTUAL_NAMES[1],
                (40.0, 20.0, 40.0, 30.0),
                false,
            ),
            virtual_declared(
                "b",
                Role::CheckBox,
                VIRTUAL_NAMES[2],
                (100.0, 0.0, 40.0, 30.0),
                true,
            ),
            virtual_declared(
                "d",
                Role::Button,
                VIRTUAL_NAMES[3],
                (0.0, 60.0, 40.0, 30.0),
                false,
            ),
            virtual_declared(
                "c",
                Role::Button,
                VIRTUAL_NAMES[4],
                (150.0, 90.0, 40.0, 30.0),
                false,
            ),
        ]
    }
}

/// The declaring canvas's border plus padding, on each side, in CSS px: its content box's
/// origin from its border box's.
pub const VIRTUAL_CANVAS_INSET: f64 = 7.0;

const VIRTUAL_CSS: &str = ".vn-hidden { display: none; }";

/// A canvas `vn-canvas` painted from Rust, its content box 200 × 120 inside a 2 px border and
/// 5 px of padding, declaring the five nodes of [`VIRTUAL_IDS`] — `b` disabled — inside a holder
/// `vn-holder` that clips its bottom 30 px; a `vn-cover` over `d`'s centre; a button `vn-drop`
/// whose click makes the canvas declare `a` alone; and a button `vn-hide` whose click hides the
/// holder by a class.
pub fn virtual_canvas_page() -> Element {
    let canvas = use_hook(|| {
        CustomWidgetAttr::new(VirtualCanvas {
            set: VirtualSet::Full,
            pressed: false,
        })
    });
    let mut set = use_signal(|| "full");
    let mut hidden = use_signal(|| false);
    rsx! {
        style { {VIRTUAL_CSS} }
        div { id: "vn-root", style: "position: relative; width: 600px; height: 400px;",
            div {
                id: "vn-holder",
                class: if hidden() { "vn-hidden" } else { "" },
                style: "position: absolute; left: 0; top: 0; width: 220px; height: 104px; overflow: hidden;",
                object {
                    id: "vn-canvas",
                    style: "display: block; box-sizing: content-box; width: 200px; height: 120px; padding: 5px; border: 2px solid black;",
                    "data-set": set(),
                    "data": canvas,
                }
            }
            div {
                id: "vn-cover",
                style: "position: absolute; left: 17px; top: 72px; width: 20px; height: 20px; z-index: 5; background: #cccccc;",
            }
            button {
                id: "vn-drop",
                style: "position: absolute; left: 300px; top: 20px;",
                onclick: move |_| set.set("less"),
                "Drop"
            }
            button {
                id: "vn-hide",
                style: "position: absolute; left: 300px; top: 80px;",
                onclick: move |_| hidden.set(true),
                "Hide"
            }
        }
    }
}

/// A `NetProvider` that keeps every request with its handler for the check to answer.
#[derive(Default)]
pub struct HeldNet {
    requests: Mutex<Vec<(String, Box<dyn NetHandler>)>>,
}

impl HeldNet {
    /// How many requests are held.
    pub fn held(&self) -> usize {
        self.requests.lock().unwrap().len()
    }

    /// The urls of the held requests, in request order.
    pub fn urls(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .map(|(url, _)| url.clone())
            .collect()
    }

    /// Answers the last held request with `body` and returns its url.
    #[track_caller]
    pub fn answer(&self, body: &'static [u8]) -> String {
        let (url, handler) = self
            .requests
            .lock()
            .unwrap()
            .pop()
            .expect("a request is held");
        handler.bytes(url.clone(), Bytes::from_static(body));
        url
    }
}

impl NetProvider for HeldNet {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        self.requests
            .lock()
            .unwrap()
            .push((request.url.to_string(), handler));
    }
}
