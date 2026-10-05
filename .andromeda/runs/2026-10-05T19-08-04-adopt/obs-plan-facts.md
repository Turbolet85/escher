# obs-plan — gathered facts

## §Obs Scope Summary

### facts-s01.md:243

- Workspace dependencies include tracing, tracing-subscriber, tracing-wasm and console_error_panic_hook; the root dev-dependencies add env_logger (Cargo.toml:169; Cargo.toml:172; Cargo.toml:183-184; Cargo.toml:282)
- `packages/debug_timer` is a workspace member (Cargo.toml:4; Cargo.toml:55)

### facts-s02.md:199

- out of slice — no observability code is in the slice

### facts-s03.md:242

- The slice's only runtime output is example console printing of timings (examples/screenshot.rs:200-214)
- out of slice — observability scope beyond example console output

### facts-s04.md:361

- Observability in this slice is `tracing` logging, optional frame/phase timing features, and an in-app FPS overlay (apps/browser/Cargo.toml:27-29; apps/browser/Cargo.toml:36; apps/browser/src/fps_overlay.rs:94-124)

### facts-s05.md:269

- Observability in the slice is optional `tracing` logging, `println!` debug dumps and `debug_timer` phase timings (packages/blitz-dom/src/lib.rs:26-29; packages/blitz-dom/src/debug.rs:6-153; packages/blitz-dom/src/resolve.rs:75)

### facts-s06.md:260

- Six `tracing` log call sites exist in the slice, all behind the `tracing` feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:752-758; packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/util.rs:34-35)

### facts-s07.md:183

- The slice emits `tracing` events only when the `tracing` feature is enabled (packages/blitz-dom/src/layout/mod.rs:131; packages/blitz-dom/src/layout/construct.rs:480; packages/blitz-dom/src/layout/damage.rs:491; packages/blitz-dom/src/layout/table.rs:507)

### facts-s08.md:236

- Observability in the slice is `tracing` log events behind optional cargo features in blitz-net, blitz-html and the blitz-dom node module (packages/blitz-net/Cargo.toml:18; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/node/element.rs:694-695)

### facts-s09.md:231

- in-app devtools offer a layout outline, hover highlight overlay, node highlight overlay and taffy tree print (packages/blitz-shell/src/window.rs:668-686; packages/blitz-paint/src/render.rs:245-275; packages/blitz-paint/src/render.rs:1203-1219)
- the debug overlay visualises content, padding, border and margin boxes of a node (packages/blitz-paint/src/debug_overlay.rs:7-16)

### facts-s10.md:277

- `tracing` is an optional dependency behind a `tracing` feature in blitz-vibey-script and blitz (packages/blitz-vibey-script/Cargo.toml:15; packages/blitz-vibey-script/Cargo.toml:40; packages/blitz/Cargo.toml:17; packages/blitz/Cargo.toml:34)
- JS console output is routed to the `log` crate (packages/blitz-vibey-script/src/runtime.rs:1245-1254)
- debug_timer provides opt-in duration timing (packages/debug_timer/Cargo.toml:3; packages/debug_timer/Cargo.toml:11-12)

### facts-s11.md:241

- Observability is optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:58-67; packages/dioxus-native-dom/Cargo.toml:19)

### facts-s12.md:250

- out of slice — no observability scope is stated in this test crate

### facts-s13.md:247

- Observability in the slice is `log`-facade logging via `env_logger` and printed run statistics (wpt/runner/src/main.rs:458; wpt/runner/src/main.rs:784-830)

## §Telemetry Strategy

### facts-s01.md:247

- observed absent — telemetry backends · searched: `sentry|opentelemetry|otel|prometheus|metrics` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py

### facts-s02.md:202

- out of slice — no telemetry configuration is in the slice

### facts-s03.md:246

- observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files

### facts-s04.md:364

- The browser `tracing` feature enables tracing in dioxus-native, blitz-html, blitz-net, blitz-paint and pulls in tracing-subscriber (apps/browser/Cargo.toml:36)
- rdme's `tracing` feature enables it in blitz-shell, blitz-net, blitz-html (apps/readme/Cargo.toml:42)
- `log-frame-times` and `log-phase-times` features forward to renderer and DOM crates (apps/browser/Cargo.toml:27-29; apps/readme/Cargo.toml:34-41; examples/todomvc/Cargo.toml:23-25; examples/counter/Cargo.toml:21-22)
- blitz-dom's `log-phase-times` enables `debug_timer/enable` (packages/blitz-dom/Cargo.toml:38)

