# Cascade compare — test-plan · wrap · 2026-10-24-upstream-sync-polish-and-ship
E items 10 · held 6 · lost at classify 3 · lost at strong 1 · cascade only 0 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### integration-test-strategy#1.1 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:10
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 10
    section: "## 5. Integration Test Strategy"
    change: "after line 10 add a new `<!-- detail: procedure · integration -->` group holding one record — `- **DOM mutation ↔ hit testing:** since 2026-10-24-upstream-sync-polish-and-ship upstream's `hit_test_dropped_nodes` (1 test, its own blitz-tests target, not unix-gated) builds an `HtmlDocument` from HTML, drops a node and hits where it stood before the next resolve, the hit skipping the dropped node (`Node::hit_inner` and `paint_tree.rs` reading `tree().get(id)` with `continue`); read red with `f102663b8`'s engine hunks to `node/node.rs` and `layout/paint_tree.rs` reversed (exit 101, a panic at `node.rs:1572:26`) and green restored (tests/blitz-tests/tests/hit_test_dropped_nodes.rs:11-22; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md`, followed by `<!-- /detail -->`"
    sidecar: "integration-test-strategy: + DOM mutation ↔ hit testing — upstream's hit_test_dropped_nodes (1), its red-before-green control (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "The report's Coverage of new surfaces names the hit test of dropped nodes as covered by `hit_test_dropped_nodes` at the integration tier, a new blitz-tests target (Changes → Counts: 144 → 145 files, +1 passed), and Deviations records its red-before-green control; no record of this boundary stands in the category file, which holds no engine hit-test boundary."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:102

### integration-test-strategy#3.1 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:28
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 28
    section: "## 5. Integration Test Strategy"
    change: "after line 28, inside the same absence group, add — `- A check of a `StyleThreading::Parallel` style traversal (the process-wide `PARALLEL_TRAVERSAL_LOCK`, two documents resolving `Parallel` serialised) · searched: `style_threading` by `git grep` at 2026-10-24-upstream-sync-polish-and-ship, 7 hits, all in blitz-dom — no headless document (harness, stand or driver) sets it; the windowed `seven_guis_native` boot smoke (`RUST_LOG=info timeout 10`, exit 124) is the one local reading of a `Parallel` launch, and upstream's `examples/multi_doc.rs` is a manual example, no test — the reading at that search`"
    sidecar: "integration-test-strategy: + absence — no check of a Parallel style traversal under PARALLEL_TRAVERSAL_LOCK; the windowed smoke the one reading (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "The report's Coverage of new surfaces states for `PARALLEL_TRAVERSAL_LOCK`: tests — the windowed smoke (gate entry 35) the one local reading of a `Parallel` launch, no headless document sets `Parallel`; and for `examples/multi_doc.rs` tests ✗. Changes → Symbols states the sweep (7 `git grep` hits, all in blitz-dom). A new cross-document boundary with no integration check is new absence detail in this category's observed-absent shape."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:100

### quality-gates-coverage-targets#2.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:14
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 14
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Append to the Coverage floors item (after its 2026-10-17 record, before the `*_tests.rs` sentence) one dated re-read: \"Re-read at 2026-10-24-upstream-sync-polish-and-ship: upstream retires `keyboard_compat.rs` at `286874cc1` (Dioxus 0.8.0-alpha.3), a commit that take leaves out, so the file stays in the tree and both `--ignore-filename-regex 'keyboard_compat\\.rs$'` lines stand, the three floors and `coverage_floor.py` unchanged; the exclusion lifts when escher takes that move, owned by the `.andromeda/residuals.md` line of that chunk's left-out upstream delta (target: next) (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md, its gate entries 14 and 15).\""
    sidecar: "test-plan §10 detail — Coverage floors: the keyboard_compat exclusion re-read at the 2026-10-24 sync; upstream retires the file at 286874cc1 (left out), the exclusion stands, owned by the next-version residuals line."
    rationale: "The report's gate entries 14 (`git ls-tree … 286874cc1 -- …/keyboard_compat.rs | wc -l` → 0) and 15 (the file present, the exclusion read twice in ci-leg.sh) measure that upstream has now retired the bridge in a commit this take leaves out (P5 card row 1, approved inputs#I5: «CARRY 2's `keyboard_compat.rs`, retired upstream there»), while the file's 'until upstream retires it' condition reads as if upstream had not; the exclusion stands (Expected amendments: kbd-compat no-change), so the new detail is a dated record, not a rewrite of the 2026-10-17 record."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:169

### test-scope-summary#3.2 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:29
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 29
    section: "## 1. Test Scope Summary"
    change: "append to the blitz-dom (node) item: since 2026-10-24-upstream-sync-polish-and-ship (upstream #1238 `46d7cf993`, #1240 `e4ca5b9b5`), none of these has a unit test of its own: `Node::out_of_flow` with `Node::record_out_of_flow`, `style_is_out_of_flow`, and `ElementData::classes` with `parse_classes`. They are read through `incremental_oracle`, `paint_tree_incremental`, `paint_order`, the blitz-dom lib tests (106) and every stand check in both layout modes. As measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "test-scope-summary: blitz-dom (node) records the new out-of-flow flag and class atoms as having no unit test of their own, only integration readings (2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "Report Changes → Symbols adds the new pub fields Node::out_of_flow and ElementData::classes, plus the pub(crate) fns record_out_of_flow, style_is_out_of_flow and parse_classes. Coverage of new surfaces (Node::out_of_flow / ElementData::classes → tests: incremental_oracle, paint_tree_incremental, paint_order, the blitz-dom lib tests and every stand check in both layout modes) names no unit test, and blitz-dom lib reads 106, unmoved (Counts). The file states such gaps elsewhere in the same form, e.g. `layout_as_text_input` at line 21."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:101

## Held

- ci-integration#5.1
- ci-integration#6.1
- test-scope-summary#11.1
- test-scope-summary#2.1
- test-scope-summary#2.3
- test-strategy#7.1

## Cascade only
