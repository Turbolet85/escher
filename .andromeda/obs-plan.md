## 1. Obs Scope Summary

**Obs tier:** 0

**Instrumentation scope (entities needing instrumentation):**

- **Workspace** — dependencies include tracing, tracing-subscriber, tracing-log, tracing-wasm and console_error_panic_hook; the root dev-dependencies add env_logger (Cargo.toml:174; Cargo.toml:177; Cargo.toml:188-190; Cargo.toml:291)
- **Workspace** — `packages/debug_timer` is a workspace member (Cargo.toml:4; Cargo.toml:57)
- **apps/browser** — observability is `tracing` logging, optional frame/phase timing features, and an in-app FPS overlay (apps/browser/Cargo.toml:27-29; apps/browser/Cargo.toml:36; apps/browser/src/fps_overlay.rs:94-124)
- **examples** — console printing of timings (examples/screenshot.rs:200-214), and the escher-telemetry events of seven_guis' two binaries, `seven_guis_native` and `escher-session`, on stderr (examples/seven_guis/src/main.rs:6-9; examples/seven_guis/src/session_host.rs:20-22)
- **escher-telemetry** — escher's telemetry bootstrap crate: the stderr subscriber with service identity, the allowlist formatter — a scrub for engine and escher targets, a drop for every other target, printing one line per event and one per closed span — and the chaining panic hook (packages/escher-telemetry/src/lib.rs:1-22; Cargo.toml:17)
- **blitz-dom** — optional `tracing` logging, `println!` debug dumps and `debug_timer` phase timings (packages/blitz-dom/src/lib.rs:26-29; packages/blitz-dom/src/debug.rs:6-153; packages/blitz-dom/src/resolve.rs:75)
- **blitz-dom (events / util)** — six `tracing` log call sites, all behind the `tracing` feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:752-758; packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/util.rs:34-35)
- **blitz-dom (layout)** — emits `tracing` events only when the `tracing` feature is enabled (packages/blitz-dom/src/layout/mod.rs:131; packages/blitz-dom/src/layout/construct.rs:480; packages/blitz-dom/src/layout/damage.rs:491; packages/blitz-dom/src/layout/table.rs:507)
- **blitz-net, blitz-html, blitz-dom node module** — `tracing` log events behind optional cargo features (packages/blitz-net/Cargo.toml:18; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/node/element.rs:694-695)
- **blitz-shell, blitz-paint** — in-app devtools offer a layout outline, hover highlight overlay, node highlight overlay and taffy tree print (packages/blitz-shell/src/window.rs:667-685; packages/blitz-paint/src/render.rs:245-275; packages/blitz-paint/src/render.rs:1208-1224); the debug overlay visualises content, padding, border and margin boxes of a node (packages/blitz-paint/src/debug_overlay.rs:7-16)
- **blitz-vibey-script, blitz** — `tracing` is an optional dependency behind a `tracing` feature (packages/blitz-vibey-script/Cargo.toml:15; packages/blitz-vibey-script/Cargo.toml:40; packages/blitz/Cargo.toml:17; packages/blitz/Cargo.toml:34); JS console output is routed to the `log` crate (packages/blitz-vibey-script/src/runtime.rs:1248-1257)
- **debug_timer** — provides opt-in duration timing (packages/debug_timer/Cargo.toml:3; packages/debug_timer/Cargo.toml:11-12)
- **dioxus-native, dioxus-native-dom** — optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:58-67; packages/dioxus-native-dom/Cargo.toml:19)
- **wpt/runner** — `log`-facade logging via `env_logger` and printed run statistics (wpt/runner/src/main.rs:458; wpt/runner/src/main.rs:784-830)

> NOT YET MEASURED — the telemetry-surface table, must-trace critical paths and telemetry triggers: the reading recorded none of them for any slice

---

## 2. Telemetry Strategy

**Telemetry mechanism (current truth):**

- Telemetry is the `tracing` crate's event macros and, in escher-driver, one span macro (`info_span!`, §4), in the engine and upstream crates compiled in only with the `tracing` cargo feature (packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/events/ime.rs:27-28); escher-telemetry depends on `tracing` with no feature and always compiles its startup and panic events into the two native binaries of the seven_guis package, `seven_guis_native` and `escher-session`, whose engine `tracing` features stay off in a package-alone build (`-p seven_guis`; a workspace-wide build turns blitz-dom's on — Feature wiring, below); escher-driver is the second escher crate to take `tracing` ungated (`{ workspace = true }`, no `[features]` table; the workspace requirement is 0.1.40, the lock holds 0.1.44): its command span is compiled into both binaries through `seven_guis → escher-driver` and is reached by no code path in either (packages/escher-telemetry/Cargo.toml:13-16; packages/escher-telemetry/src/lib.rs:161; packages/escher-telemetry/src/panic.rs:11-21; packages/escher-driver/Cargo.toml:13-16; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md)
- Each feature-gated `tracing` call has a no-op path when the feature is off (packages/blitz-dom/src/layout/mod.rs:131-139; packages/blitz-dom/src/layout/construct.rs:480-488)
- Each blitz-dom log site is compiled only with `#[cfg(feature = "tracing")]` (packages/blitz-dom/src/document.rs:1276; packages/blitz-dom/src/mutator.rs:1233)
- The blitz-dom crate root comment lists a `tracing` feature that "Enables tracing support", under a TODO to document features (packages/blitz-dom/src/lib.rs:26-29)
- `tracing` is the only telemetry dependency in blitz-paint and blitz-shell, where it is optional (packages/blitz-paint/Cargo.toml:15; packages/blitz-paint/Cargo.toml:50; packages/blitz-shell/Cargo.toml:21; packages/blitz-shell/Cargo.toml:42)
- `tracing` is an optional dependency of blitz-net and blitz-html, enabled only by the `tracing` feature (packages/blitz-net/Cargo.toml:18; packages/blitz-net/Cargo.toml:35; packages/blitz-html/Cargo.toml:15; packages/blitz-html/Cargo.toml:26)

**Feature wiring:**

- The browser `tracing` feature enables tracing in dioxus-native, blitz-html, blitz-net, blitz-paint and pulls in tracing-subscriber (apps/browser/Cargo.toml:36)
- rdme's `tracing` feature enables it in blitz-shell, blitz-net, blitz-html (apps/readme/Cargo.toml:42)
- `tracing` is a default feature of blitz and forwards to blitz-shell, blitz-html and blitz-net (packages/blitz/Cargo.toml:14; packages/blitz/Cargo.toml:17)
- By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on — blitz's default `tracing` reaches blitz-shell and blitz-html, and each of those forwards to blitz-dom — while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events: one `INFO blitz_dom::document … message=[redacted]` beside a driver `type` (packages/blitz-shell/Cargo.toml:21; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/document.rs:1693; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/evidence/feature-unification.md, from cargo's resolved feature graph, the event at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md; test-plan §9 → Engine features by runner). Not measured: the two seven_guis binaries' stderr by level as a workspace build makes them
- blitz-vibey-script's `tracing` feature is off by default and also enables `blitz-dom/tracing` (packages/blitz-vibey-script/Cargo.toml:14-15)
- dioxus-native's `tracing` feature turns on tracing across dioxus-native-dom and the blitz crates (packages/dioxus-native/Cargo.toml:67)
- `log-frame-times` and `log-phase-times` features forward to renderer and DOM crates (apps/browser/Cargo.toml:27-29; apps/readme/Cargo.toml:34-41; examples/todomvc/Cargo.toml:23-25; examples/counter/Cargo.toml:21-22)
- In dioxus-native, `log-frame-times` turns on `log_frame_times` in whichever anyrender backend is enabled; `log-phase-times` forwards to blitz-dom (packages/dioxus-native/Cargo.toml:59-66)
- blitz-dom's `log-phase-times` enables `debug_timer/enable` (packages/blitz-dom/Cargo.toml:38)

