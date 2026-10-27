# obs · cascade read · group 1 of 2

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:8) Every upstream log site the take carries must compile only behind its crate's `tracing` feature, escher-driver must keep exactly its two span macros, and the resolved `Cargo.lock` (a conflict surface) must keep the `tracing` requirement and lock entry this line cites.
  > Telemetry is the `tracing` crate's event macros and, in escher-driver, two span macros (`info_span!`, `command` and, since 2026-10-10-mcp-surface, `tool_call`, §4), in the engine and upstream crates compiled in only with the `tracing` cargo feature (packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/events/ime.rs:27-28)
  (.andromeda/obs-plan-detail/telemetry-strategy.md:8)
  > escher-driver is the second escher crate to take `tracing` ungated (`{ workspace = true }`, no `[features]` table; the workspace requirement is 0.1.40, the lock holds 0.1.44)
  (.andromeda/obs-plan.md:32)
  > escher-telemetry and escher-driver take `tracing` with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, `seven_guis_native` and `escher-session`, whose engine features stay off in a package-alone build.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:14) The capture checks must be re-run under both builds on the synced tree, and the take's edit of blitz-shell's manifest (`527394724`'s `clipboard` feature leaving the defaults) must leave the `tracing` forwarding this wiring cites in place, the host's engine-line count re-read against the recorded one.
  > By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on — blitz's default `tracing` reaches blitz-shell and blitz-html, and each of those forwards to blitz-dom — while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events: one `INFO blitz_dom::document … message=[redacted]` beside a driver `type` (packages/blitz-shell/Cargo.toml:21; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/document.rs:1872;
  (.andromeda/obs-plan-detail/telemetry-strategy.md:14)
  > re-measured at 2026-10-14-stand-requirement-sweep on the same flow, the workspace-built host prints that engine line twice from `info` up — `trace` adding no line — and the host built alone still none (as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md)
  (.andromeda/obs-plan.md:37)
  > `tracing` is the only telemetry dependency in blitz-paint and blitz-shell, where it is optional (packages/blitz-paint/Cargo.toml:15; packages/blitz-paint/Cargo.toml:50; packages/blitz-shell/Cargo.toml:21; packages/blitz-shell/Cargo.toml:42)
  (.andromeda/obs-plan.md:44)
  > `tracing` is a default feature of blitz and forwards to blitz-shell, blitz-html and blitz-net (packages/blitz/Cargo.toml:14; packages/blitz/Cargo.toml:18)
  (.andromeda/obs-plan.md:46)
  > By invocation: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on, while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events (test-plan §9 → Engine features by runner).
  (.andromeda/obs-plan.md:49)
  > dioxus-native's `tracing` feature turns on tracing across dioxus-native-dom and the blitz crates (packages/dioxus-native/Cargo.toml:69)
  (.andromeda/obs-plan.md:90)
  > the `Focussed node` INFO line standing after the new gate (packages/blitz-dom/src/document.rs:1871-1872), and `host_log` and `mcp_log` pass under both builds
- (.andromeda/obs-plan-detail/telemetry-strategy.md:22) The test crate slice's files the take edits (`inline_box_scrollable_overflow.rs`, `display_contents.rs`, `pre_overflow_scroll.rs`) and the targets it adds (`pseudo_element_flow_classification.rs`, `clipboard.rs`) must leave this absence true on the synced tree, read by the delta's census.
  > observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
  (.andromeda/obs-plan.md:56)
  > Telemetry backends observed absent over the CI and build files, telemetry, tracing or logging frameworks over the reading's examples and test crate slices, and tracing or metrics libraries over its wpt runner slice, each search standing for its slice.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:9) Where the take carries `d63ed2c8c`, `apps/browser` is moved whole, and the delta's census must re-read the apps slice for any span or instrumented function it adds.
  > observed absent — spans or instrumented functions · searched: `span!|#\[instrument|info_span|debug_span` over the 86 slice files (apps slice)
  (.andromeda/obs-plan.md:133)
  > Spans, instrumented functions and traces observed absent over the reading's apps, examples, s05, s06, layout, s09, blitz-vibey-script / blitz, s11, test crate and wpt runner slices and the 16 listed files, each search standing for its slice.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:10) The s05 slice holds `document.rs` and `mutator.rs`, a conflict surface, so the resolved files must add no span, instrumented function or subscriber.
  > observed absent — spans or instrumented functions · searched: `info_span|debug_span|#\[instrument|tracing::span` over the 15 s05 files
  (.andromeda/obs-plan.md:105)
  > observed absent — tracing subscriber or exporter setup · searched: `subscriber|opentelemetry|sentry` over the 15 s05 files
- (.andromeda/obs-plan-detail/span-trace-coverage.md:11) The s06 slice holds `scrolling.rs` (a conflict surface), `stylo.rs` and `events/{driver,pointer}.rs`, each reached by the take, so they must add no span, instrumented function or subscriber.
  > observed absent — spans or instrumented functions · searched: `span!|instrument` over the 17 listed s06 files
  (.andromeda/obs-plan.md:106)
  > observed absent — a tracing subscriber or exporter setup · searched: `tracing_subscriber|subscriber` over the 17 listed s06 files
- (.andromeda/obs-plan-detail/span-trace-coverage.md:12) The layout slice holds `construct.rs`, a conflict surface, and the layout files `0980cce5f` reaches, so the resolved layout code must add no span or instrumented function.
  > observed absent — spans or instrumented functions · searched: `span!|instrument` over the 8 slice files (layout slice)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:13) The 16 listed files hold `node/node.rs` (a conflict surface), `node/element.rs` and `node/text.rs`, each reached by the take, so they must add no span or `#[instrument]`.
  > observed absent — spans or `#[instrument]` · searched: `instrument|span` over the 16 listed files (only a doc comment "The node id for the span" matched)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:14) The s09 slice holds blitz-shell's `lib.rs`, `window.rs`, `convert_events.rs` and manifest, reached by `6587e7e79`, `a09f46792` and `527394724`, so blitz-shell must add no span or instrument beside the clipboard-failure event the census reads.
  > observed absent — span or instrument usage · searched: `instrument|span!` over the 32 listed s09 files