### facts-s05.md:272

- The crate root comment lists a `tracing` feature that "Enables tracing support", under a TODO to document features (packages/blitz-dom/src/lib.rs:26-29)
- Each log site is compiled only with `#[cfg(feature = "tracing")]` (packages/blitz-dom/src/document.rs:1260; packages/blitz-dom/src/mutator.rs:1188)

### facts-s06.md:263

- Telemetry is the `tracing` crate's event macros, compiled in only with the `tracing` cargo feature (packages/blitz-dom/src/util.rs:26-27; packages/blitz-dom/src/events/ime.rs:27-28)

### facts-s07.md:186

- Telemetry is feature-gated `tracing` events; each call has a no-op path when the feature is off (packages/blitz-dom/src/layout/mod.rs:131-139; packages/blitz-dom/src/layout/construct.rs:480-488)

### facts-s08.md:239

- `tracing` is an optional dependency enabled only by the `tracing` feature (packages/blitz-net/Cargo.toml:18; packages/blitz-net/Cargo.toml:35; packages/blitz-html/Cargo.toml:15; packages/blitz-html/Cargo.toml:26)

### facts-s09.md:235

- tracing is the only telemetry dependency, optional in blitz-paint and blitz-shell (packages/blitz-paint/Cargo.toml:15; packages/blitz-paint/Cargo.toml:50; packages/blitz-shell/Cargo.toml:21; packages/blitz-shell/Cargo.toml:42)

### facts-s10.md:282

- `tracing` is a default feature of blitz and forwards to blitz-shell, blitz-html and blitz-net (packages/blitz/Cargo.toml:14; packages/blitz/Cargo.toml:17)
- blitz-vibey-script's `tracing` feature is off by default and also enables `blitz-dom/tracing` (packages/blitz-vibey-script/Cargo.toml:14-15)

### facts-s11.md:244

- The `tracing` feature turns on tracing across dioxus-native-dom and the blitz crates (packages/dioxus-native/Cargo.toml:67)
- `log-frame-times` turns on `log_frame_times` in whichever anyrender backend is enabled; `log-phase-times` forwards to blitz-dom (packages/dioxus-native/Cargo.toml:59-66)

### facts-s12.md:253

- observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files

### facts-s13.md:250

- observed absent — tracing or metrics libraries · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files

## §Observability Harness Contract

### facts-s01.md:250

- out of slice — no observability harness code in this slice

### facts-s02.md:205

- out of slice — no observability harness is in the slice

### facts-s03.md:249

- out of slice — observability harness

### facts-s04.md:370

- Native subscribers are installed with `tracing_subscriber::fmt::init()` under the `tracing` feature (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:61-62; examples/todomvc/src/main.rs:16-17)
- wasm_hello installs `tracing_wasm::set_as_global_default()` (examples/wasm_hello/src/lib.rs:105)

### facts-s05.md:276

- observed absent — tracing subscriber or exporter setup · searched: `subscriber|opentelemetry|sentry` over the 15 s05 files

### facts-s06.md:266

- observed absent — a tracing subscriber or exporter setup · searched: `tracing_subscriber|subscriber` over the 17 listed s06 files

### facts-s07.md:189

- out of slice — no subscriber or exporter setup is in this slice

### facts-s08.md:242

- out of slice — subscriber or exporter setup is not in the listed files

### facts-s09.md:238

- out of slice — these files define no observability test harness

### facts-s10.md:286

- embedders drain JS errors with `take_js_errors` and JS messages with `take_messages` (packages/blitz-vibey-script/src/document.rs:242-261)
- at most 256 errors are retained between drains (packages/blitz-vibey-script/src/state.rs:101-103; packages/blitz-vibey-script/src/document.rs:256)

### facts-s11.md:248

- observed absent — a tracing subscriber or exporter set up in these crates · searched: `subscriber` over the 21 listed s11 files

### facts-s12.md:256

- out of slice — no observability harness in this test crate

### facts-s13.md:253

- out of slice — no observability harness is defined in these files

## §Bootstrap phases

(no fact block)

## §Span / Trace Coverage

### facts-s01.md:253

