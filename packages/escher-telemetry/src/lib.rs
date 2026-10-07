//! Telemetry bootstrap for escher binaries.
//!
//! One call — [`init`] (or [`init_with_writer`]) — installs a process-wide [`tracing`] subscriber
//! that:
//!
//! - writes to **stderr**, never stdout (stdout belongs to the driver CLI's JSON and MCP's stdio);
//! - writes one line per printed event and one per closed span — a span writes nothing when it
//!   is created, entered or exited, and its line carries its name as `span`, its own fields and
//!   the sink's timing of it as `time.busy` and `time.idle`;
//! - stamps every line with the binary's [`ServiceIdentity`] as `service.name` / `service.version`;
//! - redacts user content through an allowlist applied in the formatter: records from engine
//!   targets ([`ENGINE_TARGET_PREFIXES`]) print only their [`SAFE_FIELDS`], and records from
//!   escher's own targets ([`ESCHER_TARGET_PREFIXES`]) print every field not named in
//!   [`CONTENT_FIELDS`] — a closed span's fields are judged as an event's are;
//! - drops every event and every span from a target outside those two sets — nothing is written
//!   for it, at any level and whatever `RUST_LOG` names: the sink prints no record it has no
//!   scrub rule for;
//! - bridges `log`-facade records into the same formatter, each judged by the target it was
//!   logged under;
//! - logs every panic as an error event, then chains the previously installed panic hook.
//!
//! The level filter is read from `RUST_LOG` and defaults to `warn`.

#![deny(missing_docs)]

mod format;
mod panic;

use std::fmt;
use std::io;
use std::sync::{Mutex, OnceLock};

use tracing_log::AsLog;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;

pub use format::{
    CONTENT_FIELDS, ENGINE_TARGET_PREFIXES, ESCHER_TARGET_PREFIXES, REDACTED, SAFE_FIELDS,
};

/// The identity of the binary that installs the subscriber, printed on every line.
///
/// The keys are the OpenTelemetry resource keys `service.name` and `service.version`. Build it
/// with [`service_identity!`] so the values come from the binary's own manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceIdentity {
    /// The service name — the binary's package name.
    pub name: &'static str,
    /// The service version — the binary's package version.
    pub version: &'static str,
}

/// Expands to the [`ServiceIdentity`] of the crate it is invoked in, read from its
/// `CARGO_PKG_NAME` and `CARGO_PKG_VERSION`.
#[macro_export]
macro_rules! service_identity {
    () => {
        $crate::ServiceIdentity {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    };
}

/// What a successful init call did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitOutcome {
    /// This call installed the subscriber, the `log` bridge and the panic hook.
    Installed,
    /// An earlier call already installed them; nothing was changed.
    AlreadyInstalled,
}

/// Why an init call installed nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitError {
    /// A global `tracing` subscriber or a `log` logger was already installed by someone else.
    ForeignSubscriber,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitError::ForeignSubscriber => {
                f.write_str("a global tracing subscriber or log logger is already installed")
            }
        }
    }
}

impl std::error::Error for InitError {}

static INSTALLED: OnceLock<ServiceIdentity> = OnceLock::new();
static INIT_LOCK: Mutex<()> = Mutex::new(());

/// The sink's one layer: [`format::EscherFormat`] prints each event and each closed span — the
/// layer reports a span's close and nothing earlier of it — and [`format::SpanFields`] stores a
/// span's fields until then.
fn sink_layer<S, W>(
    identity: ServiceIdentity,
    writer: W,
) -> tracing_subscriber::fmt::Layer<S, format::SpanFields, format::EscherFormat, W>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    W: for<'w> MakeWriter<'w> + 'static,
{
    tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .fmt_fields(format::SpanFields)
        .event_format(format::EscherFormat::new(identity))
        .with_ansi(false)
        .with_writer(writer)
}

/// Installs the telemetry bootstrap with stderr as its sink.
///
/// Idempotent: a second call returns [`InitOutcome::AlreadyInstalled`] and changes nothing.
pub fn init(identity: ServiceIdentity) -> Result<InitOutcome, InitError> {
    init_with_writer(identity, io::stderr)
}

/// Installs the telemetry bootstrap with `writer` as its sink — for tests, or a caller that
/// names a file it opened.
///
/// Idempotent: a second call returns [`InitOutcome::AlreadyInstalled`] without touching the
/// subscriber, the sink or the panic hook.
pub fn init_with_writer<W>(identity: ServiceIdentity, writer: W) -> Result<InitOutcome, InitError>
where
    W: for<'w> MakeWriter<'w> + Send + Sync + 'static,
{
    let _guard = INIT_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if INSTALLED.get().is_some() {
        return Ok(InitOutcome::AlreadyInstalled);
    }
    if tracing::dispatcher::has_been_set() {
        return Err(InitError::ForeignSubscriber);
    }

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let max_level = filter
        .max_level_hint()
        .unwrap_or(tracing::level_filters::LevelFilter::TRACE);
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(sink_layer(identity, writer));

    tracing_log::LogTracer::builder()
        .with_max_level(max_level.as_log())
        .init()
        .map_err(|_| InitError::ForeignSubscriber)?;
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| InitError::ForeignSubscriber)?;

    let _ = INSTALLED.set(identity);
    panic::install();
    tracing::info!(target: "escher_telemetry", "telemetry installed");
    Ok(InitOutcome::Installed)
}

#[cfg(test)]
mod tests {
    #[test]
    fn service_identity_reads_the_invoking_crate() {
        let identity = crate::service_identity!();
        assert_eq!(identity.name, "escher-telemetry");
        assert_eq!(identity.version, env!("CARGO_PKG_VERSION"));
    }
}