- (.andromeda/obs-plan-detail/span-trace-coverage.md:15) The blitz-vibey-script / blitz slice is split into modules by `e6f4615ec` and conflicts in `runtime.rs`, so the split and its resolution must add no span or instrumentation beside the uncaught-JS-error event the census reads.
  > observed absent — tracing spans or instrumentation · searched: `span!|instrument` over the 32 slice files (blitz-vibey-script / blitz slice)
  (.andromeda/obs-plan.md:48)
  > blitz-vibey-script's `tracing` feature is off by default and also enables `blitz-dom/tracing` (packages/blitz-vibey-script/Cargo.toml:15-16)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:16) The s11 slice holds `dioxus_document.rs`, edited by `7fc9ee3af`, and dioxus-native's manifest, moved by `527394724`, so the bridge must add no span, instrumented function or subscriber.
  > observed absent — spans or instrumented functions · searched: `span!|instrument` over the 21 listed s11 files
  (.andromeda/obs-plan.md:107)
  > observed absent — a tracing subscriber or exporter set up in these crates · searched: `subscriber` over the 21 listed s11 files
- (.andromeda/obs-plan-detail/span-trace-coverage.md:24) escher-driver must hold its 2 `info_span!` sites unchanged after the take, and the re-run `host_log` must count one closed `command` span per hosted call.
  > every call handed to `Session::run` opens one `tracing` span — target `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`)
  (.andromeda/obs-plan-detail/span-trace-coverage.md:26)
  > Since 2026-10-10-driver-cli the span is also written by a hosted call: the `escher-session` host role decodes a wire `call` and runs it through `Session::run` and nothing else, so each hosted call is one closed `command` span under that binary's sink — measured, 3 lines for 3 calls at `RUST_LOG=trace` (`host_log`)
  (.andromeda/obs-plan-detail/span-trace-coverage.md:28)
  > escher-driver keeping its 2 `info_span!` sites
  (.andromeda/obs-plan.md:129)
  > `packages/escher-driver` — one span per driver command: every call handed to `Session::run` opens one INFO span `command` at target `escher_driver`, its eight fields (`verb` · `settled` · `busy` · `passes` · `added` · `removed` · `changed` · `cause`) closed words, bools and counts recorded only where they apply; never fielded — the screen text, any argument, or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`; the driver emits no event and installs no subscriber.
- (.andromeda/obs-plan-detail/span-trace-coverage.md:30) `host_log`'s fourth test, re-run under both builds on the synced tree, must again count the host's closed `command` span lines and read 0 id and 0 name needles.
  > `packages/escher-driver` — a hosted part read: `escher-session serve cells` at `RUST_LOG=trace` answers `snapshot --part 2` through `Session::run` and nothing else, so its spans are the `command` spans above, a part read recording `verb` alone; seven_guis `host_log`'s fourth test counts the host's closed `command` span lines and reads 0 id and 0 name needles under both builds (§6 Log coverage, §8 PII); no span, field or log line added (examples/seven_guis/tests/host_log.rs:134-277; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)
- (.andromeda/obs-plan-detail/span-trace-coverage.md:36) The re-run `mcp_log` must read one `tool_call` line per call on the synced tree, the span keeping its two closed-word fields.
  > built by hand with `tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)`, opened and entered once per MCP `tools/call` request only — before its params are read — and held until its reply is written; two fields and no other, each a closed word: `tool`, the verb table's word — one of its twelve, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint` — unrecorded for an unknown tool, and `ending`, `accepted` · `refused` · `error` (a session error or a protocol error);
  (.andromeda/obs-plan-detail/span-trace-coverage.md:37)
  > no field added, the span keeps its two;
  (.andromeda/obs-plan.md:131)
  > `packages/escher-driver` — a second span, `tool_call`: opened once per MCP `tools/call` request, before its params are read, and held until its reply is written; two fields and no other, each a closed word — `tool`, the verb table's word, and `ending`, `accepted` · `refused` · `error`; an instance verb run through `Session::run` makes its `command` span the child, while the session verbs and `help`, answered by the server, leave a `tool_call` with no `command` child.
  (.andromeda/obs-plan.md:80)
  > `mcp_log` reads the role's stderr at `trace` into its assertions only, under both builds — 0 typed-sentinel occurrences, 0 id needles, 0 name needles, one `tool_call` line per call —
