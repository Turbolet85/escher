use std::fmt;

use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber, span};
use tracing_log::NormalizeEvent;
use tracing_subscriber::field::RecordFields;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{FormatTime, SystemTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::registry::LookupSpan;

use crate::ServiceIdentity;

/// Target prefixes of the engine crates (and the `js_console` log target). Events whose target
/// starts with one of these print only the fields in [`SAFE_FIELDS`]; every other field, the
/// message included, is redacted — engine call sites interpolate URLs, text and attribute values
/// into their messages (obs-plan §8).
///
/// Together with [`ESCHER_TARGET_PREFIXES`] this is the sink's allowlist of targets: an event
/// whose target starts with a prefix of neither set is dropped — nothing is written for it, at
/// any level and whatever `RUST_LOG` names.
pub const ENGINE_TARGET_PREFIXES: &[&str] = &[
    "blitz",
    "dioxus_native",
    "stylo_taffy",
    "accesskit_xplat",
    "debug_timer",
    "js_console",
];

/// Target prefixes of escher's own crates. Events whose target starts with one of these print
/// every field not named in [`CONTENT_FIELDS`]. The underscore is part of the prefix: a target
/// is escher's by its crate name, never by opening with the same letters.
pub const ESCHER_TARGET_PREFIXES: &[&str] = &["escher_"];

/// Fields an engine-target event may print: node ids, statuses, counts, CSS property names, and
/// the location fields of a bridged `log` record.
pub const SAFE_FIELDS: &[&str] = &[
    "node_id",
    "status",
    "waiting_nodes",
    "property",
    "log.module_path",
    "log.file",
    "log.line",
];

/// Fields that carry user content, redacted whatever the event's target.
pub const CONTENT_FIELDS: &[&str] = &[
    "url",
    "href",
    "src",
    "html",
    "text",
    "value",
    "attrs",
    "path",
    "request",
    "error",
    "panic.payload",
];

/// The marker printed in place of a redacted field's value.
pub const REDACTED: &str = "[redacted]";

/// The field a bridged `log` record carries its real target in; the formatter prints the
/// resolved target in the target column instead.
const LOG_TARGET_FIELD: &str = "log.target";

/// The field a closed span's line prints the span's name under.
const SPAN_FIELD: &str = "span";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Print,
    Redact,
    Drop,
}

fn is_engine_target(target: &str) -> bool {
    ENGINE_TARGET_PREFIXES
        .iter()
        .any(|prefix| target.starts_with(prefix))
}

fn is_escher_target(target: &str) -> bool {
    ESCHER_TARGET_PREFIXES
        .iter()
        .any(|prefix| target.starts_with(prefix))
}

/// A target the sink has no scrub rule for: its records are dropped whole.
fn is_outside_target(target: &str) -> bool {
    !is_engine_target(target) && !is_escher_target(target)
}

fn decide(target: &str, field: &str) -> Verdict {
    if is_outside_target(target) {
        Verdict::Drop
    } else if is_engine_target(target) {
        if SAFE_FIELDS.contains(&field) {
            Verdict::Print
        } else {
            Verdict::Redact
        }
    } else if CONTENT_FIELDS.contains(&field) {
        Verdict::Redact
    } else {
        Verdict::Print
    }
}

/// One line per printed event and one per closed span:
/// `{time} {LEVEL} {target} service.name=… service.version=… {fields}` — and nothing at all, not
/// an empty line, for a record whose target is outside [`ENGINE_TARGET_PREFIXES`] and
/// [`ESCHER_TARGET_PREFIXES`]. A bridged `log` record is judged by the target it was logged
/// under.
///
/// A closed span's fields are `span`, holding the span's name, then the fields the span itself
/// carries, then the layer's own `message`, `time.busy` and `time.idle`; each is printed or
/// redacted by the rule that judges an event's field of that name under the span's target. A
/// span writes nothing before it closes, and an event inside a span reads as it does outside
/// one.
pub(crate) struct EscherFormat {
    identity: ServiceIdentity,
}

impl EscherFormat {
    pub(crate) fn new(identity: ServiceIdentity) -> Self {
        Self { identity }
    }
}

