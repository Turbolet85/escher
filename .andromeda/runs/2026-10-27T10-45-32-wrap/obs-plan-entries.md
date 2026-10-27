## 2026-10-27-upstream-sync-engine-debts — citations re-pointed into the code the take moved
**Section:** §1 Obs Scope Summary (body: the blitz-dom layout and node rows, blitz-shell · blitz-paint, blitz-vibey-script · blitz) · §2 Telemetry Strategy (body: Telemetry mechanism) · §3 Observability Harness Contract (body: Embedder drain of script diagnostics) · §6 Log Coverage (log-coverage.md: values · logs) · §7 Error Capture & Reporting (body: Error classes captured; Script (JS) error surface) · §8 PII Scrubbing & Compliance (body: Values logged as-is) · §9 CI Integration (body: Telemetry artifact handling)
**Change:**
- The script error surface's citations — the window `error` event, `window.onerror`, a handler's exception recorded with no further event, the labelled error sources — and §6's uncaught-JS-error log cite blitz-vibey-script's `runtime/events.rs` and `runtime/timers.rs` (was `runtime.rs`, which no longer holds that code); claim text unchanged.
- The citations into blitz-dom `mutator.rs`, `layout/construct.rs` and `node/element.rs`, blitz-shell `window.rs` and blitz-vibey-script `document.rs` and `runtime.rs` (the JS console bridge to `log`) re-pointed to the lines the cited code now stands at; claim text unchanged.
- The publish build's `--verbose --trace` citation, in §6 and §9, points to the `dx bundle` line, which now stands inside a retry loop in `publish-browser.yml`; the upstream-only and repository-guard claims unchanged.
**Why:** the take split blitz-vibey-script's runtime into `runtime/` modules and shifted lines in the other files it touched; a citation whose code left its file read false and was re-pointed by hand at Validate, the rest by the take's line map. No log site, field or reading moved.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the take read at `ebacd68bd` silent, and the sink re-check restated
**Section:** §3 Observability Harness Contract (body: Logging stack — the silence census) · §6 Log Coverage (log-coverage.md: absence · logs) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: values · pii, the sink allowlist re-check)
**Change:**
- §3 census: the take reading is the upstream take read at `ebacd68bd` (was the one read at `f9cbb2756`, per «2026-10-24-upstream-sync-polish-and-ship — the ten-commit take read silent, and `multi_doc`'s printed timings», its `hit_test_dropped_nodes`, commented `dbg!` and five named changes leaving the census): upstream's target `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber; the taken commits add no log site, print, listener or env read in source — the fixed-sha census over their `packages/*.rs` lines reading its two named hits, the uncaught-JS-error `tracing::error!` standing whole in `report_js_error`, still `#[cfg(feature = "tracing")]` at the engine target `blitz_vibey_script`, and a test-only `TMPDIR` read — and no wall-clock read in blitz-dom outside `clock.rs`; `ElementData::flush_open_state` (`:open`), `document_may_load`, the IME enable at a focused input's editor construction, the pseudo vote in flow classification and the shell's redraw route add no log site, span, field, log field or env read; `packages/escher-telemetry/` and `packages/escher-driver/` unchanged (was escher-telemetry alone named); `host_log` and `mcp_log` pass under both builds.
- §8: the re-check restated at `ebacd68bd` — no scrub set, redaction shape or violation schema moves; dioxus-native-dom takes one test alone, `input_focused_on_mount_enables_ime` (was named unchanged); the sink prints the JS-error site's safe fields and the message `[redacted]`; the `multi_doc` clause leaves this line.
- §6: one `observed absent` row — blitz-vibey-script `tests/dom.rs`'s `style_property_accessors`, `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` hold no print or log site; the census's one hit there is `temp_dir_with_files`'s `TMPDIR` read through `std::env::temp_dir()`, test code only and not secret, a second test-only `TMPDIR` site beside escher-driver's.
**Why:** each upstream sync is read for log sites and the sink re-checked; the take is the founder's own word on the eleven commits, relayed verbatim by the overseer, with the operator's dialog answer. At Validate the census line was replaced, not given a sibling: a body holds current truth, the earlier reading's history in this sidecar; two drafted §6 rows repeating the census were dropped. No record class, field or carrier is added.
**Kept:** the earlier entry's `multi_doc` claims (§1, §5, §6) are untouched, so it is not superseded.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the blitz-vibey-script slice's absences after the module split
**Section:** §5 Metric Coverage (metric-coverage.md: absence · metrics) · §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: absence · pii) · §4 Span / Trace Coverage (span-trace-coverage.md: absence · spans)
**Change:**
- The 32-file blitz-vibey-script / blitz slice's three absences — metrics, redaction or scrubbing, spans — each note that the slice's blitz-vibey-script code stands split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, upstream's side moved whole), 32 the reading's own count; each absence stands.
- metric-coverage: the take moves no dependency, feature or manifest and adds no log site, span or field; pii-scrubbing-compliance: the take adds no scrub or redaction shape; span-trace-coverage: the take adds no span, so the absence stands for the moved code.
**Why:** the take split blitz-vibey-script's runtime and element bindings into new files, a move; the report records no dependency moved and no log site, span, field, scrub or redaction shape added, so each slice reading holds for the moved code under its own count.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the test crate slice's absences hold `pseudo_element_flow_classification`
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: absence · spans) · §2 Telemetry Strategy (telemetry-strategy.md: absence · strategy)
**Change:**
- The 61-file test crate slice's two absences — spans or traces, telemetry or logging crates — note that the slice also holds upstream's target `pseudo_element_flow_classification`, which installs no subscriber, reads no env var and prints nothing, 61 the reading's own count; each absence stands.
- span-trace-coverage: it opens no span; telemetry-strategy: 3 tests, its `mod` line in the unbuilt `all.rs`, 0 env · 0 print · 0 subscriber and no log site.
**Why:** the take adds the target as a file of its own, read silent at the chunk's sink re-check, so the 61-file reading no longer named the whole slice.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/

## 2026-10-27-upstream-sync-engine-debts — the `file:` refusal in the script error surface
**Section:** §7 Error Capture & Reporting (body: Script (JS) error surface — blitz-vibey-script)
**Change:**
- One bullet added: a `file:` load asked by a non-`file:` document is refused by `document_may_load` as a typed `FetchError::Forbidden`, with no log site of its own — `fetch()` rejects with a TypeError, a module `import` fails, and a `<script src>` is skipped and recorded as a script error, which the recorded-error path drains and logs (§6).
**Why:** the refusal is upstream's, taken on the founder's own word on the eleven commits, relayed verbatim by the overseer, which ratifies for this take an engine behaviour change for every document, the `file:` refusal among them; the fact found no home among the drafts, and Validate placed it in §7's script error surface as the error class it is. No log site, field or carrier is added.
**Ref:** .andromeda/runs/2026-10-27T10-45-32-wrap/