- out of slice — span instrumentation lives in crates not in this slice

### facts-s02.md:208

- out of slice — no tracing code is in the slice

### facts-s03.md:252

- observed absent — spans · searched: `span!|instrument` over the 32 slice files

### facts-s04.md:374

- observed absent — spans or instrumented functions · searched: `span!|#\[instrument|info_span|debug_span` over the 86 slice files

### facts-s05.md:279

- observed absent — spans or instrumented functions · searched: `info_span|debug_span|#\[instrument|tracing::span` over the 15 s05 files

### facts-s06.md:269

- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 17 listed s06 files

### facts-s07.md:192

- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 8 slice files

### facts-s08.md:245

- observed absent — spans or `#[instrument]` · searched: `instrument|span` over the 16 listed files (only a doc comment "The node id for the span" matched)

### facts-s09.md:241

- observed absent — span or instrument usage · searched: `instrument|span!` over the 32 listed s09 files

### facts-s10.md:290

- observed absent — tracing spans or instrumentation · searched: `span!|instrument` over the 32 slice files

### facts-s11.md:251

- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 21 listed s11 files

### facts-s12.md:259

- observed absent — spans or traces · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files

### facts-s13.md:256

- observed absent — spans or traces · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files

## §Metric Coverage

### facts-s01.md:256

- Frame and phase timing logs are feature-gated (`log-frame-times`, `log-phase-times`, umbrella `log-times`) (Cargo.toml:249-252)
- WPT scores are computed into `wptscores.json` and published to Pages (.github/workflows/wpt.yml:71-74)

### facts-s02.md:211

- out of slice — no metric code is in the slice

### facts-s03.md:255

- `paint_bench` records per-iteration encode and raster microseconds and prints min/median/mean/max (examples/paint_bench.rs:28-54)
- `screenshot` records millisecond timings for fetch, setup, parse, asset fetch, style/layout, render and PNG write (examples/screenshot.rs:56; examples/screenshot.rs:68; examples/screenshot.rs:86; examples/screenshot.rs:95; examples/screenshot.rs:100; examples/screenshot.rs:134; examples/screenshot.rs:143)

### facts-s04.md:377

- The FPS overlay records frame deltas in a 60-entry ring, polls every 250 ms, and shows average FPS and ms (apps/browser/src/fps_overlay.rs:7; apps/browser/src/fps_overlay.rs:27-49; apps/browser/src/fps_overlay.rs:105-123)
- The overlay is toggled from the menu item "Toggle FPS" (apps/browser/src/toolbar.rs:437-440)
- observed absent — metrics exporters · searched: `opentelemetry|metrics|prometheus|sentry` over the 86 slice files

### facts-s05.md:282

- `resolve` records phase times named style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs and prints them prefixed `Resolve({id}): ` (packages/blitz-dom/src/resolve.rs:75-167)
- observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files

### facts-s06.md:272

- observed absent — metric emission · searched: `counter!|histogram!|gauge!` over the 17 listed s06 files

### facts-s07.md:195

- observed absent — metrics · searched: `metric|counter!|histogram` over the 8 slice files (matches are only text-layout `metrics()` calls and comments)

### facts-s08.md:248

- observed absent — metrics · searched: `metric|counter|histogram|gauge` over the 16 listed files (no match)

### facts-s09.md:244

- LayerManager keeps in-process counters layers_used, layer_depth, layers_wanted and a debugging-only layer_depth_used (packages/blitz-paint/src/layers.rs:8-16; packages/blitz-paint/src/layers.rs:58; packages/blitz-paint/src/layers.rs:81-84)

### facts-s10.md:293

- debug_timer records labelled instants and prints the total and per-step durations in ns/us/ms/s (packages/debug_timer/src/lib.rs:14-24; packages/debug_timer/src/lib.rs:33-66)
- observed absent — metrics counters, gauges or histograms · searched: `metrics|counter!|histogram|gauge` over the 32 slice files

### facts-s11.md:254

- observed absent — metrics or telemetry backends · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files
- Frame and phase timing are exposed only as log features (packages/dioxus-native/Cargo.toml:58-66)

### facts-s12.md:262

