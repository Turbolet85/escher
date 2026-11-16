//! The per-step capture: the records and panics a stretch of code caused, with their text,
//! scrubbed by field name and bounded.
//!
//! [`capture`] runs a closure with a capture open on the calling thread; the [`CaptureLayer`]
//! keeps each event dispatched there while it is open, and a panic hook the first capture
//! installs keeps each panic. A record is admitted by its [`Source`]: the app's at every level,
//! the UI framework's ([`FRAMEWORK_TARGET_PREFIXES`]) and the engine's
//! ([`ENGINE_TARGET_PREFIXES`]) at `warn` and `error` alone — they trace every re-render and
//! focus move below that — and escher's own ([`ESCHER_TARGET_PREFIXES`]) never.
//!
//! Every admitted record keeps its target, level, message, fields and location. A field named in
//! [`ANSWER_HIDDEN_FIELDS`] keeps its name and [`REDACTED`] in place of its value, whatever the
//! source; a field named `error`, the message and every other field are kept as written. Each text is cut at [`TEXT_CUT`] bytes,
//! the bytes cut counted beside it, a record keeps at most [`FIELDS_KEPT`] fields and a capture
//! at most [`RECORDS_KEPT`] records and panics, the rest counted. Nothing here prints, logs or
//! fields what it keeps: it is read back through [`Captured`] alone.

use std::cell::RefCell;
use std::fmt;
use std::panic::{self, PanicHookInfo};
use std::sync::{Once, OnceLock};

use tracing::field::{Field, Visit};
use tracing::{Dispatch, Event, Level, Subscriber};
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};

use crate::format::{ENGINE_TARGET_PREFIXES, ESCHER_TARGET_PREFIXES, REDACTED};

/// Target prefixes of the UI framework's crates: a record from one of them is admitted only at
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

/// The bytes a kept text holds at most: a message, a field's value or a panic's payload. The
/// longest admitted message measured on the stand and on real-failure fixtures read 205 bytes
/// (the engine's unparseable-link warning, before its value moved into a field), so the usual
/// text arrives whole.
pub const TEXT_CUT: usize = 1024;

/// The fields a kept record holds at most; the rest are counted. The most a measured admitted
/// record carried was 2.
pub const FIELDS_KEPT: usize = 8;

/// The records, and separately the panics, a capture keeps at most; the rest are counted. At
/// most 2 admitted records crossed one measured step.
pub const RECORDS_KEPT: usize = 16;

/// The field names a kept record reads as [`REDACTED`]: the sink's
/// [`CONTENT_FIELDS`](crate::CONTENT_FIELDS) without `error`, written out so a change to the
/// sink's set never moves the answer. A field named `error` is kept as written, since a failure
/// record carries its text there (a ratified boundary widening); the sink's own set and lines are
/// unchanged.
pub(crate) const ANSWER_HIDDEN_FIELDS: &[&str] = &[
    "url",
    "href",
    "src",
    "html",
    "text",
    "value",
    "attrs",
    "path",
    "request",
    "panic.payload",
    "password",
];

/// Where a kept record came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// The app, or any crate outside the framework's, the engine's and escher's target sets.
    App,
    /// The UI framework ([`FRAMEWORK_TARGET_PREFIXES`]).
    Framework,
    /// The engine ([`ENGINE_TARGET_PREFIXES`]).
    Engine,
}

impl Source {
    /// Every source, in the order [`Captured::records_past`] is read.
    pub const ALL: [Source; 3] = [Source::App, Source::Framework, Source::Engine];

    /// The source's closed word.
    pub fn name(self) -> &'static str {
        match self {
            Source::App => "app",
            Source::Framework => "framework",
            Source::Engine => "engine",
        }
    }

    fn index(self) -> usize {
        match self {
            Source::App => 0,
            Source::Framework => 1,
            Source::Engine => 2,
        }
    }
}

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

/// The source of a record of `level` under `target`, where it is admitted.
fn admitted(target: &str, level: Level) -> Option<Source> {
    let warn_up = level_index(level) <= 1;
    if starts_with_any(target, ESCHER_TARGET_PREFIXES) {
        None
    } else if starts_with_any(target, ENGINE_TARGET_PREFIXES) {
        warn_up.then_some(Source::Engine)
    } else if starts_with_any(target, FRAMEWORK_TARGET_PREFIXES) {
        warn_up.then_some(Source::Framework)
    } else {
        Some(Source::App)
    }
}

