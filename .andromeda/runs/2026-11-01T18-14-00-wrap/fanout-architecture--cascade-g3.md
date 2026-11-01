proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 109
    section: "## Established Decisions"
    change: "- `<picture>` is no longer `display: contents`, upstream's `picture` rule leaving `packages/blitz-dom/assets/default.css` and every select rule staying (packages/blitz-dom/src/document.rs:4061-4094) — an upstream engine change for every document, upstream #1317, since 2026-11-01-upstream-sync-ui-graph."
    sidecar: "[Layout] gains `<picture>` no longer being `display: contents`, through `dee854c76`, every select rule staying."
    rationale: "The report's Changes state `dee854c76` #1317 removing upstream's `picture` rule from the sheet, an engine behaviour change for every document (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:13); it sits beside the [Layout] run of upstream engine changes."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:13
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 109
    section: "## Established Decisions"
    change: "- `compute_layout_damage`'s `table_needs_rebuild` takes `ALL_DAMAGE` when a table box's position, padding or border changes (packages/blitz-dom/src/layout/damage.rs:442-458) — an upstream engine change for every document, upstream #1312, since 2026-11-01-upstream-sync-ui-graph."
    sidecar: "[Layout] gains `table_needs_rebuild` taking `ALL_DAMAGE`, through `8b1a0a3ce`."
    rationale: "The report's Changes state `8b1a0a3ce` #1312 at `layout/damage.rs:442-458`, an engine behaviour change for every document (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:12); [Layout] already holds the rule that damage bits decide relayout and reconstruction."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:12
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 161
    section: "## Established Decisions"
    change: "- `render_element`'s border box and culling stand on the scaled size, `location` and `box_position` kept for the transform — an upstream engine change for every document, upstream #1342, a port of `e42ee1f25` since upstream's stands on the untaken `ac6f9f582`, since 2026-11-01-upstream-sync-ui-graph."
    sidecar: "[Painting] gains `render_element`'s border box and culling on the scaled size, a port of `e42ee1f25`."
    rationale: "The report's Changes state `e42ee1f25` #1342 as a port, since upstream's stands on the untaken `ac6f9f582` (.andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:22); [Painting] already holds the culling of transformed boxes in the same file."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:22
    fact: F10
note: "F1, F2, F4, F5, F6 and F7 are established-decisions facts no entity of this group carries, their homes being the upstream history, the direct-engine-change lines, unsupported features, document defaults and resource loading, so the new-detail pass places them unless another group has; F16 and F21 near [Painting] and [JavaScript] are standard-contracts' own."