- A timing harness measures medians of `hit()`, render on a null backend, an incremental hover-only frame and a full non-incremental frame, printed as a markdown table (tests/blitz-tests/tests/paint_tree_bench.rs:1-3; tests/blitz-tests/tests/paint_tree_bench.rs:100-134; tests/blitz-tests/tests/paint_tree_bench.rs:341-370)
- The external-page timing also reports node count and hoisted stacking-context entries (tests/blitz-tests/tests/paint_tree_bench.rs:271-278)

### facts-s13.md:259

- Per-test duration in ms is recorded and printed (wpt/runner/src/main.rs:614; wpt/runner/src/main.rs:695; wpt/runner/src/main.rs:369)
- Run-level counts (pass, fail, timeout, skip, crash, subtests, fractional pass, failure buckets) and total duration are printed (wpt/runner/src/main.rs:488-508; wpt/runner/src/main.rs:784-830)
- Report generation and write times are printed in ms (wpt/runner/src/main.rs:838-852)

## §Log Coverage

### facts-s01.md:260

- Publish builds log at `CARGO_LOG: info` with `--verbose --trace` (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:154)
- out of slice — runtime log statements

### facts-s02.md:214

- observed absent — console logging calls · searched: `console\.` over the 21 s02 files

### facts-s03.md:259

- Example output uses `println!`/`eprintln!` only (examples/screenshot.rs:203; examples/paint_bench.rs:34; examples/paint_bench.rs:321)
- observed absent — structured logging · searched: `tracing::|log::|env_logger` over the 32 slice files

### facts-s04.md:382

- Document loading logs info on success and error on failure (apps/browser/src/document_loader.rs:121; apps/browser/src/document_loader.rs:152)
- Script prefetch failures and JS errors log at error (apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244)
- Urlbar parse failures log at warn (apps/browser/src/toolbar.rs:130)
- Every persistence failure path logs at warn (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:176; apps/browser/persistence/src/lib.rs:202; apps/browser/persistence/src/lib.rs:263; apps/browser/persistence/src/lib.rs:273; apps/browser/persistence/src/lib.rs:298-300)
- Dropped disk writes log at warn (apps/browser/src/browser_history.rs:161)
- wasm_hello logs "Starting app..." at info (examples/wasm_hello/src/lib.rs:107)

### facts-s05.md:286

- warn: no DOM on resolve (packages/blitz-dom/src/resolve.rs:44) and on hit test (packages/blitz-dom/src/document.rs:1801)
- warn: unimplemented form scheme/method (packages/blitz-dom/src/form.rs:152-157)
- info: image cache hit, pending queue and fetch (packages/blitz-dom/src/mutator.rs:1146-1166); image loaded and node count (packages/blitz-dom/src/document.rs:1370-1374)
- warn: iframe depth cap and unresolvable iframe URL (packages/blitz-dom/src/mutator.rs:1188-1192; packages/blitz-dom/src/mutator.rs:1215-1216)
- warn: resource load failed with and without URL (packages/blitz-dom/src/document.rs:1260-1271)
- info: focussed node (packages/blitz-dom/src/document.rs:1672-1673)
- info/warn: WOFF decompression and skipped font sources (packages/blitz-dom/src/net.rs:340-365; packages/blitz-dom/src/net.rs:476-495)
- `debug_log_node` prints layout, attributes, inline layout and children via `println!` and `tracing::info!` (packages/blitz-dom/src/debug.rs:17-153)

### facts-s06.md:275

- `debug` logs a sent IME event with a `node_id` field (packages/blitz-dom/src/events/ime.rs:28)
- `warn` logs an unparseable link href together with the document URL (packages/blitz-dom/src/events/pointer.rs:753)
- `info` logs a click on a link without href together with the element's attributes (packages/blitz-dom/src/events/pointer.rs:758)
- `warn` logs WOFF1 and WOFF2 decompression failures (packages/blitz-dom/src/util.rs:27; packages/blitz-dom/src/util.rs:35)
- `warn` logs a call to `stylo_to_cursor_icon` with `CursorKind::Auto` (packages/blitz-dom/src/stylo_to_cursor_icon.rs:10)

### facts-s07.md:198

