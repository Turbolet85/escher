proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 15
    section: "## 7. Error Capture & Reporting"
    change: "Add one row after the absence group: - blitz-net: a failed fetch reaches its handler since 2026-11-01-upstream-sync-ui-graph: the error arm calls `handler.error(url, e.to_string())` (packages/blitz-net/src/lib.rs:313), `NetHandler::error` being new on the public trait with a default that does nothing (packages/blitz-traits/src/net.rs:36-38), blitz-dom's stylesheet handler responding `Err(error)` (packages/blitz-dom/src/net.rs:144-146) and blitz-test-harness's `CountedHandler` forwarding it (packages/blitz-test-harness/src/settle.rs:121-125); no log site is added, the standing `Error fetching` and `Resource load failed` lines report it, `url` and `error` are in escher-telemetry's scrub set (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59), so the sink prints `waiting_nodes` and the message, and the error text stops in `ResourceLoadResponse` and that log line, no carrier taking it (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)."
    sidecar: "2026-11-01-upstream-sync-ui-graph: §7 detail gains the failed-fetch path, `NetHandler::error` forwarded to the handler, no log site added, its error text held to `ResourceLoadResponse` and the scrubbed log line."
    rationale: "The report's Symbols / APIs state the new error path (report.md:14, report.md:25) and its Schema / config bullet and Coverage row state its reporting and redaction (report.md:31, report.md:103); this category holds how errors are captured and reported, and its file holds no row for the path."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
note: "No D-obs-instrumentation, D-obs-stack, D-obs-pii or D-retired drift in this file: its six absence rows stand (report.md:30 no dependency, report.md:28 no tracing site added); outside this file's reach, obs-plan.md:220 still says blitz-net fetch errors are not propagated to the handler, which report.md:14 retires, so the body's §7 owes that correction."