impl<S> FormatEvent<S, SpanFields> for EscherFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, SpanFields>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let normalized = event.normalized_metadata();
        let metadata = normalized.as_ref().unwrap_or_else(|| event.metadata());
        if is_outside_target(metadata.target()) {
            return Ok(());
        }
        SystemTime.format_time(&mut writer)?;
        write!(
            writer,
            " {} {} service.name={} service.version={}",
            metadata.level(),
            metadata.target(),
            self.identity.name,
            self.identity.version,
        )?;
        let mut visitor = ScrubVisitor {
            target: metadata.target(),
            out: String::new(),
        };
        // The layer's record of a span closing carries the span's own metadata, and the span is
        // its parent.
        if event.metadata().is_span()
            && let Some(span) = ctx.parent_span()
        {
            visitor.push(SPAN_FIELD, format_args!("{:?}", span.name()));
            if let Some(stored) = span.extensions().get::<FormattedFields<SpanFields>>() {
                for (name, value) in stored_pairs(&stored.fields) {
                    visitor.push(name, format_args!("{value}"));
                }
            }
        }
        event.record(&mut visitor);
        writer.write_str(&visitor.out)?;
        writeln!(writer)
    }
}

/// Appends `value` to `out` as a line spells it: a line break is written as its escape, so one
/// record stays one line.
fn spell(out: &mut String, value: fmt::Arguments<'_>) {
    for c in value.to_string().chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
}

/// The sink's field formatter: what a span's fields are stored as until the span closes.
///
/// The layer hands a span's fields over without the span's target, so they cannot be judged
/// here. Each is stored as its name and its spelled value, one line each — a spelled value holds
/// no line break, so a name stays apart from its value whatever the value holds — and
/// [`EscherFormat`] judges every one by the span's target when it prints the closed span. The
/// value of a field named in [`CONTENT_FIELDS`] is redacted under every target, so it is not
/// stored at all.
pub(crate) struct SpanFields;

impl<'writer> FormatFields<'writer> for SpanFields {
    fn format_fields<R: RecordFields>(
        &self,
        mut writer: Writer<'writer>,
        fields: R,
    ) -> fmt::Result {
        let mut visitor = StoreVisitor(String::new());
        fields.record(&mut visitor);
        writer.write_str(&visitor.0)
    }

    // The default puts a space between two recordings, which the stored form has no place for.
    fn add_fields(
        &self,
        current: &'writer mut FormattedFields<Self>,
        fields: &span::Record<'_>,
    ) -> fmt::Result {
        self.format_fields(current.as_writer(), fields)
    }
}

/// The name and the spelled value of each field [`SpanFields`] stored, in the order recorded.
fn stored_pairs(stored: &str) -> impl Iterator<Item = (&str, &str)> {
    let mut lines = stored.split('\n');
    std::iter::from_fn(move || Some((lines.next()?, lines.next()?)))
}

struct StoreVisitor(String);

impl StoreVisitor {
    fn push(&mut self, name: &str, value: fmt::Arguments<'_>) {
        spell(&mut self.0, format_args!("{name}"));
        self.0.push('\n');
        if !CONTENT_FIELDS.contains(&name) {
            spell(&mut self.0, value);
        }
        self.0.push('\n');
    }
}

impl Visit for StoreVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.push(field.name(), format_args!("{value:?}"));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.push(field.name(), format_args!("{value:?}"));
    }
}

struct ScrubVisitor<'a> {
    target: &'a str,
    out: String,
}

impl ScrubVisitor<'_> {
    fn push(&mut self, name: &str, value: fmt::Arguments<'_>) {
        if name == LOG_TARGET_FIELD {
            return;
        }
        let redact = match decide(self.target, name) {
            Verdict::Drop => return,
            Verdict::Redact => true,
            Verdict::Print => false,
        };
        self.out.push(' ');
        self.out.push_str(name);
        self.out.push('=');
        if redact {
            self.out.push_str(REDACTED);
            return;
        }
        spell(&mut self.out, value);
    }
}

