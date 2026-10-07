//! A step taken on a held instance returns with its consequences already on the screen, in
//! both layout modes: the stand's Timer reads its delivered ticks when `Session::act` returns,
//! and a settle on an idle lean task changes nothing. The cases the lean tasks lack are proven
//! on minimal fixtures: a write made while an element mounts, a stylesheet and its import
//! answered by this file's own provider, a running animation, a hover that moves with the
//! layout — and two instances that never go quiet, which end at the bound with the class of
//! work still outstanding. No check here sleeps, reads a clock or makes a pass of its own: the
//! only passes are the input helpers' and the settle under test. A failure message carries the
//! layout mode and ids this file names, never what a screen reads.

use std::sync::{Arc, Mutex};

use blitz_test_harness::{Busy, Harness, HarnessOptions, NotSettled, Settled};
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request};
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;
use escher_driver::{Session, SessionError};
use keyboard_types::Key;
use seven_guis::stand::{self, LeanTask};

mod session_common;

/// A fixture booted as the stand boots a task: pinned viewport, bundled font, offline.
fn fixture(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
    Harness::from_vdom(VirtualDom::new(app), stand::options(incremental))
}

/// The ids reading focused in a snapshot of the held instance.
fn focused(session: &Session) -> Vec<String> {
    session
        .harness()
        .doc
        .snapshot()
        .nodes()
        .filter(|node| node.state.focused)
        .map(|node| node.id.clone())
        .collect()
}

#[test]
fn a_timer_step_returns_with_its_delayed_update_present() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (mut timer, ticks) = session_common::hold(LeanTask::Timer, incremental);
        let elapsed = |timer: &Session| timer.harness().text_content("#timer-elapsed");
        assert!(
            elapsed(&timer) == "Elapsed: 0.0s",
            "{mode}: no time has passed on a started session"
        );

        let run_out = timer.act(|_| ticks.deliver(151));
        assert!(run_out.is_ok(), "{mode}: the step that runs the timer out");
        assert!(
            elapsed(&timer) == "Elapsed: 15.0s",
            "{mode}: the timer has stopped at its duration when the step returns"
        );

        let past_the_end = timer.act(|_| ticks.deliver(3));
        assert!(past_the_end.is_ok(), "{mode}: the step past the duration");
        assert!(
            elapsed(&timer) == "Elapsed: 15.0s",
            "{mode}: ticks past the duration leave the time at it"
        );

        let mut in_step = String::new();
        let reset = timer.act(|harness| {
            harness.click("#timer-reset");
            ticks.deliver(3);
            in_step = harness.text_content("#timer-elapsed");
        });
        assert!(reset.is_ok(), "{mode}: the step that resets and ticks");
        assert!(
            in_step == "Elapsed: 0.0s",
            "{mode}: inside the step the three ticks are delivered and not applied"
        );
        assert!(
            elapsed(&timer) == "Elapsed: 0.3s",
            "{mode}: the delayed update is present when the step returns"
        );
    }
}

#[test]
fn a_settle_on_an_idle_instance_changes_nothing() {
    for incremental in [false, true] {
        for task in LeanTask::ALL {
            let mode = format!("incremental={incremental}: {task:?}");
            let (mut session, _ticks) = session_common::hold(task, incremental);
            let text_before = session.harness().doc.snapshot().to_text();
            // The boot's own mutations are still in the changed set: drain it, so the flag
            // read after the step is the step's.
            session.harness_mut().base_mut().take_changed_nodes();

            let idle = session.act(|_| {});
            assert!(
                idle == Ok(Settled {
                    passes: 1,
                    animating: false
                }),
                "{mode}: an idle instance settles in one pass and animates nothing"
            );
            let text_after = session.harness().doc.snapshot().to_text();
            assert!(
                text_before == text_after,
                "{mode}: the snapshot text is the same before and after the settle"
            );
            assert!(
                !session.harness().base().has_changes(),
                "{mode}: the settle marks no change"
            );
            assert!(
                focused(&session).is_empty(),
                "{mode}: no snapshot node reads focused after the settle"
            );

            let tab = session.act(|harness| harness.press(Key::Tab));
            assert!(tab.is_ok(), "{mode}: the Tab step");
            assert!(
                focused(&session) == ["back-btn"],
                "{mode}: the first Tab after the settle focuses back-btn"
            );
        }
    }
}

