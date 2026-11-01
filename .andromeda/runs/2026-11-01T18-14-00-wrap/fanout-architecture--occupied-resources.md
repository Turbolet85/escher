proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: "## Occupied Resources"
    change: In this line read «no served network port — no TCP or UDP port served» where it says «no network port — no TCP or UDP port», and open its parenthesis «beside it, three test-only binds, blitz-net's two on TCP loopback, stated below, and escher-driver's, since 2026-10-14-quality-gates — its unix-only unit test `client::imp::tests::an_answer_read_past_its_bound_is_a_timeout` binds a `UnixListener`…», where it now states one test-only bind; the rest of the line stands as written.
    sidecar: Network ports and listeners now reads three test-only binds, not one, and no served TCP or UDP port, since blitz-net's tests bind two on TCP loopback.
    rationale: The report names this line as changed, one to three (report.md:47), and states the two binds are TCP on loopback beside the Unix one, so the flat «no TCP or UDP port» no longer holds except as no served one (report.md:28); session rule 2026-10-15 owes the registration as test-only.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 17
    section: "## Occupied Resources"
    change: After this line add the record «- blitz-net's `#[cfg(test)] mod tests` holds two test-only TCP binds on loopback since 2026-11-01-upstream-sync-ui-graph, `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css`, each `std::net::TcpListener::bind("127.0.0.1:0")` — served by no binary and reached by no caller, so no crossing; no served port or listener is added (packages/blitz-net/src/lib.rs:526; packages/blitz-net/src/lib.rs:534; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md).»
    sidecar: Registered blitz-net's two test-only TCP loopback binds under Network ports and listeners, as no crossing.
    rationale: The report brings two new test-only binds and owes their registration in arch §Occupied Resources (report.md:28, report.md:93); the tests holding them are read from the new-text ranges 522-530 and 532-545 (report.md:288-289), and the detail exceeds what line 8 can take under the 2 000-char line limit.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 79
    section: "## Occupied Resources"
    change: After this line add the record «- `init_stylo_prefs` (`pub(crate)`) in blitz-dom's `document.rs`, process-wide since 2026-11-01-upstream-sync-ui-graph — behind a `Once`, `INIT`, it enables Stylo's prefs once in a process, before the user-agent sheet parses, called from `UserAgentStylesheet::parse` and `BaseDocument::new`, its body the Stylo 0.22 names; no env read, print or `tracing` site (packages/blitz-dom/src/document.rs:87-102; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»
    sidecar: Registered the `Once` behind `init_stylo_prefs` as new process-wide state in blitz-dom.
    rationale: The take adds a new process-wide guard (report.md:16, report.md:193-194), the sweep's `stylo-prefs` control fired on no master (report.md:59), and this category's process-wide group lists such statics, `PARALLEL_TRAVERSAL_LOCK` among them at line 78.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:16
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 110
    section: "## Occupied Resources"
    change: Replace this whole bullet with «- `Stand checks repeated` alone holds a `jlumbroso/free-disk-space` step — `matrix_test`'s, skipped on every matrix row, retired at 2026-11-01-upstream-sync-ui-graph, `test_only_stand_repeat_frees_disk` holding it there (.github/scripts/test_ci_workflows.py:110-115; as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»
    sidecar: The `matrix_test` free-disk step is retired; the line now states `Stand checks repeated` alone holds the action.
    rationale: The report's Retired bullet names the `matrix_test` free-disk step (report.md:33) and marks this line ×2 changed, removed, with line 104 no-change (report.md:43); the Harness bullet records the removal and the new pin (report.md:64).
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:43
note: D-arch-decisions fires nothing here (no dependency, runtime or locked decision moves, report.md:30); lines 9, 10 and 17 say no port of the served carriers and are left as true, and the body's `architecture.md:205`, `:96` and `:314` carry the same flat «no TCP or UDP» wording this file's line 8 loses, which is the body's to read since no proposal here may edit it.
