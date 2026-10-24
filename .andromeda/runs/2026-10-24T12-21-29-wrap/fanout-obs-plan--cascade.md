proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 16
    section: "## 5. Metric Coverage"
    change: "Add a row after the `screenshot` example row: | `multi_doc` example | Resolve timings, each read with `Instant::now()` (one read before the closure, one inside `timed_resolve`), in the mode and over the iteration count it reads from argv, with `MULTI_DOC_N` · `MULTI_DOC_SEQ` · `MULTI_DOC_FLAT` read from the environment | printed, one `println!` | (examples/multi_doc.rs:47-58; examples/multi_doc.rs:61; examples/multi_doc.rs:63-67; examples/multi_doc.rs:64; examples/multi_doc.rs:111-115) |"
    sidecar: "metric-coverage: the new root example `multi_doc` (upstream fb297a38c) added as a printed in-process timing row beside paint_bench and screenshot"
    rationale: "Report Changes → Symbols: `examples/multi_doc.rs` (new root example) has two `Instant::now()` (61, 64), one `println!` (111-115), and reads argv (48-54) and three env variables (`MULTI_DOC_N` 55-58, `MULTI_DOC_SEQ` 36-40, `MULTI_DOC_FLAT` 22). New text, by line lists `timed_resolve` at 63-67. Crates / modules: the root examples gain `multi_doc`. The table lists every example's printed timings but not this one."
    basis: .andromeda/obs-plan-detail/metric-coverage.md:16
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 33
    section: "## 6. Log Coverage"
    change: "In the examples emitter's `Example output uses println!/eprintln!` citation list, add examples/multi_doc.rs:111-115 after examples/paint_bench.rs:321, so it reads (examples/screenshot.rs:203; examples/paint_bench.rs:34; examples/paint_bench.rs:321; examples/multi_doc.rs:111-115). The rest of the line stays as it is."
    sidecar: "log-coverage: the new root example `multi_doc`'s one `println!` (example output on stdout) cited among the examples' println!/eprintln! output"
    rationale: "Report Changes → Symbols: `examples/multi_doc.rs` has one `println!` (111-115). Coverage of new surfaces: instrumentation n/a, because its `println!` is example output on stdout. That is the class this entity's line records for the other examples."
    basis: .andromeda/obs-plan-detail/log-coverage.md:33
note: "Three things for the orchestrator. (1) The report's citation-sweep file list omits apps/browser/src/{document_loader,toolbar}.rs, which gained lines 3 and 62 / 3 and 203. log-coverage#3.1's cites (document_loader.rs:120, 151, 235, 241-243; toolbar.rs:130), metric-coverage#1.4's toolbar.rs:436-439 and body §8's document_loader.rs:121 · toolbar.rs:130 may have moved; no deletion count is reported, so I propose no digits. (2) Body §5 line 139 ('the bench and screenshot examples') is the body agent's to widen for multi_doc. (3) The detectors read clean: no hot-path instrumentation is owed (lock and engine state have no log site, per evidence/sink.md), there is no off-spec logger (mimalloc is an allocator), and no PII is logged."