/// A button reveals an element whose `mounted` handler writes the status text.
fn mount_fixture() -> Element {
    let mut shown = use_signal(|| false);
    let mut status = use_signal(|| "waiting");
    rsx! {
        div { id: "fx-root",
            button { id: "fx-reveal", onclick: move |_| shown.set(true), "Reveal" }
            p { id: "fx-status", "{status}" }
            if shown() {
                div { id: "fx-revealed", onmounted: move |_| status.set("mounted"), "Revealed" }
            }
        }
    }
}

#[test]
fn a_write_made_while_mounting_is_present_after_the_step() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut screen = fixture(mount_fixture, incremental);
        assert!(
            screen.text_content("#fx-status") == "waiting",
            "{mode}: fx-status reads its first text at boot"
        );

        screen.click("#fx-reveal");
        assert!(
            screen.query("#fx-revealed").is_some(),
            "{mode}: the click mounts fx-revealed"
        );
        assert!(
            screen.text_content("#fx-status") == "waiting",
            "{mode}: the click's one pass leaves the mounting write unrendered"
        );

        let settled = screen.settle();
        assert!(
            settled.is_ok_and(|settled| settled.passes >= 2),
            "{mode}: the settle takes a second pass"
        );
        assert!(
            screen.text_content("#fx-status") == "mounted",
            "{mode}: fx-status reads the mounting write after the settle"
        );

        let mut session = Session::start("mounting", || fixture(mount_fixture, incremental))
            .expect("the label is one of the closed set");
        let step = session.act(|harness| harness.click("#fx-reveal"));
        assert!(step.is_ok(), "{mode}: the click step on a held instance");
        assert!(
            session.harness().text_content("#fx-status") == "mounted",
            "{mode}: fx-status reads the mounting write when the step returns"
        );
    }
}

/// A `NetProvider` that keeps every request for the check to answer.
#[derive(Default)]
struct ManualNetProvider {
    requests: Mutex<Vec<(String, Box<dyn NetHandler>)>>,
}

impl ManualNetProvider {
    fn held(&self) -> usize {
        self.requests.lock().unwrap().len()
    }

    /// Answers the one held request, which names `file`, with `css`.
    #[track_caller]
    fn answer(&self, file: &str, css: &'static str) {
        let (url, handler) = self
            .requests
            .lock()
            .unwrap()
            .pop()
            .expect("a request is held");
        assert!(url.ends_with(file), "the held request names {file}");
        handler.bytes(url, Bytes::from_static(css.as_bytes()));
    }
}

impl NetProvider for ManualNetProvider {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        self.requests
            .lock()
            .unwrap()
            .push((request.url.to_string(), handler));
    }
}

const LOADING_HTML: &str = r#"<!doctype html>
<html>
  <head><link rel="stylesheet" href="first.css"></head>
  <body><div id="fx-box"></div></body>
</html>"#;

const FIRST_CSS: &str = r#"@import url("second.css"); #fx-box { width: 120px; }"#;

const SECOND_CSS: &str = "#fx-box { height: 70px; }";

#[test]
fn a_load_in_flight_reads_not_settled_until_it_is_answered() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let loads = Err(NotSettled { busy: Busy::Loads });
        let net = Arc::new(ManualNetProvider::default());
        let mut page = Harness::from_html_with(
            LOADING_HTML,
            HarnessOptions {
                base_url: Some("http://settle.test/".to_string()),
                net_provider: Some(Arc::clone(&net) as _),
                ..stand::options(incremental)
            },
        );
        assert!(
            net.held() == 1 && page.base().has_pending_critical_resources(),
            "{mode}: the head stylesheet is requested and unanswered"
        );

        assert!(
            page.settle() == loads,
            "{mode}: an unanswered stylesheet reads not settled"
        );
        assert!(net.held() == 1, "{mode}: the settle answered nothing");

        net.answer("first.css", FIRST_CSS);
        assert!(
            page.settle() == loads,
            "{mode}: the import in flight reads not settled"
        );
        assert!(
            net.held() == 1 && !page.base().has_pending_critical_resources(),
            "{mode}: the import is held by the provider and on no list of the document's"
        );
        let first_only = page.layout_rect("#fx-box");
        assert!(
            first_only.width == 120.0 && first_only.height == 0.0,
            "{mode}: fx-box reads the first sheet's width and no height yet"
        );

        net.answer("second.css", SECOND_CSS);
        assert!(
            page.settle().is_ok(),
            "{mode}: every load answered reads settled"
        );
        let both = page.layout_rect("#fx-box");
        assert!(
            both.width == 120.0 && both.height == 70.0,
            "{mode}: fx-box reads the width of the first sheet and the height of its import"
        );
    }
}