**Absent:**

- observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
- observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files (examples slice)
- observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
- observed absent — tracing or metrics libraries · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files (wpt runner slice)

> NOT YET MEASURED — span, metric, log-field and resource-attribute naming conventions: the reading recorded none

---

## 3. Observability Harness Contract

**OTel SDK init:**

- observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

**Logging stack (subscriber installation):**

- `seven_guis_native` (the windowed stand binary) installs `escher_telemetry::init(escher_telemetry::service_identity!())` in the native `main` before `dioxus_native::launch`, reporting an `Err` with `eprintln!` and continuing (examples/seven_guis/src/main.rs:4-10): a global `Registry` with an `EnvFilter` from `RUST_LOG` (default `warn`) and one non-ANSI fmt layer — stated once, in the private `sink_layer`, with `with_span_events(FmtSpan::CLOSE)` and the crate-private field formatter `SpanFields` — that prints each event and, since 2026-10-07-driver-command-spans, each closed span through the escher formatter (§6 Log format), writing to stderr only, plus the `tracing_log::LogTracer` bridge for `log` records (packages/escher-telemetry/src/lib.rs:102-116; packages/escher-telemetry/src/lib.rs:130-163)
- `escher-session` (the headless session host, the second binary of the seven_guis package) installs the same `escher_telemetry::init(escher_telemetry::service_identity!())` first thing in its native `main` — before argv is read and before anything boots — reporting an `Err` with `eprintln!` and continuing; the same subscriber, stderr only, and nothing on stdout on any path; `init` has two install sites (examples/seven_guis/src/session_host.rs:20-22; examples/seven_guis/Cargo.toml:45-47; as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md)
- the headless stand `seven_guis::stand` (native only; `boot` / `boot_timer` / `options`, driven in-process by the `stand_*` checks, 15 of which read the shared module `tests/blitz-tests/tests/common/mod.rs`) installs no subscriber — no `escher_telemetry::init` and no env read in it, its checks or their shared modules, with one exception since 2026-10-07-driver-command-spans: `stand_act_spans`, whose two parents install none and re-run the test binary on two ignored children — `RUST_LOG=info` set on one, `RUST_LOG` removed from the other — each of which installs the sink with `escher_telemetry::init_with_writer` over an in-memory capture and drives the driver's commands in process, the capture read into assertions only and a failing parent relaying the child's own failure message, never a captured line — and no `println!` in any of them save `stand_id_persistence`'s re-executed child, which prints its pid and its ids to stdout that the parent captures and reads only into its assertion (never a log) — so a headless boot made in process has no escher sink, those two children's aside; the same holds for the session library `packages/escher-driver` (a `tracing` dependency, with no feature, since 2026-10-07-driver-command-spans; no subscriber, no env read, no print) — its settled step `Session::act`, its command and refusal schema and its executor included (the private modules `command`, `refusal`, `schema` and `execute` install no subscriber, read no env var and no clock, and print nothing, while `execute` opens the one span of a command (§4), which writes only where a process has installed a sink at `info` or below — `Session::run`, with the refusal detection it runs in front of `click` and `type` and its sixth verb `scroll`, and `Session::with_time` with them; `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold, and so does `Outcome`, whose `Debug` prints the ids, accessible names and control values of its diff, a password's or a file input's value as the snapshot's mask, and nothing in the crate prints, logs or fields any of the four; the session's record of the ids its screens have read — id text, at most 4096 ids, kept for the session's life behind one private field of `Session` — has no reader outside the crate and no `Debug`, and nothing prints, logs or fields it either) — for the harness's settle loop (`Harness::settle`, `packages/blitz-test-harness/src/settle.rs`: no `tracing` dependency in the crate, no env read, no print, no log), for the five `stand_session_*` checks, for `stand_settle`, for the ten driver-action checks `stand_act_*` (the two children of `stand_act_spans` aside), for the engine's into-view check `scroll_into_view_nested`, and for the module fifteen of the sixteen checks that hold a session read — four of the five `stand_session_*`, `stand_settle` and the ten `stand_act_*` — `tests/blitz-tests/tests/session_common/mod.rs`; a headless boot made by the `escher-session` binary runs under that binary's sink. Two of the session checks spawn a host: `stand_session_lifecycle`'s child has its streams sent to null, and `stand_session_quiet`'s child — the re-run test binary — installs no sink, so the `RUST_LOG=trace` it is started with is inert and its zero says nothing about a host that installs one; seven_guis' `host_binary` spawns the real binary at `RUST_LOG=info` and reads both streams into its assertions only; seven_guis' `host_log` spawns the same binary on `crud` at `RUST_LOG=trace`, drains both streams from the spawn on and reads them into its assertions only — it is the check that reads a sink-installing host at `trace`, and a failure of it prints a kind and a count, never what the host wrote; the two share the module `examples/seven_guis/tests/common/mod.rs`, which installs no subscriber (examples/seven_guis/src/stand.rs:1-118; tests/blitz-tests/tests/stand_id_persistence.rs:1-3; tests/blitz-tests/tests/common/mod.rs:1-6; tests/blitz-tests/tests/session_common/mod.rs:1-6; packages/escher-driver/Cargo.toml:13-16; tests/blitz-tests/tests/stand_act_spans.rs:91; tests/blitz-tests/tests/stand_act_spans.rs:480-488; examples/seven_guis/tests/host_log.rs:73-87; examples/seven_guis/tests/common/mod.rs:1-4; as measured at escher-0.1.0/chunks/2026-10-06-id-persistence/report.md, the shared module at escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md, the session library and its checks at escher-0.1.0/chunks/2026-10-07-driver-session/report.md, `host_log` at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/report.md, the settle loop and `stand_settle` at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md, the command and refusal schema at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md, the executor and the six `stand_act_*` checks at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md, the refusal detection, the session's record of ids, `scroll_into_view_nested` and the three further `stand_act_*` checks at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md, the `tracing` dependency, the command span and `stand_act_spans` at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md)
- The upstream apps install `tracing_subscriber::fmt::init()` under the `tracing` feature, writing to stdout (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:61-62; examples/todomvc/src/main.rs:16-17)
- wasm_hello installs `tracing_wasm::set_as_global_default()` (examples/wasm_hello/src/lib.rs:105)
- The wpt runner logs through the `log` facade via `env_logger` (wpt/runner/src/main.rs:458; wpt/runner/src/main.rs:784-830)
- observed absent — tracing subscriber or exporter setup · searched: `subscriber|opentelemetry|sentry` over the 15 s05 files
- observed absent — a tracing subscriber or exporter setup · searched: `tracing_subscriber|subscriber` over the 17 listed s06 files
- observed absent — a tracing subscriber or exporter set up in these crates · searched: `subscriber` over the 21 listed s11 files

**Embedder drain of script diagnostics:**

- Embedders drain JS errors with `take_js_errors` and JS messages with `take_messages` (packages/blitz-vibey-script/src/document.rs:242-261)
- At most 256 errors are retained between drains (packages/blitz-vibey-script/src/state.rs:101-103; packages/blitz-vibey-script/src/document.rs:256)

**Service identity and line format (escher's sink):** every line carries `service.name` / `service.version`, read from the binary's own `CARGO_PKG_NAME` / `CARGO_PKG_VERSION` by `service_identity!()` — the OTel resource keys — in a one-line text format, not JSON (packages/escher-telemetry/src/lib.rs:44-66; §6 Log format); both binaries of the seven_guis package, `seven_guis_native` and `escher-session`, log `service.name=seven_guis` — the name is the package's, so it does not tell the two apart (as measured at escher-0.1.0/chunks/2026-10-06-telemetry-bootstrap/evidence/smoke-004017Z.txt)

**Agent-run harness log (the test contract's, not escher's sink):** `scripts/agent-run.sh` — the 5-command test contract of test-plan §3 — writes JSON lines, one object per event, encoded by python3's `json` module embedded in the script, never assembled in bash: `boot`, `run.start`, one `test` per libtest result line, `run.end` (printed to stdout and appended to `target/agent-run/events.jsonl`), and `status`, `cleanup` (printed only); `logs` prints `events.jsonl` verbatim. Its state area is `target/agent-run/{status.json, events.jsonl, run.log}` — `run.log` the raw merged cargo/libtest output, never printed. It is harness metadata, not a telemetry sink: no `tracing` subscriber is involved in the harness itself, and of the stand checks only `stand_act_spans`'s two re-run children install escher's sink, over an in-memory capture that reaches no harness event (Logging stack, above) (scripts/agent-run.sh:25-142; test-plan §3; as measured at escher-0.1.0/chunks/2026-10-06-stand-test-contract/report.md)

**Cold-agent pipe log (a harness, not escher's sink):** `scripts/cold-agent.sh` — the cold-agent run pipe of test-plan §3 (`run counter` · `status` · `cleanup` · `logs`; `scripts/cold-agent.ps1` a pass-through) — writes JSON lines encoded by python3's `json` module embedded in the script: `run.start`, `run.end` (printed to stdout and appended to `target/cold-agent/events.jsonl`), `status` and `cleanup` (printed only); `logs` prints `events.jsonl`; the run's verdict is `target/cold-agent/verdict.json`. The stdlib MCP stub `scripts/cold_agent_stub.py` appends one call-log line per tool call to the run's `calls.jsonl`. No `tracing` subscriber, OTel setup or third-party logger is involved, and neither reads an env var. Its state area is `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, calls.jsonl, stub-state.json}` — `transcript.jsonl` the session's raw stream and `client.log` the client's stderr, neither printed (scripts/cold-agent.sh:26-215; scripts/cold_agent_stub.py:113-118; test-plan §3; as measured at escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/report.md)

> NOT YET MEASURED — product mode, snapshot integration, trace context propagation and heartbeat ticks, and a JSON schema or log-file location for escher's own sink: the reading recorded none of them

Contracts: .andromeda/registries/obs-plan-contracts.toml — ask registry.py contracts; read one contracts/obs-plan/{key}.md; never whole.

## 4. Span / Trace Coverage

- observed absent — spans · searched: `span!|instrument` over the 32 slice files (examples slice)
- observed absent — spans or instrumented functions · searched: `span!|#\[instrument|info_span|debug_span` over the 86 slice files (apps slice)
- observed absent — spans or instrumented functions · searched: `info_span|debug_span|#\[instrument|tracing::span` over the 15 s05 files
- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 17 listed s06 files
- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 8 slice files (layout slice)
- observed absent — spans or `#[instrument]` · searched: `instrument|span` over the 16 listed files (only a doc comment "The node id for the span" matched)
- observed absent — span or instrument usage · searched: `instrument|span!` over the 32 listed s09 files
- observed absent — tracing spans or instrumentation · searched: `span!|instrument` over the 32 slice files (blitz-vibey-script / blitz slice)
- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 21 listed s11 files
- observed absent — spans or traces · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
- observed absent — spans or traces · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files (wpt runner slice)
- `packages/escher-driver` — one span per driver command, built at 2026-10-07-driver-command-spans (the route entry "Driver command spans", delivered): every call handed to `Session::run` opens one `tracing` span — target `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`), created and entered before `validate` and held until `run` returns, so a refusal `validate` returns is inside it — whose eight fields are each created unrecorded and recorded only where they apply: `verb`, the verb table's own word for a known verb, run or refused, never the caller's verb text and unrecorded for a verb the table lacks; `settled`, `busy` (the schema's word for the busy class, when the outcome carries one), `passes` (settle's pass count, when settle answered quiet) and `added` · `removed` · `changed` (the three list lengths of the returned diff), from an acting verb's outcome; and `cause`, recorded at one site for every refusal `run` returns — one of the eight names `Cause::name` returns (`unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable`), fixed class words holding nothing a call supplied, all eight returned by code today (`unknown-verb` and `malformed` by `validate`; `not-found`, `stale`, `disabled`, `covered`, `off-screen` and `time-unavailable` by the executor); `snapshot` records `verb` alone, and a refused call records `verb` (when known) and `cause` and none of the other six; never fielded — `in_view`, `advanced_ms`, the screen text or its length, the session's label, the record of ids, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`: an argument fielded by its schema name, or a `Command` or an `Outcome` fielded with `Debug`, would print an id, typed text, an accessible name or a control's value (of the argument names only `text` is in the sink's scrub set, §8); the driver emits no event and installs no subscriber, so with none installed the span is disabled, under escher's sink it is one line per call at `info` or below (§6) and at the default `warn` it is filtered out before it is created; not driven: no stand step reads not settled, so `busy` is covered by its unit test only. Still observed absent — a span, an event or a `tracing` dependency in the settle wait itself, `Harness::settle` in `packages/blitz-test-harness`, and a span of their own on `Session::start`, `Session::act`, `serve`, `start`, `attach`, `stop` or the lifecycle wire: the span is `run`'s (packages/escher-driver/Cargo.toml:13-16; packages/escher-driver/src/execute.rs:64-79; packages/escher-driver/src/execute.rs:108-117; packages/escher-driver/src/execute.rs:183-226; packages/escher-driver/src/execute.rs:342-359; packages/escher-driver/src/command.rs:167; packages/blitz-test-harness/src/settle.rs:129-191; as measured at escher-0.1.0/chunks/2026-10-07-driver-session/report.md, the settle wait at escher-0.1.0/chunks/2026-10-07-settle-detection/report.md, the schema and its cause names at escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/report.md, the executor and the two causes it returned at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md, the six it returns now at escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md, the span at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md)

---

## 5. Metric Coverage

**In-process timing and counting (current truth):**

| Operation | What is recorded | Output | Source |
|-----------|------------------|--------|--------|
| Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:258-261); (packages/dioxus-native/Cargo.toml:58-66) |
| blitz-dom `resolve` | Phase times style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs | printed, prefixed `Resolve({id}): ` | (packages/blitz-dom/src/resolve.rs:75-167) |
| debug_timer | Labelled instants; total and per-step durations in ns/us/ms/s | printed | (packages/debug_timer/src/lib.rs:14-24; packages/debug_timer/src/lib.rs:33-66) |
| Browser FPS overlay | Frame deltas in a 60-entry ring, polled every 250 ms; average FPS and ms | in-app overlay, toggled from the menu item "Toggle FPS" | (apps/browser/src/fps_overlay.rs:7; apps/browser/src/fps_overlay.rs:27-49; apps/browser/src/fps_overlay.rs:105-123); (apps/browser/src/toolbar.rs:437-440) |
| blitz-paint LayerManager | In-process counters layers_used, layer_depth, layers_wanted and a debugging-only layer_depth_used | in-process | (packages/blitz-paint/src/layers.rs:8-16; packages/blitz-paint/src/layers.rs:58; packages/blitz-paint/src/layers.rs:81-84) |
| `paint_bench` example | Per-iteration encode and raster microseconds | printed min/median/mean/max | (examples/paint_bench.rs:28-54) |
| `screenshot` example | Millisecond timings for fetch, setup, parse, asset fetch, style/layout, render and PNG write | printed | (examples/screenshot.rs:56; examples/screenshot.rs:68; examples/screenshot.rs:86; examples/screenshot.rs:95; examples/screenshot.rs:100; examples/screenshot.rs:134; examples/screenshot.rs:143) |
| `paint_tree_bench` test | Medians of `hit()`, render on a null backend, an incremental hover-only frame and a full non-incremental frame | printed as a markdown table | (tests/blitz-tests/tests/paint_tree_bench.rs:1-3; tests/blitz-tests/tests/paint_tree_bench.rs:100-134; tests/blitz-tests/tests/paint_tree_bench.rs:341-370) |
| `paint_tree_bench` external page | Node count and hoisted stacking-context entries | printed | (tests/blitz-tests/tests/paint_tree_bench.rs:271-278) |
| WPT runner, per test | Duration in ms | printed | (wpt/runner/src/main.rs:614; wpt/runner/src/main.rs:695; wpt/runner/src/main.rs:369) |
| WPT runner, per run | Pass, fail, timeout, skip, crash, subtests, fractional pass, failure buckets and total duration | printed | (wpt/runner/src/main.rs:488-508; wpt/runner/src/main.rs:784-830) |
| WPT runner, report | Report generation and write times in ms | printed | (wpt/runner/src/main.rs:838-852) |
| WPT scores (CI) | Scores computed into `wptscores.json` | published to Pages, on upstream `DioxusLabs/blitz` only (the `wpt` job is repository-guarded) | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:72-75) |

**Absent:**

- observed absent — metrics exporters · searched: `opentelemetry|metrics|prometheus|sentry` over the 86 slice files (apps slice)
- observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files
- observed absent — metric emission · searched: `counter!|histogram!|gauge!` over the 17 listed s06 files
- observed absent — metrics · searched: `metric|counter!|histogram` over the 8 slice files (matches are only text-layout `metrics()` calls and comments)
- observed absent — metrics · searched: `metric|counter|histogram|gauge` over the 16 listed files (no match)
- observed absent — metrics counters, gauges or histograms · searched: `metrics|counter!|histogram|gauge` over the 32 slice files (blitz-vibey-script / blitz slice)
- observed absent — metrics or telemetry backends · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files

---

## 6. Log Coverage

**Log format (escher's sink):**

- One line per printed event and one per closed span, on stderr, no ANSI. An event's line: `{RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…`; newlines and carriage returns in values escaped as `\n` / `\r`, string values Debug-quoted; a bridged `log` record printed under its `log.target`, that field itself omitted; every field passes the §8 scrub; an event from a target outside the sink's allowlist (§8 → Scrubbing) prints nothing at all — not an empty line — a bridged record judged by its `log.target` (packages/escher-telemetry/src/format.rs:134-286)
- A closed span's line — PROVISIONAL: a boundary widening on the operator's answer (2026-10-07, at the plan forks of 2026-10-07-driver-command-spans; that chunk's `inputs#I1`), not yet the founder's word, kept marked at that chunk's wrap on the operator's word for the founder's batch at the Epoch 4 boundary — is written for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited: `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then the layer's close fields `message`, `time.busy` and `time.idle`; level and target are the span's, the two timings are `tracing-subscriber`'s own (no escher code reads a clock for them), and `span` is a reserved field name; every pair, `span` included, passes the §8 scrub under the span's target; a string value is Debug-quoted (`span="command"`, `message="close"`), a number or a bool bare; a span from a target outside the allowlist writes no byte at any point; an event's line is unchanged, an event emitted inside a span included — it gains no span name and no span field; as read from a real capture: `… INFO escher_driver service.name=blitz-tests service.version=0.3.0-beta.2 span="command" verb="click" passes=1 settled=true added=1 removed=0 changed=1 message="close" time.busy=11.0ms time.idle=4.51µs` (packages/escher-telemetry/src/lib.rs:102-116; packages/escher-telemetry/src/format.rs:138-177; packages/escher-telemetry/src/format.rs:200-250; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md)
- Per-module levels come from `RUST_LOG` through `EnvFilter`, defaulting to `warn` when unset or unparsable (packages/escher-telemetry/src/lib.rs:144); a directive that names a target outside the sink's allowlist (`style=trace`) lets its records past the filter and the formatter still drops them — the formatter is the last thing a record meets (tests/blitz-tests/tests/telemetry_drop.rs:14-22)

**Log format (the agent-run harness, §3):**

- One JSON object per line on stdout, the same lines appended to `target/agent-run/events.jsonl` (recreated by `boot`, removed by `cleanup`): `boot {ts, outcome, cargo_exit, head}` · `run.start {ts, selection, files}` · `test {file, test, outcome}` · `run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome}`; `status {booted, boot_ts, head, run}` and `cleanup {outcome}` are printed only (scripts/agent-run.sh:100-142)
- One `test` event per libtest result line, `file` the target binary's stem with its hash stripped or `doc:{crate}`; nothing between a `failures:` line and the next `test result:` is read, so captured stdout and panic text never reach an event; the raw cargo output goes to `target/agent-run/run.log`, truncated at each run's start and never printed (scripts/agent-run.sh:80-98; scripts/agent-run.sh:196-208)

**Log format (the cold-agent pipe, §3):**

- One JSON object per line on stdout, the same lines appended to `target/cold-agent/events.jsonl` (recreated by `run`, removed by `cleanup`): `run.start {ts, task}` · `run.end {every verdict field}`; `status {the verdict}` or `status {outcome: "none"}` (exit 3) and `cleanup {outcome: "done"}` are printed only (scripts/cold-agent.sh:178-212)
- The verdict fields: `task, client_exit, isolated, session_tools, client_version, model, tool_calls, wrong_calls, transcript_errors, counts_agree, final_count, num_turns, duration_ms, cost_usd, input_tokens, output_tokens, cache_read_input_tokens, cache_creation_input_tokens, transcript_file, outcome, reasons, ts` — counts and identities, no transcript text, tool argument, tool result or reply (scripts/cold-agent.sh:114-175)
- The stub's call log: one line per `tools/call`, `{seq, tool, outcome: ok|refused, cause}` — never an argument value; an unknown tool name is logged as `tool: null` (scripts/cold_agent_stub.py:96-118)
- The client's raw stream-json goes to `target/cold-agent/transcript.jsonl` and its stderr to `client.log`; neither is printed (scripts/cold-agent.sh:254-258)

> NOT YET MEASURED — a log JSON schema, a log-file sink and a rotation policy for escher's own sink: the reading recorded none of them

**Logged events (current truth):**

- **apps/browser**
  - Document loading logs info on success and error on failure (apps/browser/src/document_loader.rs:121; apps/browser/src/document_loader.rs:152)
  - Script prefetch failures and JS errors log at error (apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244)
  - Urlbar parse failures log at warn (apps/browser/src/toolbar.rs:130)
  - Every persistence failure path logs at warn (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:176; apps/browser/persistence/src/lib.rs:202; apps/browser/persistence/src/lib.rs:263; apps/browser/persistence/src/lib.rs:273; apps/browser/persistence/src/lib.rs:298-300)
  - Dropped disk writes log at warn (apps/browser/src/browser_history.rs:161)
- **examples**
  - wasm_hello logs "Starting app..." at info (examples/wasm_hello/src/lib.rs:107)
  - Example output uses `println!`/`eprintln!` (examples/screenshot.rs:203; examples/paint_bench.rs:34; examples/paint_bench.rs:321), except `seven_guis_native` and `escher-session`, which log through escher-telemetry (below); `escher-session`'s own stderr lines are three `eprintln!`s — a usage line on any argv but `<task> <state-dir>` (exit 2), the `SessionError` message on a failed start or serve (exit 1) and `telemetry not installed: …` — fixed strings holding no path, label, id or value, and it prints nothing to stdout (examples/seven_guis/src/session_host.rs:21; examples/seven_guis/src/session_host.rs:30; examples/seven_guis/src/session_host.rs:54)
- **escher-telemetry (in `seven_guis_native` and `escher-session`)**
  - info at target `escher_telemetry`, message `telemetry installed`, once per successful init — no argv, path or URL (packages/escher-telemetry/src/lib.rs:161)
  - ERROR at target `escher_telemetry::panic`, message `panic`, fields `panic.file`, `panic.line`, `panic.column`, `panic.payload` (redacted), once per panic (packages/escher-telemetry/src/panic.rs:11-21)
- **escher-driver (in a process that installs escher's sink at `info` or below)**
  - one closed-span line per call to `Session::run`: INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`, then `message="close"`, `time.busy`, `time.idle` — six verb words, eight cause names, three busy-class words, bools and counts; no line at the default `warn`; today only the two re-run children of `stand_act_spans` run a command with the sink installed — `escher-session`'s wire is `hello` and `stop`, and `seven_guis_native` runs none (packages/escher-driver/src/execute.rs:64-79; packages/escher-driver/src/execute.rs:183-217; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md)
- **blitz-dom (document / resolve / mutator / net)**
  - warn: no DOM on resolve (packages/blitz-dom/src/resolve.rs:44) and on hit test (packages/blitz-dom/src/document.rs:1821)
  - warn: unimplemented form scheme/method (packages/blitz-dom/src/form.rs:152-157)
  - info: image cache hit, pending queue and fetch (packages/blitz-dom/src/mutator.rs:1191-1211); image loaded and node count (packages/blitz-dom/src/document.rs:1386-1390)
  - warn: iframe depth cap and unresolvable iframe URL (packages/blitz-dom/src/mutator.rs:1233-1237; packages/blitz-dom/src/mutator.rs:1260-1261)
  - warn: resource load failed with and without URL (packages/blitz-dom/src/document.rs:1276-1287)
  - info: focussed node (packages/blitz-dom/src/document.rs:1692-1693)
  - info/warn: WOFF decompression and skipped font sources (packages/blitz-dom/src/net.rs:340-365; packages/blitz-dom/src/net.rs:476-495)
  - `debug_log_node` prints layout, attributes, inline layout and children via `println!` and `tracing::info!` (packages/blitz-dom/src/debug.rs:17-153)
- **blitz-dom (events / util)**
  - `debug` logs a sent IME event with a `node_id` field (packages/blitz-dom/src/events/ime.rs:28)
  - `warn` logs an unparseable link href together with the document URL (packages/blitz-dom/src/events/pointer.rs:753)
  - `info` logs a click on a link without href together with the element's attributes (packages/blitz-dom/src/events/pointer.rs:758)
  - `warn` logs WOFF1 and WOFF2 decompression failures (packages/blitz-dom/src/util.rs:27; packages/blitz-dom/src/util.rs:35)
  - `warn` logs a call to `stylo_to_cursor_icon` with `CursorKind::Auto` (packages/blitz-dom/src/stylo_to_cursor_icon.rs:10)
- **blitz-dom (layout)**
  - `tracing::error!` "Tried to lay out text node individually" with fields `node_id`, `data` (packages/blitz-dom/src/layout/mod.rs:131-136)
  - `tracing::warn!` "SVG parse failed" with fields `node_id`, `html`, `error` (packages/blitz-dom/src/layout/construct.rs:480-486)
  - `tracing::info!` for image loading from cache, image already pending, and image fetch start, each including the image URL (packages/blitz-dom/src/layout/damage.rs:491-492; packages/blitz-dom/src/layout/damage.rs:500-501; packages/blitz-dom/src/layout/damage.rs:506-507)
  - `tracing::info!` "Ignoring table descendent because it has no styles" (packages/blitz-dom/src/layout/table.rs:507-508)
- **blitz-dom (node)**
  - warn "Unsupported property" with field `property`, and warn "Invalid property value" with fields `property` and `value` (packages/blitz-dom/src/node/element.rs:693-708; packages/blitz-dom/src/node/element.rs:759-762)
  - `Node::print_tree` writes the tree to stdout with `println!` (packages/blitz-dom/src/node/node.rs:959-973)
- **blitz-net**
  - info "Using cache dir" with field `path` (packages/blitz-net/src/lib.rs:57-58)
  - error "Failed to clear HTTP cache: {:?}" (packages/blitz-net/src/lib.rs:140-141)
  - warn "HTTP error status" with fields `url` and `status` (packages/blitz-net/src/lib.rs:210-215)
  - error "Fetching" with fields `url` and `error`, and info "Success fetching" with field `url`, in `fetch_with_callback` and `fetch_async` (packages/blitz-net/src/lib.rs:236-243; packages/blitz-net/src/lib.rs:257-264)
  - info "Fetching" with field `url`; info "Success fetching" with field `url`; and error "Error fetching" with fields `url` and `error` in `NetProvider::fetch` (packages/blitz-net/src/lib.rs:275-276; packages/blitz-net/src/lib.rs:301-306)
- **blitz-html**
  - Each collected HTML parse error is logged at error level on sink finish (packages/blitz-html/src/html_sink.rs:181-186)
- **blitz-shell, blitz-paint**
  - `tracing::error!` is logged for unsupported Ime, PointerSource, PointerKind, ButtonSource and MouseScrollDelta variants (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-shell/src/convert_events.rs:81-85; packages/blitz-shell/src/convert_events.rs:97-101; packages/blitz-shell/src/convert_events.rs:115-119; packages/blitz-shell/src/convert_events.rs:143-147; packages/blitz-shell/src/window.rs:833-837)
  - `warn!` is logged for unimplemented image layer kinds and unsupported mask-mode luminance (packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:102-111)
- **blitz, blitz-vibey-script**
  - `launch_url` logs `tracing::info!` with "Launching" and the URL under the `tracing` feature (packages/blitz/src/lib.rs:48-49)
  - Recorded script errors are logged with a "blitz-vibey-script:" prefix under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:264-267)
  - Uncaught JS errors are logged as "Uncaught JS error in" plus the source description (packages/blitz-vibey-script/src/runtime.rs:1104-1105)
  - JS console log, info, warn and error all map to one debug-level log call (packages/blitz-vibey-script/src/runtime.rs:1253-1269)
- **dioxus-native, dioxus-native-dom**
  - Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:152-238; packages/dioxus-native-dom/src/mutation_writer.rs:338; packages/dioxus-native-dom/src/mutation_writer.rs:421)
  - Asset fetch success is logged at trace and failure at warn; fetches without a net provider are logged at warn (packages/dioxus-native/src/assets.rs:47-60)
  - A failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:14-15)
  - Injecting the document provider into windows is logged at debug (packages/dioxus-native/src/dioxus_application.rs:137-138)
- **tests/blitz-tests**
  - A `blitz-dom/log-phase-times` feature prints per-phase resolve timings (tests/blitz-tests/tests/paint_tree_bench.rs:223-224; tests/blitz-tests/tests/paint_tree_bench.rs:261)
  - Test diagnostics go to stdout/stderr via `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/paint_tree_bench.rs:266)
- **wpt/runner**
  - Log calls cover WPT_DIR setup, glob failures, net load errors, pending requests, skips, JS errors and missing harness results (wpt/runner/src/main.rs:464-469; wpt/runner/src/main.rs:290; wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220; wpt/runner/src/test_runners/harness_test.rs:151-153; wpt/runner/src/test_runners/harness_test.rs:179)
  - Log messages are free-form format strings, not structured fields (wpt/runner/src/net_provider.rs:110; wpt/runner/src/test_runners/mod.rs:182)
- **CI publish builds**
  - Publish builds log at `CARGO_LOG: info` with `--verbose --trace`, on upstream `DioxusLabs/blitz` only — the `release-cli` job is repository-guarded (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:37; .github/workflows/publish-browser.yml:155)

**Absent:**

- observed absent — console logging calls · searched: `console\.` over the 21 s02 files
- observed absent — structured logging · searched: `tracing::|log::|env_logger` over the 32 slice files (examples slice); `seven_guis_native` has since gained escher-telemetry's subscriber (§3)

---

## 7. Error Capture & Reporting

**Panic hooks:**

- `console_error_panic_hook` is a workspace dependency (Cargo.toml:177)
- WASM builds install `console_error_panic_hook` (examples/seven_guis/src/lib.rs:13; examples/todomvc/src/wasm.rs:8; examples/wasm_hello/src/lib.rs:104)
- The wpt runner's panic hook captures message, file, line, column and a forced backtrace (wpt/runner/src/panic_backtrace.rs:12-38)
- escher-telemetry's hook, installed by `init` in `seven_guis_native` and in `escher-session`, chains: it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged (packages/escher-telemetry/src/panic.rs:4-24)
- A crashed WPT test's panic message is carried into the report's `message` field (wpt/runner/src/report.rs:95)
- `pump_net_provider` logs pending items before panicking on its 500 ms timeout (wpt/runner/src/test_runners/mod.rs:392-398)

**Error classes captured:**

- Browser: JS errors are drained with `take_js_errors` and logged (apps/browser/src/document_loader.rs:241-244); load errors are shown to the user on an error page with the Debug-formatted error (apps/browser/src/document_loader.rs:154-166)
- blitz-dom: resource load errors are logged as `tracing::warn!` with `error` field and not propagated further (packages/blitz-dom/src/document.rs:1273-1292)
- blitz-dom: stylesheets are parsed with no error reporter (`None, // error_reporter`) (packages/blitz-dom/src/net.rs:166; packages/blitz-dom/src/net.rs:276; packages/blitz-dom/src/document.rs:1186)
- blitz-dom: recoverable failures are logged and a fallback is used, as with font decompression (packages/blitz-dom/src/util.rs:25-29)
- blitz-dom layout: SVG parse errors are captured into the `error` field of a warn event and not propagated (packages/blitz-dom/src/layout/construct.rs:479-489)
- blitz-net: fetch errors in `NetProvider::fetch` are logged and not propagated to the handler (packages/blitz-net/src/lib.rs:298-310); `ProviderError` implements `Display` with a message per variant (packages/blitz-net/src/lib.rs:369-382)
- blitz-shell: DataUriNetProvider's error callbacks are commented out, so parse, decode and unsupported-scheme failures are not reported (packages/blitz-shell/src/net.rs:54-67)
- dioxus-native: errors are reported only as `tracing` warn/error events (packages/dioxus-native/src/assets.rs:52-53; packages/dioxus-native/src/link_handler.rs:14-15)
- WPT subtest error strings are joined with newlines into the report's subtest `message` (wpt/runner/src/report.rs:104-108)

**Script (JS) error surface — blitz-vibey-script:**

- The window `error` event carries message, filename "", lineno 0, colno 0 and error (packages/blitz-vibey-script/src/runtime.rs:1142-1158)
- `window.onerror` is called with message, source, lineno, colno, error (packages/blitz-vibey-script/src/runtime.rs:1168-1180)
- Exceptions thrown by error handlers are recorded but fire no further error events (packages/blitz-vibey-script/src/runtime.rs:1118-1120; packages/blitz-vibey-script/src/state.rs:91-94)
- Error sources labelled in reports include "timer callback", "event listener", "error event listener", "timer microtasks", "event microtasks" (packages/blitz-vibey-script/src/runtime.rs:1630; packages/blitz-vibey-script/src/runtime.rs:1633; packages/blitz-vibey-script/src/runtime.rs:1685; packages/blitz-vibey-script/src/runtime.rs:1795; packages/blitz-vibey-script/src/runtime.rs:1165)

**Absent:**

- observed absent — an error-reporting client · searched: `sentry|analytics|gtag` over the 21 s02 files
- observed absent — error capture service · searched: `sentry|Sentry` over the 32 slice files (examples slice)
- observed absent — error reporting services · searched: `sentry|opentelemetry` over the 15 s05 files
- observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files
- observed absent — an error reporting service · searched: `sentry|opentelemetry|otel|span!` over the 16 listed files (no match)
- observed absent — an error-reporting service · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files

---

## 8. PII Scrubbing & Compliance

**Values logged as-is (current truth):**

- Visited URLs are logged at info and urlbar text at warn (apps/browser/src/document_loader.rs:121; apps/browser/src/toolbar.rs:130)
- blitz-dom log lines include resource URLs (packages/blitz-dom/src/document.rs:1277-1282; packages/blitz-dom/src/mutator.rs:1192; packages/blitz-dom/src/mutator.rs:1211)
- `debug_log_node` prints every attribute name and value of a node (packages/blitz-dom/src/debug.rs:28-32)
- Log lines include the raw href, the document URL and element attributes unfiltered (packages/blitz-dom/src/events/pointer.rs:753; packages/blitz-dom/src/events/pointer.rs:758)
- The SVG parse-failure event carries the element's full outer HTML in field `html` (packages/blitz-dom/src/layout/construct.rs:463; packages/blitz-dom/src/layout/construct.rs:481-486)
- Image-fetch info events carry the full image URL (packages/blitz-dom/src/layout/damage.rs:489-507)
- Full request URLs are logged as field `url` (packages/blitz-net/src/lib.rs:229; packages/blitz-net/src/lib.rs:276; packages/blitz-net/src/lib.rs:281)
- The CSS property value is logged in the "Invalid property value" warning (packages/blitz-dom/src/node/element.rs:706-707)
- dioxus-native debug logs record text-node contents and attribute values; asset logs record the full request (packages/dioxus-native-dom/src/mutation_writer.rs:183; packages/dioxus-native-dom/src/mutation_writer.rs:235; packages/dioxus-native-dom/src/mutation_writer.rs:421; packages/dioxus-native/src/assets.rs:48)
- Past escher's scrub: the chained std panic hook prints the raw panic message to stderr, and the allowlisted `log.file` can still carry a host path, now only for a bridged `log` record under an engine target (`js_console`) — the bridged third-party records that carried one at `RUST_LOG=info` (as measured at escher-0.1.0/chunks/2026-10-06-telemetry-bootstrap/report.md) are dropped whole since 2026-10-07-sink-target-allowlist (recorded by construction, not measured: a dropped record writes zero bytes, and `log.file` occurrences were not counted in that chunk's readings). A record from a target outside the sink's allowlist no longer reaches stderr at all (Scrubbing, below) — measured before → after the drop on both sink-installing binaries at four `RUST_LOG` settings, as stderr lines and as the instrument's fixed id and name needles found: the `escher-session` binary (the CRUD task, one `hello`, then `stop`) 0 → 0 lines unset (the default `warn`), 1 → 1 at `info`, 1165 → 1 at `debug` with ids 15 of 15 → 0, 1501 → 1 at `trace` with ids 15 of 15 → 0 and names 6 of 6 → 0; the windowed `seven_guis_native` (left on Home for 10 s, nobody clicks — its first reading by level) 1 → 0 lines unset, 3 → 1 at `info`, 12,413 → 1 at `debug` with ids 11 of 11 → 0, 47,482 → 1 at `trace` with ids 11 of 11 → 0 and names 5 of 5 → 0; the one line left at `info`, `debug` and `trace` is the install line `INFO escher_telemetry`, and stdout is 0 bytes in all 16 readings. Typed text is measured in process only: since 2026-10-07-act-by-id a typing command exists — the driver's `type`, through `Session::run` — and types into an instance held in process; `stand_act_spans`'s re-run child drives the six verbs, `type` included, with the sink installed over a capture at `RUST_LOG=info`, in both layout modes, and the capture holds 0 of 36 id needles, 0 of 8 name needles and 0 occurrences of the three supplied texts (the typed text, the id naming no element, the unknown verb's text), with no value reading redacted on a driver line. Typed text in a HOST's log is still NOT measured: no command reaches the instance of a sink-installing host (`escher-session`). The session host's reading above was re-measured unchanged at 2026-10-07-driver-command-spans — 0 · 1 · 1 · 1 stderr lines, ids 0 of 15, names 0 of 6, stdout 0 B (the readings above as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md; the typing command at escher-0.1.0/chunks/2026-10-07-act-by-id/report.md; the in-process capture and the re-measurement at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md)

**Scrubbing:**

- escher's own sink applies an allowlist of targets in its formatter (packages/escher-telemetry/src/format.rs:22-111) — two public prefix sets, three outcomes: an event whose target — or a bridged `log` record's `log.target` — starts with `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` or `js_console` (`ENGINE_TARGET_PREFIXES`) prints only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`, every other field (the message included) as `{name}=[redacted]`; an event whose target starts with `escher_` (`ESCHER_TARGET_PREFIXES` — the underscore is part of the prefix: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted, `escher` alone is not) prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted; an event from any other target is dropped whole before anything is written — no time, no target, no newline — at every level, WARN and ERROR included, and whatever `RUST_LOG` names (packages/escher-telemetry/src/format.rs:144-148); since 2026-10-07-driver-command-spans a closed span's line is judged pair by pair by the same rule — its name (`span`), its own fields and the layer's `message`, `time.busy` and `time.idle`: under an escher target every pair prints unless its name is in the content-named set, under an engine target only the safe fields print and the rest, `span` and the three close fields among them, read `[redacted]`, and a span from any other target writes no byte at creation, on a recorded value, on enter, on exit or on close; the value of a content-named span field is never stored (the marker is written at print time), and every other span field's value is held in process memory until the span closes; the five scrub sets and the rule are unchanged, and `escher_driver` now has an emitter, the command span (§4) — PROVISIONAL, as §6 → Log format marks the closed span's line (packages/escher-telemetry/src/format.rs:97-111; packages/escher-telemetry/src/format.rs:162-173; packages/escher-telemetry/src/format.rs:200-250; as measured at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md); no engine call site is edited
- Its reach is that sink, installed by the two binaries of the seven_guis package (`seven_guis_native`, `escher-session`): in both the engine `tracing` features stay off in a package-alone build (`-p seven_guis`), so engine events reach it only when one is turned on — which a workspace-wide build does for blitz-dom (§2 → Feature wiring); a record from any target outside the two prefix sets — Stylo's `style::*`, `selectors::matching`, `dioxus_core::*` and `dioxus_signals::*` on both binaries, and on the windowed stand also `naga::*`, `wgpu_core::*`, `wgpu_hal::*`, `winit_wayland::*`, `sctk` and `calloop::*` — is dropped, delivered by the route entry "Sink target allowlist" (the founder's ruling, 2026-10-07; Values logged as-is, above); the cost is that a third-party WARN or ERROR no longer reaches stderr at any setting — the windowed stand's one default-level line, `WARN winit_wayland::window::state`, no longer prints (as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md); the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed, and the values above remain logged as-is there
- The agent-run harness log (§3) sits outside that sink and carries no user content: its events hold only `event`, `ts`, `outcome`, `cargo_exit`, `head`, `selection`, `files`, `file`, `test`, `passed`, `failed`, `ignored`, `booted`, `boot_ts`, `run`, `state` — none in the content-named set — and no captured test output; its `target/agent-run/run.log` is raw cargo/libtest output, unscrubbed by design like `target/ci-logs/`, never printed by `logs` and gitignored under `target/` (as measured at escher-0.1.0/chunks/2026-10-06-stand-test-contract/report.md — a live `logs` read held 0 scrub-set keys)
- The cold-agent pipe log (§3) sits outside that sink and carries no user content: no event or verdict key is in the content-named set, and no transcript text, tool argument, tool result or reply enters an event or the verdict (a marker-string contract test); the stub's call log holds only `seq`, `tool`, `outcome`, `cause`. Its `target/cold-agent/transcript.jsonl` is raw model output and tool I/O, unscrubbed by design — it also holds host paths (init `cwd`, `memory_paths`), the client's local socket path and a rate-limit line — never printed or logged, gitignored under `target/`; the one committed copy (the live run's evidence) masks the two host paths (as measured at escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/report.md — live events and verdict held 0 scrub-set keys)
- The searches below predate it and stand for their slices:

- observed absent — redaction or scrubbing · searched: `redact|scrub|mask` (case-insensitive) over the 86 slice files; hits are a comment and CSS `mask-image` only (apps slice)
- observed absent — scrubbing or redaction · searched: `sanitiz|redact|scrub` over the 15 s05 files
- observed absent — redaction or scrubbing of logged values · searched: `redact|scrub` over the 17 listed s06 files
- observed absent — scrubbing or redaction · searched: `scrub|redact|sanitize` over the 8 slice files (layout slice)
- observed absent — redaction or scrubbing · searched: `redact|scrub|pii` over the 16 listed files (no match)
- observed absent — scrubbing or redaction code · searched: `scrub|redact|pii|PII` over the 32 listed s09 files
- observed absent — redaction or scrubbing · searched: `redact|scrub|sanitiz|mask` over the 32 slice files (blitz-vibey-script / blitz slice)
- observed absent — scrubbing or redaction · searched: `sanitiz|scrub|redact|pii` over the 21 listed s11 files
- observed absent — redaction or scrubbing · searched: `redact|scrub|pii` (case-insensitive) over the 12 listed files (wpt runner slice)

---

## 9. CI Integration

**Platform:** workflows under `.github/workflows/` (.github/workflows/wpt.yml:95-119)

**Telemetry artifact handling (current truth):**

| Artifact | Storage | Source |
|----------|---------|--------|
| WPT report and scores | archived to GitHub Pages and dispatched to `DioxusLabs/blitz-wpt-results` on main, on upstream `DioxusLabs/blitz` only (`wpt` and `trigger-archive` are repository-guarded; neither runs on the fork) | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:95-119) |
| `wptscores.json` | computed from the WPT run and published to Pages, upstream only | (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:72-75) |
| Per-leg CI log | each ci.yml leg's merged stdout+stderr, written by `ci-leg.sh` to `target/ci-logs/{leg}.log` (matrix: `target/ci-logs/matrix-{platform}.log`), truncated at the leg's start; uploaded only on failure as artifact `ci-log-{job id}` (`if-no-files-found: ignore`), kept 7 days, from `target/ci-logs/` alone; unscrubbed build output (no user data — §8) | (.github/scripts/ci-leg.sh:62-63; .github/workflows/ci.yml:48-54; .github/workflows/ci.yml:391-400) |
| Coverage report | the `coverage` leg's lcov file `target/coverage/lcov.info` — line counts of the workspace's public source, no user data (§8); uploaded only on success as artifact `coverage-report` from `target/coverage/`, kept 7 days — the one fork-CI artifact outside `target/ci-logs/` | (.github/scripts/ci-leg.sh:35-39; .github/workflows/ci.yml:286-291) |
| Agent-run harness state | `target/agent-run/{status.json, events.jsonl, run.log}` — the test contract's status, its JSON-line events (§6) and the raw merged cargo/libtest output (unscrubbed, no user data — §8); local only, gitignored under `target/`, uploaded by no CI leg, removed by `agent-run.sh cleanup`, which touches nothing else under `target/` | (scripts/agent-run.sh:9; scripts/agent-run.sh:165-174; scripts/agent-run.sh:215-218) |
| Session state directory | the directory a session's caller names — in the checks `ss-life` and `ss-quiet` (blitz-tests) and `hb-serve`, `hb-refuse` and `hl-trace` (seven_guis) under `target/tmp/` — created `0700` by the host, holding the socket file `session.sock` alone while a session is up and nothing after `stop`; no log, event or screen content is written into it; local only, uploaded by no CI leg; it belongs to the session library and its checks, not to `agent-run.sh` | (packages/escher-driver/src/host.rs:35-73; tests/blitz-tests/tests/session_common/mod.rs:232-240; examples/seven_guis/tests/common/mod.rs:12-20; examples/seven_guis/tests/host_log.rs:73) |
| Cold-agent pipe state | `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, calls.jsonl, stub-state.json}` — the run's verdict, its JSON-line events (§6), the stub's call log and state, and the client's raw transcript and stderr (raw by design — §8); local only, gitignored under `target/`, uploaded by no CI leg (CI runs only the shim contract tests), removed by `cold-agent.sh cleanup`, which touches nothing else under `target/`; the per-run `mktemp -d` session directory is removed when the run exits | (scripts/cold-agent.sh:13; scripts/cold-agent.sh:244-266; scripts/cold-agent.sh:272-275) |

- Publish builds log at `CARGO_LOG: info` with `--verbose --trace`, on upstream `DioxusLabs/blitz` only (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:37; .github/workflows/publish-browser.yml:155)

> NOT YET MEASURED — snapshot artifact upload and CI resource attributes: the reading recorded none

---

## 10. SLO Invariants & Telemetry Budgets

> NO RECORDED INTENT

---

## 11. Obs Anti-Patterns (NEVER do these)

> NO RECORDED INTENT

---

## 12. Obs Decisions Log

> NO RECORDED INTENT
