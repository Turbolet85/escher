proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 9
    section: "## Standard Contracts"
    change: "In line 9, after `NetHandler::bytes(resolved_url, bytes)` per resource kind and its citations, add: `NetHandler::error(url, error)` stands beside `bytes`, the stylesheet's `ResourceHandler<StylesheetHandler>::error` responding `Err(error)`, so `pending_critical_resources` loses the request (packages/blitz-dom/src/net.rs:144-146), and `net.rs`'s other resource kinds keeping the trait's do-nothing default."
    sidecar: "Document core: `NetHandler::error` beside `bytes`, a failed stylesheet leaving `pending_critical_resources` through `ResourceHandler<StylesheetHandler>::error` (`f6edb9701`)."
    rationale: "report.md:14 and report.md:48 mark line 9 changed, per resource kind; report.md:268 shows `net.rs`'s one added range."
    basis: packages/blitz-dom/src/net.rs:144
    fact: F14
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 28
    section: "## Standard Contracts"
    change: "In line 28, after `resolved_style_value` returns an empty string for unknown properties and used values for layout-dependent ones, add: the resolved `width` and `height` reading `node.unrounded_layout()`, so a reader gets sub-pixel values where `final_layout` rounded (packages/blitz-dom/src/resolved_style.rs:468)."
    sidecar: "Mutation, query and CSSOM: the resolved `width` and `height` read sub-pixel (`ae774d7f4`)."
    rationale: "report.md:21 states the read and report.md:53 marks line 28 changed."
    basis: packages/blitz-dom/src/resolved_style.rs:468
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 33
    section: "## Standard Contracts"
    change: "- Restyle on mutation (upstream `fe33b778b`, taken at 2026-11-01-upstream-sync-ui-graph, an engine change for every document): `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` replace the blanket `RestyleHint::restyle_subtree()` on `set_attribute`, `clear_attribute`, removal and insertion (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807); public `BaseDocument::snapshot_node` takes a new `attr: &LocalName`, its two callers both in `mutator.rs` (packages/blitz-dom/src/mutator.rs:288; packages/blitz-dom/src/mutator.rs:429); the changed-set insert in `remove_and_drop_all_children` stays first."
    sidecar: "Mutation, query and CSSOM: the restyle helpers replace the blanket subtree restyle, and `snapshot_node` takes `attr` (`fe33b778b`)."
    rationale: "report.md:11 states the helpers and the signature; report.md:51 records that standard-contracts owes the restyle helpers a line."
    basis: packages/blitz-dom/src/mutator.rs:689
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 58
    section: "## Standard Contracts"
    change: "- Style data and the resolve context (upstream `fe33b778b`, taken at 2026-11-01-upstream-sync-ui-graph, an engine change for every document): `TElement::borrow_data` filters data with no primary style (packages/blitz-dom/src/stylo.rs:771-776), and the style-resolve context is scoped (packages/blitz-dom/src/stylo.rs:231-238)."
    sidecar: "Scrolling, selection, tree: beside the Stylo readers, `borrow_data` filters data with no primary style and the style-resolve context is scoped (`fe33b778b`)."
    rationale: "report.md:11 states both, and line 58 is where this file lists `resolve_undisplayed_style` and `resolve_stylist` of `stylo.rs`."
    basis: packages/blitz-dom/src/stylo.rs:771
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 75
    section: "## Standard Contracts"
    change: "- `Node`'s stacking-context arm reads a `mix-blend-mode` other than `normal` (packages/blitz-dom/src/node/node.rs:1382-1384); upstream `6fca6308d`, taken at 2026-11-01-upstream-sync-ui-graph, an engine change for every document."
    sidecar: "Layout: `mix-blend-mode` other than `normal` makes a stacking context (`6fca6308d`), beside the paint-tree API's `StackingContext`."
    rationale: "report.md:15 states the arm and report.md:33 retires `Node`'s `TODO: mix-blend-mode`; the paint-tree API on line 74 lists `StackingContext`."
    basis: packages/blitz-dom/src/node/node.rs:1382
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 82
    section: "## Standard Contracts"
    change: "In line 82, after the `Widget` trait's list of methods and its citations, add: `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` with no old value, behind `custom-widget` (packages/blitz-dom/src/node/element.rs:902-906)."
    sidecar: "Nodes and widgets: `set_custom_widget` replays existing attributes through `attribute_changed` (`a26c6ddeb`)."
    rationale: "report.md:18 states the replay and report.md:52 marks line 82 changed."
    basis: packages/blitz-dom/src/node/element.rs:902
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 95
    section: "## Standard Contracts"
    change: "- `NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN` and the public reader `Node::is_inline_with_only_block_children` (packages/blitz-dom/src/node/node.rs:531-537), set in `collect_layout_children_with_wrap` (packages/blitz-dom/src/layout/construct.rs:587-599); `render_element` skips such a box's own shadow, background and border through `paints_own_box`, with no frame test; upstream `f76777766`, taken at 2026-11-01-upstream-sync-ui-graph, an engine change for every document."
    sidecar: "Nodes and widgets: the inline-with-only-block-children flag, its public reader and its paint skip (`f76777766`)."
    rationale: "report.md:19 states the flag, the reader and the skip; report.md:106 records the frame gap."
    basis: packages/blitz-dom/src/node/node.rs:531
    fact: F20
note: This listing stops at standard-contracts line 96, so F13, F15, F16, F17, F19 and F21-F24 and the report's standard-contracts lines 103, 131, 135, 227, 240, 266, 324, 327 and 334 are not placed here; F18's proposal binds with design-system depth-strategy line 20, which the report marks changed.
