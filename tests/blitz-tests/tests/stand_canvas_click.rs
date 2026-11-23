//! An app's click and double click on an element carrying a widget painted from Rust reach its
//! handlers. A pointer press and release on a custom-widget target are handed to the widget
//! first, then blitz-dom counts the press, queues `click` on the release and, for a second click
//! in quick succession, `dblclick`, as for any element — so an `onclick` and an `ondoubleclick`
//! on a canvas `<object>` fire, and the widget still hears every pointer event. Upstream's widget
//! arm of `handle_dom_event` returns before that synthesis, so an app's click handlers on a
//! widget never fire there. Read through the driver's `click` on a component booted alone
//! through the stand, in both layout modes. A failure message carries a layout mode and an index
//! only.

use std::sync::{Arc, Mutex, PoisonError};

use blitz_dom::Widget;
use blitz_dom::node::ComputedStyles;
use blitz_traits::events::UiEvent;
use dioxus::prelude::*;
use dioxus_native_dom::CustomWidgetAttr;
use escher_driver::Session;
use seven_guis::stand;

mod session_common;
use session_common::{act, advance, click};

/// The id of the canvas the app listens on.
const CANVAS: &str = "canvas";

/// The id of the paragraph that reads the app's two counts.
const COUNTS: &str = "canvas-counts";

type Log = Arc<Mutex<Vec<&'static str>>>;

/// A widget that paints nothing and records each pointer down and up it hears.
struct PointerLog(Log);

impl Widget for PointerLog {
    fn handle_event(&mut self, event: &UiEvent) {
        let heard = match event {
            UiEvent::PointerDown(_) => "down",
            UiEvent::PointerUp(_) => "up",
            _ => return,
        };
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(heard);
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

/// One `<object>` carrying the widget, with an app's click and double-click handlers counting
/// into a paragraph.
fn click_canvas(props: CanvasProps) -> Element {
    let log = props.log.clone();
    let attr = use_hook(move || CustomWidgetAttr::new(PointerLog(log)));
    let mut clicks = use_signal(|| 0u32);
    let mut double = use_signal(|| 0u32);
    rsx! {
        object {
            id: CANVAS,
            style: "display: block; width: 200px; height: 100px;",
            "data": attr,
            onclick: move |_| clicks += 1,
            ondoubleclick: move |_| double += 1,
        }
        p { id: COUNTS, "clicks {clicks} dbl {double}" }
    }
}

/// A session on the canvas, booted alone through the stand, and the log its widget records
/// into.
fn canvas_session(incremental: bool) -> (Session, Log) {
    let log: Log = Arc::new(Mutex::new(Vec::new()));
    let given = log.clone();
    let session = Session::start("canvas", move || {
        stand::boot_alone(
            click_canvas,
            move || CanvasProps { log: given.clone() },
            stand::options(incremental),
        )
    })
    .expect("canvas is a label");
    (session, log)
}

/// What the app's paragraph reads.
fn counts(session: &Session) -> Option<String> {
    session
        .harness()
        .doc
        .snapshot()
        .get(COUNTS)
        .map(|node| node.name.clone())
}

/// What the widget has heard so far.
fn heard(log: &Log) -> Vec<&'static str> {
    log.lock().unwrap_or_else(PoisonError::into_inner).clone()
}

#[test]
fn one_click_on_a_canvas_reaches_the_apps_click_handler() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let (mut session, log) = canvas_session(incremental);
        assert!(
            session.harness().doc.snapshot().get(CANVAS).is_some(),
            "{at}: the snapshot holds the canvas"
        );

        assert!(
            act(&mut session, &click(CANVAS)).is_some(),
            "{at}: the click is accepted"
        );
        assert!(
            counts(&session).as_deref() == Some("clicks 1 dbl 0"),
            "{at}: the app counts one click"
        );
        assert!(
            heard(&log) == ["down", "up"],
            "{at}: the widget heard the press"
        );
    }
}

#[test]
fn two_clicks_with_no_advance_reach_the_apps_double_click_handler() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let (mut session, log) = canvas_session(incremental);

        for index in 0..2 {
            assert!(
                act(&mut session, &click(CANVAS)).is_some(),
                "{at}: click {index} is accepted"
            );
        }
        assert!(
            counts(&session).as_deref() == Some("clicks 2 dbl 1"),
            "{at}: two clicks in quick succession are a double click"
        );
        assert!(
            heard(&log) == ["down", "up", "down", "up"],
            "{at}: the widget heard both presses"
        );

        assert!(
            act(&mut session, &advance(600)).is_some(),
            "{at}: the advance is accepted"
        );
        assert!(
            act(&mut session, &click(CANVAS)).is_some(),
            "{at}: the third click is accepted"
        );
        assert!(
            counts(&session).as_deref() == Some("clicks 3 dbl 1"),
            "{at}: a click past the double-click window is a single click"
        );
    }
}
