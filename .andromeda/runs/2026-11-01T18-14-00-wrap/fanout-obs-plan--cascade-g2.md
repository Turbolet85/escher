proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 13
    section: "## 8. PII Scrubbing & Compliance"
    change: "Follow line 13, which stays as the last take's history, with a continuation line of the same item: ` The sink allowlist re-read after the upstream take read at `e42ee1f25` holds: no scrub set or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged; `host_log` and `mcp_log` read 0 needles at `trace` under both builds; the census names its 2 hits, and the twelve commits and escher's companions add no env read, print, `tracing` site or wall-clock read (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).`"
    sidecar: "obs-plan §8 values: this take's sink re-read at `e42ee1f25` follows the `ebacd68bd` re-check, which stays as history."
    rationale: "The report reads `pii-scrubbing-compliance.md:13` as history of the last take, no-change as such, with a new sentence for this take beside it (report.md:35, report.md:98); the sentence's facts are its Symbols, Schema and acceptance lines (report.md:28, report.md:31, report.md:145)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:35
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 27
    section: "## 8. PII Scrubbing & Compliance"
    change: "Follow line 27 with a new item: `- A failed load's error text now travels `NetHandler::error` into `ResourceLoadResponse` and the `Resource load failed` line; `url` and `error` are in escher-telemetry's scrub set (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59), so the sink prints `waiting_nodes` and the message, the message as the engine-target rule above writes it, and no other carrier takes the text (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).`"
    sidecar: "obs-plan §8 scrubbing: a failed load's error text, its one log line and its scrub recorded (F5)."
    rationale: "The report's Schema line and its coverage row for `NetHandler::error` state the path and read PII as redacted (report.md:31, report.md:103); the scrub-set entity is the fact's home, and escher-telemetry itself has no diff (report.md:8)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 51
    section: "## 8. PII Scrubbing & Compliance"
    change: "After line 51's measured reading, which stays, add to the same line: `; the JS bootstrap files read ten since 2026-11-01-upstream-sync-ui-graph, `url_search_params` first, that take moving no scrub set (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)`"
    sidecar: "obs-plan §8 absence, blitz-vibey-script slice: JS bootstrap files nine to ten, no scrub set moved."
    rationale: "The report moves the bootstrap scripts from nine to ten with `url_search_params.js` first in `BOOTSTRAP_JS` (report.md:17, report.md:46) and moves no scrub set (report.md:31); its `url-params` sweep lists two sites and not this one, a listing reaching its patterns' wording and no further (report.md:62)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17
note: F5's «prints the message» (report.md:31) reads against entity #2.1's rule that an engine target writes the message as `[redacted]`, so the proposal defers to that rule and asserts no raw message; no D-obs-pii drift, no D-obs-stack drift; F1-F4 (log-coverage) and F6-F7 (error-capture-reporting) are not this group's.
