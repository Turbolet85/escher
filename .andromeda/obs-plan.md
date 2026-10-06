## 1. Obs Scope Summary

**Obs tier:** 0

**Instrumentation scope (entities needing instrumentation):**

- **Workspace** — dependencies include tracing, tracing-subscriber, tracing-log, tracing-wasm and console_error_panic_hook; the root dev-dependencies add env_logger (Cargo.toml:172; Cargo.toml:175; Cargo.toml:186-188; Cargo.toml:289)
- **Workspace** — `packages/debug_timer` is a workspace member (Cargo.toml:4; Cargo.toml:56)
- **apps/browser** — observability is `tracing` logging, optional frame/phase timing features, and an in-app FPS overlay (apps/browser/Cargo.toml:27-29; apps/browser/Cargo.toml:36; apps/browser/src/fps_overlay.rs:94-124)
- **examples** — console printing of timings (examples/screenshot.rs:200-214), and `seven_guis_native`'s escher-telemetry events on stderr (examples/seven_guis/src/main.rs:6-9)
- **escher-telemetry** — escher's telemetry bootstrap crate: the stderr subscriber with service identity, the allowlist scrub formatter and the chaining panic hook (packages/escher-telemetry/src/lib.rs:1-15; Cargo.toml:17)
- **blitz-dom** — optional `tracing` logging, `println!` debug dumps and `debug_timer` phase timings (packages/blitz-dom/src/lib.rs:26-29; packages/blitz-dom/src/debug.rs:6-153; packages/blitz-dom/src/resolve.rs:75)
- **blitz-dom (events / util)** — six `tracing` log call sites, all behind the `tracing` feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:752-758; packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/util.rs:34-35)
- **blitz-dom (layout)** — emits `tracing` events only when the `tracing` feature is enabled (packages/blitz-dom/src/layout/mod.rs:131; packages/blitz-dom/src/layout/construct.rs:480; packages/blitz-dom/src/layout/damage.rs:491; packages/blitz-dom/src/layout/table.rs:507)
- **blitz-net, blitz-html, blitz-dom node module** — `tracing` log events behind optional cargo features (packages/blitz-net/Cargo.toml:18; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/node/element.rs:694-695)
- **blitz-shell, blitz-paint** — in-app devtools offer a layout outline, hover highlight overlay, node highlight overlay and taffy tree print (packages/blitz-shell/src/window.rs:668-686; packages/blitz-paint/src/render.rs:245-275; packages/blitz-paint/src/render.rs:1208-1224); the debug overlay visualises content, padding, border and margin boxes of a node (packages/blitz-paint/src/debug_overlay.rs:7-16)
- **blitz-vibey-script, blitz** — `tracing` is an optional dependency behind a `tracing` feature (packages/blitz-vibey-script/Cargo.toml:15; packages/blitz-vibey-script/Cargo.toml:40; packages/blitz/Cargo.toml:17; packages/blitz/Cargo.toml:34); JS console output is routed to the `log` crate (packages/blitz-vibey-script/src/runtime.rs:1248-1257)
- **debug_timer** — provides opt-in duration timing (packages/debug_timer/Cargo.toml:3; packages/debug_timer/Cargo.toml:11-12)
- **dioxus-native, dioxus-native-dom** — optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:58-67; packages/dioxus-native-dom/Cargo.toml:19)
- **wpt/runner** — `log`-facade logging via `env_logger` and printed run statistics (wpt/runner/src/main.rs:458; wpt/runner/src/main.rs:784-830)

> NOT YET MEASURED — the telemetry-surface table, must-trace critical paths and telemetry triggers: the reading recorded none of them for any slice

---

## 2. Telemetry Strategy

**Telemetry mechanism (current truth):**

