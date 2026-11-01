proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 72
    section: "## 6. Log Coverage"
    change: "  - warn: the resource-load-failed line's reach, since the upstream take read at `e42ee1f25`: a stylesheet's failed load now reaches it, `ResourceHandler<StylesheetHandler>::error` responding `Err(error)` to `NetHandler::error`; no site added, the line standing; at escher's sink it prints `waiting_nodes` and the message, `url` and `error` being in the scrub set, and the error text stops in `ResourceLoadResponse` and that line, no carrier taking it (packages/blitz-dom/src/net.rs:144-146; packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md)"
    sidecar: "Log Coverage: a failed stylesheet now reaches the standing resource-load-failed warn line through NetHandler::error, url and error scrubbed at escher's sink, no site added."
    rationale: "The report's Symbols entry for f6edb9701, its Schema / config bullet and its Coverage row for NetHandler::error name this path through the standing line and its redaction; the file's line 72 lists the site without this reach (report.md:14, report.md:31, report.md:103)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 98
    section: "## 6. Log Coverage"
    change: "  - since the upstream take read at `e42ee1f25`, the \"Error fetching\" arm of `NetProvider::fetch` also hands the failure to the handler, `handler.error(url, e.to_string())`, the waker moved after the handler; no site added, the error line standing (packages/blitz-net/src/lib.rs:313; packages/blitz-net/src/lib.rs:316; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)"
    sidecar: "Log Coverage: blitz-net's Error fetching arm now forwards the failure through NetHandler::error after its standing log line, no site added."
    rationale: "The report states the error arm's new call and the waker's move, and its Coverage row keeps `Error fetching` as the path's standing instrumentation (report.md:14, report.md:103)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 136
    section: "## 6. Log Coverage"
    change: "- observed absent — an env read, print, `tracing` site or wall-clock read added by the upstream take read at `e42ee1f25` and escher's own changes beside it (the twelve commits, `deallocate_anonymous_block`'s fix, `Harness::text_contrast`'s `paints_more_than_colour`, `CountedHandler`'s forward of `NetHandler::error`, `URLSearchParams` and its native `parse_search_params`, blitz-paint's `blend` module) · searched: the census in `evidence/sink.md`, its 2 hits named there, and the gates' `Instant::now|SystemTime::now` census and silence census over the three checks, each exit 1, last line 0; a listener is not among the absent, blitz-net's `#[cfg(test)]` module holding 2 test-only loopback binds, served by no binary; `packages/escher-driver`, `packages/escher-telemetry` and `examples/seven_guis` unchanged, escher-driver holding 2 `info_span!` sites, `stand_act_spans` its eight fields, and `host_log` and `mcp_log` reading 0 needles at `trace` under both builds (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md and its evidence/sink.md)"
    sidecar: "Log Coverage: the silence reading for the upstream take at e42ee1f25 stands beside the last take's, which stays as history; two test-only loopback binds are named and no listener is claimed absent."
    rationale: "The report keeps log-coverage.md:136 as history and owes a new sentence for this take beside it, and it states no site added, the censuses, the untouched crates, the span counts and the needle reads (report.md:35, report.md:28, report.md:8, report.md:31, report.md:145, report.md:155)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
note: Not this file's to edit - obs-plan.md:220 says blitz-net fetch errors are not propagated to the handler, false since f6edb9701 (report.md:14), and obs-plan.md:99 owes this take's silence line by replacement (report.md:98); D-obs-instrumentation, D-obs-stack, D-obs-pii and D-retired find no violation in log-coverage.md.
