proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 28
    section: "## Depth Strategy"
    change: "Line 28 gains, after its closing parenthesis: «; for a row or a group of an open select list, since 2026-10-31-select-popup, it answers the list's scrollport (packages/blitz-dom/src/scrolling.rs:754-756; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»"
    sidecar: "Depth Strategy: `visible_region` answers an open select list's scrollport for a row or a group."
    rationale: "Report Symbols / APIs, Seen through the list: `visible_region` answers the list's scrollport for a row or a group of an open list; Counts / qualifiers moved, `visible-reg`, names depth-strategy:28 as changed."
    basis: packages/blitz-dom/src/scrolling.rs:754-756
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 26
    section: "## Depth Strategy"
    change: "New row following line 26: «- An open select's list paints over the page: `paint_scene` paints it after the root's render and before the devtools overlays, whole past an opacity ancestor and a box that clips its select; a non-finite box goes unpainted (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; tests/blitz-tests/tests/select_popup_painted.rs:211-235; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»"
    sidecar: "Depth Strategy: new row, an open select list's paint level, after the root's render and before the devtools overlays."
    rationale: "Report Symbols / APIs, Paint, and the design acceptance row on clips and `opacity`; Expected amendments carries design-system §Depth Strategy under Symbols / APIs (placement), and no row of this file states the list's level."
    basis: packages/blitz-paint/src/render.rs:246-247
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 27
    section: "## Depth Strategy"
    change: "New row following line 27: «- An open select's list answers a hit before the page: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, so `hit`, `set_hover_to` and `handle_pointerdown` read the list, `hit_inner` not edited; a box that clips the select clips no row, and a point outside the list reads what shows (packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/document.rs:962-988; tests/blitz-tests/tests/select_popup.rs:677-716; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»"
    sidecar: "Depth Strategy: new row, an open select list answers a hit first, ahead of the walk `hit_inner` keeps."
    rationale: "Report Symbols / APIs, The hit: `hit_with_scrollbar` answers `select_list_hit` first, `hit_inner` not edited, so line 26's walk stands as written and the list's hit is a new row beside it."
    basis: packages/blitz-dom/src/document.rs:2404-2406
note: "D-design-tokens not fired here: the list's `tokens hardcoded✗` holds engine colour and spacing constants owed to §Color Palette and §Spacing, and the report names no depth token; no Retired item stands in this file."