- Telemetry is the `tracing` crate's event macros, in the engine and upstream crates compiled in only with the `tracing` cargo feature (packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/events/ime.rs:27-28); escher-telemetry depends on `tracing` with no feature and always compiles its startup and panic events into `seven_guis_native`, whose engine `tracing` features stay off (packages/escher-telemetry/Cargo.toml:13-16; packages/escher-telemetry/src/lib.rs:132; packages/escher-telemetry/src/panic.rs:11-21)
- Each feature-gated `tracing` call has a no-op path when the feature is off (packages/blitz-dom/src/layout/mod.rs:131-139; packages/blitz-dom/src/layout/construct.rs:480-488)
- Each blitz-dom log site is compiled only with `#[cfg(feature = "tracing")]` (packages/blitz-dom/src/document.rs:1259; packages/blitz-dom/src/mutator.rs:1188)
- The blitz-dom crate root comment lists a `tracing` feature that "Enables tracing support", under a TODO to document features (packages/blitz-dom/src/lib.rs:26-29)
- `tracing` is the only telemetry dependency in blitz-paint and blitz-shell, where it is optional (packages/blitz-paint/Cargo.toml:15; packages/blitz-paint/Cargo.toml:50; packages/blitz-shell/Cargo.toml:21; packages/blitz-shell/Cargo.toml:42)
- `tracing` is an optional dependency of blitz-net and blitz-html, enabled only by the `tracing` feature (packages/blitz-net/Cargo.toml:18; packages/blitz-net/Cargo.toml:35; packages/blitz-html/Cargo.toml:15; packages/blitz-html/Cargo.toml:26)

**Feature wiring:**

- The browser `tracing` feature enables tracing in dioxus-native, blitz-html, blitz-net, blitz-paint and pulls in tracing-subscriber (apps/browser/Cargo.toml:36)
- rdme's `tracing` feature enables it in blitz-shell, blitz-net, blitz-html (apps/readme/Cargo.toml:42)
- `tracing` is a default feature of blitz and forwards to blitz-shell, blitz-html and blitz-net (packages/blitz/Cargo.toml:14; packages/blitz/Cargo.toml:17)
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

- `seven_guis_native` (the windowed stand binary) installs `escher_telemetry::init(escher_telemetry::service_identity!())` in the native `main` before `dioxus_native::launch`, reporting an `Err` with `eprintln!` and continuing (examples/seven_guis/src/main.rs:4-10): a global `Registry` with an `EnvFilter` from `RUST_LOG` (default `warn`) and one non-ANSI fmt layer using the escher formatter, writing to stderr only, plus the `tracing_log::LogTracer` bridge for `log` records (packages/escher-telemetry/src/lib.rs:99-134)
- the headless stand `seven_guis::stand` (native only; `boot` / `boot_timer` / `options`, driven in-process by the `stand_*` checks) installs no subscriber — no `escher_telemetry::init` and no env read in it or its checks, and no `println!` in it or its checks save `stand_id_persistence`'s re-executed child, which prints its pid and its ids to stdout that the parent captures and reads only into its assertion (never a log) — so a headless boot has no escher sink (examples/seven_guis/src/stand.rs:1-103; tests/blitz-tests/tests/stand_id_persistence.rs:1-3; as measured at escher-0.1.0/chunks/2026-10-06-id-persistence/report.md)
- The upstream apps install `tracing_subscriber::fmt::init()` under the `tracing` feature, writing to stdout (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:61-62; examples/todomvc/src/main.rs:16-17)
- wasm_hello installs `tracing_wasm::set_as_global_default()` (examples/wasm_hello/src/lib.rs:105)
- The wpt runner logs through the `log` facade via `env_logger` (wpt/runner/src/main.rs:458; wpt/runner/src/main.rs:784-830)
- observed absent — tracing subscriber or exporter setup · searched: `subscriber|opentelemetry|sentry` over the 15 s05 files
- observed absent — a tracing subscriber or exporter setup · searched: `tracing_subscriber|subscriber` over the 17 listed s06 files
- observed absent — a tracing subscriber or exporter set up in these crates · searched: `subscriber` over the 21 listed s11 files

**Embedder drain of script diagnostics:**

- Embedders drain JS errors with `take_js_errors` and JS messages with `take_messages` (packages/blitz-vibey-script/src/document.rs:242-261)
- At most 256 errors are retained between drains (packages/blitz-vibey-script/src/state.rs:101-103; packages/blitz-vibey-script/src/document.rs:256)

