proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "Follow line 13 with a continuation line, line 13 left as the last take's history: «The sink allowlist re-read after the upstream take read at `e42ee1f25` holds: no scrub set, config key or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` untouched, escher-driver holding its 2 `info_span!` and `stand_act_spans` its eight fields; `host_log` and `mcp_log` read 0 needles at `trace` under both builds; the census names its 2 hits, and the take adds no env read, print, `tracing` site or wall-clock read (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).»"
    sidecar: "PII Scrubbing & Compliance: the sink allowlist's re-read after the take read at e42ee1f25 recorded beside the last take's."
    rationale: "report.md:35 keeps pii-scrubbing-compliance.md:13 as history with a new sentence for this take beside it, report.md:98 carries obs-plan §8 on evidence/sink.md; the facts stand at report.md:8, :28, :31 and :145."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 20
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add one item after line 20: «- A failed load's text now reaches `NetHandler::error`, blitz-net's error arm handing it `url` and `e.to_string()` (packages/blitz-net/src/lib.rs:313): the error text stops in `ResourceLoadResponse` and the standing `Resource load failed` line, no carrier taking it, and on that line `url` and `error` are in escher-telemetry's scrub set, the sink printing `waiting_nodes` (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).»"
    sidecar: "PII Scrubbing & Compliance: a failed load's url and error text through NetHandler::error recorded, redacted on the standing line, no carrier."
    rationale: "report.md:14 opens the path, report.md:31 bounds where the error text goes, report.md:103 rates its PII redacted; D-obs-pii holds, and this new crossing of a url and an error text is not yet in the file."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 20
    section: "## 8. PII Scrubbing & Compliance"
    change: "Add one item after line 20: «- `URLSearchParams` and `URL.prototype.searchParams`, a page's script reading and writing `href` and `search` on the registered script crossing, add no log site and no carrier; the native `parse_search_params` is registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script (packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»"
    sidecar: "PII Scrubbing & Compliance: URLSearchParams recorded as adding no log site or carrier."
    rationale: "report.md:17 adds the surface and report.md:102 rates it PII n/a, no log site, no carrier; lines 15-20 hold one such item per new value surface."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 51
    section: "## 8. PII Scrubbing & Compliance"
    change: "In line 51's parenthesis, «9 JS bootstrap files» becomes «10 JS bootstrap files since 2026-11-01-upstream-sync-ui-graph, `url_search_params` first, 9 at the reading», the rest of the line unedited."
    sidecar: "PII Scrubbing & Compliance: blitz-vibey-script's bootstrap scripts nine to ten in the slice's standing description."
    rationale: "report.md:17 and report.md:46 move the bootstrap scripts from nine to ten; line 51 says the code «stands split» with 9, a present claim the `url-params` pattern's 2 rows at report.md:46 did not reach."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
note: "Two for the orchestrator, no proposal here: .andromeda/obs-plan.md:220 (§7, outside this file) still reads blitz-net fetch errors as not propagated to the handler, where report.md:14 now calls handler.error; and report.md:31 says the sink prints `waiting_nodes` and the message, while .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:26 redacts the message under an engine target, so the failed-load item names `waiting_nodes` alone. D-obs-instrumentation, D-obs-stack and D-obs-pii read no violation in this file."
