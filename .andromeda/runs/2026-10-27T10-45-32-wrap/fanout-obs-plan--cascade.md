proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "Restate the take reading at `ebacd68bd`: The sink allowlist re-checked after the upstream take read at `ebacd68bd` (merge base `23b9b26a9` unmoved, eleven taken, sixteen left out) holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged, dioxus-native-dom gaining one test alone (`input_focused_on_mount_enables_ime`, 0 lines removed); `host_log` and `mcp_log` pass under both builds; the fixed-sha census over the eleven commits' `packages/*.rs` reads its two named hits and blitz-dom holds 0 wall-clock reads outside `clock.rs`; the take adds no log site, span, field, log field or env read in source, the uncaught-JS-error `tracing::error!` moving whole into `report_js_error` (`runtime/events.rs`), still `#[cfg(feature = \"tracing\")]` under the engine target `blitz_vibey_script`, the sink printing its safe fields with the message `[redacted]`; upstream's target `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md) — as the take read at `f9cbb2756` held before (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)"
    sidecar: "pii-scrubbing-compliance: the sink allowlist re-check restated for the upstream take read at `ebacd68bd` (2026-10-27-upstream-sync-engine-debts), dioxus-native-dom no longer unchanged, the earlier `f9cbb2756` reading kept as before"
    rationale: "The report's `upstream` row lists `obs-plan-detail/pii-scrubbing-compliance.md:13` changed (report line 34); its Outcome re-asserts the sink re-check at `ebacd68bd` (line 123), escher-telemetry and escher-driver unchanged (line 20), dioxus-native-dom one test added and 0 lines removed (line 18), the JS-error site moved whole and feature-gated (lines 16, 89), and the new target 0 · 0 · 0 (line 123); the old line's `packages/dioxus-native-dom/` unchanged no longer holds for this take"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:34
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 22
    section: "## 2. Telemetry Strategy"
    change: "Keep the absence and append: ; the test crate slice has since gained upstream's target `pseudo_element_flow_classification` (3 tests, with its `mod` line in the unbuilt `all.rs`), which installs no subscriber, reads no env var and prints nothing (0 env · 0 print · 0 subscriber), the take adding no log site — the absence's claim unmoved by it (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)"
    sidecar: "telemetry-strategy: the test crate slice's logging absence notes the new silent target `pseudo_element_flow_classification` (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report adds the new file `tests/blitz-tests/tests/pseudo_element_flow_classification.rs` and `all.rs:54`'s mod line (report lines 9, 21) and measures it 0 env · 0 print · 0 subscriber with no log site added (lines 19, 123), so the 61-file reading no longer covers the whole slice"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 17
    section: "## 4. Span / Trace Coverage"
    change: "Keep the absence and append: ; the test crate slice has since gained upstream's target `pseudo_element_flow_classification`, which installs no subscriber, reads no env var and prints nothing, the take adding no span or log site — the absence's claim unmoved by it (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)"
    sidecar: "span-trace-coverage: the test crate slice's span absence notes the new silent target `pseudo_element_flow_classification` (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report adds the new target (report lines 9, 21) and records no new log site and 0 env · 0 print · 0 subscriber (lines 19, 123); F2's take adds no span"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 15
    section: "## 4. Span / Trace Coverage"
    change: "Keep the absence and append: ; since 2026-10-27-upstream-sync-engine-debts the slice's blitz-vibey-script code is split — the runtime and the element bindings moved, upstream's side whole, into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files) — and the take adds no span, so the absence stands for the moved code, 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)"
    sidecar: "span-trace-coverage: the blitz-vibey-script / blitz slice's span absence notes the runtime and element-binding split into 22 new files, no span added (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report names the 22 new blitz-vibey-script files and the split as a move (report lines 9, 16, 21, 25) and the take adds no log site, span or field (line 19; task F2)"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 34
    section: "## 5. Metric Coverage"
    change: "Keep the absence and append: ; since 2026-10-27-upstream-sync-engine-debts the slice's blitz-vibey-script code is split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, a move), and the take moves no dependency, feature or manifest and adds no log site, span or field — 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)"
    sidecar: "metric-coverage: the blitz-vibey-script / blitz slice's metrics absence notes the module split, no dependency or feature moved (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report names the split and its 22 new files (report lines 9, 21) and states Dependencies none, every manifest byte-identical (line 22), with no log site added (line 19)"
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 44
    section: "## 8. PII Scrubbing & Compliance"
    change: "Keep the absence and append: ; since 2026-10-27-upstream-sync-engine-debts the slice's blitz-vibey-script code is split into the `runtime/` and `dom/element/` module trees (13 Rust modules, 9 JS bootstrap files, a move, upstream's side whole), and the take adds no scrub or redaction shape — 32 the reading's own count (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)"
    sidecar: "pii-scrubbing-compliance: the blitz-vibey-script / blitz slice's redaction absence notes the module split, no scrub or redaction shape added (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report names the split and its 22 new files (report lines 9, 16, 21) and states no config key, violation schema, scrub or redaction shape changed (line 23)"
note: F1 and F2 belong with obs-plan §3's silence census in the body (obs-plan.md:87–98, :98 the `f9cbb2756` take's reading), F3 with error-capture-reporting's error-class or script-error groups and F4 with an env-read entity, none picked; the §1 vibey row, §3 drain rows and §7 script rows move by citation only (the sweep's).
