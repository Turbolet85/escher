# Apply — obs-plan

## Verdicts
- fanout-obs-plan.md 1 · applied · .andromeda/obs-plan.md:98#1
- fanout-obs-plan.md 2 · applied · .andromeda/obs-plan.md:229+#1
- fanout-obs-plan.md 3 · applied · .andromeda/obs-plan.md:226#1, .andromeda/obs-plan.md:227#1, .andromeda/obs-plan.md:228#1, .andromeda/obs-plan.md:229#1
- fanout-obs-plan--cascade-new-log-coverage.md 3 · applied · .andromeda/obs-plan-detail/log-coverage.md:125+#1
- fanout-obs-plan.md 4 · applied · .andromeda/obs-plan-detail/log-coverage.md:101#1
- fanout-obs-plan--cascade.md 1 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13#1
- fanout-obs-plan--cascade.md 6 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:44#1
- fanout-obs-plan--cascade.md 2 · applied · .andromeda/obs-plan-detail/telemetry-strategy.md:22#1
- fanout-obs-plan--cascade.md 3 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:17#1
- fanout-obs-plan--cascade.md 4 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:15#1
- fanout-obs-plan--cascade.md 5 · applied · .andromeda/obs-plan-detail/metric-coverage.md:34#1

## Edits
- line: .andromeda/obs-plan.md:98
  old: «The upstream take read at `f9cbb2756` is silent: upstream's target `hit_test_dropped_nodes` installs no subscriber, reads no env var and prints nothing, its readings going into assertions only; the taken commits add no tracing site over `packages/` (the fixed-sha census reading one commented `dbg!`) and no wall-clock read in blitz-dom outside `clock.rs`, and the process-wide `PARALLEL_TRAVERSAL_LOCK`, `Node::out_of_flow`, `ElementData::classes`, the dropped-node hit skip and a replaced element's `aspect-ratio` height add no log site, span, field, log field or env read; `packages/escher-telemetry/` is unchanged and `host_log` and `mcp_log` pass under both builds (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/sink.md).»
  new: «The upstream take read at `ebacd68bd` is silent: upstream's target `pseudo_element_flow_classification` installs no subscriber, reads no env var and prints nothing (0 env · 0 print · 0 subscriber), its readings going into assertions only; the taken commits add no log site, print, listener or env read in source — the fixed-sha census over their `packages/*.rs` lines reading its two named hits, the uncaught-JS-error `tracing::error!` standing whole in `report_js_error`, still under `#[cfg(feature = "tracing")]` at the engine target `blitz_vibey_script` (packages/blitz-vibey-script/src/runtime/events.rs:20-21), and a test-only `TMPDIR` read (§6) — and no wall-clock read in blitz-dom outside `clock.rs`; `ElementData::flush_open_state` (`:open`), `document_may_load`, the IME enable at a focused input's editor construction, the pseudo vote in flow classification and the shell's redraw route add no log site, span, field, log field or env read; `packages/escher-telemetry/` and `packages/escher-driver/` are unchanged and `host_log` and `mcp_log` pass under both builds (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md).»
- line: .andromeda/obs-plan.md:226
  old: «runtime.rs:1142-1158»
  new: «runtime/events.rs:58-74»
- line: .andromeda/obs-plan.md:227
  old: «runtime.rs:1168-1180»
  new: «runtime/events.rs:84-96»
- line: .andromeda/obs-plan.md:228
  old: «runtime.rs:1118-1120»
  new: «runtime/events.rs:34-36»
- line: .andromeda/obs-plan.md:229
  old: «runtime.rs:1636; packages/blitz-vibey-script/src/runtime.rs:1639; packages/blitz-vibey-script/src/runtime.rs:1691; packages/blitz-vibey-script/src/runtime.rs:1810; packages/blitz-vibey-script/src/runtime.rs:1165»
  new: «runtime/timers.rs:32; packages/blitz-vibey-script/src/runtime/timers.rs:35; packages/blitz-vibey-script/src/runtime/events.rs:191; packages/blitz-vibey-script/src/runtime/events.rs:310; packages/blitz-vibey-script/src/runtime/events.rs:81»
- after: .andromeda/obs-plan.md:229
  anchor: «- Error sources labelled in reports include "timer callback", "event listener", "error event listener", "timer microtasks", "event microtasks" (packages/blitz-vibey-script/src/runtime.rs:1636; packages/blitz-vibey-script/src/runtime.rs:1639; packages/blitz-vibey-script/src/runtime.rs:1691; packages/blitz-vibey-script/src/runtime.rs:1810; packages/blitz-vibey-script/src/runtime.rs:1165)»
  add: «- A `file:` load asked by a non-`file:` document is refused by `document_may_load` as a typed `FetchError::Forbidden`, with no log site of its own: `fetch()` rejects with a TypeError, a module `import` fails, and a `<script src>` is skipped and recorded as a script error, which the recorded-error path drains and logs (§6) (packages/blitz-vibey-script/src/fetch.rs:40-46; packages/blitz-vibey-script/src/runtime/fetch.rs:40-44; packages/blitz-vibey-script/src/document.rs:163-169)»
