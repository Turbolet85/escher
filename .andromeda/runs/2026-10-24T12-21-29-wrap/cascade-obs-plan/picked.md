### telemetry-strategy#3.2 · absence · strategy · ## 2. Telemetry Strategy · lines 21–21 · stale
- observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files (examples slice)
### telemetry-strategy#3.3 · absence · strategy · ## 2. Telemetry Strategy · lines 22–22 · stale
- observed absent — telemetry or logging crates · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
### span-trace-coverage#1.1 · absence · spans · ## 4. Span / Trace Coverage · lines 8–8 · stale
- observed absent — spans · searched: `span!|instrument` over the 32 slice files (examples slice)
### span-trace-coverage#1.5 · absence · spans · ## 4. Span / Trace Coverage · lines 12–12 · stale
- observed absent — spans or instrumented functions · searched: `span!|instrument` over the 8 slice files (layout slice)
### span-trace-coverage#1.10 · absence · spans · ## 4. Span / Trace Coverage · lines 17–17 · stale
- observed absent — spans or traces · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
### metric-coverage#1.2 · values · metrics · ## 5. Metric Coverage · lines 11–11 · stale
| blitz-dom `resolve` | Phase times style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs | printed, prefixed `Resolve({id}): ` | (packages/blitz-dom/src/resolve.rs:77-169) |
### metric-coverage#1.4 · values · metrics · ## 5. Metric Coverage · lines 13–13 · stale
| Browser FPS overlay | Frame deltas in a 60-entry ring, polled every 250 ms; average FPS and ms | in-app overlay, toggled from the menu item "Toggle FPS" | (apps/browser/src/fps_overlay.rs:7; apps/browser/src/fps_overlay.rs:27-49; apps/browser/src/fps_overlay.rs:105-123); (apps/browser/src/toolbar.rs:436-439) |
### metric-coverage#1.6 · values · metrics · ## 5. Metric Coverage · lines 15–15 · add-to
| `paint_bench` example | Per-iteration encode and raster microseconds | printed min/median/mean/max | (examples/paint_bench.rs:28-54) |
### metric-coverage#1.7 · values · metrics · ## 5. Metric Coverage · lines 16–16 · add-to
| `screenshot` example | Millisecond timings for fetch, setup, parse, asset fetch, style/layout, render and PNG write | printed | (examples/screenshot.rs:56; examples/screenshot.rs:68; examples/screenshot.rs:86; examples/screenshot.rs:95; examples/screenshot.rs:100; examples/screenshot.rs:134; examples/screenshot.rs:143) |
### metric-coverage#1.10 · values · metrics · ## 5. Metric Coverage · lines 19–19 · stale
| WPT runner, per test | Duration in ms | printed | (wpt/runner/src/main.rs:624; wpt/runner/src/main.rs:694; wpt/runner/src/main.rs:375) |
### metric-coverage#1.11 · values · metrics · ## 5. Metric Coverage · lines 20–20 · stale
| WPT runner, per run | Pass, fail, timeout, skip, crash, subtests, fractional pass, failure buckets and total duration | printed | (wpt/runner/src/main.rs:498-518; wpt/runner/src/main.rs:783-829) |
### metric-coverage#1.12 · values · metrics · ## 5. Metric Coverage · lines 21–21 · stale
| WPT runner, report | Report generation and write times in ms | printed | (wpt/runner/src/main.rs:837-851) |
### log-coverage#3.1 · values · emitter-apps-browser · ## 6. Log Coverage · lines 20–25 · stale
- **apps/browser**
  - Document loading logs info on success and error on failure (apps/browser/src/document_loader.rs:120; apps/browser/src/document_loader.rs:151)
  - Script prefetch failures and JS errors log at error (apps/browser/src/document_loader.rs:235; apps/browser/src/document_loader.rs:241-243)
  - Urlbar parse failures log at warn (apps/browser/src/toolbar.rs:130)
  - Every persistence failure path logs at warn (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:176; apps/browser/persistence/src/lib.rs:202; apps/browser/persistence/src/lib.rs:263; apps/browser/persistence/src/lib.rs:273; apps/browser/persistence/src/lib.rs:298-300)
  - Dropped disk writes log at warn (apps/browser/src/browser_history.rs:161)
