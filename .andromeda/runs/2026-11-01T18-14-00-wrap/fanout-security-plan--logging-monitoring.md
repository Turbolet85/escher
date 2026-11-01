proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 27
    section: "## Logging & Monitoring"
    change: A failed load's error text in the log — blitz-net's error arm now hands `e.to_string()` to `NetHandler::error` (packages/blitz-net/src/lib.rs:313) and a stylesheet's handler responds `Err(error)` (packages/blitz-dom/src/net.rs:144-146), so a failed stylesheet's path ends on the standing `Error fetching` and `Resource load failed` lines, no log site added; `url` and `error` on `Resource load failed` are in escher-telemetry's scrub set, unmoved (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59), so escher's sink prints neither's value, and the error text stops in `ResourceLoadResponse` and that log line, no carrier takes it — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
    sidecar: Logging & Monitoring gains a failed load's error text on the standing log lines, its value scrubbed by the unmoved set and taken by no carrier.
    rationale: The take's `f6edb9701` gives a failed stylesheet a path to the log through `NetHandler::error` (report.md:14), and the report names its fields, the scrub set and the text's stopping point (report.md:31, report.md:103); this file holds the sink's rule and no row for that path.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31
note: report.md:31 says the sink prints `waiting_nodes` and the message where this file's line 8 has an engine target's message `[redacted]`, so the proposal says nothing of the message and the conflict is the orchestrator's to read; no D-security-input, D-security-auth, D-security-deps or D-retired hit stands in this file.