/// A kept text: its first bytes, at most [`TEXT_CUT`], and how many were cut after them.
#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    /// The text kept, cut on a character boundary.
    pub text: String,
    /// The bytes cut from its end; 0 for a text kept whole.
    pub cut: usize,
}

impl Text {
    /// `text` cut at [`TEXT_CUT`] bytes on a character boundary.
    pub fn cut(mut text: String) -> Text {
        let len = text.len();
        if len <= TEXT_CUT {
            return Text { text, cut: 0 };
        }
        let mut end = TEXT_CUT;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
        Text {
            text,
            cut: len - end,
        }
    }
}

/// One field of a kept record.
#[derive(Clone, PartialEq, Eq)]
pub struct KeptField {
    /// The field's name.
    pub name: String,
    /// Its value, or [`REDACTED`] where the name is in [`ANSWER_HIDDEN_FIELDS`]; a field named
    /// `error` keeps its value.
    pub value: Text,
}

/// One admitted record a capture kept.
#[derive(Clone, PartialEq, Eq)]
pub struct Record {
    /// Where it came from.
    pub source: Source,
    /// Its target; a bridged `log` record's own.
    pub target: String,
    /// Its level.
    pub level: Level,
    /// Its message; empty where it has none.
    pub message: Text,
    /// Its fields in recorded order, the `log.*` bridge fields left out, at most
    /// [`FIELDS_KEPT`].
    pub fields: Vec<KeptField>,
    /// The fields past [`FIELDS_KEPT`].
    pub fields_past: u32,
    /// The file it was recorded in, as the compiler wrote it.
    pub file: Option<String>,
    /// Its line.
    pub line: Option<u32>,
}

/// One panic a capture kept.
#[derive(Clone, PartialEq, Eq)]
pub struct Panic {
    /// Its payload where it is a string, else `<non-string payload>`.
    pub payload: Text,
    /// The file it panicked in, as the compiler wrote it.
    pub file: Option<String>,
    /// Its line.
    pub line: Option<u32>,
    /// Its column.
    pub column: Option<u32>,
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Text")
            .field("bytes", &self.text.len())
            .field("cut", &self.cut)
            .finish()
    }
}

impl fmt::Debug for KeptField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("KeptField")
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Record")
            .field("source", &self.source)
            .field("level", &self.level)
            .field("fields", &self.fields.len())
            .finish()
    }
}

impl fmt::Debug for Panic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Panic")
    }
}

/// What one open capture has kept so far.
#[derive(Default)]
struct Frame {
    counts: [u32; 5],
    records: Vec<Record>,
    records_past: [u32; 3],
    panics: Vec<Panic>,
    panics_past: u32,
}

thread_local! {
    static OPEN: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) };
}

/// Runs `f` on the innermost capture open on this thread, if any.
fn with_frame(f: impl FnOnce(&mut Frame)) {
    let _ = OPEN.try_with(|open| {
        if let Ok(mut open) = open.try_borrow_mut()
            && let Some(frame) = open.last_mut()
        {
            f(frame);
        }
    });
}

/// The capture's own dispatcher where the default holds no [`CaptureLayer`]: built once for the
/// process and never dropped.
static SCOPED: OnceLock<[Dispatch; 2]> = OnceLock::new();

fn scoped() -> &'static Dispatch {
    // The twin is never used: it keeps tracing-core out of its one-dispatcher mode, where a call
    // site's interest is cached from the calling thread's default alone, as `never` with none.
    let [dispatch, _twin] = SCOPED.get_or_init(|| {
        let build = || Dispatch::new(tracing_subscriber::registry().with(CaptureLayer));
        [build(), build()]
    });
    dispatch
}

/// Reads an event's message and fields, each content-named field's value never formatted.
struct Values {
    message: Option<String>,
    fields: Vec<KeptField>,
    fields_past: u32,
}