- (.andromeda/obs-plan-detail/metric-coverage.md:34) The blitz-vibey-script / blitz slice is moved whole by `e6f4615ec`'s split and `runtime.rs`'s conflict, so the census must find no metrics counter, gauge or histogram added in it.
  > observed absent — metrics counters, gauges or histograms · searched: `metrics|counter!|histogram|gauge` over the 32 slice files (blitz-vibey-script / blitz slice)
  (.andromeda/obs-plan.md:147)
  > Metrics exporters, metric emission and counters, gauges or histograms observed absent over the reading's apps, s05, s06, layout, blitz-vibey-script / blitz and s11 slices and the 16 listed files.
- (.andromeda/obs-plan.md:7) The take moves `Cargo.toml` and `Cargo.lock` (`arboard` out, winit's pin, possibly git dependencies), so the workspace's telemetry dependencies must stand as listed and their cited lines be re-read.
  > **Workspace** — dependencies include tracing, tracing-subscriber, tracing-log, tracing-wasm and console_error_panic_hook; the root dev-dependencies add env_logger (Cargo.toml:174; Cargo.toml:177; Cargo.toml:188-190; Cargo.toml:298)
- (.andromeda/obs-plan.md:11) The sink allowlist is re-read against every target upstream's delta adds, escher-telemetry itself left unchanged by the take.
  > **escher-telemetry** — escher's telemetry bootstrap crate: the stderr subscriber with service identity, the allowlist formatter — a scrub for engine and escher targets, a drop for every other target, printing one line per event and one per closed span — and the chaining panic hook (packages/escher-telemetry/src/lib.rs:1-22; Cargo.toml:17)
- (.andromeda/obs-plan.md:13) `events/pointer.rs` is reached by `0980cce5f`, so its log sites must stay behind the `tracing` feature through the take and their cited lines be re-read.
  > **blitz-dom (events / util)** — six `tracing` log call sites, all behind the `tracing` feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:900-906; packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/util.rs:34-35)
