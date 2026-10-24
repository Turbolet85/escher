proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 34
    section: "## 6. Log Coverage"
    change: "Add one item to the **examples** group, after line 34: `- multi_doc` (since 2026-10-24-upstream-sync-polish-and-ship, upstream's root example taken from `fb297a38c`) writes its output with one `println!` on stdout. That is example output, with no `tracing` call, subscriber or sink, like the other examples' `println!` output above. Its one other ending is a `panic!` on an unknown mode (examples/multi_doc.rs:111-115; examples/multi_doc.rs:108; as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)"
    sidecar: "obs-plan §6 Log Coverage, examples emitter: the new root example multi_doc recorded, its one println! being example output on stdout with no tracing site (2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "The report's Changes → Symbols lists `examples/multi_doc.rs` (new) with 'one `println!` (111-115)' and 'a `panic!` on an unknown mode (108)'. Coverage of new surfaces says 'instrumentation n/a (its `println!` is example output on stdout)'. Expected amendments names 'obs-plan ... §6 (examples emitter)' as carried. The examples group (line 33) lists the example `println!` emitters by cite and does not yet name this one."
    basis: .andromeda/obs-plan-detail/log-coverage.md:33
note: "No other drift. The report adds no log site, span, subscriber or telemetry dependency (mimalloc is an allocator), and no new logging touches user data. Lines 21-23, 50-53, 66, 70-71 and 97 cite files the take moved (apps/browser document_loader.rs/toolbar.rs, blitz-dom resolve/mutator/construct/element/node, wpt/runner main.rs), so the P2 citation sweep has to re-point them. Its file list in Expected amendments omits apps/browser/src/{document_loader,toolbar}.rs; confirm the sweep's line map covers both. The silence census entry for hit_test_dropped_nodes belongs in the body's §3, not in this file."