impl Values {
    fn keep(&mut self, field: &Field, value: impl FnOnce() -> String) {
        let name = field.name();
        if name == "message" {
            self.message = Some(value());
        } else if name.starts_with("log.") {
        } else if self.fields.len() >= FIELDS_KEPT {
            self.fields_past = self.fields_past.saturating_add(1);
        } else {
            let value = if ANSWER_HIDDEN_FIELDS.contains(&name) {
                Text {
                    text: REDACTED.to_string(),
                    cut: 0,
                }
            } else {
                Text::cut(value())
            };
            self.fields.push(KeptField {
                name: name.to_string(),
                value,
            });
        }
    }
}

impl Visit for Values {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.keep(field, || value.to_string());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.keep(field, || format!("{value:?}"));
    }
}

/// The layer that keeps the records dispatched while a capture is open on their thread.
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
        let Some(source) = admitted(metadata.target(), level) else {
            return;
        };
        let open = OPEN
            .try_with(|open| open.try_borrow().is_ok_and(|open| !open.is_empty()))
            .unwrap_or(false);
        if !open {
            return;
        }
        let mut values = Values {
            message: None,
            fields: Vec::new(),
            fields_past: 0,
        };
        event.record(&mut values);
        let record = Record {
            source,
            target: metadata.target().to_string(),
            level,
            message: Text::cut(values.message.unwrap_or_default()),
            fields: values.fields,
            fields_past: values.fields_past,
            file: metadata.file().map(str::to_string),
            line: metadata.line(),
        };
        with_frame(|frame| {
            let count = &mut frame.counts[level_index(level)];
            *count = count.saturating_add(1);
            if frame.records.len() < RECORDS_KEPT {
                frame.records.push(record);
            } else {
                let past = &mut frame.records_past[source.index()];
                *past = past.saturating_add(1);
            }
        });
    }
}

/// What a capture kept: the admitted records with their counts per level, and the panics.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Captured {
    counts: [u32; 5],
    records: Vec<Record>,
    records_past: [u32; 3],
    panics: Vec<Panic>,
    panics_past: u32,
}

impl Captured {
    /// The count of admitted records at `level`, kept or past the bound.
    pub fn count(&self, level: Level) -> u32 {
        self.counts[level_index(level)]
    }

    /// Each level with its count, in the order of [`CAPTURED_LEVELS`].
    pub fn levels(&self) -> impl Iterator<Item = (Level, u32)> + '_ {
        CAPTURED_LEVELS
            .iter()
            .map(|level| (*level, self.count(*level)))
    }

    /// The records kept, in the order they were recorded.
    pub fn records(&self) -> &[Record] {
        &self.records
    }

    /// The records of `source` past [`RECORDS_KEPT`].
    pub fn records_past(&self, source: Source) -> u32 {
        self.records_past[source.index()]
    }

    /// The panics kept, in the order they happened.
    pub fn panics(&self) -> &[Panic] {
        &self.panics
    }

    /// The panics past [`RECORDS_KEPT`].
    pub fn panics_past(&self) -> u32 {
        self.panics_past
    }

    /// Whether nothing was admitted and nothing panicked.
    pub fn is_empty(&self) -> bool {
        self.counts.iter().all(|count| *count == 0) && self.panics.is_empty()
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

/// Keeps `info`'s panic on the capture open on the panicking thread, if any.
fn keep_panic(info: &PanicHookInfo<'_>) {
    with_frame(|frame| {
        if frame.panics.len() >= RECORDS_KEPT {
            frame.panics_past = frame.panics_past.saturating_add(1);
            return;
        }
        let payload = info.payload_as_str().unwrap_or("<non-string payload>");
        let location = info.location();
        frame.panics.push(Panic {
            payload: Text::cut(payload.to_string()),
            file: location.map(|location| location.file().to_string()),
            line: location.map(|location| location.line()),
            column: location.map(|location| location.column()),
        });
    });
}

static HOOK: Once = Once::new();

/// Sets, once per process, a panic hook that keeps a panic on the capture open on its thread,
/// then runs the hook it replaced. Outside a capture it only runs the hook it replaced.
fn install_hook() {
    HOOK.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info: &PanicHookInfo<'_>| {
            keep_panic(info);
            previous(info);
        }));
    });
}

/// The open capture of one [`capture`] call, closed on drop, an unwind included.
struct Open {
    depth: usize,
}