### log-coverage#4.1 · values · emitter-examples · ## 6. Log Coverage · lines 31–34 · add-to
- **examples**
  - wasm_hello logs "Starting app..." at info (examples/wasm_hello/src/lib.rs:107)
  - Example output uses `println!`/`eprintln!` (examples/screenshot.rs:203; examples/paint_bench.rs:34; examples/paint_bench.rs:321), except `seven_guis_native` and `escher-session`, which log through escher-telemetry (below); `escher-session`'s own lines, since 2026-10-10-driver-cli, are the binary's one `eprintln!` — `telemetry not installed: …` — and the endings the driver's command line writes for it: usage — one fixed line on stderr built from `VERBS` and the task names, stdout empty, status 2; session error — the error object on stdout, carrying its fixed `remedy` beside `kind` and `message` since 2026-10-12-self-description, and its fixed message on stderr, status 3 (the host's was 1); accepted (status 0) and refused (status 1) — one line of JSON on stdout, stderr empty with `RUST_LOG` unset; and the host role's quiet end, status 0; since 2026-10-12-self-description `help` (at most one `--session <dir>`, not read) — `describe()`'s one line of JSON on stdout, status 0, reaching no session, `help` with an argument refused `malformed`, status 1, `help --session` with no directory or given twice usage, status 2, the usage line listing `help` after `stop` — every text a fixed string of the tables (packages/escher-driver/src/cli.rs:178-183; packages/escher-driver/src/error.rs:79-177); since 2026-10-10-mcp-surface the `mcp` role — `escher-session mcp` writes JSON-RPC protocol lines only on stdout, nothing at its end, status 0 on `Ok` and status 3 with the error's fixed message on stderr on `Err`, `mcp` followed by anything but one `--record <dir>` usage (status 2, stdout empty), as is a `--record` anywhere else, twice or with no value once the call passed `validate`, the usage line gaining `; mcp role: mcp [--record <dir>]` after the host role, with `start <app> [--record <dir>]` and `host role: serve <app> --session <dir> [--record <dir>]`;
 a recording directory open to its group or others ends `start` with status 3 and nothing spawned and `mcp --record` with status 3, its fixed message on stderr and stdout empty, the two recording session errors ending status 3 like the rest; the two-argument form `<task> <state-dir>` is gone (it reads as an unknown verb, refused, status 1), every diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed (examples/seven_guis/src/session_host.rs:21; packages/escher-driver/src/cli.rs:278-293; packages/escher-driver/src/cli.rs:341-368; packages/escher-driver/src/cli.rs:370-383)
### log-coverage#6.1 · values · logs · ## 6. Log Coverage · lines 49–57 · stale
- **blitz-dom (document / resolve / mutator / net)**
  - warn: no DOM on resolve (packages/blitz-dom/src/resolve.rs:45) and on hit test (packages/blitz-dom/src/document.rs:2035)
  - warn: unimplemented form scheme/method (packages/blitz-dom/src/form.rs:153-158)
  - info: image cache hit, pending queue and fetch (packages/blitz-dom/src/mutator.rs:1321-1341); image loaded and node count (packages/blitz-dom/src/document.rs:1551-1555)
  - warn: iframe depth cap and unresolvable iframe URL (packages/blitz-dom/src/mutator.rs:1363-1367; packages/blitz-dom/src/mutator.rs:1390-1391)
  - warn: resource load failed with and without URL (packages/blitz-dom/src/document.rs:1441-1452)
  - info: focussed node (packages/blitz-dom/src/document.rs:1871-1872)
  - info/warn: WOFF decompression and skipped font sources (packages/blitz-dom/src/net.rs:340-365; packages/blitz-dom/src/net.rs:479-498)
  - `debug_log_node` prints layout, attributes, inline layout and children via `println!` and `tracing::info!` (packages/blitz-dom/src/debug.rs:19-155)
### log-coverage#6.3 · values · logs · ## 6. Log Coverage · lines 64–68 · stale
- **blitz-dom (layout)**
  - `tracing::error!` "Tried to lay out text node individually" with fields `node_id`, `data` (packages/blitz-dom/src/layout/mod.rs:247-252)
  - `tracing::warn!` "SVG parse failed" with fields `node_id`, `html`, `error` (packages/blitz-dom/src/layout/construct.rs:480-486)
  - `tracing::info!` for image loading from cache, image already pending, and image fetch start, each including the image URL (packages/blitz-dom/src/layout/damage.rs:583-584; packages/blitz-dom/src/layout/damage.rs:592-593; packages/blitz-dom/src/layout/damage.rs:598-599)
  - `tracing::info!` "Ignoring table descendent because it has no styles" (packages/blitz-dom/src/layout/table.rs:536-537)
### log-coverage#6.4 · values · logs · ## 6. Log Coverage · lines 69–71 · stale
- **blitz-dom (node)**
  - warn "Unsupported property" with field `property`, and warn "Invalid property value" with fields `property` and `value` (packages/blitz-dom/src/node/element.rs:780-795; packages/blitz-dom/src/node/element.rs:846-849)
  - `Node::print_tree` writes the tree to stdout with `println!` (packages/blitz-dom/src/node/node.rs:1007-1021)
### log-coverage#6.11 · values · logs · ## 6. Log Coverage · lines 96–98 · stale
- **wpt/runner**
  - Log calls cover WPT_DIR setup, glob failures, net load errors, pending requests, skips, JS errors and missing harness results (wpt/runner/src/main.rs:470-475; wpt/runner/src/main.rs:295; wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220; wpt/runner/src/test_runners/harness_test.rs:151-153; wpt/runner/src/test_runners/harness_test.rs:179)
  - Log messages are free-form format strings, not structured fields (wpt/runner/src/net_provider.rs:110; wpt/runner/src/test_runners/mod.rs:184)
### log-coverage#7.2 · absence · logs · ## 6. Log Coverage · lines 107–107 · stale
- observed absent — structured logging · searched: `tracing::|log::|env_logger` over the 32 slice files (examples slice); `seven_guis_native` has since gained escher-telemetry's subscriber (§3)
### error-capture-reporting#1.2 · absence · errors · ## 7. Error Capture & Reporting · lines 9–9 · stale
- observed absent — error capture service · searched: `sentry|Sentry` over the 32 slice files (examples slice)
### pii-scrubbing-compliance#5.4 · absence · pii · ## 8. PII Scrubbing & Compliance · lines 39–39 · stale
- observed absent — scrubbing or redaction · searched: `scrub|redact|sanitize` over the 8 slice files (layout slice)
