proposals:
  - detector: D-new-detail
    severity: warning
    fact: F6
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 15
    section: "## 7. Error Capture & Reporting"
    change: "- blitz-dom: a failed stylesheet is captured as a typed answer: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, `pending_critical_resources` loses the request, and a failed, empty or non-CSS stylesheet stops blocking render (packages/blitz-dom/src/net.rs:144-146)"
    sidecar: "obs-plan error-capture-reporting: new record, a failed stylesheet answers `Err(error)` and stops blocking render (2026-11-01-upstream-sync-ui-graph)."
    rationale: "The report's Changes state the handler's `Err(error)`, the lost pending request and the stopped block for a failed, empty or non-CSS stylesheet (report.md:14); the file holds no error-class record yet, only the absence block."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    fact: F7
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 15
    section: "## 7. Error Capture & Reporting"
    change: "- blitz-traits: `NetHandler::error` is a public trait method with a default that does nothing, so a handler without an override drops the error; blitz-test-harness's `CountedHandler` forwards it (packages/blitz-traits/src/net.rs:36-38; packages/blitz-test-harness/src/settle.rs:121-125)"
    sidecar: "obs-plan error-capture-reporting: new record, `NetHandler::error`'s do-nothing default drops an error unless overridden, and `CountedHandler` forwards it (2026-11-01-upstream-sync-ui-graph)."
    rationale: "The report's Changes state the public method and its do-nothing default (report.md:14) and `CountedHandler`'s forward (report.md:25); its Decisions name the default hiding the fix through that wrapper (report.md:122)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
note: "F6 and F7 both follow line 15, F6 first, outside the absence block that line 15 closes."
