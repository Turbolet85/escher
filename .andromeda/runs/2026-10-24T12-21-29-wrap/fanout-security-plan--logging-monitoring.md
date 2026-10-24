proposals:
  - detector: D-security-input
    severity: warning
    file: .andromeda/security-plan-detail/logging-monitoring.md
    near: 15
    section: "## Logging & Monitoring"
    change: "In the Stdout output item, add the new root example to the cites of 'examples print through `println!` and `eprintln!` only': `examples/multi_doc.rs:111-115` — its one `println!`, the example's timing output on stdout, no log site (since 2026-10-24-upstream-sync-polish-and-ship, upstream's `fb297a38c`, taken by cherry-pick). Leave the 'logging framework observed absent' search as it stands: it covers the 32 s03 files, and multi_doc.rs is not one of them."
    sidecar: "2026-10-24-upstream-sync-polish-and-ship: Logging & Monitoring › Stdout output adds examples/multi_doc.rs:111-115 (the new root example's one println!, example output on stdout)."
    rationale: "The report's Changes → Symbols names `examples/multi_doc.rs` as a new root example with one `println!` (111-115). Its Coverage of new surfaces gives instrumentation as n/a because 'its `println!` is example output on stdout'. The Outcome lists that `println!` for registration (`evidence/sink.md`). The Stdout output item lists every example's stdout print site by cite, and this one is missing. No other line of this file is moved: the take changes no escher-telemetry, escher-driver or seven_guis file (gate entry 6), and none of this file's cites points into a file the take moved."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24
note: "D-security-auth and D-security-deps found nothing in this category: no auth, secret or log-sink change, and mimalloc and tokio are not logging matters. The new println! is filed under D-security-input, the nearest detector, because multi_doc.rs is the chunk's one new input surface; it is new detail, not a validation gap. The body's cites into apps/browser/src/document_loader.rs and wpt/runner/src/main.rs (security-plan.md:381, :393) moved with the take; the citation sweep owns those digits."