impl Open {
    fn push() -> Open {
        let depth = OPEN.with(|open| {
            let mut open = open.borrow_mut();
            open.push(Frame::default());
            open.len()
        });
        Open { depth }
    }

    fn close(self) -> Captured {
        let frame = OPEN.with(|open| {
            let mut open = open.borrow_mut();
            open.truncate(self.depth);
            open.pop().unwrap_or_default()
        });
        std::mem::forget(self);
        Captured {
            counts: frame.counts,
            records: frame.records,
            records_past: frame.records_past,
            panics: frame.panics,
            panics_past: frame.panics_past,
        }
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
/// and panics it caused.
///
/// The records go through the current default dispatcher's [`CaptureLayer`] where it holds one;
/// otherwise — no subscriber, or one without the layer — `f` runs under a dispatcher holding
/// the layer alone, so it prints nothing. The first call installs the process's capture panic
/// hook, which chains the hook it found.
pub fn capture<R>(f: impl FnOnce() -> R) -> (R, Captured) {
    install_hook();
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
    fn a_record_is_admitted_by_its_target_and_level() {
        let warn_up = [true, true, false, false, false];
        let rows: [(&str, Option<Source>, [bool; 5]); 10] = [
            ("blitz_dom::layout", Some(Source::Engine), warn_up),
            ("dioxus_native_dom::events", Some(Source::Engine), warn_up),
            ("js_console", Some(Source::Engine), warn_up),
            ("escher_driver", None, [false; 5]),
            ("escher_telemetry::panic", None, [false; 5]),
            ("dioxus_core::runtime", Some(Source::Framework), warn_up),
            ("dioxus_signals::signal", Some(Source::Framework), warn_up),
            ("generational_box::unsync", Some(Source::Framework), warn_up),
            ("warnings::warnings", Some(Source::Framework), warn_up),
            ("stand_app::handlers", Some(Source::App), [true; 5]),
        ];
        assert_eq!(rows.len(), 10);
        for (row, (target, source, expected)) in rows.iter().enumerate() {
            for (index, level) in CAPTURED_LEVELS.iter().enumerate() {
                let read = admitted(target, *level);
                assert_eq!(read.is_some(), expected[index], "row {row}, level {index}");
                if read.is_some() {
                    assert_eq!(read, *source, "row {row}, level {index}");
                }
            }
        }
        let words: Vec<&str> = Source::ALL.iter().map(|source| source.name()).collect();
        assert_eq!(words, ["app", "framework", "engine"]);
    }

    fn emit_one_of_each() {
        tracing::error!(target: "blitz_dom", "engine");
        tracing::warn!(target: "escher_driver", "escher");
        tracing::trace!(target: "dioxus_core::diff", "framework below warn");
        tracing::warn!(target: "dioxus_core::runtime", "framework at warn");
        tracing::debug!(target: "blitz_dom::document", "engine below warn");
        tracing::error!(target: "stand_app", "app");
        tracing::warn!(target: "stand_app", "app");
        tracing::info!(target: "stand_app", "app");
        tracing::debug!(target: "stand_app", "app");
        tracing::trace!(target: "stand_app", "app");
        tracing::info!(target: "stand_app", "app again");
    }

    const EXPECTED: [u32; 5] = [2, 2, 2, 1, 1];

    fn counts(captured: &Captured) -> [u32; 5] {
        let rows: Vec<u32> = captured.levels().map(|(_, count)| count).collect();
        assert_eq!(rows.len(), 5);
        [rows[0], rows[1], rows[2], rows[3], rows[4]]
    }

    #[test]
    fn a_capture_keeps_the_records_its_closure_caused() {
        let ((), captured) = capture(emit_one_of_each);
        assert_eq!(counts(&captured), EXPECTED);
        assert!(!captured.is_empty());
        let kept: Vec<(Source, &str, &str)> = captured
            .records()
            .iter()
            .map(|record| {
                (
                    record.source,
                    record.target.as_str(),
                    record.message.text.as_str(),
                )
            })
            .collect();
        let expected = [
            (Source::Engine, "blitz_dom", "engine"),
            (
                Source::Framework,
                "dioxus_core::runtime",
                "framework at warn",
            ),
            (Source::App, "stand_app", "app"),
            (Source::App, "stand_app", "app"),
            (Source::App, "stand_app", "app"),
            (Source::App, "stand_app", "app"),
            (Source::App, "stand_app", "app"),
            (Source::App, "stand_app", "app again"),
        ];
        assert_eq!(expected.len(), 8);
        assert_eq!(kept, expected);
        let first = &captured.records()[0];
        assert_eq!(first.level, Level::ERROR);
        assert_eq!(first.file.as_deref(), Some(file!()));
        assert!(first.line.is_some_and(|line| line > 0));
    }

    #[test]
    fn a_content_named_field_is_hidden_on_every_source_and_the_rest_kept() {
        let ((), captured) = capture(|| {
            tracing::warn!(target: "stand_app", seats = 4, password = "s3cret", value = "typed", "booking rejected");
            tracing::warn!(target: "dioxus_core::scope_arena", value = "typed", seats = 4, "framework says");
            tracing::warn!(target: "blitz_dom::events::pointer", href = "http://[::1", node_id = 7, "engine says");
        });
        let rows: [(Source, &str, &[(&str, &str)]); 3] = [
            (
                Source::App,
                "booking rejected",
                &[("seats", "4"), ("password", REDACTED), ("value", REDACTED)],
            ),
            (
                Source::Framework,
                "framework says",
                &[("value", REDACTED), ("seats", "4")],
            ),
            (
                Source::Engine,
                "engine says",
                &[("href", REDACTED), ("node_id", "7")],
            ),
        ];
        assert_eq!(rows.len(), 3);
        assert_eq!(captured.records().len(), rows.len());
        for (row, (record, (source, message, fields))) in
            captured.records().iter().zip(rows).enumerate()
        {
            assert_eq!(record.source, source, "row {row}");
            assert_eq!(record.message.text, message, "row {row}");
            let kept: Vec<(&str, &str)> = record
                .fields
                .iter()
                .map(|field| (field.name.as_str(), field.value.text.as_str()))
                .collect();
            assert_eq!(kept, fields, "row {row}");
        }
    }

    #[test]
    fn of_the_sinks_content_names_error_alone_keeps_its_text() {
        let ((), captured) = capture(|| {
            tracing::warn!(target: "stand_app", url = "logged", "row");
            tracing::warn!(target: "stand_app", href = "logged", "row");
            tracing::warn!(target: "stand_app", src = "logged", "row");
            tracing::warn!(target: "stand_app", html = "logged", "row");
            tracing::warn!(target: "stand_app", text = "logged", "row");
            tracing::warn!(target: "stand_app", value = "logged", "row");
            tracing::warn!(target: "stand_app", attrs = "logged", "row");
            tracing::warn!(target: "stand_app", path = "logged", "row");
            tracing::warn!(target: "stand_app", request = "logged", "row");
            tracing::warn!(target: "stand_app", error = "logged", "row");
            tracing::event!(target: "stand_app", Level::WARN, { panic.payload = "logged" }, "row");
            tracing::warn!(target: "stand_app", password = "logged", "row");
        });
        let rows: [(&str, &str); 12] = [
            ("url", "[redacted]"),
            ("href", "[redacted]"),
            ("src", "[redacted]"),
            ("html", "[redacted]"),
            ("text", "[redacted]"),
            ("value", "[redacted]"),
            ("attrs", "[redacted]"),
            ("path", "[redacted]"),
            ("request", "[redacted]"),
            ("error", "logged"),
            ("panic.payload", "[redacted]"),
            ("password", "[redacted]"),
        ];
        assert_eq!(rows.len(), 12);
        assert_eq!(captured.records().len(), rows.len());
        for (row, (record, (name, value))) in captured.records().iter().zip(rows).enumerate() {
            assert_eq!(record.fields.len(), 1, "row {row}");
            assert_eq!(record.fields[0].name, name, "row {row}");
            assert_eq!(record.fields[0].value.text, value, "row {row}");
        }
    }

    #[test]
    fn a_text_is_cut_on_a_char_boundary_and_says_how_much() {
        let rows: [(String, usize, usize); 5] = [
            (String::new(), 0, 0),
            ("a".repeat(TEXT_CUT), TEXT_CUT, 0),
            ("a".repeat(TEXT_CUT + 1), TEXT_CUT, 1),
            ("a".repeat(3000), TEXT_CUT, 3000 - TEXT_CUT),
            (format!("{}é", "a".repeat(TEXT_CUT - 1)), TEXT_CUT - 1, 2),
        ];
        assert_eq!(rows.len(), 5);
        for (row, (text, kept, cut)) in rows.into_iter().enumerate() {
            let len = text.len();
            let read = Text::cut(text);
            assert_eq!(read.text.len(), kept, "row {row}");
            assert_eq!(read.cut, cut, "row {row}");
            assert_eq!(read.text.len() + read.cut, len, "row {row}");
        }
    }

    #[test]
    fn fields_and_records_past_their_bounds_are_counted_by_source() {
        let ((), captured) = capture(|| {
            tracing::warn!(
                target: "stand_app",
                f0 = 0, f1 = 1, f2 = 2, f3 = 3, f4 = 4, f5 = 5, f6 = 6, f7 = 7, f8 = 8, f9 = 9,
                "many fields"
            );
            for _ in 0..RECORDS_KEPT {
                tracing::info!(target: "stand_app", "fill");
            }
            tracing::warn!(target: "dioxus_core::runtime", "framework past");
            tracing::warn!(target: "blitz_dom", "engine past");
            tracing::warn!(target: "blitz_dom", "engine past");
            tracing::info!(target: "stand_app", "app past");
        });
        let first = &captured.records()[0];
        assert_eq!(first.fields.len(), FIELDS_KEPT);
        assert_eq!(first.fields_past, 2);
        assert_eq!(captured.records().len(), RECORDS_KEPT);
        let past: Vec<u32> = Source::ALL
            .iter()
            .map(|source| captured.records_past(*source))
            .collect();
        assert_eq!(past, [2, 1, 2]);
        assert_eq!(captured.count(Level::WARN), 4);
        assert_eq!(captured.count(Level::INFO), 17);
    }

    #[test]
    fn a_panic_inside_a_capture_is_kept_with_its_location() {
        let (unwound, captured) = capture(|| {
            std::panic::catch_unwind(|| {
                let rows: Vec<u32> = Vec::new();
                rows[3]
            })
        });
        assert!(unwound.is_err());
        assert_eq!(captured.panics().len(), 1);
        let panic = &captured.panics()[0];
        assert_eq!(
            panic.payload.text,
            "index out of bounds: the len is 0 but the index is 3"
        );
        assert_eq!(panic.file.as_deref(), Some(file!()));
        assert!(panic.line.is_some_and(|line| line > 0));
        assert!(!captured.is_empty());
    }

    #[test]
    fn nothing_is_kept_outside_a_capture() {
        emit_one_of_each();
        let ((), captured) = capture(|| ());
        assert!(captured.is_empty());
        emit_one_of_each();
        let ((), captured) = capture(|| ());
        assert_eq!(counts(&captured), [0; 5]);
        assert!(captured.records().is_empty());
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
    fn the_records_are_the_same_under_the_sinks_subscriber_none_and_a_foreign_one() {
        let ((), none) = capture(emit_one_of_each);

        let sink = Dispatch::new(crate::subscriber(
            crate::service_identity!(),
            Discard,
            tracing::level_filters::LevelFilter::WARN,
        ));
        assert!(sink.downcast_ref::<CaptureLayer>().is_some());
        let ((), held) = tracing::dispatcher::with_default(&sink, || capture(emit_one_of_each));

        let foreign = Dispatch::new(tracing_subscriber::registry());
        let ((), other) = tracing::dispatcher::with_default(&foreign, || capture(emit_one_of_each));

        let rows = [none, held, other];
        assert_eq!(rows.len(), 3);
        for (row, captured) in rows.iter().enumerate() {
            assert_eq!(counts(captured), EXPECTED, "row {row}");
            assert_eq!(captured.records().len(), 8, "row {row}");
        }
        assert!(rows[0] == rows[1] && rows[1] == rows[2]);
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
            "Captured { error: 2, warn: 2, info: 2, debug: 1, trace: 1 }"
        );
        let record = &captured.records()[0];
        assert_eq!(
            format!("{record:?}"),
            "Record { source: Engine, level: Level(Error), fields: 0 }"
        );
    }
}
