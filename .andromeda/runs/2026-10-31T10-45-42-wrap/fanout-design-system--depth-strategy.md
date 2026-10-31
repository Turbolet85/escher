proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 26
    section: "## Depth Strategy"
    change: "Add one list item after line 26, in the same detail group: - Hit-testing answers `None` for the whole subtree of a node whose transform has a determinant of 0 or not finite, so a box under a non-invertible transform takes no hit, and its invertible twin is hit where it shows, in both layout modes (ratified by the founder, 2026-10-31) (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; tests/blitz-tests/tests/hit_non_invertible_transform.rs:58-71; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "2026-10-31-engine-debts: Depth Strategy gains the hit-test item for a non-invertible transform, F2: a determinant of 0 or not finite answers `None` for the whole subtree."
    rationale: "The report's Symbols / APIs F2 states the new `Node::hit_inner` answer, its Expected amendments carries design-system §Depth Strategy for it as new text, and the `non-invertible` control is unfired, so no master states it; line 26 holds the hit-test walk this item joins, and none of its text is retired."
    basis: packages/blitz-dom/src/node/node.rs:1472-1477
