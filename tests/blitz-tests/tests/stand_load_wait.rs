//! An acting driver command that meets a load in flight waits it out within `LOAD_WAIT`, in both
//! layout modes. On a fixture whose click adds a body stylesheet, a stand-in that answers on a
//! thread of its own after 200 ms ends the `click` settled, the styled box changed in its diff
//! and on the next screen; the shared held stand-in, which never answers, ends it at the bound
//! with the load still outstanding. Neither moves the session's clock. The call's wall time and
//! the stand-in's own delay are this check's two wall reads. A failure message carries a layout
//! mode and an arm only.

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request};
use dioxus::prelude::*;
use dioxus_native_dom::Reading;
use escher_driver::{Busy, LOAD_WAIT, Outcome, Session};
use seven_guis::stand;

mod fixtures;
mod session_common;
use fixtures::HeldNet;
use session_common::{Acted, act, click, snapshot};

/// The sheet the button's click links: the one rule that gives the box a height.
const LATE_CSS: &str = "#fx-box { height: 70px; }";

/// How long the self-answering stand-in holds a request before it answers.
const ANSWER_DELAY: Duration = Duration::from_millis(200);

/// The slack past `LOAD_WAIT` a never-answered wait may take: the settles around the bound.
const OVERRUN: Duration = Duration::from_secs(2);

/// A button whose click links a body stylesheet, and a box only that sheet gives a height.
fn load_page() -> Element {
    let mut linked = use_signal(|| false);
    rsx! {
        div { id: "fx-root",
            button { id: "fx-load", onclick: move |_| linked.set(true), "Load" }
            if linked() {
                link { rel: "stylesheet", href: "http://load.test/late.css" }
            }
            div { id: "fx-box", style: "width: 120px;" }
        }
    }
}

/// A `NetProvider` that answers every request with [`LATE_CSS`] from a thread of its own, after
/// [`ANSWER_DELAY`].
struct LateNet;

impl NetProvider for LateNet {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        let url = request.url.to_string();
        thread::spawn(move || {
            thread::sleep(ANSWER_DELAY);
            handler.bytes(url, Bytes::from_static(LATE_CSS.as_bytes()));
        });
    }
}

/// A session on the fixture, its loads fetched through `net`.
fn hold(net: Arc<dyn NetProvider>, incremental: bool) -> Session {
    Session::start("load-wait", || {
        Harness::from_vdom(
            VirtualDom::new(load_page),
            HarnessOptions {
                net_provider: Some(net),
                ..stand::options(incremental)
            },
        )
    })
    .expect("the label is one of the closed set")
}

/// The box's height as a `snapshot` through the driver reads it.
#[track_caller]
fn box_height(session: &mut Session, mode: &str) -> String {
    let Ok(Outcome::Screen { text }) = session.run(&snapshot()) else {
        panic!("{mode}: a snapshot is answered with the screen");
    };
    let line = text
        .lines()
        .find(|line| line.contains("id=\"fx-box\""))
        .unwrap_or_else(|| panic!("{mode}: the screen reads the box"));
    let (_, size) = line
        .rsplit_once(' ')
        .unwrap_or_else(|| panic!("{mode}: the box's line ends with its size"));
    let (_, height) = size
        .split_once('x')
        .unwrap_or_else(|| panic!("{mode}: the box's size reads width by height"));
    height.to_string()
}

/// Clicks the button through the driver: what the click returned, the wall time it took, and
/// whether the session's clock read the same after it.
#[track_caller]
fn click_load(session: &mut Session, mode: &str) -> (Acted, Duration, bool) {
    let time = session.harness().time();
    let start = Instant::now();
    let acted = act(session, &click("fx-load"))
        .unwrap_or_else(|| panic!("{mode}: the click is accepted and acts"));
    let took = start.elapsed();
    (acted, took, session.harness().time() == time)
}

#[test]
fn a_load_answered_inside_the_bound_ends_the_step_settled_with_its_content_on_screen() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental} answered");
        let mut session = hold(Arc::new(LateNet), incremental);
        assert!(
            box_height(&mut session, &mode) == "0",
            "{mode}: the box reads no height before the sheet"
        );

        let (acted, took, clock_still) = click_load(&mut session, &mode);
        assert!(
            acted.settled && acted.busy.is_none(),
            "{mode}: the step reads settled once the load is answered"
        );
        assert!(
            acted.diff.changed.iter().any(|node| node.id == "fx-box"
                && node.differs.contains(&Reading::Bounds)
                && node.bounds.height == 70.0),
            "{mode}: the diff names the box changed in its bounds, at the sheet's height"
        );
        assert!(
            box_height(&mut session, &mode) == "70",
            "{mode}: the next screen reads the box at the sheet's height"
        );
        assert!(
            took >= ANSWER_DELAY && took < LOAD_WAIT,
            "{mode}: the step waited for the answer and returned inside the bound"
        );
        assert!(clock_still, "{mode}: the wait moved no time");
    }
}

#[test]
fn a_load_never_answered_ends_the_step_at_the_bound_still_loading() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental} never answered");
        let net = Arc::new(HeldNet::default());
        let mut session = hold(Arc::clone(&net) as _, incremental);
        assert!(net.held() == 0, "{mode}: nothing is requested at boot");

        let (acted, took, clock_still) = click_load(&mut session, &mode);
        assert!(
            !acted.settled && acted.busy == Some(Busy::Loads),
            "{mode}: the step reads the load still outstanding"
        );
        assert!(
            took >= LOAD_WAIT && took < LOAD_WAIT + OVERRUN,
            "{mode}: the step returned at the bound"
        );
        assert!(net.held() == 1, "{mode}: the one request is still held");
        assert!(clock_still, "{mode}: the wait moved no time");
        assert!(
            box_height(&mut session, &mode) == "0",
            "{mode}: the box still reads no height"
        );
    }
}