**Service identity and line format (escher's sink):** every line carries `service.name` / `service.version`, read from the binary's own `CARGO_PKG_NAME` / `CARGO_PKG_VERSION` by `service_identity!()` — the OTel resource keys — in a one-line text format, not JSON (packages/escher-telemetry/src/lib.rs:32-54; §6 Log format); `seven_guis_native` logs `service.name=seven_guis` (as measured at escher-0.1.0/chunks/2026-10-06-telemetry-bootstrap/evidence/smoke-004017Z.txt)

**Agent-run harness log (the test contract's, not escher's sink):** `scripts/agent-run.sh` — the 5-command test contract of test-plan §3 — writes JSON lines, one object per event, encoded by python3's `json` module embedded in the script, never assembled in bash: `boot`, `run.start`, one `test` per libtest result line, `run.end` (printed to stdout and appended to `target/agent-run/events.jsonl`), and `status`, `cleanup` (printed only); `logs` prints `events.jsonl` verbatim. Its state area is `target/agent-run/{status.json, events.jsonl, run.log}` — `run.log` the raw merged cargo/libtest output, never printed. It is harness metadata, not a telemetry sink: no `tracing` subscriber is involved and the stand checks install no escher sink (scripts/agent-run.sh:25-142; test-plan §3; as measured at escher-0.1.0/chunks/2026-10-06-stand-test-contract/report.md)

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

---

## 5. Metric Coverage

**In-process timing and counting (current truth):**

| Operation | What is recorded | Output | Source |
|-----------|------------------|--------|--------|
| Frame / phase timing | Timing logs gated by `log-frame-times`, `log-phase-times`, umbrella `log-times` | log features only | (Cargo.toml:256-259); (packages/dioxus-native/Cargo.toml:58-66) |
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

- One line per event on stderr, no ANSI: `{RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…`; newlines and carriage returns in values escaped as `\n` / `\r`, string values Debug-quoted; a bridged `log` record printed under its `log.target`, that field itself omitted; every field passes the §8 allowlist scrub (packages/escher-telemetry/src/format.rs:96-165)
- Per-module levels come from `RUST_LOG` through `EnvFilter`, defaulting to `warn` when unset or unparsable (packages/escher-telemetry/src/lib.rs:113)

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
  - Example output uses `println!`/`eprintln!` (examples/screenshot.rs:203; examples/paint_bench.rs:34; examples/paint_bench.rs:321), except `seven_guis_native`, which logs through escher-telemetry (below)
- **escher-telemetry (in `seven_guis_native`)**
  - info at target `escher_telemetry`, message `telemetry installed`, once per successful init — no argv, path or URL (packages/escher-telemetry/src/lib.rs:132)
  - ERROR at target `escher_telemetry::panic`, message `panic`, fields `panic.file`, `panic.line`, `panic.column`, `panic.payload` (redacted), once per panic (packages/escher-telemetry/src/panic.rs:11-21)
- **blitz-dom (document / resolve / mutator / net)**
  - warn: no DOM on resolve (packages/blitz-dom/src/resolve.rs:44) and on hit test (packages/blitz-dom/src/document.rs:1800)
  - warn: unimplemented form scheme/method (packages/blitz-dom/src/form.rs:152-157)
  - info: image cache hit, pending queue and fetch (packages/blitz-dom/src/mutator.rs:1146-1166); image loaded and node count (packages/blitz-dom/src/document.rs:1369-1373)
  - warn: iframe depth cap and unresolvable iframe URL (packages/blitz-dom/src/mutator.rs:1188-1192; packages/blitz-dom/src/mutator.rs:1215-1216)
  - warn: resource load failed with and without URL (packages/blitz-dom/src/document.rs:1259-1270)
  - info: focussed node (packages/blitz-dom/src/document.rs:1671-1672)
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
  - `tracing::error!` is logged for unsupported Ime, PointerSource, PointerKind, ButtonSource and MouseScrollDelta variants (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-shell/src/convert_events.rs:81-85; packages/blitz-shell/src/convert_events.rs:97-101; packages/blitz-shell/src/convert_events.rs:115-119; packages/blitz-shell/src/convert_events.rs:143-147; packages/blitz-shell/src/window.rs:834-838)
  - `warn!` is logged for unimplemented image layer kinds and unsupported mask-mode luminance (packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:102-111)
- **blitz, blitz-vibey-script**
  - `launch_url` logs `tracing::info!` with "Launching" and the URL under the `tracing` feature (packages/blitz/src/lib.rs:48-49)
  - Recorded script errors are logged with a "blitz-vibey-script:" prefix under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:264-267)
  - Uncaught JS errors are logged as "Uncaught JS error in" plus the source description (packages/blitz-vibey-script/src/runtime.rs:1104-1105)
  - JS console log, info, warn and error all map to one debug-level log call (packages/blitz-vibey-script/src/runtime.rs:1253-1269)
