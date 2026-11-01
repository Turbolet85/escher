proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §7 Error Capture & Reporting
    change: In the blitz-net line under Error classes captured, replace «logged and not propagated to the handler» with «logged and handed to the handler through `NetHandler::error`, a default that does nothing unless the handler overrides it; a stylesheet's handler responds `Err(error)`, so the failed load leaves `pending_critical_resources`, the error text stopping in `ResourceLoadResponse` and the `Resource load failed` line»; the `ProviderError` clause stands.
    sidecar: §7 blitz-net fetch errors now reach the handler through `NetHandler::error` (upstream `f6edb9701`), no longer logged only; no log site added.
    rationale: The report's Symbols / APIs bullet for `f6edb9701` has blitz-net's error arm call `handler.error(url, e.to_string())` at `blitz-net/src/lib.rs:313`, and its Coverage row names the standing `Error fetching` and `Resource load failed` lines as this path's instrumentation; the instrumentation holds, while §7's statement of where the error goes is now false. The report's `net-error` sweep did not list this line.
    basis: packages/blitz-net/src/lib.rs:313
  - detector: D-new-detail
    severity: warning
    section: §3 Observability Harness Contract
    file: new
    near: obs-plan.md:99
    change: New bullet beside the line for the take read at `ebacd68bd`, reading «The upstream take read at `e42ee1f25` is silent — its twelve commits and the fork's own `deallocate_anonymous_block` fix add no env read, print, `tracing` site or wall-clock read; the only binds added are test-only, two loopback `TcpListener` binds in blitz-net's `#[cfg(test)] mod tests` (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534), served by no binary and reached by no caller; `URLSearchParams` holds no log site and no carrier; a failed load reaches only the standing `Error fetching` and `Resource load failed` lines, `url` and `error` in escher-telemetry's scrub set, so the sink prints `waiting_nodes` and the message; `packages/escher-telemetry/` and `packages/escher-driver/` are unchanged, escher-driver holding 2 `info_span!` sites and `stand_act_spans` its eight fields, and `host_log` and `mcp_log` read 0 needles at `trace` under both builds (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).»
    sidecar: §3 gains the silence line for the upstream take read at `e42ee1f25`, beside the one for `ebacd68bd`.
    rationale: The report's Expected amendments owe this take's silence line at `obs-plan.md:99`, and §3 has no category file, so the new detail is this detector's; every fact is the report's own, from Symbols / APIs (no env read, print, `tracing` site or wall-clock read added; the two test-only binds), Schema / config (the scrub set, `info_span!`, `stand_act_spans`), Coverage of new surfaces and the (obs) acceptance criterion.
    basis: packages/blitz-net/src/lib.rs:465-546
note: The plan's amendment says this silence line replaces `obs-plan.md:99`, while the report's Counts bullet reads that line as history, no-change, with a new sentence beside it; the proposal adds beside and retires nothing, so the orchestrator may rule otherwise. D-obs-stack, D-obs-pii and D-retired found no drift. The `evidence/sink.md` path in the new line follows the form of `obs-plan.md:99` and was not opened.
