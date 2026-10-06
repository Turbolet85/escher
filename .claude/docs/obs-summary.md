# Observability Summary — escher

_Distilled from `.andromeda/obs-plan.md` (adopted reading). wrap-session's cascade re-derives it when the plan changes._

## Obs tier

**Tier:** 0
**Justification:** NOT YET MEASURED — the reading recorded no telemetry-surface table or must-trace paths.

## Harness contract (§3)

- **escher's sink:** `seven_guis_native` calls `escher_telemetry::init(service_identity!())` before `launch` (an `Err` is `eprintln!`ed, the stand continues): a global `Registry` + `EnvFilter` (`RUST_LOG`, default `warn`) + one non-ANSI fmt layer with the escher formatter, **stderr only**, plus the `log` → `tracing` bridge (`LogTracer`). Idempotent; a foreign subscriber is `InitError::ForeignSubscriber`, never a panic. The headless stand (`seven_guis::stand`, booted in-process by the `stand_*` checks) installs no subscriber.
- **Line format:** `{RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…` — one text line per event, not JSON; identity from the binary's own `CARGO_PKG_NAME` / `CARGO_PKG_VERSION` (the OTel resource keys).
- **Upstream loggers:** engine crates emit `tracing` 0.1 events behind per-crate `tracing` features (no-op when off; off in the stand); `tracing-subscriber` 0.3 `fmt::init()` to stdout in the upstream apps; `tracing_wasm` on wasm; `log` + `env_logger` in the WPT runner.
- **OTel SDK:** observed absent — escher 0.1.0 ships no OTel export (ruled by the founder, 2026-10-06): no OTel crate, egress or `OTEL_EXPORTER_OTLP_HEADERS` credential path; the transport and the credential path stay undecided, held in `.andromeda/residuals.md` for a later version.
- **Agent-run harness log (measured, not a telemetry sink):** `scripts/agent-run.sh` (test-plan §3) prints JSON lines — `boot`, `run.start`, `test`, `run.end` (also appended to `target/agent-run/events.jsonl`), `status`, `cleanup` — encoded by python3's `json`; harness metadata only, no captured test output, no content-named field; raw cargo output in `target/agent-run/run.log`, never printed.
- **Cold-agent pipe log (measured, not a telemetry sink):** `scripts/cold-agent.sh` (test-plan §3) prints JSON lines — `run.start {ts, task}`, `run.end {every verdict field}` (also appended to `target/cold-agent/events.jsonl`), `status`, `cleanup` — encoded by python3's `json`; the stdlib MCP stub appends `{seq, tool, outcome, cause}` per call to `calls.jsonl`; no `tracing`, OTel or env read.
- **escher's sink — JSON schema · log-file sink · rotation · heartbeat:** NOT YET MEASURED.
- **Embedder drain:** `take_js_errors` / `take_messages` on `ScriptDocument` (≤256 errors retained between drains).

## Metrics and timing (as built)
| Operation | Output |
|---|---|
| `resolve` phase times (style … subdocs) | printed, `log-phase-times` |
| Frame times | `log-frame-times` per anyrender backend |
| `debug_timer` labelled instants | stdout when `enable` |
| Browser FPS overlay | in-app |
| WPT per-test / per-run stats | printed; scores to Pages (upstream `DioxusLabs/blitz` only) |

No counters, histograms or exporters exist.

CI artifacts (§9): each ci.yml leg's merged output, `target/ci-logs/{leg}.log`, is uploaded only when the leg fails (`ci-log-{job id}`, kept 7 days, unscrubbed build output); the `coverage` job also uploads `coverage-report` (`target/coverage/`, line counts, no user data) on success, kept 7 days. The agent-run state area `target/agent-run/` and the cold-agent pipe's `target/cold-agent/` are local only — no CI job uploads either.

## Events and panics (§6, §7)
- escher-telemetry: `info` `telemetry installed` at target `escher_telemetry` (no argv / path / URL); ERROR `panic` at target `escher_telemetry::panic` with `panic.file` · `panic.line` · `panic.column` · `panic.payload` (redacted).
- Panic hooks: escher's native hook chains (logs, then runs the previous hook — std still prints the raw message, exit code unchanged); `console_error_panic_hook` on WASM; the WPT runner's hook replaces.

## SLO invariants (§10)
> NO RECORDED INTENT.

## PII (§8)
- **escher's sink scrubs by allowlist:** a target starting `blitz` · `dioxus_native` · `stylo_taffy` · `accesskit_xplat` · `debug_timer` · `js_console` prints only `node_id` · `status` · `waiting_nodes` · `property` · `log.module_path` · `log.file` · `log.line` — the message and every other field read `[redacted]`; `url` · `href` · `src` · `html` · `text` · `value` · `attrs` · `path` · `request` · `error` · `panic.payload` are redacted at any target.
- **Logged as-is elsewhere:** the upstream apps' `fmt::init()` and the WPT runner's `env_logger` carry visited URLs, resource URLs, link hrefs, element attributes, outer HTML of failed SVGs, CSS values and text-node contents unscrubbed. Past escher's scrub: std's panic hook prints the raw message, and `log.file` carries a host path for bridged third-party records at `RUST_LOG=info`.
- **Agent-run harness:** outside escher's scrub but carries no user content — its event fields take no content-named name and no captured test output; `target/agent-run/run.log` is raw cargo/libtest output, unscrubbed like `target/ci-logs/`, gitignored.
- **Cold-agent pipe:** outside escher's scrub; no event or verdict key is content-named and no transcript text, tool argument, tool result or reply reaches one; the stub never logs an argument value. `target/cold-agent/transcript.jsonl` is raw by design (model output, tool I/O, host paths, the client's socket path, a rate-limit line), never printed, gitignored; the one committed copy (live evidence) host-path-masked.

## Bootstrap phases (owners on the working route)
- `pii-scrubbing-wire` — discharged for escher's sink; still open for the upstream sinks.
- `otel-sdk-install` — not in escher 0.1.0 (the founder, 2026-10-06); a cross-version residual in `.andromeda/residuals.md`. The driver's spans stay with "Driver command spans".

## Universal anti-patterns
> NO RECORDED INTENT.

## Critical decisions
> NO RECORDED INTENT.

---

**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.claude/rules/observability.md`.
