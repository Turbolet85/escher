# Cascade compare — obs-plan · wrap · 2026-10-31-select-popup
E items 13 · held 5 · lost at classify 1 · lost at strong 1 · beside a placed fact 1 · cascade only 5 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### log-coverage#8.7 · lost at strong · beside a placed fact F6 (near 43, log-coverage#5.1) · .andromeda/obs-plan-detail/log-coverage.md:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 100
    section: "## 6. Log Coverage"
    change: New item after this row, in the blitz-shell, blitz-paint group - no site added - blitz-paint's `render::select_list` (`paint_select_list`, `device_rect`, `paint_list_box`, `paint_list_row`, `paint_list_thumb`) and `text.rs`'s `stroke_text_as` with `RunPaint` add no log site, the censuses reading clean at the pre-CI tree and the `text.rs` fix at `715798ce2` adding no such token (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, its gate entries 4 and 5 and its (obs) acceptance).
    sidecar: log-coverage - new no-site item for blitz-paint's select list painting and `stroke_text_as` (2026-10-31-select-popup).
    rationale: The report's Symbols / APIs, Paint, adds these names; its gate entry on `packages/blitz-paint/src/render/select_list.rs` reads exit 1 and its (obs) acceptance states the fix adds no such token.

### pii-scrubbing-compliance#1.4 · lost at classify · beside a placed fact F10 (near 13, pii-scrubbing-compliance#1.1) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:16
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 16
    section: "## 8. PII Scrubbing & Compliance"
    change: "- An open select list crosses the ratified carriers and adds none: an option's label and a group's `label`, page text, are new painted pixels in every `screenshot` of an open list, and a label otherwise leaves only as the snapshot's existing `name`; a row's own `bounds` reads in the snapshot and the diff, `differs` naming `bounds`, a closed set inside the ratified crossing; no key name, node key, field, cause, session error, exit, carrier, listener, port, env var, log field or span is added and no redaction shape moves; the pixels are a boundary widening, carried to the founder's answer card at this wrap's P2 on his recorded word, that report's inputs#I4 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "§8: new row, a page's option and group labels as pixels in a shot of an open list, the boundary widening carried to the founder's card."
    rationale: "Report Schema / config states the crossing and the new pixel content, Coverage of new surfaces marks the list's pixels «boundary widening, P2's halt», and Expected amendments has the playbook's Boundary widening halt once at P2 on inputs#I4; §8 holds no statement of this crossing."

### pii-scrubbing-compliance#1.4 · lost at classify · beside a placed fact F13 (near 14, pii-scrubbing-compliance#1.2) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 16
    section: "## 8. PII Scrubbing & Compliance"
    change: "- A select list's labels reach no log: `SelectListRow.label` holds page text in a public read type and is never logged, the two new `select_list` modules and the added `packages/*.rs` lines hold no log site, the census reading 0, and a list opened and chosen through the `escher-session` host and through the `mcp` role reads 0 option needles on stderr at `trace` (`option_needles`; `host_log`, unix only; `mcp_log`, which records nothing), `stand_act_spans` still reading the `command` span's eight fields (packages/blitz-dom/src/node/select_list.rs:93-108; examples/seven_guis/tests/common/mod.rs:425-453; examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "§8: new row, the select list's labels reach no log, with the host and mcp readings at trace that hold it."
    rationale: "Report Coverage of new surfaces (`PII no log site`; `SelectListRow.label` page text in a read type, never logged), Outcome's (security) and (obs) acceptance lines and the gate entries read 0 needles and 0 added log tokens; D-obs-pii holds, and §8 records each such reading as a row."
    basis: packages/blitz-dom/src/node/select_list.rs:93-108

### pii-scrubbing-compliance#1.4 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 16
    section: "## 8. PII Scrubbing & Compliance"
    change: "- The driver's restated `click`, `type` and `press` help and `covered`'s meaning, with its clause on an option of a closed select, are the program's own fixed `'static` text and hold no call text; the `command` span's fields stay eight (`stand_act_spans`) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)."
    sidecar: "§8: new row, the driver's restated help and `covered` text hold no call text."
    rationale: "Report Coverage of new surfaces, `driver help and covered text`, gives this PII reading (`no call text`), and Schema / config states the texts are the program's own `'static` text."

### span-trace-coverage#2.2 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 30
    section: "## 4. Span / Trace Coverage"
    change: "Add after line 30, one row: - `packages/escher-driver` and the engine — the select list adds no span, field, event or log line (since 2026-10-31-select-popup): blitz-dom's `node::select_list`, blitz-paint's `render::select_list`, the `BaseDocument` readers `open_select` · `select_list_bounds` · `select_list_view` · `select_row_layout` and writer `close_select_list`, dioxus-native-dom's `is_select` and the driver's `focus_select` open no span; escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`), the restated `click` · `type` · `press` help and `covered`'s restated meaning being fixed `'static` text that reaches no field; seven_guis `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` (under `#[cfg(unix)]`) and `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` drive an open list at `trace`, reading 0 needles (§6 Log coverage, §8 PII), `host_log` counting the host's closed `command` span lines (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "§4 Span / Trace Coverage: new row — the select list adds no span, field, event or log line; 2 info_span! sites and the command span's eight fields stand; host_log and mcp_log read an open list at trace."
    rationale: "Report Coverage of new surfaces: `select list, engine overlay` instrumentation none added, none required (silence census, gate entries 2, 4, 5), and `driver help and covered text` span fields eight, unmoved; Schema / config: no log field or span; Outcome (obs): 2 `info_span!`, silence census 0, added lines clean; (security) `host_log`, `mcp_log` 0 needles at `trace`, `stand_act_spans` eight fields; New text, by line lists both tests and `command_spans`. The file records each such chunk's span standing, as line 30 does for a hosted part read."

## Held

- ci-integration#1.6
- log-coverage#6.1
- log-coverage#8.1
- log-coverage#9.7
- pii-scrubbing-compliance#1.3

## Cascade only

- log-coverage#5.1
- log-coverage#8.4
- log-coverage#9.3
- pii-scrubbing-compliance#1.1
- pii-scrubbing-compliance#1.2
