proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 25
    section: "## Design Philosophy"
    change: "New sub-bullet following this line, under «Embedder services are injected, with no-op defaults» - `NetHandler::error`, new on the public trait, is a default that does nothing (packages/blitz-traits/src/net.rs:36-38); blitz-net's error arm calls it (packages/blitz-net/src/lib.rs:313) and blitz-dom's stylesheet handler responds with the error (packages/blitz-dom/src/net.rs:144-146), so a failed stylesheet stops blocking render, and a wrapping handler must forward it or the failure stops at the wrapper, as blitz-test-harness's `CountedHandler` now forwards it (packages/blitz-test-harness/src/settle.rs:121-125)"
    sidecar: "Design Philosophy, embedder services - a failed load reaches the document through the no-op default `NetHandler::error`, which a wrapping handler must forward."
    rationale: "The report's Symbols / APIs (f6edb9701 and the `CountedHandler` bullet) and Decisions & corrections add a new method with a no-op default to the provider trait surface this group describes; the group does not state it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 49
    section: "## Design Philosophy"
    change: "New sub-bullet following this line, under «One incremental pipeline, safe identities» - attribute and child-list mutations restyle narrowly, `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` replacing the blanket `RestyleHint::restyle_subtree()` on `set_attribute`, `clear_attribute`, removal and insertion (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807)"
    sidecar: "Design Philosophy, incremental pipeline - attribute and child-list mutations no longer take a blanket subtree restyle (upstream fe33b778b)."
    rationale: "The report's Symbols / APIs, fe33b778b #1276, narrows the invalidation this group's heading describes as incremental; the group's restyle line speaks only of damage bits and state changes."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 52
    section: "## Design Philosophy"
    change: "New sub-bullet following this line, under «One incremental pipeline, safe identities» - dropping an anonymous block leaves no stale id behind it, `BaseDocument::deallocate_anonymous_block` taking the block's `layout_children` and setting `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503)"
    sidecar: "Design Philosophy, safe identities - a dropped anonymous block's children no longer name it as `layout_parent`."
    rationale: "The report's fix of the key-press panic cures every reader of a stale `layout_parent` because the id stops being stale, a new instance of the safe-identities rule this group states; the group holds no line for it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 53
    section: "## Design Philosophy"
    change: "New sub-bullet following this line, under «One incremental pipeline, safe identities» - beside its one recorded style fact, `Node` records a box-construction fact in `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN`, set in `collect_layout_children_with_wrap` (packages/blitz-dom/src/layout/construct.rs:587-599) and read through the public `Node::is_inline_with_only_block_children` (packages/blitz-dom/src/node/node.rs:531-537); `render_element` skips such a box's own shadow, background and border"
    sidecar: "Design Philosophy, node state - `Node` records whether an inline box holds only block children, set at box construction and read at paint (upstream f76777766)."
    rationale: "Line 53 enumerates what `Node` carries; the report's f76777766 #1319 adds a flag and a public reader to it, and its Counts record no master naming them (`inline-flag` unfired)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:19
note: "D-arch-resources and D-arch-decisions found no violation in this category's text, and no item of the report's Retired bullet stands in this file."