- (.andromeda/obs-plan.md:14) `construct.rs` is a conflict surface and `0980cce5f` reaches the layout code, so its events must stay feature-gated through the resolution and their cited lines be re-read.
  > **blitz-dom (layout)** — emits `tracing` events only when the `tracing` feature is enabled (packages/blitz-dom/src/layout/mod.rs:247; packages/blitz-dom/src/layout/construct.rs:471; packages/blitz-dom/src/layout/damage.rs:583; packages/blitz-dom/src/layout/table.rs:536)
- (.andromeda/obs-plan.md:15) `node/element.rs` is edited by `53e9d81c6`, so its warning must stay behind its feature and its cited line be re-read.
  > **blitz-net, blitz-html, blitz-dom node module** — `tracing` log events behind optional cargo features (packages/blitz-net/Cargo.toml:18; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/node/element.rs:798-799)
- (.andromeda/obs-plan.md:16) `blitz-shell/src/window.rs` is reached by `6587e7e79`, so the devtools lines it cites are re-read on the synced tree.
  > **blitz-shell, blitz-paint** — in-app devtools offer a layout outline, hover highlight overlay, node highlight overlay and taffy tree print (packages/blitz-shell/src/window.rs:667-685; packages/blitz-paint/src/render.rs:245-275; packages/blitz-paint/src/render.rs:1350-1366); the debug overlay visualises content, padding, border and margin boxes of a node (packages/blitz-paint/src/debug_overlay.rs:7-16)
- (.andromeda/obs-plan.md:17) The JS console bridge to `log` lives in `runtime.rs`, a conflict surface split into modules by `e6f4615ec`, so its recorded location must be re-read where the split moves it.
  > **blitz-vibey-script, blitz** — `tracing` is an optional dependency behind a `tracing` feature (packages/blitz-vibey-script/Cargo.toml:16; packages/blitz-vibey-script/Cargo.toml:43; packages/blitz/Cargo.toml:18; packages/blitz/Cargo.toml:36); JS console output is routed to the `log` crate (packages/blitz-vibey-script/src/runtime.rs:1248-1257)
- (.andromeda/obs-plan.md:19) `dioxus_document.rs` is edited by `7fc9ee3af` and dioxus-native's manifest by `527394724`, so the bridge's optional `tracing` must stand and its cited lines be re-read.
  > **dioxus-native, dioxus-native-dom** — optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:60-69; packages/dioxus-native-dom/Cargo.toml:19)
- (.andromeda/obs-plan.md:80) `host_log` is re-run at `trace` under both builds on the synced tree and must again read 0 id and 0 name needles.
  > seven_guis' `host_log` spawns the same binary on `crud` and, in its fourth test, on `cells` at `RUST_LOG=trace` (a paged part read through the host's own sink: a sentinel committed into a part-2 cell, then `snapshot --part 2`, 0 id and 0 name needles under both builds), drains both streams from the spawn on and reads them into its assertions only — it is the check that reads a sink-installing host at `trace`, since 2026-10-10-driver-cli over three hosted calls and under both builds, and a failure of it prints a kind and a count, never what the host wrote;
