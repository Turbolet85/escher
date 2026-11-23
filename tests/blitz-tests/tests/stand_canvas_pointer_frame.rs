//! A widget painted from Rust hears pointer events in the frame it paints in. The pointer events
//! blitz-dom forwards to a widget's `handle_event` carry page and client coordinates relative to
//! the element's content-box origin — its border-box origin plus its left and top border and
//! padding, untransformed — so a padded, bordered `<object>` hears a press and a move where its
//! paint, placed at the content box, stands. Read on a component booted alone through the stand,
//! in both layout modes. The paint's own placement is upstream's
//! <https://github.com/DioxusLabs/blitz/issues/1578>; this is its companion on the event side. A
//! failure message carries a layout mode and the coordinates read only.

use std::sync::{Arc, Mutex};

use blitz_dom::Widget;
use blitz_dom::node::ComputedStyles;
use blitz_traits::events::UiEvent;
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use escher_driver::Session;
use seven_guis::stand;

/// The id of the element the widget listens on.
const CANVAS: &str = "pointer-canvas";

/// A content-box object of 100 × 60 with 12 px of padding and a 3 px border: its border box
/// stands at (8, 8)–(138, 98) past the body's margin, its content box at (23, 23)–(123, 83).
const PADDED: &str = "display: block; box-sizing: content-box; width: 100px; height: 60px; \
                      padding: 12px; border: 3px solid black;";

/// One pointer event the widget heard: its kind and its page and client coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Heard {
    Down {
        page: (f32, f32),
        client: (f32, f32),
    },
    Move {
        page: (f32, f32),
        client: (f32, f32),
    },
}

type Log = Arc<Mutex<Vec<Heard>>>;

/// A widget that paints nothing and records each pointer down and move it hears.
struct PointerLog(Log);

impl Widget for PointerLog {
    fn handle_event(&mut self, event: &UiEvent) {
        let heard = match event {
            UiEvent::PointerDown(e) => Heard::Down {
                page: (e.coords.page_x, e.coords.page_y),
                client: (e.coords.client_x, e.coords.client_y),
            },
            UiEvent::PointerMove(e) => Heard::Move {
                page: (e.coords.page_x, e.coords.page_y),
                client: (e.coords.client_x, e.coords.client_y),
            },
            _ => return,
        };
        self.0.lock().expect("the log is not poisoned").push(heard);
    }

    fn paint(
        &mut self,
        _ctx: &mut dyn anyrender::RenderContext,
        _styles: &ComputedStyles,
        _width: u32,
        _height: u32,
        _scale: f64,
    ) -> anyrender::Scene {
        anyrender::Scene::new()
    }
}

#[derive(Props, Clone)]
struct CanvasProps {
    log: Log,
}

impl PartialEq for CanvasProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.log, &other.log)
    }
}

/// One padded `<object>` carrying the widget.
fn pointer_canvas(props: CanvasProps) -> Element {
    let log = props.log.clone();
    let attr = use_hook(move || CustomWidgetAttr::new(PointerLog(log)));
    rsx! {
        object { id: CANVAS, style: PADDED, "data": attr }
    }
}

/// A session on the padded canvas, booted alone through the stand, and the log its widget
/// records into.
fn canvas_session(incremental: bool) -> (Session, Log) {
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let given = log.clone();
    let session = Session::start("canvas", move || {
        stand::boot_alone(
            pointer_canvas,
            move || CanvasProps { log: given.clone() },
            stand::options(incremental),
        )
    })
    .expect("canvas is a label");
    (session, log)
}

#[test]
fn a_padded_widget_hears_pointer_events_from_its_content_box() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let (mut session, log) = canvas_session(incremental);

        session.harness_mut().mouse_down_at(50.0, 50.0);
        let downs = log
            .lock()
            .expect("the log is not poisoned")
            .iter()
            .filter(|heard| matches!(heard, Heard::Down { .. }))
            .count();
        assert!(downs == 1, "{at}: the press reaches the widget once");

        session.harness_mut().move_mouse_to(120.0, 80.0);
        let heard = log.lock().expect("the log is not poisoned").clone();
        assert!(
            heard
                == [
                    Heard::Down {
                        page: (27.0, 27.0),
                        client: (27.0, 27.0),
                    },
                    Heard::Move {
                        page: (97.0, 57.0),
                        client: (97.0, 57.0),
                    },
                ],
            "{at}: the widget hears the press and the move from its content-box origin, \
             heard {heard:?}"
        );
    }
}
