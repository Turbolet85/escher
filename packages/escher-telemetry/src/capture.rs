//! The per-step capture: how many records a stretch of code caused, by level alone, from the
//! targets the sink has no rule for.
//!
//! [`capture`] runs a closure with a capture open on the calling thread; the [`CaptureLayer`]
//! counts each event dispatched there while it is open. A record from an engine target
//! ([`ENGINE_TARGET_PREFIXES`]) or one of escher's own ([`ESCHER_TARGET_PREFIXES`]) is never
//! counted, nor is one below `warn` from the UI framework's own crates
//! ([`FRAMEWORK_TARGET_PREFIXES`]), which trace every re-render. The layer never reads a
//! message, a field's value or the target beyond that admission, and keeps nothing but counts.

use std::cell::RefCell;
use std::fmt;
use std::sync::OnceLock;

use tracing::{Dispatch, Event, Level, Subscriber};
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};

use crate::format::{ENGINE_TARGET_PREFIXES, ESCHER_TARGET_PREFIXES};

/// Target prefixes of the UI framework's crates: a record from one of them is counted only at
/// `warn` and `error`, since they log at `trace` on every re-render and signal write.
pub const FRAMEWORK_TARGET_PREFIXES: &[&str] = &["dioxus", "generational_box", "warnings"];

/// The levels a capture counts, in the order [`Captured`] holds them.
pub const CAPTURED_LEVELS: [Level; 5] = [
    Level::ERROR,
    Level::WARN,
    Level::INFO,
    Level::DEBUG,
    Level::TRACE,
];

fn starts_with_any(target: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| target.starts_with(prefix))
}

fn level_index(level: Level) -> usize {
    match level {
        Level::ERROR => 0,
        Level::WARN => 1,
        Level::INFO => 2,
        Level::DEBUG => 3,
        Level::TRACE => 4,
    }
}

/// Whether a record of `level` under `target` is counted.
fn counted(target: &str, level: Level) -> bool {
    if starts_with_any(target, ENGINE_TARGET_PREFIXES)
        || starts_with_any(target, ESCHER_TARGET_PREFIXES)
    {
        return false;
    }
    !starts_with_any(target, FRAMEWORK_TARGET_PREFIXES) || level_index(level) <= 1
}

thread_local! {
    static OPEN: RefCell<Vec<[u32; 5]>> = const { RefCell::new(Vec::new()) };
}

/// The capture's own dispatcher where the default holds no [`CaptureLayer`]: built once for the
/// process and never dropped, beside a twin that is never used. While tracing-core holds a
/// single registered dispatcher it takes a new callsite's interest from the calling thread's
/// default alone, so a callsite first reached outside a capture under no subscriber would cache
/// `never` and later captures would miss it; with two always registered, every callsite's
/// interest is read from every live dispatcher, this one among them.
static SCOPED: OnceLock<[Dispatch; 2]> = OnceLock::new();

fn scoped() -> &'static Dispatch {
    let [dispatch, _twin] = SCOPED.get_or_init(|| {
        let build = || Dispatch::new(tracing_subscriber::registry().with(CaptureLayer));
        [build(), build()]
    });
    dispatch
}

/// The layer that counts the records dispatched while a capture is open on their thread.
///
/// It sits unfiltered beside the sink's own layer in the one subscriber [`init`](crate::init)
/// installs, so it sees every level whatever the sink prints.
#[derive(Debug, Clone, Copy, Default)]
pub struct CaptureLayer;

impl<S: Subscriber> Layer<S> for CaptureLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let normalized = event.normalized_metadata();
        let metadata = normalized.as_ref().unwrap_or_else(|| event.metadata());
        let level = *metadata.level();
        if !counted(metadata.target(), level) {
            return;
        }
        let _ = OPEN.try_with(|open| {
            if let Ok(mut open) = open.try_borrow_mut()
                && let Some(counts) = open.last_mut()
            {
                let count = &mut counts[level_index(level)];
                *count = count.saturating_add(1);
            }
        });
    }
}

/// How many records a capture counted, per level.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Captured {
    counts: [u32; 5],
}

impl Captured {
    /// The count of records at `level`.
    pub fn count(&self, level: Level) -> u32 {
        self.counts[level_index(level)]
    }

    /// Each level with its count, in the order of [`CAPTURED_LEVELS`].
    pub fn levels(&self) -> impl Iterator<Item = (Level, u32)> + '_ {
        CAPTURED_LEVELS
            .iter()
            .map(|level| (*level, self.count(*level)))
    }

    /// Whether nothing was counted.
    pub fn is_empty(&self) -> bool {
        self.counts.iter().all(|count| *count == 0)
    }
}

impl fmt::Debug for Captured {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [error, warn, info, debug, trace] = self.counts;
        f.debug_struct("Captured")
            .field("error", &error)
            .field("warn", &warn)
            .field("info", &info)
            .field("debug", &debug)
            .field("trace", &trace)
            .finish()
    }
}

/// The open capture of one [`capture`] call, closed on drop, an unwind included.
struct Open {
    depth: usize,
}

impl Open {
    fn push() -> Open {
        let depth = OPEN.with(|open| {
            let mut open = open.borrow_mut();
            open.push([0; 5]);
            open.len()
        });
        Open { depth }
    }

    fn close(self) -> Captured {
        let counts = OPEN.with(|open| {
            let mut open = open.borrow_mut();
            open.truncate(self.depth);
            open.pop().unwrap_or_default()
        });
        std::mem::forget(self);
        Captured { counts }
    }
}

