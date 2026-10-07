---
paths:
  - "packages/debug_timer/**"
  - "**/telemetry/**"
  - "**/tracing/**"
  - "**/logging/**"
  - "**/*telemetry*.rs"
  - "**/*tracing*.rs"
  - "**/*logging*.rs"
  - "**/panic*.rs"
---

# Observability Rules

Path-scoped rules for telemetry, logging and timing code. Source: `.andromeda/obs-plan.md` (obs tier 0).

## Logging (as built)
- **escher binaries:** install `escher_telemetry::init(escher_telemetry::service_identity!())` at start (today the two binaries of seven_guis — `seven_guis_native`, before `launch`, and the session host `escher-session`, first thing in `main` — both stamping `service.name=seven_guis`; an `Err` is `eprintln!`ed and the binary continues; the in-process headless stand `seven_guis::stand`, the driver library `escher-driver` — its session and its command schema — and their checks install none) — stderr only, never stdout; filter from `RUST_LOG`, default `warn`; every line `{RFC 3339 UTC} {LEVEL} {target} service.name=… service.version=… {field}={value}…`; `log` records bridged into the same formatter. A second `init` is `AlreadyInstalled`; a foreign subscriber is `InitError::ForeignSubscriber`, never a panic.
- **Engine and upstream crates:** `tracing` 0.1 events compiled only with each crate's `tracing` cargo feature; every call site is `#[cfg(feature = "tracing")]` with a `#[cfg(not(feature = "tracing"))] let _ = …;` fallback — keep the gate (the stand leaves the engine features off).
- The `tracing` feature forwards down the crate chain (dioxus-native → dioxus-native-dom, blitz-shell, blitz-dom, blitz-html, blitz-net); `blitz` turns it on by default, `blitz-vibey-script` does not.
- Upstream subscribers: `tracing_subscriber::fmt::init()` (stdout) in the apps under `tracing`; `tracing_wasm` on wasm; the WPT runner uses the `log` facade with `env_logger`.
- JS console output goes to the `log` crate at debug level, target `js_console`, keeping stdout/stderr clean; embedders drain JS errors with `take_js_errors` (≤256 retained between drains).
- Panics: escher-telemetry's hook logs one ERROR at target `escher_telemetry::panic` (`panic.payload` redacted) and chains the previous hook.
- No spans, `#[instrument]`, metrics or OTel exist yet (observed absent).
- The agent-run test contract (`scripts/agent-run.sh`) writes JSON-line harness events to stdout and `target/agent-run/events.jsonl` — harness metadata, not a telemetry sink and not escher's line format; its rules live in `.claude/rules/verification-harness.md`.
- The cold-agent pipe (`scripts/cold-agent.sh`) writes JSON-line run events (`run.start`, `run.end`, `status`, `cleanup`) to stdout and `target/cold-agent/events.jsonl` — counts and identities only, never transcript text, tool arguments or results; its raw session transcript stays in `target/cold-agent/transcript.jsonl`, never printed (test-plan §3).

## Timing
- Phase timing is opt-in: `log-phase-times` (→ `debug_timer/enable`) and `log-frame-times`; `debug_timer` swaps in a zero-cost dummy when `enable` is off and prints to stdout when on.

## PII
- escher's sink applies an allowlist of targets in the formatter (obs-plan §8): engine targets (`blitz*`, `dioxus_native*`, `stylo_taffy`, `accesskit_xplat`, `debug_timer`, `js_console`) print only `node_id` · `status` · `waiting_nodes` · `property` · `log.module_path` · `log.file` · `log.line`; escher's own targets (`escher_*`) print every field except `url` · `href` · `src` · `html` · `text` · `value` · `attrs` · `path` · `request` · `error` · `panic.payload`, which are redacted; a record from any other target (Stylo's `style::*`, `selectors::matching`, `dioxus_core::*`; on the windowed stand also `naga::*`, `wgpu_*`, `winit_wayland::*`) is dropped whole, at every level and whatever `RUST_LOG` names — a third-party WARN or ERROR included. A sink-installing binary's stderr holds no id and no name at any level (measured on `escher-session` and on the windowed stand, before and after the drop; typed text unmeasured) — and a "nothing in the logs" check must still run against a host that installs the sink (seven_guis' `host_log` does), since a zero from one that does not proves nothing. A new field carrying user content takes a name in the content set — or is not logged.
- The upstream apps' `fmt::init()` and the WPT runner's `env_logger` still log URLs, attribute values, text-node contents and outer HTML as-is.

## Not yet measured — owned by the working route
- One span per driver command (settle wait, diff size, refusal cause) → "Driver command spans" chunk. The settle wait exists (`Harness::settle`, `Session::act`) and carries no span, event or log line — add none before that chunk. Its refusal-cause field takes an `escher_driver::Cause` name (eight fixed class words); never field a `Command`, `Call` or `ArgValue` with `?`, nor an argument by its schema name — both print an id and typed text, and of the argument names only `text` is scrubbed.
- Opt-in OTel export (`otel-sdk-install`) is out of escher 0.1.0 (the founder, 2026-10-06): add no OTel crate, egress or `OTEL_EXPORTER_OTLP_HEADERS` credential path; the transport and credential path are undecided — `.andromeda/residuals.md`.

## Session Additions
_This section is owned by `/andromeda-wrap-session`. setup-project preserves content added here on re-run._
- 2026-10-06: With the workspace's tracing 0.1.44, `tracing::error!(target: "…", a.b = x, "msg")` does not parse dotted field names (local macro ambiguity) and rejects string-literal keys — write `tracing::event!(target: "…", tracing::Level::ERROR, { a.b = x }, "msg")`.