impl Visit for ScrubVisitor<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.push(field.name(), format_args!("{value:?}"));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.push(field.name(), format_args!("{value:?}"));
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::{Arc, Mutex};

    use tracing_log::log;
    use tracing_subscriber::layer::SubscriberExt;

    use super::*;

    const SENTINEL: &str = "ESCHER-SENTINEL-9c21";

    #[test]
    fn engine_target_prints_only_safe_fields() {
        assert_eq!(decide("blitz_dom::mutator", "node_id"), Verdict::Print);
        assert_eq!(decide("blitz_net", "status"), Verdict::Print);
        assert_eq!(decide("blitz_dom::mutator", "message"), Verdict::Redact);
        assert_eq!(decide("dioxus_native_dom", "attribute"), Verdict::Redact);
        assert_eq!(decide("js_console", "log.file"), Verdict::Print);
    }

    #[test]
    fn content_fields_redact_for_any_target() {
        assert_eq!(decide("escher_driver", "url"), Verdict::Redact);
        assert_eq!(decide("escher_driver", "html"), Verdict::Redact);
        assert_eq!(
            decide("escher_telemetry::panic", "panic.payload"),
            Verdict::Redact
        );
        assert_eq!(decide("blitz_net", "url"), Verdict::Redact);
    }

    #[test]
    fn other_fields_of_escher_targets_print() {
        assert_eq!(decide("escher_driver", "message"), Verdict::Print);
        assert_eq!(decide("escher_driver", "node_id"), Verdict::Print);
    }

    #[test]
    fn outside_target_is_dropped_whatever_the_field() {
        for target in [
            "style::traversal",
            "style::style_resolver",
            "selectors::matching",
            "dioxus_core::diff::node",
            "dioxus_signals::signal",
            "warnings::warnings",
            "winit::platform",
        ] {
            for field in ["node_id", "url", "message", "event"] {
                assert_eq!(decide(target, field), Verdict::Drop, "{target} {field}");
            }
        }
    }

    #[test]
    fn outside_target_is_not_admitted_by_a_shared_prefix() {
        for target in ["escher", "style", "dioxus_core", "log", ""] {
            assert_eq!(decide(target, "message"), Verdict::Drop, "{target:?}");
        }
        for target in [
            "escher_telemetry",
            "escher_telemetry::panic",
            "escher_driver",
            "escher_stand_probe",
        ] {
            assert_eq!(decide(target, "message"), Verdict::Print, "{target}");
        }
    }

    #[derive(Clone, Default)]
    struct Capture(Arc<Mutex<Vec<u8>>>);

    impl io::Write for Capture {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// What the sink's layer writes for the records `emit` sends through a subscriber of its
    /// own. `emit` is handed a reader of what has been written so far.
    fn capturing(emit: impl FnOnce(&dyn Fn() -> String)) -> String {
        let capture = Capture::default();
        let sink = capture.clone();
        let subscriber = tracing_subscriber::registry()
            .with(crate::sink_layer(crate::service_identity!(), move || {
                sink.clone()
            }));
        let written = || String::from_utf8(capture.0.lock().unwrap().clone()).unwrap();
        tracing::subscriber::with_default(subscriber, || emit(&written));
        written()
    }

    /// What the sink's layer writes for the records `emit` sends through a subscriber of its
    /// own.
    fn captured(emit: impl FnOnce()) -> String {
        capturing(|_| emit())
    }

    /// Sends one `log` record, logged under `target`, over the bridge.
    fn bridge(target: &str, level: log::Level, args: fmt::Arguments<'_>) {
        tracing_log::format_trace(
            &log::Record::builder()
                .target(target)
                .level(level)
                .args(args)
                .build(),
        )
        .unwrap();
    }

    #[test]
    fn bridged_log_record_is_judged_by_its_log_target() {
        let output = captured(|| {
            bridge(
                "js_console",
                log::Level::Warn,
                format_args!("console says {SENTINEL}"),
            )
        });
        assert!(output.contains(" WARN js_console "), "{output}");
        assert!(output.contains("message=[redacted]"), "{output}");
        assert!(!output.contains(SENTINEL), "{output}");
        assert!(!output.contains("log.target"), "{output}");
    }

    #[test]
    fn outside_target_native_record_writes_no_byte() {
        let output = captured(|| {
            tracing::error!(target: "winit::platform", "surface lost {SENTINEL}");
            tracing::warn!(target: "style::traversal", node_id = 7, "restyled {SENTINEL}");
            tracing::trace!(target: "dioxus_core::diff::node", text = SENTINEL, "diffed");
        });
        assert!(output.is_empty(), "{output:?}");
    }

    #[test]
    fn outside_target_bridged_record_writes_no_byte() {
        let dropped = captured(|| {
            bridge(
                "selectors::matching",
                log::Level::Debug,
                format_args!("matched {SENTINEL}"),
            )
        });
        assert!(dropped.is_empty(), "{dropped:?}");

        // Every bridged record's own target is `log`, itself outside the allowlist: one logged
        // under an escher target prints, so it is the logged target that is judged.
        let admitted = captured(|| {
            bridge(
                "escher_driver",
                log::Level::Warn,
                format_args!("session attached"),
            )
        });
        assert!(admitted.contains(" WARN escher_driver "), "{admitted}");
        assert!(admitted.contains("message=session attached"), "{admitted}");
    }

    #[test]
    fn outside_target_drop_leaves_engine_and_escher_records_printing() {
        let output = captured(|| {
            tracing::warn!(target: "style::traversal", "restyled {SENTINEL}");
            tracing::warn!(target: "blitz_dom::mutator", node_id = 7, "loading image {SENTINEL}");
            tracing::warn!(target: "escher_driver", html = SENTINEL, "snapshot taken");
        });
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 2, "{output}");
        assert!(lines[0].contains(" WARN blitz_dom::mutator "), "{output}");
        assert!(lines[0].contains("message=[redacted]"), "{output}");
        assert!(lines[0].contains("node_id=7"), "{output}");
        assert!(lines[1].contains(" WARN escher_driver "), "{output}");
        assert!(lines[1].contains("message=snapshot taken"), "{output}");
        assert!(lines[1].contains("html=[redacted]"), "{output}");
        assert!(!output.contains(SENTINEL), "{output}");
    }

    #[test]
    fn escher_span_closes_into_one_line_and_writes_nothing_before() {
        let output = capturing(|written| {
            let span = tracing::info_span!(
                target: "escher_driver",
                "command",
                verb = "click",
                passes = tracing::field::Empty,
            );
            assert!(written().is_empty(), "written at creation");
            span.record("passes", 2u32);
            assert!(written().is_empty(), "written on a recorded value");
            {
                let _entered = span.enter();
                assert!(written().is_empty(), "written on enter");
            }
            assert!(written().is_empty(), "written on exit");
        });
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 1, "{output}");
        assert!(
            lines[0].contains(" INFO escher_driver service.name=escher-telemetry "),
            "{output}"
        );
        for pair in [
            " span=\"command\"",
            " verb=\"click\"",
            " passes=2",
            " message=\"close\"",
            " time.busy=",
            " time.idle=",
        ] {
            assert_eq!(lines[0].matches(pair).count(), 1, "{pair}: {output}");
        }
        assert!(!output.contains(REDACTED), "{output}");
    }

    #[test]
    fn content_named_span_field_is_redacted_and_a_value_stays_one_pair() {
        let output = captured(|| {
            let span = tracing::info_span!(
                target: "escher_driver",
                "command",
                html = SENTINEL,
                text = tracing::field::Empty,
                note = tracing::field::display("a b=c\nd\re"),
            );
            span.record("text", SENTINEL);
        });
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 1, "{output:?}");
        assert!(lines[0].contains(" html=[redacted] "), "{output}");
        assert!(lines[0].contains(" text=[redacted] "), "{output}");
        assert!(lines[0].contains(" note=a b=c\\nd\\re "), "{output}");
        assert!(!output.contains(SENTINEL), "{output}");
    }

    #[test]
    fn outside_target_span_writes_no_byte() {
        let output = captured(|| {
            let span = tracing::error_span!(
                target: "winit::platform",
                "surface",
                node_id = 7,
                text = SENTINEL,
                late = tracing::field::Empty,
            );
            span.record("late", SENTINEL);
            let _entered = span.enter();
        });
        assert!(output.is_empty(), "{output:?}");
    }

    #[test]
    fn engine_target_span_prints_only_safe_fields() {
        let output = captured(|| {
            let span = tracing::warn_span!(
                target: "blitz_dom::mutator",
                "escher-span-77aa",
                node_id = 7,
                attribute = SENTINEL,
                url = SENTINEL,
                late = tracing::field::Empty,
            );
            span.record("late", SENTINEL);
        });
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 1, "{output}");
        assert!(lines[0].contains(" WARN blitz_dom::mutator "), "{output}");
        for pair in [
            " span=[redacted]",
            " node_id=7",
            " attribute=[redacted]",
            " url=[redacted]",
            " late=[redacted]",
            " message=[redacted]",
            " time.busy=[redacted]",
            " time.idle=[redacted]",
        ] {
            assert_eq!(lines[0].matches(pair).count(), 1, "{pair}: {output}");
        }
        assert!(!output.contains(SENTINEL), "{output}");
        assert!(!output.contains("escher-span-77aa"), "{output}");
    }

    #[test]
    fn event_inside_a_span_reads_as_it_does_outside_one() {
        let output = captured(|| {
            tracing::warn!(target: "escher_driver", count = 3, "step done");
            let span = tracing::info_span!(target: "escher_driver", "command", verb = "click");
            let _entered = span.enter();
            tracing::warn!(target: "escher_driver", count = 3, "step done");
        });
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3, "{output}");
        // A line opens with its time, which the two events do not share.
        let after_time = |line: &str| line.split_once(' ').map(|(_, rest)| rest.to_string());
        let outside = after_time(lines[0]);
        let expected = format!(
            "WARN escher_driver service.name=escher-telemetry service.version={} \
             message=step done count=3",
            env!("CARGO_PKG_VERSION")
        );
        assert_eq!(outside, Some(expected), "{output}");
        assert_eq!(after_time(lines[1]), outside, "{output}");
        assert!(lines[2].contains(" span=\"command\""), "{output}");
    }
}
