//! The headless stand: mounts one of the lean four 7GUIs tasks, Cells, or the load page inside
//! the stand's `TaskShell` chrome with no window and no GPU, at one pinned viewport, with the
//! bundled DejaVu Sans, building a fresh document per boot. Every boot is offline but the load
//! page's, whose one stylesheet an in-process stand-in answers after 200 ms from a thread of its
//! own, reaching no network.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request};
use blitz_traits::shell::ColorScheme;
use dioxus_native::prelude::*;
use dioxus_native::{DioxusDocument, FontContext, build_single_font_ctx};

use crate::app::{Task, TaskShell, task_in_shell};
use crate::tasks::timer::TimerTicks;

/// The four tasks the stand proves capabilities on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeanTask {
    Counter,
    FlightBooker,
    Timer,
    Crud,
}

impl LeanTask {
    /// Every lean task, in 7GUIs order.
    pub const ALL: [LeanTask; 4] = [
        LeanTask::Counter,
        LeanTask::FlightBooker,
        LeanTask::Timer,
        LeanTask::Crud,
    ];

    /// The app's task this lean task names.
    pub fn task(self) -> Task {
        match self {
            LeanTask::Counter => Task::Counter,
            LeanTask::FlightBooker => Task::FlightBooker,
            LeanTask::Timer => Task::Timer,
            LeanTask::Crud => Task::Crud,
        }
    }
}

/// Pinned viewport width, in CSS pixels.
pub const VIEWPORT_WIDTH: u32 = 800;
/// Pinned viewport height, in CSS pixels.
pub const VIEWPORT_HEIGHT: u32 = 600;
/// Pinned hidpi scale.
pub const HIDPI_SCALE: f32 = 1.0;
/// Pinned colour scheme.
pub const COLOR_SCHEME: ColorScheme = ColorScheme::Light;

/// A font context holding only the bundled DejaVu Sans, with system fonts off.
pub fn font_ctx() -> FontContext {
    build_single_font_ctx(crate::DEJAVU_SANS)
}

/// The stand's harness options: the pinned viewport and the bundled font, with no
/// net provider (so the document is offline). Replace fields with `..` to inject one.
pub fn options(incremental: bool) -> HarnessOptions {
    HarnessOptions {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
        scale: HIDPI_SCALE,
        color_scheme: COLOR_SCHEME,
        base_url: None,
        net_provider: None,
        font_ctx: Some(font_ctx()),
        incremental: Some(incremental),
    }
}

/// Boot `task` in a fresh document. The timer gets a tick source nobody drives,
/// so its elapsed time stays at zero; use [`boot_timer`] to advance it.
pub fn boot(task: LeanTask, options: HarnessOptions) -> Harness<DioxusDocument> {
    boot_with_ticks(task, TimerTicks::default(), options)
}

/// Boot the timer in a fresh document, returning the handle that delivers its ticks.
/// Ticks apply on the harness's next [`Harness::pump`].
pub fn boot_timer(options: HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks) {
    let ticks = TimerTicks::default();
    let harness = boot_with_ticks(LeanTask::Timer, ticks.clone(), options);
    (harness, ticks)
}

/// The app time one Timer tick stands for, in milliseconds.
pub const TIMER_TICK_MS: u32 = 100;

/// The Timer's time step for a driver session (`escher_driver::Session::with_time`): handed
/// the milliseconds the session's clock just moved, it delivers through `ticks` the whole
/// ticks the clock now holds beyond those already delivered. The remainder below one tick is
/// carried to the next call, so the Timer's elapsed time is always the session clock's whole
/// ticks: 250 ms then 250 ms deliver 2 then 3 ticks. The session moves the clock itself; the
/// step moves only the Timer. The ticks apply on the harness's next pass.
pub fn timer_step(ticks: TimerTicks) -> impl FnMut(&mut Harness<DioxusDocument>, u32) {
    let mut carried_ms = 0;
    move |_harness, ms| {
        let held = carried_ms + u64::from(ms);
        let tick = u64::from(TIMER_TICK_MS);
        ticks.deliver(held / tick);
        carried_ms = held % tick;
    }
}

fn boot_with_ticks(
    task: LeanTask,
    ticks: TimerTicks,
    options: HarnessOptions,
) -> Harness<DioxusDocument> {
    let vdom = VirtualDom::new_with_props(stand_root, StandRoot { task, ticks });
    Harness::from_vdom(vdom, options)
}

#[derive(Clone)]
struct StandRoot {
    task: LeanTask,
    ticks: TimerTicks,
}

fn stand_root(props: StandRoot) -> Element {
    use_hook(|| provide_context(props.ticks.clone()));
    task_in_shell(props.task.task(), EventHandler::new(|_| {}))
}

/// Boot Cells in a fresh document. Cells is no [`LeanTask`]: its screen passes the snapshot
/// text's budget and reads in parts.
pub fn boot_cells(options: HarnessOptions) -> Harness<DioxusDocument> {
    Harness::from_vdom(VirtualDom::new(cells_root), options)
}

fn cells_root() -> Element {
    task_in_shell(Task::Cells, EventHandler::new(|_| {}))
}

/// Boot the load page in a fresh document: its `load-fetch` button links a stylesheet that
/// shows the `load-done` message, and the in-process stand-in this boot alone carries answers
/// it after [`LOAD_ANSWER_DELAY`]. Every other field of `options` is kept.
pub fn boot_load(options: HarnessOptions) -> Harness<DioxusDocument> {
    Harness::from_vdom(
        VirtualDom::new(load_root),
        HarnessOptions {
            net_provider: Some(Arc::new(LoadNet)),
            ..options
        },
    )
}

