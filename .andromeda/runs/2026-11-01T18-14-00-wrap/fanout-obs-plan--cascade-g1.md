proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 15
    section: "## 4. Span / Trace Coverage"
    change: "On this line replace «9 JS bootstrap files» with «10 JS bootstrap files, `url_search_params.js` the tenth and first in `BOOTSTRAP_JS` since 2026-11-01-upstream-sync-ui-graph, adding no span»; the rest of the line stands."
    sidecar: "span-trace-coverage: blitz-vibey-script slice now reads ten JS bootstrap files, `url_search_params.js` first, no span added."
    rationale: "report.md:17 states ten bootstrap scripts where nine stood, and report.md:28 states no `tracing` site added; the `url-params` sweep at report.md:46 lists two other rows and, per report.md:62, reaches its patterns' wording and no further, so this wording was not listed."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 34
    section: "## 5. Metric Coverage"
    change: "On this line replace «9 JS bootstrap files» with «10 JS bootstrap files, `url_search_params.js` the tenth and first in `BOOTSTRAP_JS` since 2026-11-01-upstream-sync-ui-graph, adding no metric»; the rest of the line stands."
    sidecar: "metric-coverage: blitz-vibey-script slice now reads ten JS bootstrap files, `url_search_params.js` first, no metric added."
    rationale: "Second occurrence of the same count: report.md:17 states ten bootstrap scripts where nine stood, and report.md:30 states no dependency moved."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 72
    section: "## 6. Log Coverage"
    change: "Add after this line:   - no site added: a failed stylesheet now reaches the resource-load-failed line above through `NetHandler::error` and `ResourceHandler<StylesheetHandler>::error`, the error text stopping in `ResourceLoadResponse` and that line, and a bad `href` starts no load, so it reaches no fetch or load line (packages/blitz-dom/src/net.rs:144-146; packages/blitz-dom/src/mutator.rs:1392-1394; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "log-coverage: blitz-dom's resource-load-failed line now also carries a failed stylesheet, no site added; a bad `href` starts no load."
    rationale: "report.md:14 states the handler's `Err(error)` and that a bad `href` starts no load; report.md:31 states the error text stops in `ResourceLoadResponse` and that log line; report.md:103 names this line as the new path's instrumentation."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:103
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 77
    section: "## 6. Log Coverage"
    change: "Add after this line:   - no site added: the restyle narrowing, `NetHandler::error` and the failed-stylesheet path, blitz-paint's `blend` module and its blend layer, `init_stylo_prefs`, `is_inline_with_only_block_children` and its paint skip, dioxus-native-dom's mounted-event guard and the `deallocate_anonymous_block` fix add no env read, print, `tracing` site or wall-clock read (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)"
    sidecar: "log-coverage: records the engine changes of the take read at `e42ee1f25` as adding no log site."
    rationale: "F2; report.md:11-23 name the changes and report.md:28 states no env read, print, `tracing` site or wall-clock read added."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 98
    section: "## 6. Log Coverage"
    change: "Add after this line:   - no site added: the arm that logs `Error fetching` now also calls `handler.error(url, e.to_string())`, then the waker, so a failed fetch logs this line and reaches its handler (packages/blitz-net/src/lib.rs:313; packages/blitz-net/src/lib.rs:316; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "log-coverage: blitz-net's `Error fetching` arm now also reaches the handler, no site added."
    rationale: "report.md:14 states the error arm's call at `:313` and the waker at `:316`; report.md:28 states no `tracing` site added; report.md:103 names `Error fetching` as the new path's instrumentation."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 109
    section: "## 6. Log Coverage"
    change: "Add after this line:   - no site added: `URLSearchParams`, `URL.prototype.searchParams` and the native `parse_search_params`, registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script, hold no log site and no carrier for what a page passes them (packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "log-coverage: records `URLSearchParams` and its native helper as holding no log site and no carrier."
    rationale: "F4; report.md:17 states the surface and the helper's deletion, and report.md:102 reads PII n/a with no log site and no carrier."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 130
    section: "## 6. Log Coverage"
    change: "Add after this line: - observed absent — an env read, print or `tracing` site in the new test targets `key_press_after_container_lost_its_box` (blitz-tests, 3 tests, both layout modes, no `stand_` prefix) and `url_search_params` (blitz-vibey-script, 4 tests) · searched: the sink census over this chunk's added lines (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)"
    sidecar: "log-coverage: records the two new test targets as silent."
    rationale: "F3; report.md:27 and report.md:102 state the targets and their counts, and report.md:28 states no env read, print or `tracing` site added."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 136
    section: "## 6. Log Coverage"
    change: "Add after this line: - observed absent — an env read, print, `tracing` site or wall-clock read added by the upstream take read at `e42ee1f25`, its twelve commits and escher's companions · searched: the census of this take, naming its 2 hits; escher-driver holds 2 `info_span!`, `stand_act_spans` eight fields, and `host_log` and `mcp_log` read 0 needles at `trace` under both builds (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)"
    sidecar: "log-coverage: adds the silence record of the take read at `e42ee1f25` beside the last take's, which stays as history."
    rationale: "F1; report.md:35 keeps this line as the last take's history with a new sentence beside it, and report.md:28, report.md:31 and report.md:145 state the silence, the span counts and the needles."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:145
    fact: F1
note: "Not proposed here: the body's `obs-plan.md:99` replacement (report.md:98) is the body's, and §7's blitz-net class with F5, F6 and F7 are group 2's categories; `telemetry-strategy.md:14` follows the report's no-change, though `fast` now ends on a `cargo test -p blitz-tests` step (report.md:41, report.md:64)."
