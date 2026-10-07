# escher-telemetry

_Crate notes. Primary source: `.andromeda/obs-plan.md` §3 / §6 / §8 and `.andromeda/architecture.md` §Standard Contracts Telemetry bootstrap._

## Responsibility
escher's telemetry bootstrap (unpublished, no `[features]`): one process-global stderr `tracing` subscriber that stamps the service identity on every line, scrubs fields by allowlist in its formatter, bridges `log` records and chains a panic hook. Not its job: OTel export (escher 0.1.0 ships none — ruled by the founder, 2026-10-06; the transport and credential path are a residual in `.andromeda/residuals.md`), a log file or JSON schema, and the agent-run / cold-agent harness logs (harness metadata, not this sink).

## Key integrations

### Consumes from
- `tracing` (ungated), `tracing-subscriber` (features env-filter · fmt · registry · std · tracing-log), `tracing-log`.
- `RUST_LOG`, read through `EnvFilter` — the default is `warn` when it is unset or unparsable.

### Publishes to
- `init(ServiceIdentity) -> Result<InitOutcome, InitError>` (the stderr sink) and `init_with_writer(ServiceIdentity, W: MakeWriter)` (a caller-given sink).
- `ServiceIdentity { name, version }`, built by `service_identity!()` from the CALLER's `CARGO_PKG_NAME` / `CARGO_PKG_VERSION`; `InitOutcome::{Installed, AlreadyInstalled}`; `InitError::ForeignSubscriber` (`Display` + `Error`, never a panic).
- Line shape, one per event, no ANSI: `{RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…` — newlines escaped, string values Debug-quoted, a bridged `log` record printed under its `log.target`.
- Events of its own: `info` `telemetry installed` at target `escher_telemetry`, once per successful init; ERROR `panic` at target `escher_telemetry::panic` with `panic.file` · `panic.line` · `panic.column` · `panic.payload` (redacted), then the previous hook.
- Consumed by seven_guis' two binaries — `seven_guis_native` (`main`, before `dioxus_native::launch`) and the session host `escher-session` (first thing in `main`); in both an `Err` is reported with `eprintln!` and the binary continues, and both stamp `service.name=seven_guis` — and, as a dev-dependency, by blitz-tests.

## Internal conventions
- Its startup and panic events are ungated — the engine crates gate every `tracing` call site behind their `tracing` feature; this crate has no feature to gate on.
- stderr only, never stdout.
- The scrub lives in `format.rs`: an event whose target (or a bridged record's `log.target`) starts with a prefix in `ENGINE_TARGET_PREFIXES` (`blitz` · `dioxus_native` · `stylo_taffy` · `accesskit_xplat` · `debug_timer` · `js_console`) prints only `SAFE_FIELDS` (`node_id` · `status` · `waiting_nodes` · `property` · `log.module_path` · `log.file` · `log.line`), every other field — the message included — as `{name}=[redacted]`; a field in `CONTENT_FIELDS` (`url` · `href` · `src` · `html` · `text` · `value` · `attrs` · `path` · `request` · `error` · `panic.payload`) is redacted at any target. No engine call site is edited.

## Crate-specific gotchas
- Process-global: a second `init` changes nothing (`AlreadyInstalled`), and another global subscriber or `log` logger makes it `ForeignSubscriber`. The first install is recorded in a static `OnceLock<ServiceIdentity>` behind a static `Mutex<()>`; it spawns no thread.
- The chained std panic hook still prints the raw panic message to stderr, and the allowlisted `log.file` carries a host path for bridged third-party records at `RUST_LOG=info`.
- The scrub reaches this sink only: the upstream apps' `fmt::init()` (stdout) and the WPT runner's `env_logger` stay unscrubbed.
- The headless stand (`seven_guis::stand`) and the session library (`escher-driver`) install no subscriber — a headless boot made in process has no escher sink; one made by the `escher-session` binary runs under that binary's sink.
- The scrub is an allowlist of ENGINE targets: a record from any other target (Stylo's `style::*`, `selectors::matching`, `dioxus_core::*`) prints its message and fields as written — on `escher-session` that is element ids at `RUST_LOG=debug` and accessible names at `trace` (none at the default level or `info`). Owed by the route entry "Sink target allowlist".

## Entry points for modification
- `src/lib.rs` (`init`, the identity, the filter) · `src/format.rs` (the formatter and the three scrub sets) · `src/panic.rs` (the chaining hook) · `Cargo.toml`

## Testing this crate
- `cargo test -p escher-telemetry` — 5 inline unit tests (three scrub decision branches, a bridged `log` record, the identity macro).
- `cargo test -p blitz-tests --test telemetry_stdout_silent` · `--test telemetry_scrub` · `--test telemetry_panic_hook` · `--test telemetry_init_idempotent` — the process-global behaviours; `telemetry_stdout_silent` re-executes its own test binary once (ignored child `child_emits`).
- `RUST_LOG=info just seven_guis` — the windowed stand with the sink's lines visible.

## References
- `.claude/rules/observability.md` · `.claude/docs/obs-summary.md` · `.claude/rules/security.md` (env reads)
