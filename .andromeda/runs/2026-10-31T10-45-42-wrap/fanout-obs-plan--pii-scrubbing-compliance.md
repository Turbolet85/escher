proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a continuation line after line 13: `The scrub re-read at 2026-10-31-engine-debts holds: no scrub or redaction shape moves, and the chunk's added lines hold no log site, print, env read, listener, wall-clock read or panic path (the added-lines census reading 0, `packages/blitz-shell/src/event.rs` held out, its addition an inline test); the 2 `info_span!` sites are unmoved, the `focussed node` line re-read reads its needles at 0, and `host_log` and `mcp_log` read 0 needles under both builds (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the `focussed node` re-read at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/sink.md)`"
    sidecar: "§8 gains the scrub's re-read at 2026-10-31-engine-debts: no scrub or redaction shape moved, no log site added, 0 needles under both builds."
    rationale: "The report's Changes, Schema / config, states no scrub or redaction shape moved and no log site, print, env read, listener, wall-clock read or panic path on the added lines; Coverage of new surfaces and Outcome (closed sets) state the `focussed node` re-read and the 0 needles. The file's line 13 holds the same re-check for the prior chunk and no line holds this one's."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add a continuation line after line 13, following the re-read: `The surfaces 2026-10-31-engine-debts adds cross into no log: a `multiple` select's chosen set leaves only as form entries on `input` and `change`, one `(name, FormValue::Text(value))` per chosen option (`multiple_select_values`, packages/dioxus-native-dom/src/dioxus_document.rs:31-45), and as the snapshot's `value` and the options' tree nodes; an `optgroup`'s non-empty `label` is a name in the tree, inside the ratified name crossing; the engine's `change` on a checkbox or a radio carries the value class `input` already carries (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)`"
    sidecar: "§8 gains where the chunk's new values leave: a `multiple` select's set, an `optgroup`'s label and the engine's `change`, none in a log."
    rationale: "The report's Coverage of new surfaces gives each surface's PII reading (`multiple` select, `optgroup`, `change` on a checkbox and a radio), and Symbols / APIs F6 and F10 give `multiple_select_values` and the engine's `change`; §8 of this file names no such crossing, and D-obs-pii holds because the report adds no log site on them."
note: "D-obs-instrumentation, D-obs-stack and D-obs-pii find no violated occurrence in this file (the report adds no log site, telemetry dependency or raw value in a log), and no Retired bullet's item stands in it: the `runtime/events.rs` at line 13 is `report_js_error`, not the retired `change` synthesis."
