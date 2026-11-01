proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 15
    section: "## 4. Span / Trace Coverage"
    change: "Where the line reads `(13 Rust modules, 9 JS bootstrap files)` it should read `(13 Rust modules, 10 JS bootstrap files, `url_search_params.js` the tenth, first in `BOOTSTRAP_JS` since 2026-11-01-upstream-sync-ui-graph, adding no span)`; the rest of the line stands."
    sidecar: "§4 absence, blitz-vibey-script slice: JS bootstrap files nine to ten, `url_search_params.js` added, no span added."
    rationale: "The report moves the bootstrap scripts from nine to ten (report.md:17, report.md:46) and adds no `tracing` site (report.md:28); its `url-params` sweep listed only `existing-scopes.md:146` and `standard-contracts.md:240`, and this line's wording `JS bootstrap files` sits outside that listing, which reaches its patterns' wording and no further (report.md:62)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: "## 4. Span / Trace Coverage"
    change: "Follow the line with one item: - `packages/escher-driver` and the engine — the upstream take read at `e42ee1f25` and escher's fixes beside it add no span, field, event or log line: `packages/escher-driver` and `packages/escher-telemetry` are untouched, escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`); blitz-dom's `restyle_for_attribute_change` · `restyle_for_child_list_change`, `table_needs_rebuild`, `init_stylo_prefs`, `is_inline_with_only_block_children` and `deallocate_anonymous_block`'s fix, blitz-paint's `blend` module and its blend layers, `NetHandler::error` and `CountedHandler`'s forward of it, blitz-vibey-script's `URLSearchParams` and `parse_search_params`, `set_custom_widget`'s attribute replay, the mounted-event guard in `flush_queued_mounted_events` and `Harness::text_contrast`'s blend read open no span; `host_log` and `mcp_log` read 0 needles at `trace` under both builds (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "§4 values: the take at `e42ee1f25` and its companions recorded as adding no span or field; escher-driver at 2 `info_span!`, `command` at eight fields."
    rationale: "The report brings new engine operations (report.md:11-25) and states no `tracing` site added (report.md:28), escher-driver at 2 `info_span!` and `stand_act_spans` at eight fields (report.md:31), the driver and telemetry crates untouched (report.md:8), and the census and needles met (report.md:145); this file records each such surface's span silence, as its select-list item does, and §4 requires no engine span, so D-obs-instrumentation is not violated."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:145
note: "D-obs-instrumentation, D-obs-stack, D-obs-pii and D-retired find no violation in this file; line 27's load-wait absence stays no-change as report.md:49 rules, a failed load ending the wait adding no span or field."