- `tracing::error!` "Tried to lay out text node individually" with fields `node_id`, `data` (packages/blitz-dom/src/layout/mod.rs:131-136)
- `tracing::warn!` "SVG parse failed" with fields `node_id`, `html`, `error` (packages/blitz-dom/src/layout/construct.rs:480-486)
- `tracing::info!` for image loading from cache, image already pending, and image fetch start, each including the image URL (packages/blitz-dom/src/layout/damage.rs:491-492; packages/blitz-dom/src/layout/damage.rs:500-501; packages/blitz-dom/src/layout/damage.rs:506-507)
- `tracing::info!` "Ignoring table descendent because it has no styles" (packages/blitz-dom/src/layout/table.rs:507-508)

### facts-s08.md:251

- info "Using cache dir" with field `path` (packages/blitz-net/src/lib.rs:57-58)
- error "Failed to clear HTTP cache: {:?}" (packages/blitz-net/src/lib.rs:140-141)
- warn "HTTP error status" with fields `url` and `status` (packages/blitz-net/src/lib.rs:210-215)
- error "Fetching" with fields `url` and `error`, and info "Success fetching" with field `url`, in `fetch_with_callback` and `fetch_async` (packages/blitz-net/src/lib.rs:236-243; packages/blitz-net/src/lib.rs:257-264)
- info "Fetching" with field `url`; info "Success fetching" with field `url`; and error "Error fetching" with fields `url` and `error` in `NetProvider::fetch` (packages/blitz-net/src/lib.rs:275-276; packages/blitz-net/src/lib.rs:301-306)
- Each collected HTML parse error is logged at error level on sink finish (packages/blitz-html/src/html_sink.rs:181-186)
- warn "Unsupported property" with field `property`, and warn "Invalid property value" with fields `property` and `value` (packages/blitz-dom/src/node/element.rs:693-708; packages/blitz-dom/src/node/element.rs:759-762)
- `Node::print_tree` writes the tree to stdout with `println!` (packages/blitz-dom/src/node/node.rs:959-973)

### facts-s09.md:247

- tracing::error! is logged for unsupported Ime, PointerSource, PointerKind, ButtonSource and MouseScrollDelta variants (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-shell/src/convert_events.rs:81-85; packages/blitz-shell/src/convert_events.rs:97-101; packages/blitz-shell/src/convert_events.rs:115-119; packages/blitz-shell/src/convert_events.rs:143-147; packages/blitz-shell/src/window.rs:834-838)
- warn! is logged for unimplemented image layer kinds and unsupported mask-mode luminance (packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:102-111)

### facts-s10.md:297

- `launch_url` logs `tracing::info!` with "Launching" and the URL under the `tracing` feature (packages/blitz/src/lib.rs:48-49)
- recorded script errors are logged with a "blitz-vibey-script:" prefix under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:264-267)
- uncaught JS errors are logged as "Uncaught JS error in" plus the source description (packages/blitz-vibey-script/src/runtime.rs:1101-1102)
- console log, info, warn and error all map to one debug-level log call (packages/blitz-vibey-script/src/runtime.rs:1250-1266)

### facts-s11.md:258

- Every DOM mutation (assign_node_id, create_placeholder, create_text_node, append/insert/replace, remove_node, push_root, set_node_text, load_template, set_attribute) is logged at debug through `trace!` (packages/dioxus-native-dom/src/mutation_writer.rs:119-205; packages/dioxus-native-dom/src/mutation_writer.rs:305; packages/dioxus-native-dom/src/mutation_writer.rs:388)
- Asset fetch success is logged at trace and failure at warn; fetches without a net provider are logged at warn (packages/dioxus-native/src/assets.rs:47-60)
- A failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:14-15)
- Injecting the document provider into windows is logged at debug (packages/dioxus-native/src/dioxus_application.rs:137-138)

### facts-s12.md:266

