proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 97
    section: "## 1. Test Scope Summary"
    change: "after the `focus_within` / `focus_redraw` sentence, add one item: since 2026-10-24-upstream-sync-polish-and-ship (upstream #1244, `f102663b8`), upstream's `hit_test_dropped_nodes` 1, `hit_test_skips_dropped_nodes`: a hit test skips nodes dropped since the last resolve (tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82). It is its own auto-discovered target, not unix-gated, so the macOS and windows jobs run it too (CI#40092092468). The workspace `test` leg and `run all` run it, the `a11y` leg's six and `run stand` do not. It reads no env, prints nothing and installs no subscriber. It read red with `f102663b8`'s engine hunks to `node/node.rs` and `layout/paint_tree.rs` reversed, then green once they were restored. As measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "test-scope-summary: tests/blitz-tests gains upstream's hit_test_dropped_nodes 1 (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Files (new tests/blitz-tests/tests/hit_test_dropped_nodes.rs, from f102663b8), Counts (a new blitz-tests target, 1 test; every per-OS tally +1 line +1 passed, not unix-gated; run all 616 → 617; 0 detail-file hits), Coverage of new surfaces (hit test of dropped nodes → tests hit_test_dropped_nodes, red-before-green), Outcome (0 env · 0 print · 0 subscriber). The file lists every other upstream-merged blitz-tests check (lines 89-90, 96-97) and this one is missing."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:48
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 29
    section: "## 1. Test Scope Summary"
    change: "append to the blitz-dom (node) item: since 2026-10-24-upstream-sync-polish-and-ship (upstream #1238 `46d7cf993`, #1240 `e4ca5b9b5`), none of these has a unit test of its own: `Node::out_of_flow` with `Node::record_out_of_flow`, `style_is_out_of_flow`, and `ElementData::classes` with `parse_classes`. They are read through `incremental_oracle`, `paint_tree_incremental`, `paint_order`, the blitz-dom lib tests (106) and every stand check in both layout modes. As measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "test-scope-summary: blitz-dom (node) records the new out-of-flow flag and class atoms as having no unit test of their own, only integration readings (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Symbols adds the new pub fields Node::out_of_flow and ElementData::classes, plus the pub(crate) fns record_out_of_flow, style_is_out_of_flow and parse_classes. Coverage of new surfaces (Node::out_of_flow / ElementData::classes → tests: incremental_oracle, paint_tree_incremental, paint_order, the blitz-dom lib tests and every stand check in both layout modes) names no unit test, and blitz-dom lib reads 106, unmoved (Counts). The file states such gaps elsewhere in the same form, e.g. `layout_as_text_input` at line 21."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:101
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 22
    section: "## 1. Test Scope Summary"
    change: "after the blitz-dom covered behaviours item, add one item: since 2026-10-24-upstream-sync-polish-and-ship (upstream #1243, `fb297a38c`), blitz-dom's process-wide `PARALLEL_TRAVERSAL_LOCK` serialises every `StyleThreading::Parallel` traversal. No check reads it: no headless document (harness, stand or driver) sets `Parallel`, so its one local reading is the windowed boot smoke `RUST_LOG=info timeout 10 target/debug/seven_guis_native`, through `dioxus_native::launch`. As measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "test-scope-summary: blitz-dom records PARALLEL_TRAVERSAL_LOCK as read by the windowed smoke alone, no headless check (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Symbols adds static PARALLEL_TRAVERSAL_LOCK, taken around every Parallel traversal, and makes dioxus_native::launch use Parallel. It also says no headless document sets style_threading. Coverage of new surfaces (PARALLEL_TRAVERSAL_LOCK → tests: the windowed smoke (gate entry 35) the one local reading of a Parallel launch; no headless document sets Parallel) gives the gap this item records."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:100
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "after 'the other examples and the apps named still hold no tests', add: among them, since 2026-10-24-upstream-sync-polish-and-ship, upstream's root example `examples/multi_doc.rs` (`fb297a38c`), which has no test and is only built, by `cargo build --workspace` inside the `fast` leg's clippy. As measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "test-scope-summary: apps/examples absence names the new root example multi_doc (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Crates / modules (the root examples gain multi_doc) and Coverage of new surfaces (examples/multi_doc.rs → tests ✗, an upstream example; built by cargo build --workspace within fast's clippy). The file's absence record for the examples (line 19) still holds but does not name the new example."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:99
note: "D-tests-framework and D-tests-obs-harness find no drift (the harness, runner, scripts and CI files are unchanged, gate entry 6). The report names no check for the replaced element's natural height through `aspect-ratio` (compute_replaced_layout); the orchestrator may want that gap owned. The file's line moves in file:line citations (node.rs, element.rs, stylo.rs, wpt main.rs) are left to P2's citation sweep."
