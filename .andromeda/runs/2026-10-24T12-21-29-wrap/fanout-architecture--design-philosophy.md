proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 53
    section: "## Design Philosophy"
    change: "In the line's first clause, replace \"`Node` carries only tree-structure information, other fields living on element data (packages/blitz-dom/src/node/element.rs:84-87)\" with \"`Node` is described as carrying only tree-structure information, other fields living on element data (packages/blitz-dom/src/node/element.rs:84-87), yet since the 2026-10-24 upstream sync it also holds one style-derived flag, `Node::out_of_flow`, an `AtomicBool` the style traversal records from the node's new primary style through `Node::record_out_of_flow` (packages/blitz-dom/src/node/node.rs:125-128; packages/blitz-dom/src/node/node.rs:1271-1279), read by `LayoutPassState::is_out_of_flow` alone (packages/blitz-dom/src/layout/mod.rs:856-858) while `Node::is_out_of_flow` and its other callers keep reading the style\"; the rest of the line unchanged."
    sidecar: "Design Philosophy › One incremental pipeline: `Node` no longer carries tree structure only — the recorded `Node::out_of_flow` flag (upstream #1238, taken 2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `Node::out_of_flow: AtomicBool` is a new pub field on `Node`, written by the style traversal through the new `Node::record_out_of_flow`, and only `LayoutPassState::is_out_of_flow` now reads it; the line's standing claim that `Node` carries only tree-structure information is contradicted. Expected amendments name this site (§Design Philosophy :53, `Node::out_of_flow`)."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:16
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 53
    section: "## Design Philosophy"
    change: "New sub-bullet after line 53: \"  - `ElementData::classes` holds the `class` attribute as a `ThinVec<Atom>`, parsed by `parse_classes` and filled by `ElementData::new` and `Clone` (packages/blitz-dom/src/node/element.rs:45-47; packages/blitz-dom/src/node/element.rs:56-61; packages/blitz-dom/src/node/element.rs:422-426), kept in sync by `DocumentMutator::set_attribute`'s `class` arm and `remove_attribute` (packages/blitz-dom/src/mutator.rs:375-378; packages/blitz-dom/src/mutator.rs:499-502); Stylo's `has_class` and `each_class` read those atoms (packages/blitz-dom/src/stylo.rs:586-590; packages/blitz-dom/src/stylo.rs:697-699; packages/blitz-dom/src/stylo.rs:710)\""
    sidecar: "Design Philosophy › One incremental pipeline: new sub-bullet — class atoms on `ElementData`, synced by the mutator and read by Stylo (upstream #1240, taken 2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `ElementData::classes` is a new pub field filled from the `class` attribute, kept in sync by the mutator's `set_attribute`/`remove_attribute`, and Stylo's `has_class`/`each_class` now read it. It is element-data state that sits under the line 53 claim that fields beyond tree structure live on element data. Expected amendments name §Design Philosophy :53 for `ElementData::classes`."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:17
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 52
    section: "## Design Philosophy"
    change: "New sub-bullet after line 52: \"  - Hit testing and the paint tree skip a node dropped since the last resolve rather than index its stale id: `Node::hit_inner` looks each hoisted child, paint child and text-cluster node up with `tree().get(id)` and continues past one that no longer resolves, and `paint_tree.rs` does the same (packages/blitz-dom/src/node/node.rs:1572-1574; packages/blitz-dom/src/node/node.rs:1587-1589; packages/blitz-dom/src/node/node.rs:1621-1623; packages/blitz-dom/src/node/node.rs:1646-1650; packages/blitz-dom/src/layout/paint_tree.rs:164-172; tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82)\""
    sidecar: "Design Philosophy › One incremental pipeline, safe identities: new sub-bullet — hit testing skips nodes dropped since the last resolve (upstream #1244, taken 2026-10-24-upstream-sync-polish-and-ship)."
    rationale: "Report Changes → Symbols: `Node::hit_inner` (four loops) and `paint_tree.rs` now use `self.tree().get(id)` with `continue` in place of `self.with(id)`, skipping nodes dropped since the last resolve, pinned by the new `hit_test_dropped_nodes` target. It is an engine change for every document, ratified (inputs#I4). It applies the group's safe-identities principle that a dropped node's id stops resolving (line 52)."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:22
note: "The `file:line` cites in this file that point into files the take moved (line 47 construct.rs:619-624 and :949-951, where `build_inline_layout_into` moved to layout/inline/build.rs; line 49 node.rs:712-722; line 53 element.rs; line 34 replaced.rs, paint_tree.rs and layout/mod.rs; line 61 attr_test.rs and main.rs) belong to P2's citation sweep (report.md:95). This pass proposes no digits for them."