- A `blitz-dom/log-phase-times` feature prints per-phase resolve timings (tests/blitz-tests/tests/paint_tree_bench.rs:223-224; tests/blitz-tests/tests/paint_tree_bench.rs:261)
- Test diagnostics go to stdout/stderr via `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/paint_tree_bench.rs:266)

### facts-s13.md:264

- Log calls cover WPT_DIR setup, glob failures, net load errors, pending requests, skips, JS errors and missing harness results (wpt/runner/src/main.rs:464-469; wpt/runner/src/main.rs:290; wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220; wpt/runner/src/test_runners/harness_test.rs:151-153; wpt/runner/src/test_runners/harness_test.rs:179)
- Log messages are free-form format strings, not structured fields (wpt/runner/src/net_provider.rs:110; wpt/runner/src/test_runners/mod.rs:182)

## §Error Capture & Reporting

### facts-s01.md:264

- `console_error_panic_hook` is a workspace dependency (Cargo.toml:172)
- out of slice — panic/error capture code

### facts-s02.md:217

- observed absent — an error-reporting client · searched: `sentry|analytics|gtag` over the 21 s02 files

### facts-s03.md:263

- observed absent — error capture service · searched: `sentry|Sentry` over the 32 slice files

### facts-s04.md:390

- WASM builds install `console_error_panic_hook` (examples/seven_guis/src/lib.rs:13; examples/todomvc/src/wasm.rs:8; examples/wasm_hello/src/lib.rs:104)
- JS errors are drained with `take_js_errors` and logged (apps/browser/src/document_loader.rs:241-244)
- Load errors are shown to the user on an error page with the Debug-formatted error (apps/browser/src/document_loader.rs:154-166)

### facts-s05.md:296

- Resource load errors are logged as `tracing::warn!` with `error` field and not propagated further (packages/blitz-dom/src/document.rs:1257-1276)
- Stylesheets are parsed with no error reporter (`None, // error_reporter`) (packages/blitz-dom/src/net.rs:166; packages/blitz-dom/src/net.rs:276; packages/blitz-dom/src/document.rs:1170)
- observed absent — error reporting services · searched: `sentry|opentelemetry` over the 15 s05 files

### facts-s06.md:282

- Recoverable failures are logged and a fallback is used, as with font decompression (packages/blitz-dom/src/util.rs:25-29)
- observed absent — an error reporting service or panic hook · searched: `sentry|panic::set_hook|catch_unwind` over the 17 listed s06 files

### facts-s07.md:204

- SVG parse errors are captured into the `error` field of a warn event and not propagated (packages/blitz-dom/src/layout/construct.rs:479-489)

### facts-s08.md:261

- Fetch errors in `NetProvider::fetch` are logged and not propagated to the handler (packages/blitz-net/src/lib.rs:298-310)
- `ProviderError` implements `Display` with a message per variant (packages/blitz-net/src/lib.rs:369-382)
- observed absent — an error reporting service · searched: `sentry|opentelemetry|otel|span!` over the 16 listed files (no match)

### facts-s09.md:251

- DataUriNetProvider's error callbacks are commented out, so parse, decode and unsupported-scheme failures are not reported (packages/blitz-shell/src/net.rs:54-67)

### facts-s10.md:303

- the window `error` event carries message, filename "", lineno 0, colno 0 and error (packages/blitz-vibey-script/src/runtime.rs:1139-1155)
- `window.onerror` is called with message, source, lineno, colno, error (packages/blitz-vibey-script/src/runtime.rs:1165-1177)
- exceptions thrown by error handlers are recorded but fire no further error events (packages/blitz-vibey-script/src/runtime.rs:1115-1117; packages/blitz-vibey-script/src/state.rs:91-94)
- error sources labelled in reports include "timer callback", "event listener", "error event listener", "timer microtasks", "event microtasks" (packages/blitz-vibey-script/src/runtime.rs:1627; packages/blitz-vibey-script/src/runtime.rs:1630; packages/blitz-vibey-script/src/runtime.rs:1682; packages/blitz-vibey-script/src/runtime.rs:1792; packages/blitz-vibey-script/src/runtime.rs:1162)

### facts-s11.md:264

- Errors are reported only as `tracing` warn/error events (packages/dioxus-native/src/assets.rs:52-53; packages/dioxus-native/src/link_handler.rs:14-15)
- observed absent — an error-reporting service · searched: `metric|counter!|histogram|opentelemetry|sentry|prometheus` over the 21 listed s11 files

### facts-s12.md:270

- out of slice — no error capture or reporting in this test crate

### facts-s13.md:268

- The panic hook captures message, file, line, column and a forced backtrace (wpt/runner/src/panic_backtrace.rs:12-38)
- A crashed test's panic message is carried into the report's `message` field (wpt/runner/src/report.rs:95)
- Subtest error strings are joined with newlines into the report's subtest `message` (wpt/runner/src/report.rs:104-108)
- `pump_net_provider` logs pending items before panicking on its 500 ms timeout (wpt/runner/src/test_runners/mod.rs:392-398)

## §PII Scrubbing & Compliance

### facts-s01.md:268

