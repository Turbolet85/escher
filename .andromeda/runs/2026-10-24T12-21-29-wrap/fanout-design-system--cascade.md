proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 26
    section: "## Depth Strategy"
    change: "append to the line, after the 2026-10-14-stand-requirement-sweep clause: «; since 2026-10-24-upstream-sync-polish-and-ship (upstream #1244, `f102663b8`) each of those walks — the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup — skips a node dropped since the last resolve (`self.tree().get(id)` with `continue` in place of `self.with(id)`), as the paint tree's walk does, `hit_inner`, `hit` and `hit_with_scrollbar` keeping their signatures — an engine change for every document, ratified by the founder (packages/blitz-dom/src/node/node.rs:1414-1417; packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1646-1650; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82; as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md)»"
    sidecar: "depth-strategy: hit-test walk order line gains the dropped-node skip (upstream #1244, taken 2026-10-24-upstream-sync-polish-and-ship)"
    rationale: "report Changes → Symbols, the Hit testing bullet: `Node::hit_inner` skips nodes dropped since the last resolve in the positive-z hoisted loop, the paint-children loop, the negative-z hoisted loop and the text-cluster lookup, `paint_tree.rs` the same; Changes → Symbols, three engine changes for every document ratified by the founder's word (inputs#I4); Coverage: `hit_test_dropped_nodes` red with `f102663b8`'s hunks reversed, green restored"
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:22
note: typography#1.14 cites construct.rs:1106-1112 and `build_inline_layout_into` moved from construct.rs to layout/inline/build.rs (report :21) — the citation sweep should confirm that cite still lands in construct.rs rather than across the move; D-design-tokens fired on nothing (every Coverage `tokens` flag reads n/a).