- **dioxus-native, dioxus-native-dom**
  - Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:119-205; packages/dioxus-native-dom/src/mutation_writer.rs:305; packages/dioxus-native-dom/src/mutation_writer.rs:388)
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

- `console_error_panic_hook` is a workspace dependency (Cargo.toml:175)
- WASM builds install `console_error_panic_hook` (examples/seven_guis/src/lib.rs:13; examples/todomvc/src/wasm.rs:8; examples/wasm_hello/src/lib.rs:104)
- The wpt runner's panic hook captures message, file, line, column and a forced backtrace (wpt/runner/src/panic_backtrace.rs:12-38)
- `seven_guis_native`'s escher-telemetry hook chains: it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged (packages/escher-telemetry/src/panic.rs:4-24)
- A crashed WPT test's panic message is carried into the report's `message` field (wpt/runner/src/report.rs:95)
- `pump_net_provider` logs pending items before panicking on its 500 ms timeout (wpt/runner/src/test_runners/mod.rs:392-398)

**Error classes captured:**

- Browser: JS errors are drained with `take_js_errors` and logged (apps/browser/src/document_loader.rs:241-244); load errors are shown to the user on an error page with the Debug-formatted error (apps/browser/src/document_loader.rs:154-166)
- blitz-dom: resource load errors are logged as `tracing::warn!` with `error` field and not propagated further (packages/blitz-dom/src/document.rs:1256-1275)
- blitz-dom: stylesheets are parsed with no error reporter (`None, // error_reporter`) (packages/blitz-dom/src/net.rs:166; packages/blitz-dom/src/net.rs:276; packages/blitz-dom/src/document.rs:1169)
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
- blitz-dom log lines include resource URLs (packages/blitz-dom/src/document.rs:1260-1265; packages/blitz-dom/src/mutator.rs:1147; packages/blitz-dom/src/mutator.rs:1166)
- `debug_log_node` prints every attribute name and value of a node (packages/blitz-dom/src/debug.rs:28-32)
- Log lines include the raw href, the document URL and element attributes unfiltered (packages/blitz-dom/src/events/pointer.rs:753; packages/blitz-dom/src/events/pointer.rs:758)
- The SVG parse-failure event carries the element's full outer HTML in field `html` (packages/blitz-dom/src/layout/construct.rs:463; packages/blitz-dom/src/layout/construct.rs:481-486)
- Image-fetch info events carry the full image URL (packages/blitz-dom/src/layout/damage.rs:489-507)
- Full request URLs are logged as field `url` (packages/blitz-net/src/lib.rs:229; packages/blitz-net/src/lib.rs:276; packages/blitz-net/src/lib.rs:281)
- The CSS property value is logged in the "Invalid property value" warning (packages/blitz-dom/src/node/element.rs:706-707)
- dioxus-native debug logs record text-node contents and attribute values; asset logs record the full request (packages/dioxus-native-dom/src/mutation_writer.rs:150; packages/dioxus-native-dom/src/mutation_writer.rs:202; packages/dioxus-native-dom/src/mutation_writer.rs:388; packages/dioxus-native/src/assets.rs:48)
- Past escher's scrub: the chained std panic hook prints the raw panic message to stderr, and the allowlisted `log.file` carries a host path for bridged third-party `log` records at `RUST_LOG=info` (as measured at escher-0.1.0/chunks/2026-10-06-telemetry-bootstrap/report.md)

**Scrubbing:**

- escher's own sink scrubs by allowlist in its formatter (packages/escher-telemetry/src/format.rs:17-84): an event whose target — or a bridged `log` record's `log.target` — starts with `blitz`, `dioxus_native`, `stylo_taffy`, `accesskit_xplat`, `debug_timer` or `js_console` prints only `node_id`, `status`, `waiting_nodes`, `property`, `log.module_path`, `log.file`, `log.line`, every other field (the message included) as `{name}=[redacted]`; `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload` are redacted at any target; no engine call site is edited
- Its reach is that sink: in `seven_guis_native` the engine `tracing` features stay off, so engine events reach it only when one is turned on; the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed, and the values above remain logged as-is there
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
