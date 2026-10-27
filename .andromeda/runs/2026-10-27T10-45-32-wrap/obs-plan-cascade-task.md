# Task — obs-plan · 2026-10-27-upstream-sync-engine-debts

## The session
Upstream sync: eleven upstream DioxusLabs/blitz commits were taken by no-commit cherry-pick from upstream main `ebacd68bd` (it was read at `f9cbb2756` before). The merge base `23b9b26a9` is unmoved and sixteen commits were left out. Three conflict hunks were resolved by hand. No dependency, lock, `deny.toml`, manifest, config key or scrub shape changed.
Engine behaviour changed in every document. `:open` now matches a `details` or `dialog` carrying `open`. Every box-generating `::before`/`::after` votes in flow classification. A text input that is focused before its editor exists enables IME when the editor is built. The windowed shell routes redraw requests through its event loop.
blitz-vibey-script changed in three ways. A `file:` URL now loads only for a `file:` document (`document_may_load`, with a new `FetchError` variant, a narrowing). CSS properties are accessors on `CSSStyleDeclaration`. The runtime and the element bindings were split into the module trees `runtime/` and `dom/element/`. The uncaught-JS-error `tracing::error!` moved whole into `report_js_error` (`runtime/events.rs:21`), still feature-gated.
New test target: blitz-tests `pseudo_element_flow_classification`. Its silence was measured at 0 env · 0 print · 0 subscriber. One new test-only `TMPDIR` read was added in blitz-vibey-script `tests/dom.rs`. No port, socket, listener, env read in source, print or log site was added.
Sink re-check: escher-telemetry and escher-driver are byte-unchanged. `host_log` and `mcp_log` are green under both builds. The fixed-sha census reads its two named hits, and blitz-dom has 0 wall-clock reads outside `clock.rs`. Nothing was retired.

## Reached files
- .github/workflows/publish-browser.yml · merge surface
- apps/browser/src/main.rs · merge surface
- apps/browser/src/tab.rs · merge surface
- wpt/WPT_COMMIT · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-shell/src/lib.rs · merge surface
- packages/blitz-shell/src/window.rs · merge surface
- packages/blitz-vibey-script/src/document.rs · merge surface
- packages/blitz-vibey-script/src/fetch.rs · merge surface
- packages/blitz-vibey-script/src/lib.rs · merge surface
- packages/blitz-vibey-script/src/runtime.rs · merge surface
- packages/blitz-vibey-script/src/dom/element.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/attributes.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/form_controls.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/geometry.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/markup.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/scrolling.rs · merge surface
- packages/blitz-vibey-script/src/dom/element/selectors.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/css.rs · merge surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/fetch.rs · merge surface
- packages/blitz-vibey-script/src/runtime/modules.rs · merge surface
- packages/blitz-vibey-script/src/runtime/timers.rs · merge surface
- packages/blitz-vibey-script/src/runtime/window.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/{css,dom_exception,fetch,fonts,interfaces,node_constants,style,stylesheets,window}.js · merge surface
- packages/blitz-vibey-script/tests/dom.rs · merge surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/all.rs · merge surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · merge surface