- line: .andromeda/obs-plan-detail/log-coverage.md:101
  old: «runtime.rs:1104-1105»
  new: «runtime/events.rs:20-21»
- after: .andromeda/obs-plan-detail/log-coverage.md:125
  anchor: «- observed absent — a print, log site, span, field, env read or subscriber in the private items that factor the lint's findings walk and the two table-asserting tests (escher-driver `lint.rs`'s `readings` and `placement`; `mcp_tests.rs`'s `#[track_caller]` `assert_described`, `assert_schema` and `assert_property`; `stand_requirement_sweep`'s `#[track_caller]` `assert_0_2_0_row` and `assert_0_2_0_witness` and its target predicate `inside_stand_repeat`), the test helpers' assertion messages naming a row and an index only · searched: the silence census over `lint.rs`, `mcp_tests.rs` and `stand_requirement_sweep.rs`, reading 0 (exit 1), `mcp_tests.rs`'s first census; `mcp_tests.rs`'s env reads the 2 compile-time `env!` alone (`CARGO_MANIFEST_DIR`, `CARGO_PKG_VERSION`); escher-driver holding 2 `info_span!` sites, no event and no `#[instrument]` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, its Counts / qualifiers moved census bullet)»
  add: «- observed absent — a print or log site in blitz-vibey-script `tests/dom.rs`'s `style_property_accessors`, `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` · searched: the fixed-sha census over the added `packages/*.rs` lines of the upstream take read at `ebacd68bd`, its one hit in the file a test-only env read — `temp_dir_with_files` reads `TMPDIR` through `std::env::temp_dir()`, test code only and not secret, the second test-only `TMPDIR` site beside escher-driver's client unit test (packages/blitz-vibey-script/tests/dom.rs:863; as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13
  old: «The sink allowlist re-checked after the upstream take read at `f9cbb2756` holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/`, `packages/escher-driver/` and `packages/dioxus-native-dom/` unchanged; `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build; the taken commits' added engine lines hold one commented `dbg!` and no log site, `hit_test_dropped_nodes` reads 0 env · 0 print · 0 subscriber, and the root example `multi_doc` writes its timings with one `println!` to stdout, outside escher's sink, holding no page or user content (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)»
  new: «The sink allowlist re-checked after the upstream take read at `ebacd68bd` holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged and dioxus-native-dom taking one test alone (`input_focused_on_mount_enables_ime`); `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build; the fixed-sha census over the taken commits' `packages/*.rs` lines reads its two named hits, blitz-dom holds 0 wall-clock reads outside `clock.rs`, and the take adds no log site, span, field, log field or env read in source — the uncaught-JS-error `tracing::error!` stands whole in `report_js_error` (`runtime/events.rs`), still `#[cfg(feature = "tracing")]` under the engine target `blitz_vibey_script`, the sink printing its safe fields and the message `[redacted]`; `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:44
  old: «over the 32 slice files (blitz-vibey-script / blitz slice)»
  new: «over the 32 slice files (blitz-vibey-script / blitz slice); the slice's blitz-vibey-script code stands split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, upstream's side moved whole), and the take adds no scrub or redaction shape — 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)»
- line: .andromeda/obs-plan-detail/telemetry-strategy.md:22
  old: «over the 61 slice files (test crate slice)»
  new: «over the 61 slice files (test crate slice); the slice also holds upstream's target `pseudo_element_flow_classification` (3 tests, its `mod` line in the unbuilt `all.rs`), which installs no subscriber, reads no env var and prints nothing (0 env · 0 print · 0 subscriber) and holds no log site — 61 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:17
  old: «over the 61 slice files (test crate slice)»
  new: «over the 61 slice files (test crate slice); the slice also holds upstream's target `pseudo_element_flow_classification`, which installs no subscriber, reads no env var, prints nothing and opens no span — 61 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:15
  old: «over the 32 slice files (blitz-vibey-script / blitz slice)»
  new: «over the 32 slice files (blitz-vibey-script / blitz slice); the slice's blitz-vibey-script code stands split — the runtime and the element bindings, upstream's side moved whole, in the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files) — and the take adds no span, so the absence stands for the moved code — 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)»
- line: .andromeda/obs-plan-detail/metric-coverage.md:34
  old: «over the 32 slice files (blitz-vibey-script / blitz slice)»
  new: «over the 32 slice files (blitz-vibey-script / blitz slice); the slice's blitz-vibey-script code stands split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, upstream's side moved whole), and the take moves no dependency, feature or manifest and adds no log site, span or field — 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)»

## Patterns
[[pattern]]
id = "obs-01"
fixed = "take read at `f9cbb2756` is silent"

[[pattern]]
id = "obs-02"
fixed = "re-checked after the upstream take read at `f9cbb2756` holds"
