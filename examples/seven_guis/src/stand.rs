//! The headless stand: mounts one of the lean four 7GUIs tasks inside the stand's
//! `TaskShell` chrome with no window and no GPU, at one pinned viewport, with the
//! bundled DejaVu Sans and no network, building a fresh document per boot.

use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::shell::ColorScheme;
use dioxus_native::prelude::*;
use dioxus_native::{DioxusDocument, FontContext, build_single_font_ctx};

use crate::app::{Task, task_in_shell};
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
/// the milliseconds asked, it delivers the whole ticks they hold through `ticks` and returns
/// the milliseconds those ticks stand for. The remainder below one tick is dropped, and the
/// harness's animation clock stays where it was. The ticks apply on the harness's next pass.
pub fn timer_step(ticks: TimerTicks) -> impl FnMut(&mut Harness<DioxusDocument>, u32) -> u32 {
    move |_harness, ms| {
        let whole = ms / TIMER_TICK_MS;
        ticks.deliver(u64::from(whole));
        whole * TIMER_TICK_MS
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_timer_step_maps_milliseconds_onto_whole_ticks() {
        // Milliseconds asked, milliseconds moved, and what the Timer reads after them.
        let rows = [
            (1, 0, Some("Elapsed: 0.0s")),
            (99, 0, Some("Elapsed: 0.0s")),
            (100, 100, Some("Elapsed: 0.1s")),
            (250, 200, Some("Elapsed: 0.2s")),
            (60_000, 60_000, None),
        ];
        assert_eq!(rows.len(), 5);
        assert_eq!(TIMER_TICK_MS, 100);
        for incremental in [false, true] {
            for (row, (asked, moved, elapsed)) in rows.into_iter().enumerate() {
                let (mut harness, ticks) = boot_timer(options(incremental));
                let mut step = timer_step(ticks);
                let clock = harness.time();
                assert!(
                    step(&mut harness, asked) == moved,
                    "incremental={incremental}: row {row}"
                );
                assert!(
                    harness.time() == clock,
                    "incremental={incremental}: row {row} leaves the animation clock"
                );
                if let Some(elapsed) = elapsed {
                    harness.pump();
                    assert!(
                        harness.text_content("#timer-elapsed") == elapsed,
                        "incremental={incremental}: row {row} reads its elapsed time"
                    );
                }
            }
        }
    }
}