impl Drop for Open {
    fn drop(&mut self) {
        let _ = OPEN.try_with(|open| {
            if let Ok(mut open) = open.try_borrow_mut() {
                open.truncate(self.depth.saturating_sub(1));
            }
        });
    }
}

/// Runs `f` with a capture open on this thread, and returns what it returned with the records
/// it caused, counted per level.
///
/// The counts go through the current default dispatcher's [`CaptureLayer`] where it holds one;
/// otherwise — no subscriber, or one without the layer — `f` runs under a dispatcher holding
/// the layer alone, so it prints nothing.
pub fn capture<R>(f: impl FnOnce() -> R) -> (R, Captured) {
    let held = tracing::dispatcher::get_default(|dispatch| {
        dispatch.downcast_ref::<CaptureLayer>().is_some()
    });
    let open = Open::push();
    let result = if held {
        f()
    } else {
        tracing::dispatcher::with_default(scoped(), f)
    };
    (result, open.close())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt::MakeWriter;

    #[test]
    fn a_record_is_counted_by_its_target_and_level() {
        let rows: [(&str, [bool; 5]); 10] = [
            ("blitz_dom::layout", [false; 5]),
            ("dioxus_native_dom::events", [false; 5]),
            ("js_console", [false; 5]),
            ("escher_driver", [false; 5]),
            ("escher_telemetry::panic", [false; 5]),
            ("dioxus_core::runtime", [true, true, false, false, false]),
            ("dioxus_signals::signal", [true, true, false, false, false]),
            (
                "generational_box::unsync",
                [true, true, false, false, false],
            ),
            ("warnings::warnings", [true, true, false, false, false]),
            ("stand_app::handlers", [true; 5]),
        ];
        assert_eq!(rows.len(), 10);
        for (row, (target, expected)) in rows.iter().enumerate() {
            for (index, level) in CAPTURED_LEVELS.iter().enumerate() {
                assert_eq!(
                    counted(target, *level),
                    expected[index],
                    "row {row}, level {index}"
                );
            }
        }
    }

    fn emit_one_of_each() {
        tracing::error!(target: "blitz_dom", "engine");
        tracing::warn!(target: "escher_driver", "escher");
        tracing::trace!(target: "dioxus_core::diff", "framework below warn");
        tracing::warn!(target: "dioxus_core::runtime", "framework at warn");
        tracing::error!(target: "stand_app", "app");
        tracing::warn!(target: "stand_app", "app");
        tracing::info!(target: "stand_app", "app");
        tracing::debug!(target: "stand_app", "app");
        tracing::trace!(target: "stand_app", "app");
        tracing::info!(target: "stand_app", "app again");
    }

    const EXPECTED: [u32; 5] = [1, 2, 2, 1, 1];

    fn counts(captured: &Captured) -> [u32; 5] {
        let rows: Vec<u32> = captured.levels().map(|(_, count)| count).collect();
        assert_eq!(rows.len(), 5);
        [rows[0], rows[1], rows[2], rows[3], rows[4]]
    }

    #[test]
    fn a_capture_counts_the_records_its_closure_caused() {
        let ((), captured) = capture(emit_one_of_each);
        assert_eq!(counts(&captured), EXPECTED);
        assert!(!captured.is_empty());
    }

    #[test]
    fn nothing_is_counted_outside_a_capture() {
        emit_one_of_each();
        let ((), captured) = capture(|| ());
        assert!(captured.is_empty());
        emit_one_of_each();
        let ((), captured) = capture(|| ());
        assert_eq!(counts(&captured), [0; 5]);
    }

    #[derive(Clone, Default)]
    struct Discard;

    impl<'w> MakeWriter<'w> for Discard {
        type Writer = std::io::Sink;
        fn make_writer(&'w self) -> Self::Writer {
            std::io::sink()
        }
    }

    #[test]
    fn the_counts_are_the_same_under_the_sinks_subscriber_none_and_a_foreign_one() {
        let ((), none) = capture(emit_one_of_each);

        let sink = Dispatch::new(crate::subscriber(
            crate::service_identity!(),
            Discard,
            tracing_subscriber::EnvFilter::new("warn"),
        ));
        assert!(sink.downcast_ref::<CaptureLayer>().is_some());
        let ((), held) = tracing::dispatcher::with_default(&sink, || capture(emit_one_of_each));

        let foreign = Dispatch::new(tracing_subscriber::registry());
        let ((), other) = tracing::dispatcher::with_default(&foreign, || capture(emit_one_of_each));

        let rows = [none, held, other];
        assert_eq!(rows.len(), 3);
        for (row, captured) in rows.iter().enumerate() {
            assert_eq!(counts(captured), EXPECTED, "row {row}");
        }
    }

    #[test]
    fn a_capture_closed_by_an_unwind_leaves_none_open() {
        let unwound = std::panic::catch_unwind(|| capture(|| panic!("inside")));
        assert!(unwound.is_err());
        assert!(OPEN.with(|open| open.borrow().is_empty()));
        let ((), captured) = capture(emit_one_of_each);
        assert_eq!(counts(&captured), EXPECTED);
    }

    #[test]
    fn the_debug_form_prints_the_counts_alone() {
        let ((), captured) = capture(emit_one_of_each);
        assert_eq!(
            format!("{captured:?}"),
            "Captured { error: 1, warn: 2, info: 2, debug: 1, trace: 1 }"
        );
    }
}
