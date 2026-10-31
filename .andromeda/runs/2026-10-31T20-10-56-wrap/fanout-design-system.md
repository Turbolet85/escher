proposals:
  - detector: D-design-tokens
    severity: warning
    section: "Color Palette → Engine-painted defaults"
    change: "Add one body line under **Engine-painted defaults:** — the open select list's colours are engine constants, the associated constants of `SelectListView` (`HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT`, `BORDER`), not design tokens and not in `default.css`: the engine holds no tokens, a decision of the founder's F6."
    sidecar: "Color Palette: recorded the select list's colours as hardcoded engine constants on `SelectListView`, by the founder's F6; 2026-10-31-select-popup."
    rationale: "The report's Coverage row `select list, engine overlay` reads `tokens hardcoded✗ — engine constants on his F6, the engine holding no tokens; new rows owed in design-system §Color Palette`; it is the chunk's only `hardcoded✗`, and Reverted lists colours in `default.css` as a rejected approach."
    basis: "packages/blitz-dom/src/node/select_list.rs:110-123"
note: "D-retired found no occurrence: the body of design-system names none of the Retired items, whose hits are in surface.md (the cascade's). The value rows themselves (the four hex colours; §Spacing's constants; §Motion's no-motion row) are new detail into existing category files, so the cascade's, and the proposal above names no value so as not to duplicate them. Outside my detectors, so no proposal: §Depth Strategy's body line «Hit-testing stops at the padding box of a node paint clips» states no exception for an open list's row, which the report says answers first in `hit_with_scrollbar` past a box that clips its select (`a_box_that_clips_the_select_clips_no_row_and_a_point_outside_reads_what_shows`), and the report's `hit-first` and `visible-reg` rows do not name that line."
