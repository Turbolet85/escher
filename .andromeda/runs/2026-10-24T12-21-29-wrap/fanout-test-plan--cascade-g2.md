proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 97
    section: "## 1. Test Scope Summary"
    change: "after the `focus_within` 7 / `focus_redraw` 4 clause add: upstream's `hit_test_dropped_nodes` 1 (#1244, `f102663b8`), taken at 2026-10-24-upstream-sync-polish-and-ship, byte-identical to upstream's file: a hit test on a parsed page skips nodes dropped since the last resolve, so a hit does not answer a removed node (tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82); it read red with `f102663b8`'s `node/node.rs` and `layout/paint_tree.rs` hunks reversed (a panic in `Node::hit_inner`) and green again once they were restored; it is its own auto-discovered target, run by the workspace `test` leg (206 · 1099 · 0 · 12) and `run all` (617 · 0 · 12), and not by the `a11y` leg's six or `run stand` — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md"
    sidecar: "test-scope-summary: blitz-tests scope names upstream's `hit_test_dropped_nodes` (1), taken at 2026-10-24-upstream-sync-polish-and-ship"
    rationale: "report Changes → Files lists the new `tests/blitz-tests/tests/hit_test_dropped_nodes.rs` (82 lines, from `f102663b8`); Counts names it as a new blitz-tests target with 1 test (`tests/*.rs` 144 → 145, `run all` 616 → 617, every per-OS tally +1); Coverage of new surfaces and Deviations record its red-before-green control. The scope entity lists every check taken from upstream (the 2026-10-10 and 2026-10-13 merges, `focus_within`/`focus_redraw` at 2026-10-19), but not this one"
    basis: tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82
note: test-scope-summary#12.1's `wpt/runner/src/main.rs:854-914` cite moves by the three lines added at main.rs 1-3 (the mimalloc global allocator); that re-point belongs to P2's citation sweep, so this file proposes no change for it.
