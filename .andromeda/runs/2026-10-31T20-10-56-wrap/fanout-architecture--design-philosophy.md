proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 13
    section: "## Design Philosophy"
    change: "after the clause «painting assumes styles and layout are already resolved» and its citation add: «; an open select's list is painted by `paint_select_list` after the root's render inside `paint_scene`, from the public read `select_list_view`, no shell or renderer moved, and its rows' text is the carve-out, laid out at paint through `select_row_layout`, only the rows in the scrollport (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:85-128; packages/blitz-dom/src/document.rs:1022-1052; packages/blitz-dom/src/document.rs:1054-1064)»"
    sidecar: "Design Philosophy, modular engine: an open select's list paints through `paint_scene` alone, its rows' text laid out at paint, a carve-out to painting's assumption of resolved layout (2026-10-31-select-popup)."
    rationale: "Report Changes › Symbols / APIs › Paint names `paint_list_row` and «only rows in the scrollport laid out», and › `BaseDocument` names the public `select_row_layout`; Outcome (arch) has the list through `paint_scene` alone. The file's line states painting's assumption with no carve-out, and no sweep pattern in the report reaches this wording."
    basis: packages/blitz-dom/src/document.rs:1054-1064
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/design-philosophy.md
    near: 51
    section: "## Design Philosophy"
    change: "after the clause «user-initiated and programmatic scrolls share one scrolling primitive, differing only in the `ScrollRequest`» and its citation add: «; an open select's list stands outside that primitive, its `scroll_y` the list's own state, written through the crate-private `scroll_select_list_to` and `scroll_select_list_by`: a wheel over the list scrolls the list alone, and `scroll_into_view` on a row writes it at once and returns, no chain or viewport moving (packages/blitz-dom/src/scrolling.rs:669-672; packages/blitz-dom/src/document.rs:930-945; packages/blitz-dom/src/document.rs:947-960)»"
    sidecar: "Design Philosophy, one pipeline: the one scrolling primitive gains its exception, an open select's list scrolling its own `scroll_y` (2026-10-31-select-popup)."
    rationale: "Report Changes › Symbols / APIs: › Seen through the list has `scroll_into_view` on a row write the list's `scroll_y` at once and return, › Pointer has `handle_wheel` scroll the list over it, › `BaseDocument` names the two crate-private scroll writers. The file's line reads as universal, and the report's `region-scroll` control never fired over the masters' text."
    basis: packages/blitz-dom/src/scrolling.rs:669-672
note: "No registry or decision text stands in this category, so D-arch-resources and D-arch-decisions find nothing here; outside this file, .andromeda/architecture.md:5 repeats painting's assumption and .andromeda/architecture.md:250's `resolve` order names no `reconcile_select_list` (report: `resolve.rs:133-135`), a section the report's expected amendments do not list."
