proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 16
    section: "## 5. Metric Coverage"
    change: "Add a table row after the `screenshot` example row: `| \\`multi_doc\\` example | Style-resolve timings taken by its \\`timed_resolve\\` closure with \\`Instant::now()\\`, over documents sized by \\`MULTI_DOC_N\\`, its style threading chosen by \\`MULTI_DOC_SEQ\\` and its mode by its first argument | printed (one \\`println!\\`, stdout) | (examples/multi_doc.rs:61; examples/multi_doc.rs:64; examples/multi_doc.rs:63-67; examples/multi_doc.rs:111-115) |`"
    sidecar: "§5 Metric Coverage: the new root example `multi_doc` (upstream fb297a38c) registered as a printed in-process timing emitter beside `paint_bench` and `screenshot`."
    rationale: "The report's Changes → Files and Symbols add `examples/multi_doc.rs` (116 lines, from `fb297a38c`) with two `Instant::now()` (61, 64), the `timed_resolve` closure (New text 63-67) and one `println!` (111-115); Coverage of new surfaces calls that `println!` example output on stdout. The table records every example's printed timings (`paint_bench`, `screenshot`), so the new timing emitter is new detail this category holds; it adds no metric library, exporter or log site, so the absence block stands."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:24
note: "The WPT runner rows (wpt/runner/src/main.rs:375, 498-518, 624, 694, 783-829, 837-851, after main.rs added 1-3) and the FPS overlay row's apps/browser/src/toolbar.rs:436-439 (after toolbar.rs added 3, 203) are line re-points owned by P2's citation sweep per the report's Expected amendments, so no proposal here; the body's §5 broad line (obs-plan.md:139, \"the bench and screenshot examples\") may name `multi_doc` too, which is the body's own edit."