const ANIMATED_HTML: &str = r#"<!doctype html>
<html>
  <head><style>
    @keyframes fx-slide { from { margin-left: 0px; } to { margin-left: 100px; } }
    #fx-animated { width: 50px; height: 50px; animation: fx-slide 1s linear infinite; }
  </style></head>
  <body><div id="fx-animated"></div></body>
</html>"#;

#[test]
fn a_running_animation_is_reported_and_does_not_hold_the_step() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut page = Harness::from_html_with(ANIMATED_HTML, stand::options(incremental));
        assert!(
            page.base().is_animating(),
            "{mode}: fx-animated is animating at boot"
        );

        assert!(
            page.settle()
                == Ok(Settled {
                    passes: 1,
                    animating: true
                }),
            "{mode}: a running animation settles in one pass and is reported"
        );
    }
}

/// Every render mounts a freshly keyed element whose `mounted` handler bumps the key.
fn restless_fixture() -> Element {
    let mut round = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            p { id: "fx-label", "Restless" }
            for key in [round()] {
                div { key: "{key}", id: "fx-spinner", onmounted: move |_| round += 1 }
            }
        }
    }
}

#[test]
fn an_instance_that_never_goes_quiet_ends_at_the_bound() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = Session::start("restless", || fixture(restless_fixture, incremental))
            .expect("the label is one of the closed set");

        let outcome = session.act(|_| {});
        assert!(
            outcome == Err(SessionError::NotSettled(Busy::Render)),
            "{mode}: an instance that renders on every pass ends as still rendering"
        );
        assert!(
            session.harness().text_content("#fx-label") == "Restless"
                && session.harness().query("#fx-spinner").is_some(),
            "{mode}: the instance is still readable after the bound"
        );
        let message = outcome.map_or_else(|error| error.to_string(), |_| String::new());
        assert!(
            !message.is_empty() && !message.contains('/'),
            "{mode}: the outcome's message is fixed text and holds no path"
        );
    }
}

const HOVER_CSS: &str = "
    .fx-box { width: 100px; height: 40px; }
    #fx-lower:hover { width: 200px; }
";

/// The upper box's click removes it, so the lower box moves up under the pointer.
fn hover_fixture() -> Element {
    let mut upper = use_signal(|| true);
    rsx! {
        style { {HOVER_CSS} }
        div { id: "fx-root",
            if upper() {
                div { id: "fx-upper", class: "fx-box", onclick: move |_| upper.set(false) }
            }
            div { id: "fx-lower", class: "fx-box" }
        }
    }
}

#[test]
fn a_hover_that_moves_with_layout_is_resolved_before_the_step_returns() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut screen = fixture(hover_fixture, incremental);

        screen.click("#fx-upper");
        assert!(
            screen.query("#fx-upper").is_none(),
            "{mode}: the click removes fx-upper"
        );
        assert!(
            screen.hovered() == screen.query("#fx-lower"),
            "{mode}: fx-lower has moved under the pointer"
        );
        assert!(
            screen.layout_rect("#fx-lower").width == 100.0,
            "{mode}: the click's one pass leaves fx-lower at its un-hovered width"
        );

        assert!(screen.settle().is_ok(), "{mode}: the settle");
        assert!(
            screen.layout_rect("#fx-lower").width == 200.0,
            "{mode}: fx-lower reads its hovered width after the settle"
        );
    }
}

const FLICKER_HTML: &str = r#"<!doctype html>
<html>
  <head><style>
    #fx-flicker { width: 100px; height: 100px; }
    #fx-flicker:hover { display: none; }
  </style></head>
  <body><div id="fx-flicker"></div></body>
</html>"#;

#[test]
fn a_hover_that_never_rests_ends_at_the_bound() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut page = Harness::from_html_with(FLICKER_HTML, stand::options(incremental));
        let (x, y) = page.center_of("#fx-flicker");

        page.move_mouse_to(x, y);
        assert!(
            page.hovered() != page.query("#fx-flicker"),
            "{mode}: the hover has left fx-flicker one pass after the pointer was put on it"
        );

        assert!(
            page.settle() == Err(NotSettled { busy: Busy::Layout }),
            "{mode}: a hover that moves on every pass ends as layout still moving"
        );
    }
}
