# Cascade compare — obs-plan · wrap · 2026-11-01-upstream-sync-ui-graph
E items 11 · held 7 · lost at classify 1 · lost at strong 1 · beside a placed fact 1 · cascade only 3 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### error-capture-reporting#1.6 · lost at classify · beside a placed fact F6 (near 15, error-capture-reporting#1.6) · .andromeda/obs-plan-detail/error-capture-reporting.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 15
    section: "## 7. Error Capture & Reporting"
    change: "Add one row after the absence group: - blitz-net: a failed fetch reaches its handler since 2026-11-01-upstream-sync-ui-graph: the error arm calls `handler.error(url, e.to_string())` (packages/blitz-net/src/lib.rs:313), `NetHandler::error` being new on the public trait with a default that does nothing (packages/blitz-traits/src/net.rs:36-38), blitz-dom's stylesheet handler responding `Err(error)` (packages/blitz-dom/src/net.rs:144-146) and blitz-test-harness's `CountedHandler` forwarding it (packages/blitz-test-harness/src/settle.rs:121-125); no log site is added, the standing `Error fetching` and `Resource load failed` lines report it, `url` and `error` are in escher-telemetry's scrub set (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59), so the sink prints `waiting_nodes` and the message, and the error text stops in `ResourceLoadResponse` and that log line, no carrier taking it (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)."
    sidecar: "2026-11-01-upstream-sync-ui-graph: §7 detail gains the failed-fetch path, `NetHandler::error` forwarded to the handler, no log site added, its error text held to `ResourceLoadResponse` and the scrubbed log line."
    rationale: "The report's Symbols / APIs state the new error path (report.md:14, report.md:25) and its Schema / config bullet and Coverage row state its reporting and redaction (report.md:31, report.md:103); this category holds how errors are captured and reported, and its file holds no row for the path."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14

### pii-scrubbing-compliance#1.7 · lost at classify · beside a placed fact F5 (near 27, pii-scrubbing-compliance#2.1) · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 20
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add one item after line 20: «- A failed load's text now reaches `NetHandler::error`, blitz-net's error arm handing it `url` and `e.to_string()` (packages/blitz-net/src/lib.rs:313): the error text stops in `ResourceLoadResponse` and the standing `Resource load failed` line, no carrier taking it, and on that line `url` and `error` are in escher-telemetry's scrub set, the sink printing `waiting_nodes` (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).»"
    sidecar: "PII Scrubbing & Compliance: a failed load's url and error text through NetHandler::error recorded, redacted on the standing line, no carrier."
    rationale: "report.md:14 opens the path, report.md:31 bounds where the error text goes, report.md:103 rates its PII redacted; D-obs-pii holds, and this new crossing of a url and an error text is not yet in the file."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31

### pii-scrubbing-compliance#1.7 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 20
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add one item after line 20: «- `URLSearchParams` and `URL.prototype.searchParams`, a page's script reading and writing `href` and `search` on the registered script crossing, add no log site and no carrier; the native `parse_search_params` is registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script (packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»"
    sidecar: "PII Scrubbing & Compliance: URLSearchParams recorded as adding no log site or carrier."
    rationale: "report.md:17 adds the surface and report.md:102 rates it PII n/a, no log site, no carrier; lines 15-20 hold one such item per new value surface."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102

### span-trace-coverage#2.3 · lost at strong · .andromeda/obs-plan-detail/span-trace-coverage.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: "## 4. Span / Trace Coverage"
    change: "Follow the line with one item: - `packages/escher-driver` and the engine — the upstream take read at `e42ee1f25` and escher's fixes beside it add no span, field, event or log line: `packages/escher-driver` and `packages/escher-telemetry` are untouched, escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`); blitz-dom's `restyle_for_attribute_change` · `restyle_for_child_list_change`, `table_needs_rebuild`, `init_stylo_prefs`, `is_inline_with_only_block_children` and `deallocate_anonymous_block`'s fix, blitz-paint's `blend` module and its blend layers, `NetHandler::error` and `CountedHandler`'s forward of it, blitz-vibey-script's `URLSearchParams` and `parse_search_params`, `set_custom_widget`'s attribute replay, the mounted-event guard in `flush_queued_mounted_events` and `Harness::text_contrast`'s blend read open no span; `host_log` and `mcp_log` read 0 needles at `trace` under both builds (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "§4 values: the take at `e42ee1f25` and its companions recorded as adding no span or field; escher-driver at 2 `info_span!`, `command` at eight fields."
    rationale: "The report brings new engine operations (report.md:11-25) and states no `tracing` site added (report.md:28), escher-driver at 2 `info_span!` and `stand_act_spans` at eight fields (report.md:31), the driver and telemetry crates untouched (report.md:8), and the census and needles met (report.md:145); this file records each such surface's span silence, as its select-list item does, and §4 requires no engine span, so D-obs-instrumentation is not violated."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:145

## Held

- log-coverage#8.1
- log-coverage#8.5
- log-coverage#9.9
- metric-coverage#2.6
- pii-scrubbing-compliance#1.1
- pii-scrubbing-compliance#5.7
- span-trace-coverage#1.8

## Cascade only

- log-coverage#8.8
- log-coverage#9.3
- pii-scrubbing-compliance#2.1