/// How long the load page's stand-in holds a request before it answers.
pub const LOAD_ANSWER_DELAY: Duration = Duration::from_millis(200);

/// The stylesheet the load page links, under the `.test` TLD no resolver answers.
const LOAD_URL: &str = "http://stand.test/load.css";

/// The one rule the linked stylesheet holds: it shows the message.
const LOADED_CSS: &str = ".load-body .load-done { display: block; }";

/// A `NetProvider` that answers every request with [`LOADED_CSS`] from a thread of its own,
/// after [`LOAD_ANSWER_DELAY`].
struct LoadNet;

impl NetProvider for LoadNet {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        let url = request.url.to_string();
        thread::spawn(move || {
            thread::sleep(LOAD_ANSWER_DELAY);
            handler.bytes(url, Bytes::from_static(LOADED_CSS.as_bytes()));
        });
    }
}

fn load_root() -> Element {
    rsx! {
        TaskShell { title: "Load", on_back: EventHandler::new(|_| {}),
            LoadPage {}
        }
    }
}

#[component]
fn LoadPage() -> Element {
    let mut linked = use_signal(|| false);
    rsx! {
        div { class: "load-root",
            style { {LOAD_CSS} }
            div { class: "load-body",
                button { id: "load-fetch", class: "load-btn", onclick: move |_| linked.set(true), "Load" }
                if linked() {
                    link { rel: "stylesheet", href: LOAD_URL }
                }
                p { id: "load-done", class: "load-done", "Loaded" }
            }
        }
    }
}

const LOAD_CSS: &str = r#"
.load-root {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    background-color: #f5f5f5;
    font-family: sans-serif;
}

.load-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
}

.load-btn {
    font-size: 15px;
    font-weight: 500;
    color: #ffffff;
    background-color: #3a5ce5;
    border: none;
    border-radius: 6px;
    padding: 10px 24px;
    cursor: pointer;
}

.load-btn:hover {
    background-color: #2a4cd3;
}

.load-btn:active {
    background-color: #1f3fc1;
}

.load-done {
    display: none;
    font-size: 15px;
    color: #1a1a1a;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_timer_step_carries_milliseconds_onto_whole_ticks() {
        // One session's steps in order: the milliseconds handed, the clock's total after
        // them, and what the Timer reads then. The third and fourth rows carry a remainder.
        let rows = [
            (1, 1, "Elapsed: 0.0s"),
            (99, 100, "Elapsed: 0.1s"),
            (250, 350, "Elapsed: 0.3s"),
            (250, 600, "Elapsed: 0.6s"),
            (99, 699, "Elapsed: 0.6s"),
            (1, 700, "Elapsed: 0.7s"),
        ];
        assert_eq!(rows.len(), 6);
        assert_eq!(TIMER_TICK_MS, 100);
        let mut total = 0;
        for (row, (asked, held, _)) in rows.into_iter().enumerate() {
            total += asked;
            assert_eq!(total, held, "row {row}");
        }
        for incremental in [false, true] {
            let (mut harness, ticks) = boot_timer(options(incremental));
            let mut step = timer_step(ticks);
            for (row, (asked, _, elapsed)) in rows.into_iter().enumerate() {
                step(&mut harness, asked);
                harness.pump();
                assert!(
                    harness.text_content("#timer-elapsed") == elapsed,
                    "incremental={incremental}: row {row} reads its elapsed time"
                );
            }
        }
    }

    #[test]
    fn the_stand_options_stay_offline() {
        for incremental in [false, true] {
            assert!(
                options(incremental).net_provider.is_none(),
                "incremental={incremental}"
            );
        }
    }

    #[test]
    fn cells_boots_in_the_shell() {
        for incremental in [false, true] {
            let harness = boot_cells(options(incremental));
            assert!(
                harness.text_content("#task-title") == "Cells",
                "incremental={incremental}: the shell's title names Cells"
            );
            assert!(
                harness.query("#cell-A0").is_some() && harness.query("#cell-Z25").is_some(),
                "incremental={incremental}: the grid's first and last cells are mounted"
            );
        }
    }

    #[test]
    fn the_load_page_click_leaves_a_load_in_flight() {
        for incremental in [false, true] {
            let mut harness = boot_load(options(incremental));
            assert!(
                harness.text_content("#task-title") == "Load",
                "incremental={incremental}: the shell's title names the page"
            );
            assert!(
                harness.settle().is_ok(),
                "incremental={incremental}: nothing loads at boot"
            );
            harness.click("#load-fetch");
            assert!(
                harness.settle()
                    == Err(blitz_test_harness::NotSettled {
                        busy: blitz_test_harness::Busy::Loads
                    }),
                "incremental={incremental}: the click's stylesheet is a load in flight"
            );
        }
    }

    #[test]
    fn the_timer_step_takes_the_largest_advance() {
        for incremental in [false, true] {
            let (mut harness, ticks) = boot_timer(options(incremental));
            let mut step = timer_step(ticks);
            step(&mut harness, 60_000);
            harness.pump();
            assert!(
                harness.text_content("#timer-elapsed") != "Elapsed: 0.0s",
                "incremental={incremental}"
            );
        }
    }
}