## Touches
- The upstream take-read reading at `obs-plan.md:98`: upstream main `f9cbb2756` → `ebacd68bd`, merge base `23b9b26a9` unmoved, eleven taken, sixteen left out.
- The same take-read reading at `obs-plan-detail/pii-scrubbing-compliance.md:13`.
- The §3 silence census (the `absence · logs` block, `obs-plan.md:87`–`:98`) is extended. The new target `pseudo_element_flow_classification` joins it, and P2 places it among the census sites at `:90`–`:97`.
- The uncaught-JS-error site and the `js_console` bridge citation at `obs-plan-detail/log-coverage.md:101` is re-pointed from blitz-vibey-script `runtime.rs` to `runtime/events.rs:21`, inside `report_js_error` (13-32).
- The §1 instrumentation-scope row for blitz-vibey-script cites `packages/blitz-vibey-script/src/runtime.rs:29-38` for JS console output routed to the `log` crate. That code moved into the `runtime/` modules, and `runtime.rs` keeps 151 lines.
- The §7 Script (JS) error-surface rows cite `runtime.rs`, and that code moved into the `runtime/` modules. The rows are the window `error` event, `window.onerror`, handler exceptions and the error-source labels. The cited lines are `:1142-1158`, `:1168-1180`, `:1118-1120`, `:1165`, `:1636`, `:1639`, `:1691` and `:1810`. The same rows in error-capture-reporting are re-pointed by the take's line map. The `state.rs` cites are unmoved.
- The §3 embedder-drain rows cite blitz-vibey-script `document.rs:242-261` and `:256` (`take_js_errors`, `take_messages`, the 256 cap). 11 lines were added above them, at 16, 163-169, 210-211 and 222.
- Citations into blitz-dom `mutator.rs` (`:1379` in §2, `:1338` and `:1357` in §8) shift: 8 lines were added at 368-371 and 520-523.
- Citations into blitz-dom `layout/construct.rs` shift: 5 lines were added above them at 313-317, and further lines at 860 and 867-872. The cited lines are `:444`, `:460-470`, `:461`, `:461-469` and `:462-467`.
- Citations into blitz-dom `node/element.rs` (`:808-809` in §1, `:820-821` in §8) shift: 10 lines were added at 488-489 and 543-550.
- Citations into blitz-dom `stylo.rs` past line 510 shift by 1. Citations into dioxus-native-dom `dioxus_document.rs` past 725 shift by 35 (the added test at 725-759).
- The blitz-shell devtools citation `window.rs:668-686` (§1) shifts: 2 lines were added at 193-194. Any citation into blitz-shell `lib.rs` past line 90 shifts (lines added at 90, 93-98 and 104-106).
- The §9 publish-builds row cites `publish-browser.yml:33`, `:37` and `:157`. 15 lines were added at 126 and 153-166, so `:157` moves, while `:33` and `:37` stand. The upstream-only and repository-guard claims are unchanged.
- The apps/browser `fmt::init` citation `apps/browser/src/main.rs:73-74` is unmoved, because the 2 added lines sit at 192-193. Citations into `apps/browser/src/tab.rs` past 130 shift (63 lines were added at 130-136, 177-182 and 294-343).
- The Logged-events engine-emitter row for blitz-vibey-script still holds one site. The `tracing::error!` moved whole into `report_js_error`, gated `#[cfg(feature = "tracing")]`, under engine target `blitz_vibey_script`. The sink prints its safe fields with the message `[redacted]`.
- The §2 rule that each feature-gated `tracing` call has a no-op path was re-read: the moved JS-error site keeps its gate.
- The absence searches that stand for the blitz-vibey-script / blitz slice (spans, metrics, redaction, error-reporting clients) were re-read and still hold, since the take adds no tracing site. The slice's file set grew by 13 Rust modules and 9 JS bootstrap files, so any file count those searches name is stale.
- The §8 statements past the scrub were re-read and are unmoved. One is that the allowlisted `log.file` carries a host path only for a bridged `js_console` record under an engine target. The other is the sink's 0 id, name and typed-text needle readings. The bridge's file moved, while its target and print rules did not.
- The escher sink allowlist re-check was re-read and green. `host_log` and `mcp_log` pass under both builds, and escher-telemetry is unchanged. The census over the eleven commits' `packages/*.rs` reads its two named hits.
- The clock silence entry's claim was re-read and holds: blitz-dom has no `Instant::now()` or `SystemTime::now()` outside `clock.rs`.
- The service-identity claim was re-read: the windowed `seven_guis_native` smoke at `RUST_LOG=info` logs `service.name=seven_guis`.
- Statements of the env-read sets were re-read: escher-driver's set and the blitz-tests set are unchanged. The test-only `TMPDIR` reads move from 1 to 2 sites, the new one in blitz-vibey-script `tests/dom.rs:863` (test code, not secret).

## New facts
- F1 · log-coverage · Upstream's blitz-tests target `pseudo_element_flow_classification` (3 tests) installs no subscriber, reads no env var and prints nothing (0 env · 0 print · 0 subscriber). Its readings go into assertions only.
- F2 · log-coverage · The upstream take read at `ebacd68bd` is silent. Its changes add no log site, span, field, log field or env read in source: `:open` matching (`ElementData::flush_open_state`), the pseudo vote, IME-on-construct, the `file:` refusal, the CSS accessors and the redraw route. escher-telemetry and escher-driver are unchanged (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md).
- F3 · error-capture-reporting · blitz-vibey-script refuses a `file:` URL for a non-`file:` document through `document_may_load`. The refusal is a typed `FetchError` variant, re-exported from the crate root and checked by `fetch()`, module `import` and `<script src>`. It is an error class with no log site.
- F4 · log-coverage · blitz-vibey-script `tests/dom.rs` `temp_dir_with_files` reads `TMPDIR` through `std::env::temp_dir()`. This is a test-only env read, test code only and not secret, and the second such site beside escher-driver's client unit test.

## Untouched
- escher-driver's `command` and `tool_call` spans, their eight and two fields, and the driver's JSON answer and MCP writers: escher-driver is byte-unchanged.
- escher's sink line format, its closed-span record class, the content-named scrub set and the `EnvFilter` default `warn`: escher-telemetry's code is byte-unchanged.
- The logging-stack install sites of `seven_guis_native` and `escher-session`, the command line's stdout endings and the `mcp` role's protocol lines: the seven_guis package is unchanged.
- The agent-run harness log, its event shapes and its `target/agent-run/` state: `scripts/agent-run.sh` is unchanged.
- The cold-agent pipe log, its verdict fields, the `server.log` span counting and the owner-only `record/` state: `scripts/cold-agent.sh` is unchanged.
- §5 Metric Coverage's in-process timing and counting rows and the metrics-exporter absence: no feature, manifest or dependency moved.
- §2 Telemetry Strategy's feature wiring (the `tracing`, `log-frame-times` and `log-phase-times` forwards): every manifest is byte-identical.
- The OTel SDK absence and the telemetry-backend search: no package entered or left.
- escher-telemetry's chaining panic hook and the WASM `console_error_panic_hook` installs: neither is reached.
- The fork CI's two uploaded artifacts, the local state under `target/`, the recording directory and the package-install diagnostic line: `ci.yml`, `ci-leg.sh` and the install script are unchanged.
- The dated silence-census entries of §3 other than the 2026-10-24 take's: engine debts, clock, session recording, upstream focus, test splits, shared fixtures, visual lint, load wait, select, served stand and parts. Each stands as measured at its own chunk.
- Rows naming `boot_load`, `LoadNet`, `LOAD_ANSWER_DELAY` or `type-select`: the design-system stand-row move touches no obs-plan claim.
- The IME event-plumbing row (`obs-plan-detail/log-coverage.md:74`) and the rows naming the `focus_redraw` target or `requires_redraw`: the IME-on-construct and redraw-route changes add no log site.
