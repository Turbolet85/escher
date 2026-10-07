use std::fmt;

use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_log::NormalizeEvent;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::{FormatTime, SystemTime};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
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

/// One line per event: `{time} {LEVEL} {target} service.name=… service.version=… {fields}` —
/// and nothing at all, not an empty line, for an event whose target is outside
/// [`ENGINE_TARGET_PREFIXES`] and [`ESCHER_TARGET_PREFIXES`]. A bridged `log` record is judged by
/// the target it was logged under.
pub(crate) struct EscherFormat {
    identity: ServiceIdentity,
}

impl EscherFormat {
    pub(crate) fn new(identity: ServiceIdentity) -> Self {
        Self { identity }
    }
}

impl<S, N> FormatEvent<S, N> for EscherFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
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
        event.record(&mut visitor);
        writer.write_str(&visitor.out)?;
        writeln!(writer)
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
        for c in value.to_string().chars() {
            match c {
                '\n' => self.out.push_str("\\n"),
                '\r' => self.out.push_str("\\r"),
                c => self.out.push(c),
            }
        }
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

    /// What the formatter writes for the records `emit` sends through a subscriber of its own.
    fn captured(emit: impl FnOnce()) -> String {
        let capture = Capture::default();
        let sink = capture.clone();
        let subscriber = tracing_subscriber::registry().with(
            tracing_subscriber::fmt::layer()
                .event_format(EscherFormat::new(crate::service_identity!()))
                .with_ansi(false)
                .with_writer(move || sink.clone()),
        );
        tracing::subscriber::with_default(subscriber, emit);
        String::from_utf8(capture.0.lock().unwrap().clone()).unwrap()
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
}
