proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 10
    section: "## 5. Integration Test Strategy"
    change: "after line 10 add a new `<!-- detail: procedure · integration -->` group holding one record — `- **DOM mutation ↔ hit testing:** since 2026-10-24-upstream-sync-polish-and-ship upstream's `hit_test_dropped_nodes` (1 test, its own blitz-tests target, not unix-gated) builds an `HtmlDocument` from HTML, drops a node and hits where it stood before the next resolve, the hit skipping the dropped node (`Node::hit_inner` and `paint_tree.rs` reading `tree().get(id)` with `continue`); read red with `f102663b8`'s engine hunks to `node/node.rs` and `layout/paint_tree.rs` reversed (exit 101, a panic at `node.rs:1572:26`) and green restored (tests/blitz-tests/tests/hit_test_dropped_nodes.rs:11-22; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md`, followed by `<!-- /detail -->`"
    sidecar: "integration-test-strategy: + DOM mutation ↔ hit testing — upstream's hit_test_dropped_nodes (1), its red-before-green control (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "The report's Coverage of new surfaces names the hit test of dropped nodes as covered by `hit_test_dropped_nodes` at the integration tier, a new blitz-tests target (Changes → Counts: 144 → 145 files, +1 passed), and Deviations records its red-before-green control; no record of this boundary stands in the category file, which holds no engine hit-test boundary."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:102
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 28
    section: "## 5. Integration Test Strategy"
    change: "after line 28, inside the same absence group, add — `- A check of a `StyleThreading::Parallel` style traversal (the process-wide `PARALLEL_TRAVERSAL_LOCK`, two documents resolving `Parallel` serialised) · searched: `style_threading` by `git grep` at 2026-10-24-upstream-sync-polish-and-ship, 7 hits, all in blitz-dom — no headless document (harness, stand or driver) sets it; the windowed `seven_guis_native` boot smoke (`RUST_LOG=info timeout 10`, exit 124) is the one local reading of a `Parallel` launch, and upstream's `examples/multi_doc.rs` is a manual example, no test — the reading at that search`"
    sidecar: "integration-test-strategy: + absence — no check of a Parallel style traversal under PARALLEL_TRAVERSAL_LOCK; the windowed smoke the one reading (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "The report's Coverage of new surfaces states for `PARALLEL_TRAVERSAL_LOCK`: tests — the windowed smoke (gate entry 35) the one local reading of a `Parallel` launch, no headless document sets `Parallel`; and for `examples/multi_doc.rs` tests ✗. Changes → Symbols states the sweep (7 `git grep` hits, all in blitz-dom). A new cross-document boundary with no integration check is new absence detail in this category's observed-absent shape."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:100
note: D-tests-framework and D-tests-obs-harness read clean (hit_test_dropped_nodes runs under the standard harness as its own target; no script, harness or log format changed); test-plan body §5 Boundaries covered (test-plan.md:242-265) holds no broad line for the two proposals' boundaries — the orchestrator may want a matching body line.
