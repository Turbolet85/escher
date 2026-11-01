proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 27
    section: "## Cross-cutting Patterns"
    change: "Add after line 27, in the same group, the item «- A stylesheet that fails, comes back empty or is not CSS no longer holds the redraw skip for pending critical resources, on every document: `ResourceHandler<StylesheetHandler>::error` responds `Err(error)`, so `pending_critical_resources` loses the request and it stops blocking render (packages/blitz-dom/src/net.rs:144-146); the trait method it answers is §Standard Contracts → Provider traits' and the render-blocking rule §Established Decisions → Resource loading's»"
    sidecar: "Service wiring gains the failed stylesheet's release of the redraw skip for pending critical resources (f6edb9701)."
    rationale: "report.md:14 states a failed, empty or non-CSS stylesheet stops blocking render; line 21's skip while critical resources are pending stands and gains this carve-out, which no row of the report's net-error or load-wait sweep places in this file."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 33
    section: "## Cross-cutting Patterns"
    change: "Add after line 33, in the same group, the item «- A `mounted` event queued for an element removed before the flush is skipped: `flush_queued_mounted_events` reads the element through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239); the guard's reader is §Standard Contracts → Dioxus DOM bridge's»"
    sidecar: "Event processing gains the mounted-event guard for an element removed since its event was queued (6ef4b532d)."
    rationale: "report.md:20 lands the guard; line 33 states when `mounted` listeners fire and names no exception, and report.md:56 changes only standard-contracts.md:266 for it."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 42
    section: "## Cross-cutting Patterns"
    change: "Add after line 42, in the same group, the item «- A mutation's restyle is narrowed, on every document: `DocumentMutator::restyle_for_attribute_change` and `restyle_for_child_list_change` replace the blanket `RestyleHint::restyle_subtree()` on `set_attribute`, `clear_attribute`, removal and insertion (packages/blitz-dom/src/mutator.rs:689-735; packages/blitz-dom/src/mutator.rs:737-807), and the changed-set insert in `remove_and_drop_all_children` stays first; the helpers' contract line is §Standard Contracts → Document core's»"
    sidecar: "Invalidation gains the restyle narrowing on attribute and child-list mutations (fe33b778b)."
    rationale: "report.md:11 replaces the blanket restyle; the group states damage and the changed set and holds no restyle rule, and report.md:51 owes the helpers a contract line in standard-contracts.md, the other side of this record."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 42
    section: "## Cross-cutting Patterns"
    change: "Add after line 42, in the same group, the item «- `TElement::borrow_data` filters data with no primary style (packages/blitz-dom/src/stylo.rs:771-776)»"
    sidecar: "Invalidation gains the `borrow_data` filter on data with no primary style (fe33b778b)."
    rationale: "report.md:11 lists the filter under the restyle narrowing; pattern snapshot-node at report.md:51 reads no such line in this file, and it is state integrity on the style data this group describes."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 42
    section: "## Cross-cutting Patterns"
    change: "Add after line 42, in the same group, the item «- `compute_layout_damage`'s `table_needs_rebuild` takes `ALL_DAMAGE` when a table box's position, padding or border changes (packages/blitz-dom/src/layout/damage.rs:442-458)»"
    sidecar: "Invalidation gains the table rebuild rule in `compute_layout_damage` (8b1a0a3ce)."
    rationale: "report.md:12 lands the rule; report.md:59 reads pattern table-damage unfired over the seven masters, so no master states it, and this group states the damage rules."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 42
    section: "## Cross-cutting Patterns"
    change: "Add after line 42, in the same group, the item «- `append_text_to_node` also damages the parent (packages/blitz-dom/src/mutator.rs:257-259), beside the node it marks in the changed set»"
    sidecar: "Invalidation gains the parent damage on `append_text_to_node` (f76777766)."
    rationale: "report.md:19 lands the parent damage; line 40 names `append_text_to_node` as marking the node itself in the changed set, a claim that stands and now sits beside damage reaching the parent."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 42
    section: "## Cross-cutting Patterns"
    change: "Add after line 42, in the same group, the item «- A dropped anonymous block leaves no stale `layout_parent`: `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so every reader of a stale `layout_parent` is cured with it, held by `key_press_after_container_lost_its_box` in both layout modes; a direct engine change for every document, §Established Decisions'»"
    sidecar: "Invalidation gains the `layout_parent` clear when an anonymous block is dropped, escher's own fix of the key-press panic."
    rationale: "report.md:23 lands the fix as a state-integrity fact; report.md:50 reads line 39's anonymous-block claim as true and unmoved, so this is a new record and no edit of that line; report.md:27 names the check."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:23
note: No item the report's Retired bullet names stands in this file, D-arch-resources and D-arch-decisions find nothing violated here, the report's rows on lines 39 and 40 read no-change, and `init_stylo_prefs`, the blend layer and the paint skip are left to their registry and contract sections.