- (.andromeda/obs-plan.md:98) The take's silence record for its taken commits must be written as the last take's was: no tracing site, wall-clock read in blitz-dom outside `clock.rs`, span, field or env read added, `packages/escher-telemetry/` unchanged, and `host_log` and `mcp_log` passing under both builds.
  > The upstream take read at `f9cbb2756` is silent: upstream's target `hit_test_dropped_nodes` installs no subscriber, reads no env var and prints nothing, its readings going into assertions only; the taken commits add no tracing site over `packages/` (the fixed-sha census reading one commented `dbg!`) and no wall-clock read in blitz-dom outside `clock.rs`, and the process-wide `PARALLEL_TRAVERSAL_LOCK`, `Node::out_of_flow`, `ElementData::classes`, the dropped-node hit skip and a replaced element's `aspect-ratio` height add no log site, span, field, log field or env read; `packages/escher-telemetry/` is unchanged and `host_log` and `mcp_log` pass under both builds (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/sink.md).
- (.andromeda/obs-plan.md:102) Where the take carries `d63ed2c8c`, the browser app's `fmt::init()` stdout subscriber and its cited lines are re-read.
  > The upstream apps install `tracing_subscriber::fmt::init()` under the `tracing` feature, writing to stdout (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:76-77; examples/todomvc/src/main.rs:16-17)
- (.andromeda/obs-plan.md:111) The embedder drain and its 256-error retention must stand after `e6f4615ec`'s split, their recorded locations re-read where the split moves them.
  > Embedders drain JS errors with `take_js_errors` and JS messages with `take_messages` (packages/blitz-vibey-script/src/document.rs:233-252)
  > - At most 256 errors are retained between drains (packages/blitz-vibey-script/src/state.rs:103-105; packages/blitz-vibey-script/src/document.rs:247)

## Bindings
- **telemetry-strategy ↔ span-trace-coverage**: the two span macros escher-driver takes ungated (.andromeda/obs-plan-detail/telemetry-strategy.md:8) are the `command` and `tool_call` spans (.andromeda/obs-plan-detail/span-trace-coverage.md:24, .andromeda/obs-plan-detail/span-trace-coverage.md:36), so one re-read of the 2 `info_span!` sites and one re-run of `host_log` and `mcp_log` witness both.
- **telemetry-strategy (feature wiring) ↔ span-trace-coverage (host_log)**: the both-builds rule (.andromeda/obs-plan-detail/telemetry-strategy.md:14) is what makes `host_log`'s span and needle counts (.andromeda/obs-plan-detail/span-trace-coverage.md:30) a two-build reading; both are re-run on the synced tree.
- **span-trace-coverage ↔ metric-coverage**: the blitz-vibey-script / blitz slice's span absence (.andromeda/obs-plan-detail/span-trace-coverage.md:15) and metrics absence (.andromeda/obs-plan-detail/metric-coverage.md:34) are read by one census over `e6f4615ec`'s split and `runtime.rs`'s resolution.
- **span-trace-coverage absences ↔ the body's instrumentation scope**: each slice absence over a reached file (.andromeda/obs-plan-detail/span-trace-coverage.md:10–16) sits beside that file's feature-gated event sites (.andromeda/obs-plan.md:13, .andromeda/obs-plan.md:14, .andromeda/obs-plan.md:15), so the delta's census reads spans and events together per file.
- **the body's engine log sites ↔ group 2's log and PII categories**: the sites whose locations the take re-reads (.andromeda/obs-plan.md:13, .andromeda/obs-plan.md:14, .andromeda/obs-plan.md:15, .andromeda/obs-plan.md:17) are the ones §8 records as values logged as-is (.andromeda/obs-plan.md:244, .andromeda/obs-plan.md:246, .andromeda/obs-plan.md:247, .andromeda/obs-plan.md:250, .andromeda/obs-plan.md:253), so a moved line moves in both records.
- **telemetry-strategy ↔ the body's silence record**: the census's rule that engine sites compile behind their feature (.andromeda/obs-plan-detail/telemetry-strategy.md:8) is read with the last take's silence record (.andromeda/obs-plan.md:98) that this take's record follows.
