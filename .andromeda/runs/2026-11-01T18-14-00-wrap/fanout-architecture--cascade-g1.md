proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 53
    section: "## Design Philosophy"
    change: "In the sentence on `Node`, after its one recorded style fact `Node::out_of_flow` and before «other fields living on element data», add: and, since 2026-11-01-upstream-sync-ui-graph, one box-construction fact, the flag `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, which `collect_layout_children_with_wrap` sets and the public `Node::is_inline_with_only_block_children` reads, `render_element` skipping such a box's own shadow, background and border (packages/blitz-dom/src/node/node.rs:531-537; packages/blitz-dom/src/layout/construct.rs:587-599)"
    sidecar: "design-philosophy: the `Node` sentence names the new box-construction flag beside its one recorded style fact (`f76777766`)"
    rationale: "report.md:19 puts the flag on `Node`, set in box construction by `f76777766`, so «one recorded style fact … other fields living on element data» no longer states all a `Node` records; `packages/blitz-dom/src/node/node.rs` is a merge surface"
    basis: packages/blitz-dom/src/node/node.rs:531-537
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 22
    section: "## Stack and Technologies"
    change: "In the Role, end the clause at «with no `pub use` of it», dropping «, the comment at lib.rs:96-97 heading none», since the orphan comment is gone and `pub use clock` beneath it is unmoved"
    sidecar: "stack-and-technologies: Layout's Role drops the retired orphan «Re-export taffy» comment"
    rationale: "report.md:26 removes the two-line comment at `blitz-dom/src/lib.rs:96-97`, report.md:33 retires it as item (5), and report.md:55 rows `stack-and-technologies.md:22` ×2 changed, the comment gone"
    basis: .andromeda/architecture-detail/stack-and-technologies.md:22
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 76
    section: "## Stack and Technologies"
    change: "The parenthesis on `bash .github/scripts/ci-leg.sh fast` reads (fmt → clippy → test → ci-scripts → embolden, stopping at the first red)"
    sidecar: "stack-and-technologies: the `fast` leg's order gains `embolden`, last"
    rationale: "report.md:41 rows `stack-and-technologies.md:76` ×2 changed, fmt → clippy → test → ci-scripts → embolden; report.md:64 puts `embolden` last in `FAST_LEGS` at `ci-leg.sh:8-9`"
    basis: .github/scripts/ci-leg.sh:8-9
note: "No `## New facts` item is of these two categories. Of what the picked entities brushed, F11, F13, F16, F20 and F23 are standard-contracts' and F6 established-decisions'; line 76's edit carries only the order, the `embolden` leg's own statement staying F23's. D-arch-decisions reads no drift: no dependency moved, and `BlendMode` and `url` are already in the stack."
