proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "Before «the other examples and the apps named still hold no tests», add: since 2026-10-24-upstream-sync-polish-and-ship the root examples gain upstream's `examples/multi_doc.rs` (116 lines, from `fb297a38c`), which holds no test — it is built by `cargo build --workspace` within `fast`'s clippy and read by no check, as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "test-scope-summary#2.1: the new root example `multi_doc` recorded as holding no test (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Crates / modules: «The root examples gain `multi_doc`»; Coverage of new surfaces: `examples/multi_doc.rs` → «tests ✗ (an upstream example; built by `cargo build --workspace` within `fast`'s clippy)». The entity enumerates the examples' test holdings and names the untested rest; the new example joins them unrecorded."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:19
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: "## 1. Test Scope Summary"
    change: "Append to the covered behaviours: since 2026-10-24-upstream-sync-polish-and-ship (upstream #1244, `f102663b8`) a hit test skipping nodes dropped since the last resolve (`Node::hit_inner` and the paint tree), by blitz-tests' `hit_test_dropped_nodes` 1 (`hit_test_skips_dropped_nodes`, tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82), upstream's text as written, not unix-gated — red with `f102663b8`'s engine hunks reversed (exit 101, a panic in `node.rs`) and green restored; its own auto-discovered target, run by the workspace `test` leg, `run hit_test_dropped_nodes` and `run all`, not by `run stand`"
    sidecar: "test-scope-summary#2.3: hit testing past dropped nodes recorded as covered by the new target `hit_test_dropped_nodes` (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Symbols (hit testing skips nodes dropped since the last resolve, an engine change for every document); Changes → Counts: `hit_test_dropped_nodes` a new blitz-tests target, 1 test, `run all` 616 → 617, `run stand` unmoved, every per-OS row +1 (not unix-gated); Coverage of new surfaces: «tests `hit_test_dropped_nodes` (integ; red with `f102663b8`'s engine hunks reversed, green restored)»; Deviations: «exit 101, panic at `node.rs:1572:26`»; New text, by line: hit_test_dropped_nodes.rs 32-82 «fn hit_test_skips_dropped_nodes() {»."
    basis: tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: "## 1. Test Scope Summary"
    change: "Append: the process-wide `PARALLEL_TRAVERSAL_LOCK` (stylo.rs, since 2026-10-24-upstream-sync-polish-and-ship), serialising every `StyleThreading::Parallel` traversal, is asserted by no check — no headless document (harness, stand or driver) sets `Parallel`, and the windowed smoke of `seven_guis_native` under `dioxus_native::launch` is its one local reading"
    sidecar: "test-scope-summary#2.3: the `Parallel` traversal lock recorded as asserted by no check, the windowed smoke its one reading (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Coverage of new surfaces: `PARALLEL_TRAVERSAL_LOCK` → «tests: the windowed smoke (gate entry 35) the one local reading of a `Parallel` launch; no headless document sets `Parallel`»; Changes → Symbols: «No headless document — harness, stand or driver — sets `style_threading`». A new engine path with no check at the mandated tier, stated in the entity's own gap form (as the text extent's and the select's limits are)."
    basis: packages/blitz-dom/src/stylo.rs:157
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "In the broad line, after «the single-choice `select` model», add «and hit testing past dropped nodes»; after «the select's stated limits are asserted by no check», add «, nor is the process-wide `Parallel` traversal lock»"
    sidecar: "test-plan §1: the broad blitz-dom covered-behaviours line names hit testing past dropped nodes and the unchecked `Parallel` lock (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "The broad line test-plan.md:19 summarises test-scope-summary#2.3; the two detail additions above (report Coverage of new surfaces: `hit_test_dropped_nodes`; `PARALLEL_TRAVERSAL_LOCK` read by no headless document) would leave it short."
    basis: .andromeda/test-plan.md:19
    dependent-of: D-cascade
note: the take's engine change to a replaced element's natural height through `aspect-ratio` (`compute_replaced_layout`) has no row in the report's Coverage of new surfaces, so no test is named for it — not proposed here; the orchestrator may want the body agent or the operator to settle whether a check covers it.
