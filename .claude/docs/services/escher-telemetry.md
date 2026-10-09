# escher-telemetry

_Crate notes. Primary source: `.andromeda/obs-plan.md` §3 / §6 / §8 and `.andromeda/architecture.md` §Standard Contracts Telemetry bootstrap._

## Responsibility
escher's telemetry bootstrap (unpublished, no `[features]`): one process-global stderr `tracing` subscriber that stamps the service identity on every line, applies a target allowlist in its formatter — it scrubs the records of engine and escher targets and drops every other target's — bridges `log` records and chains a panic hook. Not its job: OTel export (escher 0.1.0 ships none — ruled by the founder, 2026-10-06; the transport and credential path are a residual in `.andromeda/residuals.md`), a log file or JSON schema, and the agent-run / cold-agent harness logs (harness metadata, not this sink).

## Key integrations

### Consumes from
- `tracing` (ungated), `tracing-subscriber` (features env-filter · fmt · registry · std · tracing-log), `tracing-log`.
- `RUST_LOG`, read through `EnvFilter` — the default is `warn` when it is unset or unparsable. A directive naming a target outside the allowlist does not re-admit it: the formatter is the last thing a record meets.

### Publishes to
- `init(ServiceIdentity) -> Result<InitOutcome, InitError>` (the stderr sink) and `init_with_writer(ServiceIdentity, W: MakeWriter)` (a caller-given sink).
- `ServiceIdentity { name, version }`, built by `service_identity!()` from the CALLER's `CARGO_PKG_NAME` / `CARGO_PKG_VERSION`; `InitOutcome::{Installed, AlreadyInstalled}`; `InitError::ForeignSubscriber` (`Display` + `Error`, never a panic).
- Five public scrub constants: `ENGINE_TARGET_PREFIXES`, `ESCHER_TARGET_PREFIXES`, `SAFE_FIELDS`, `CONTENT_FIELDS`, `REDACTED`.
- Line shape, one per printed event, no ANSI: `{RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…` — newlines escaped, string values Debug-quoted, a bridged `log` record printed under its `log.target`. An event from a target outside the allowlist prints nothing at all, not an empty line.
- A second record class — a boundary widening, ratified by the founder (2026-10-09) as built: one line per closed span whose target the allowlist admits, and nothing when a span is created, recorded to, entered or exited. The line is the same prefix, then `span={the span's name}` (`span` a reserved field name), the span's own fields in the order recorded, then the layer's `message`, `time.busy` and `time.idle`; every pair is judged as an event's field of that name is under the span's target, an outside target's span writes no byte, and the value of a content-named span field is never stored. An event's line is unchanged, inside a span included. The layer is stated once, in the private `sink_layer` (`with_span_events(FmtSpan::CLOSE)`, the crate-private field formatter `SpanFields`); no public item moved.
- Events of its own: `info` `telemetry installed` at target `escher_telemetry`, once per successful init; ERROR `panic` at target `escher_telemetry::panic` with `panic.file` · `panic.line` · `panic.column` · `panic.payload` (redacted), then the previous hook.
- Consumed by seven_guis' two binaries — `seven_guis_native` (`main`, before `dioxus_native::launch`) and the session host `escher-session` (first thing in `main`); in both an `Err` is reported with `eprintln!` and the binary continues, and both stamp `service.name=seven_guis` — and, as a dev-dependency, by blitz-tests.

## Internal conventions
- Its startup and panic events are ungated — the engine crates gate every `tracing` call site behind their `tracing` feature; this crate has no feature to gate on.
- stderr only, never stdout.
- The target allowlist and the scrub live in `format.rs`, three outcomes by target (a bridged record judged by its `log.target`): a target starting with a prefix in `ENGINE_TARGET_PREFIXES` (`blitz` · `dioxus_native` · `stylo_taffy` · `accesskit_xplat` · `debug_timer` · `js_console`) prints only `SAFE_FIELDS` (`node_id` · `status` · `waiting_nodes` · `property` · `log.module_path` · `log.file` · `log.line`), every other field — the message included — as `{name}=[redacted]`; a target starting with a prefix in `ESCHER_TARGET_PREFIXES` (`escher_`, the underscore part of the prefix) prints every field not in `CONTENT_FIELDS` (`url` · `href` · `src` · `html` · `text` · `value` · `attrs` · `path` · `request` · `error` · `panic.payload`), those redacted; a target starting with a prefix of neither set is dropped whole — zero bytes, at every level, WARN and ERROR included, whatever `RUST_LOG` names. No engine call site is edited, and no filter layer is added: the drop sits in the formatter.

## Crate-specific gotchas
- Process-global: a second `init` changes nothing (`AlreadyInstalled`), and another global subscriber or `log` logger makes it `ForeignSubscriber`. The first install is recorded in a static `OnceLock<ServiceIdentity>` behind a static `Mutex<()>`; it spawns no thread.
- The chained std panic hook still prints the raw panic message to stderr, and the allowlisted `log.file` can carry a host path only for a bridged record under an engine target (by construction, not measured — a bridged third-party record is dropped whole).
- The allowlist reaches this sink only: the upstream apps' `fmt::init()` (stdout) and the WPT runner's `env_logger` stay unscrubbed.
- The headless stand (`seven_guis::stand`) and the session library (`escher-driver`) install no subscriber — a headless boot made in process has no escher sink, save in `stand_act_spans`'s two re-run children, which install it over an in-memory capture; the session library emits one span per command under the target `escher_driver`, printed only where a sink is installed at `info` or below; one made by the `escher-session` binary runs under that binary's sink.
- A record from any target outside the two prefix sets (Stylo's `style::*`, `selectors::matching`, `dioxus_core::*`; on the windowed stand also `naga::*`, `wgpu_*`, `winit_wayland::*`) is dropped: both sink-installing binaries print no element id and no accessible name at any `RUST_LOG` level — nothing at the default level, the install line alone at `info`, `debug` and `trace`. The cost: a third-party WARN or ERROR no longer prints either (the windowed stand's one default-level `winit_wayland` WARN is gone). Typed text is measured in process only — 0 occurrences in a sink capture of the driver's commands (`stand_act_spans`) — and not measured in a host's log. The readings are of a `-p seven_guis` build; a workspace-wide build compiles blitz-dom's `tracing` call sites in, and the binaries' stderr by level as that build makes them is not measured.
- A new escher crate's log target is admitted only if it starts with `escher_`; `escher` alone, or another crate's name sharing letters with a prefix, is dropped.

## Entry points for modification
- `src/lib.rs` (`init`, the identity, the filter) · `src/format.rs` (the formatter, the two target sets and the two field sets) · `src/panic.rs` (the chaining hook) · `Cargo.toml`

## Testing this crate
- `cargo test -p escher-telemetry` — 15 inline unit tests (three scrub decision branches, a bridged `log` record, five on the outside-target drop, five on the closed span's line, the identity macro).
- `cargo test -p blitz-tests --test telemetry_stdout_silent` · `--test telemetry_scrub` · `--test telemetry_panic_hook` · `--test telemetry_init_idempotent` · `--test telemetry_drop` — the process-global behaviours; `telemetry_stdout_silent` and `telemetry_drop` each re-execute their own test binary once (ignored child `child_emits`).
- `cargo test -p seven_guis --test host_log` — the real `escher-session` binary at `RUST_LOG=trace`: no stable id and no accessible name on stderr.
- `RUST_LOG=info just seven_guis` — the windowed stand with the sink's install line visible.

## References
- `.claude/rules/observability.md` · `.claude/docs/obs-summary.md` · `.claude/rules/security.md` (env reads)
