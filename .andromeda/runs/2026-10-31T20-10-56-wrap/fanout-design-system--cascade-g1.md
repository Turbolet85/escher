proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 29
    section: "## Color Palette › ### Core Colors"
    change: "After this row add three rows, one per state — first row: | blitz-dom select list highlighted row | `#c8c8c8` background, `#000000` text | An open select list's highlighted row: `SelectListView::HIGHLIGHT_BACKGROUND` and `HIGHLIGHT_TEXT`, fixed constants, hardcoded on the founder's fork F6 as the engine holds no tokens, not in the default sheet (packages/blitz-dom/src/node/select_list.rs:110-123; tests/blitz-tests/tests/select_popup_painted.rs:119-159) | — second row: | blitz-dom select list disabled row | `#6d6d6d` text | An open select list's disabled row: `SelectListView::DISABLED_TEXT`, a fixed constant, hardcoded on the founder's fork F6, not in the default sheet (packages/blitz-dom/src/node/select_list.rs:110-123; tests/blitz-tests/tests/select_popup_painted.rs:119-159) | — third row: | blitz-dom select list border | `#999999` | An open select list's border: `SelectListView::BORDER`, a fixed constant, hardcoded on the founder's fork F6, not in the default sheet (packages/blitz-dom/src/node/select_list.rs:110-123) |"
    sidecar: "color-palette: the open select list's highlight, disabled and border colours added as engine constants."
    rationale: "Report Symbols / APIs lists the four `SelectListView` constants; Coverage reads tokens hardcoded✗ with new rows owed in §Color Palette; `select-colours` places them beside color-palette:29, whose `multiple` pair stands."
    basis: packages/blitz-dom/src/node/select_list.rs:110-123
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 29
    section: "## Color Palette › ### Core Colors"
    change: "After this row add: | blitz-dom select list plain row | no `SelectListView` constant; value not stated | An open select list's plain pair, measured by `select_popup_painted` at the default sheet and at the stand; a highlighted row's contrast is unread by the lint, a limit left for later; a closed select's own pair stays unread (tests/blitz-tests/tests/select_popup_painted.rs:161-187; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md) |"
    sidecar: "color-palette: the open select list's plain pair recorded as measured, with the highlighted row's and closed select's pairs unread."
    rationale: "Report `ua-pair` has `select_popup_painted` measure the open list's plain pair and leaves the closed select's pair unread; the P5 card's row names the highlighted row's contrast unread by the lint."
    basis: tests/blitz-tests/tests/select_popup_painted.rs:161-187
    fact: F2
  - detector: D-design-tokens
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 120
    section: "## Color Palette › ### Border Progression"
    change: "At this line's end, after its searched list, add: ; since 2026-10-31-select-popup the open select list's colours are fixed constants on `SelectListView`, hardcoded on the founder's fork F6, and add no token or custom property."
    sidecar: "color-palette: the absence of colour tokens records the select list's hardcoded constants as adding none."
    rationale: "Report Coverage for the select list reads tokens hardcoded✗, engine constants on the founder's fork F6, the engine holding no tokens; the absence stays true and the hardcoded surface is now stated at it."
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 51
    section: "## Typography"
    change: "Replace the clause «with hinting off when emboldening» with «with hinting off on a build that emboldens; a synthesized group header on a plain build is emboldened with hinting on, and on an emboldening build it takes twice the stated strength»."
    sidecar: "typography: emboldening's hinting and strength restated for a synthesized group header."
    rationale: "Report Spec claims disproved, `stroke-text`: typography:51's hinting clause no longer holds for a synthesized header on a plain build, `hint` staying `!FONT_EMBOLDEN_ENABLED` at text.rs:695, and an emboldening build gives that header twice the strength."
    basis: packages/blitz-paint/src/text.rs:695
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 51
    section: "## Typography"
    change: "After this line add: - A group's `label` in an open select list paints as a header heavier than a row on every build, through `RunPaint { color, synthesize_bold }` in `stroke_text_as`; a synthesized bold adds one embolden weight to a build's own, and a run with no synthesized bold paints as before (packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; tests/blitz-tests/tests/select_popup_painted.rs:282-297)"
    sidecar: "typography: a select list group header's synthesized bold added."
    rationale: "Report Symbols / APIs, Paint, names `stroke_text_as` and `RunPaint`; Deviations records the operator's direction of a header heavier than a row on every build, with a run with no synthesized bold painting as before."
    basis: packages/blitz-paint/src/text.rs:586-593
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 44
    section: "## Typography"
    change: "After this line add: - An open select list's row height is the select's font on one line, laid out through `select_row_layout`; a larger authored font clips, a limit left for later (packages/blitz-dom/src/document.rs:1054-1064; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "typography: a select list row's one-line height and its larger-font limit added."
    rationale: "Report Deviations: the row height is the select's font on one line; Symbols / APIs names `select_row_layout`; the P5 card's row states a larger authored font clipping as a limit left for later."
    basis: packages/blitz-dom/src/document.rs:1054-1064
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 21
    section: "## Spacing"
    change: "After this line add: - blitz-dom `multiple` select with `size`: `size_rows` hints a `height` of `size × 1.2em`, capped at 65534, `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto` and its rows `line-height: 1.2` (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193)"
    sidecar: "spacing: a sized `multiple` select's height hint, overflow and row line height added."
    rationale: "Report Symbols / APIs, `size`, states the hint, the cap and the default sheet's block; Retired ends the limit that a `multiple` select's `size` is unread."
    basis: packages/blitz-dom/src/stylo.rs:1124-1139
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 29
    section: "## Spacing"
    change: "After this line add: - An open select list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT` (tests/blitz-tests/tests/select_popup.rs:870-911; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "spacing: the select list's row limit and engine spacing constants added."
    rationale: "Report Symbols / APIs lists `MAX_ROWS` = 20 and the three spacing constants with `BORDER_WIDTH`; `a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel` holds the limit; design-system's expected amendments owe §Spacing new text."
    basis: tests/blitz-tests/tests/select_popup.rs:870-911
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 29
    section: "## Spacing"
    change: "After this line add: - `place` sets an open select list at least its select's width and at most the viewport's, inside the viewport, opening where its rows fit and else on the side with more room; every edge stays finite (packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:399-457)"
    sidecar: "spacing: the select list's placement and width bounds added."
    rationale: "Report Symbols / APIs names `place`; the new-text listing's unit tests at select_list.rs:399-457 state where the list opens, its width bounds and finite edges; design-system's expected amendments owe §Spacing new text."
    basis: packages/blitz-dom/src/node/select_list.rs:218-263
    fact: F6
note: "No local gate builds `blitz-paint/font-embolden` (typography:31's neighbour fact) is not this category's: the report routes it at P5 and a gate's coverage is test-plan's."
