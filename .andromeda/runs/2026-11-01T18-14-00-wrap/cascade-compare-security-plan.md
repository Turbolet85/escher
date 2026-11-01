# Cascade compare — security-plan · wrap · 2026-11-01-upstream-sync-ui-graph
E items 18 · held 11 · lost at classify 0 · lost at strong 0 · beside a placed fact 2 · cascade only 5 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### input-validation#1.118 · lost at classify · beside a placed fact F7 (near 146, input-validation#1.42) · .andromeda/security-plan-detail/input-validation.md:398
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 398
    section: "## Input Validation"
    change: New row after this line, in its neighbours' three lines — **Boundary:** JS API arguments · **What to validate:** `URLSearchParams` and `URL.prototype.searchParams`, `href` and `search` · **How (stack-specific):** A page's script reaches a global class and `URL.prototype.searchParams`, `href` and `search` live both ways, from the bootstrap script `url_search_params.js`, first in `BOOTSTRAP_JS`; its TypeError arms refuse invalid inputs and receivers and its values pass `usv` (packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:6-14; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:20-22; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:29-105; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:123-133; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:134-146); the native `parse_search_params`, over `url::form_urlencoded::parse`, is registered as `__blitz_parse_search_params` and deleted from `globalThis` before any page script (packages/blitz-vibey-script/src/runtime/bootstrap.rs:167-184; packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:3-4); no log site and no carrier; pinned by `url_search_params` 4, `invalid_inputs_and_receivers` and `url_search_params_are_bidirectionally_live` among them (packages/blitz-vibey-script/tests/url_search_params.rs:61-78; packages/blitz-vibey-script/tests/url_search_params.rs:80-95) — a realization on the registered script crossing, not a boundary widening, an engine change for every document, ratified — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: JS API arguments gains a row for `URLSearchParams` and `URL.prototype.searchParams`, `91ef65e21`, a realization on the registered script crossing; ratified on the founder's «Двенадцать коммитов (Рекомендую)», inputs#I3.
    rationale: The plan's security amendment owes §Input Validation this row, the script's new public surface being unnamed in security-plan, and its validation is present (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:17, :46, :96, :102).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102

### logging-monitoring#3.1 · lost at classify · beside a placed fact F12 (near 14, logging-monitoring#1.1) · .andromeda/security-plan-detail/logging-monitoring.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 27
    section: "## Logging & Monitoring"
    change: A failed load's error text in the log — blitz-net's error arm now hands `e.to_string()` to `NetHandler::error` (packages/blitz-net/src/lib.rs:313) and a stylesheet's handler responds `Err(error)` (packages/blitz-dom/src/net.rs:144-146), so a failed stylesheet's path ends on the standing `Error fetching` and `Resource load failed` lines, no log site added; `url` and `error` on `Resource load failed` are in escher-telemetry's scrub set, unmoved (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59), so escher's sink prints neither's value, and the error text stops in `ResourceLoadResponse` and that log line, no carrier takes it — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: Logging & Monitoring gains a failed load's error text on the standing log lines, its value scrubbed by the unmoved set and taken by no carrier.
    rationale: The take's `f6edb9701` gives a failed stylesheet a path to the log through `NetHandler::error` (report.md:14), and the report names its fields, the scrub set and the text's stopping point (report.md:31, report.md:103); this file holds the sink's rule and no row for that path.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31

## Held

- api-security#1.0
- api-security#2.6
- dependency-security#2.7
- dependency-security#6.3
- error-handling#6.1
- error-handling#7.1
- error-handling#8.1
- input-validation#1.31
- input-validation#1.95
- threat-model-summary#1.1
- threat-model-summary#2.1

## Cascade only

- input-validation#1.117
- input-validation#1.135
- input-validation#1.42
- input-validation#1.90
- logging-monitoring#1.1
