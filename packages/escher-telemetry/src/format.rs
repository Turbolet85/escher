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
pub const ENGINE_TARGET_PREFIXES: &[&str] = &[
    "blitz",
    "dioxus_native",
    "stylo_taffy",
    "accesskit_xplat",
    "debug_timer",
    "js_console",
];

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
}

fn is_engine_target(target: &str) -> bool {
    ENGINE_TARGET_PREFIXES
        .iter()
        .any(|prefix| target.starts_with(prefix))
}

fn decide(target: &str, field: &str) -> Verdict {
    if is_engine_target(target) {
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

/// One line per event: `{time} {LEVEL} {target} service.name=… service.version=… {fields}`.
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
        self.out.push(' ');
        self.out.push_str(name);
        self.out.push('=');
        match decide(self.target, name) {
            Verdict::Redact => self.out.push_str(REDACTED),
            Verdict::Print => {
                for c in value.to_string().chars() {
                    match c {
                        '\n' => self.out.push_str("\\n"),
                        '\r' => self.out.push_str("\\r"),
                        c => self.out.push(c),
                    }
                }
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
    fn other_fields_of_non_engine_targets_print() {
        assert_eq!(decide("escher_driver", "message"), Verdict::Print);
        assert_eq!(decide("escher_driver", "node_id"), Verdict::Print);
        assert_eq!(decide("winit::platform", "event"), Verdict::Print);
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

    #[test]
    fn bridged_log_record_is_judged_by_its_log_target() {
        let capture = Capture::default();
        let sink = capture.clone();
        let subscriber = tracing_subscriber::registry().with(
            tracing_subscriber::fmt::layer()
                .event_format(EscherFormat::new(crate::service_identity!()))
                .with_ansi(false)
                .with_writer(move || sink.clone()),
        );
        tracing::subscriber::with_default(subscriber, || {
            tracing_log::format_trace(
                &log::Record::builder()
                    .target("js_console")
                    .level(log::Level::Warn)
                    .args(format_args!("console says {SENTINEL}"))
                    .build(),
            )
            .unwrap();
        });
        let output = String::from_utf8(capture.0.lock().unwrap().clone()).unwrap();
        assert!(output.contains(" WARN js_console "), "{output}");
        assert!(output.contains("message=[redacted]"), "{output}");
        assert!(!output.contains(SENTINEL), "{output}");
        assert!(!output.contains("log.target"), "{output}");
    }
}