- out of slice — no PII handling in this slice

### facts-s02.md:220

- out of slice — no PII handling code is in the slice

### facts-s03.md:266

- out of slice — PII handling

### facts-s04.md:395

- Visited URLs are logged at info and urlbar text at warn (apps/browser/src/document_loader.rs:121; apps/browser/src/toolbar.rs:130)
- observed absent — redaction or scrubbing · searched: `redact|scrub|mask` (case-insensitive) over the 86 slice files; hits are a comment and CSS `mask-image` only

### facts-s05.md:301

- Log lines include resource URLs (packages/blitz-dom/src/document.rs:1261-1266; packages/blitz-dom/src/mutator.rs:1147; packages/blitz-dom/src/mutator.rs:1166)
- `debug_log_node` prints every attribute name and value of a node (packages/blitz-dom/src/debug.rs:28-32)
- observed absent — scrubbing or redaction · searched: `sanitiz|redact|scrub` over the 15 s05 files

### facts-s06.md:286

- Log lines include the raw href, the document URL and element attributes unfiltered (packages/blitz-dom/src/events/pointer.rs:753; packages/blitz-dom/src/events/pointer.rs:758)
- observed absent — redaction or scrubbing of logged values · searched: `redact|scrub` over the 17 listed s06 files

### facts-s07.md:207

- The SVG parse-failure event carries the element's full outer HTML in field `html` (packages/blitz-dom/src/layout/construct.rs:463; packages/blitz-dom/src/layout/construct.rs:481-486)
- Image-fetch info events carry the full image URL (packages/blitz-dom/src/layout/damage.rs:489-507)
- observed absent — scrubbing or redaction · searched: `scrub|redact|sanitize` over the 8 slice files

### facts-s08.md:266

- Full request URLs are logged as field `url` (packages/blitz-net/src/lib.rs:229; packages/blitz-net/src/lib.rs:276; packages/blitz-net/src/lib.rs:281)
- The CSS property value is logged in the "Invalid property value" warning (packages/blitz-dom/src/node/element.rs:706-707)
- observed absent — redaction or scrubbing · searched: `redact|scrub|pii` over the 16 listed files (no match)

### facts-s09.md:254

- observed absent — scrubbing or redaction code · searched: `scrub|redact|pii|PII` over the 32 listed s09 files

### facts-s10.md:309

- observed absent — redaction or scrubbing · searched: `redact|scrub|sanitiz|mask` over the 32 slice files

### facts-s11.md:268

- observed absent — scrubbing or redaction · searched: `sanitiz|scrub|redact|pii` over the 21 listed s11 files
- Debug logs record text-node contents and attribute values; asset logs record the full request (packages/dioxus-native-dom/src/mutation_writer.rs:150; packages/dioxus-native-dom/src/mutation_writer.rs:202; packages/dioxus-native-dom/src/mutation_writer.rs:388; packages/dioxus-native/src/assets.rs:48)

### facts-s12.md:273

- out of slice — no PII handling in this test crate

### facts-s13.md:274

- observed absent — redaction or scrubbing · searched: `redact|scrub|pii` (case-insensitive) over the 12 listed files

## §CI Integration

### facts-s01.md:271

- The WPT report and scores are archived to GitHub Pages and dispatched to `DioxusLabs/blitz-wpt-results` on main (.github/workflows/wpt.yml:94-118)

### facts-s02.md:223

- out of slice — no CI configuration is in the slice

### facts-s03.md:269

- out of slice — CI configuration

### facts-s04.md:399

- out of slice — CI workflow files

### facts-s05.md:306

- out of slice — CI configuration

### facts-s06.md:290

- out of slice — no CI configuration is in the s06 files

### facts-s07.md:212

- out of slice — no CI configuration is in this slice

### facts-s08.md:271

- out of slice — no CI configuration is in the listed files

### facts-s09.md:257

- out of slice — no CI configuration is among these files

### facts-s10.md:312

- out of slice — no CI configuration is among these files

### facts-s11.md:272

- out of slice — no CI configuration in these files

### facts-s12.md:276

- out of slice — CI workflow files

### facts-s13.md:277

- out of slice — no CI configuration in these files

## §SLO Invariants & Telemetry Budgets

(no fact block)

## §Obs Anti-Patterns

(no fact block)

## §Obs Decisions Log

(no fact block)
