proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 56
    section: "## Error Handling"
    change: "- Failed load answered through the handler: `NetHandler::error` is new on the public trait, a default that does nothing (packages/blitz-traits/src/net.rs:36-38), so a wrapping implementer that does not forward it compiles and drops the failure silently; `ResourceHandler<StylesheetHandler>::error` answers `Err(error)` (packages/blitz-dom/src/net.rs:144-146), so `pending_critical_resources` loses the request and a failed, empty or non-CSS stylesheet stops blocking render — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "security-plan §Error Handling gains a group on a failed load: the do-nothing default of `NetHandler::error`, what a wrapper not forwarding it loses, and the stylesheet answer that stops blocking render."
    rationale: "report.md:14 states the trait default, the stylesheet answer and the render no longer blocked; report.md:122 states the silent drop through a wrapper. No standing group of this file holds a failed load, so it is a new group under the file's one origin."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
    fact: F9
