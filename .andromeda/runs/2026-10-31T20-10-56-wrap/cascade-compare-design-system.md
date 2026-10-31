# Cascade compare — design-system · wrap · 2026-10-31-select-popup
E items 20 · held 7 · lost at classify 4 · lost at strong 1 · beside a placed fact 4 · cascade only 9 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### color-palette#5.1 · lost at classify · .andromeda/design-system-detail/color-palette.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 71
    section: "## Color Palette › ### Semantic Colors"
    change: "Add after this row: | Disabled select list row | `#6d6d6d` text | An open select list's disabled row: `SelectListView`'s constant `DISABLED_TEXT`, hardcoded as the engine holds no tokens; read as painted by `select_popup_painted` (packages/blitz-dom/src/node/select_list.rs:110-123; packages/blitz-paint/src/render/select_list.rs:85-128; tests/blitz-tests/tests/select_popup_painted.rs:119-159) |"
    sidecar: "Color Palette › Semantic Colors: new row for the open select list's disabled row text, `#6d6d6d`, an engine constant."
    rationale: "Report Symbols / APIs names `DISABLED_TEXT` `#6d6d6d` among `SelectListView`'s constants, and Coverage owes new rows in §Color Palette; the file's one row per state holds no select list state."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:10

### color-palette#7.3 · lost at classify · .andromeda/design-system-detail/color-palette.md:97
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 97
    section: "## Color Palette › ### Border Progression"
    change: "Add after this row: | blitz-dom select list box border | `#999999` | An open select list's border: `SelectListView`'s constant `BORDER`, hardcoded as the engine holds no tokens (packages/blitz-dom/src/node/select_list.rs:110-123; packages/blitz-paint/src/render/select_list.rs:54-83) |"
    sidecar: "Color Palette › Border Progression: new row for the open select list's border, `#999999`, an engine constant."
    rationale: "Report Symbols / APIs names `BORDER` `#999999` among `SelectListView`'s constants, and Coverage owes new rows in §Color Palette; `select-colours` keeps the sheet's border as it stands, so this is a new row and no edit to one."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:10

### depth-strategy#3.2 · lost at classify · beside a placed fact F8 (near 34, depth-strategy#4.1) · .andromeda/design-system-detail/depth-strategy.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 27
    section: "## Depth Strategy"
    change: "New row following line 27: «- An open select's list answers a hit before the page: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, so `hit`, `set_hover_to` and `handle_pointerdown` read the list, `hit_inner` not edited; a box that clips the select clips no row, and a point outside the list reads what shows (packages/blitz-dom/src/document.rs:2404-2406; packages/blitz-dom/src/document.rs:962-988; tests/blitz-tests/tests/select_popup.rs:677-716; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»"
    sidecar: "Depth Strategy: new row, an open select list answers a hit first, ahead of the walk `hit_inner` keeps."
    rationale: "Report Symbols / APIs, The hit: `hit_with_scrollbar` answers `select_list_hit` first, `hit_inner` not edited, so line 26's walk stands as written and the list's hit is a new row beside it."
    basis: packages/blitz-dom/src/document.rs:2404-2406

### spacing#3.2 · lost at strong · beside a placed fact F5 (near 29, spacing#3.3) · .andromeda/design-system-detail/spacing.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 28
    section: "## Spacing"
    change: Add item after this line - "- `size` on a `multiple` select hints `height` of `size` × 1.2em, parsed by `size_rows` and capped at 65534, `0` or a bad value hinting nothing; its rows take `line-height` 1.2 and the select `overflow-y` auto; a single select's `size` stays unread (packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193)"
    sidecar: Spacing gains the `size` height hint and row line-height on a `multiple` select (2026-10-31-select-popup)
    rationale: Report Symbols / APIs under `size` states the hint, the cap and the sheet rule, and Retired names «a `multiple` select's `size` is unread»; the file's attribute-to-spacing mappings hold no row for it
    basis: packages/blitz-dom/src/stylo.rs:1124-1139

### spacing#3.6 · lost at classify · beside a placed fact F6 (near 29, spacing#3.3) · .andromeda/design-system-detail/spacing.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 32
    section: "## Spacing"
    change: Add item after this line - "- A single select's open list shows at most `MAX_ROWS` = 20 rows and scrolls past them, each row the height of the select's font on one line (`row_height`, packages/blitz-dom/src/node/select_list.rs:28-46)"
    sidecar: Spacing gains the select list's row cap and row height (2026-10-31-select-popup)
    rationale: Report Symbols / APIs states `MAX_ROWS` = 20 and the `row_height` field, and Deviations states the row height is the select's font on one line; the Expected amendments list §Spacing rows as new text with no master site standing
    basis: packages/blitz-dom/src/node/select_list.rs:28-46

### surface#1.2 · lost at classify · .andromeda/design-system-detail/surface.md:9
  - detector: D-design-tokens
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: An open select list's highlight, disabled text and border are engine constants on `SelectListView`, not rules of that sheet and no design token, on the founder's F6, the engine holding no tokens: `HIGHLIGHT_TEXT` and `HIGHLIGHT_BACKGROUND` on the highlighted row, `DISABLED_TEXT` on a disabled row, `BORDER` and `BORDER_WIDTH` on the list's box; their literals at §Color Palette (packages/blitz-dom/src/node/select_list.rs:110-123; packages/blitz-paint/src/render/select_list.rs:54-83; packages/blitz-paint/src/render/select_list.rs:85-128)"
    sidecar: "surface: token map gains the select list's engine constants, hardcoded on the founder's F6, literals at Color Palette."
    rationale: "Report Coverage for `select list, engine overlay` reads `tokens hardcoded✗ — engine constants on his F6, the engine holding no tokens`; Symbols / APIs names the constants; `select-colours` lists surface:9's neighbour as a changed site."
    basis: .andromeda/design-system-detail/surface.md:9

### surface#1.3 · lost at strong · .andromeda/design-system-detail/surface.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 10
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "New item after this line: In that sheet `select[multiple][size]` takes `overflow-y: auto` and its rows `line-height: 1.2`, beside the engine's height hint of `size` × 1.2em on a `multiple` select, the parse capped at 65534 and `0` or a bad value hinting nothing, so it stands that many rows tall — engine defaults, no design token (packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/stylo.rs:1124-1139)"
    sidecar: "surface: token map gains the sheet's sized multiple-select rules and the size height hint."
    rationale: "Report Symbols / APIs, `size`: `stylo.rs:1124-1139` hints the height and `default.css:183-193` adds the overflow and line-height rules; Coverage `size on a multiple select`."
    basis: packages/blitz-dom/assets/default.css:183-193

### surface#4.1 · lost at classify · beside a placed fact F19 (near 63, surface#7.2) · .andromeda/design-system-detail/surface.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New group after this line, `<!-- detail: values · desktop-native-select-list -->` closed by `<!-- /detail -->`, its first item: An open single select's list is painted by the engine over the page, its rows no boxes of the page's own: `paint_scene` paints it after the root's render and before the devtools overlays, whole past an `opacity` ancestor and a box that clips its select, with no motion; only rows in the scrollport are laid out, a non-finite box goes unpainted, and a shot of an open list shows an option's label and a group's `label` as pixels (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40)"
    sidecar: "surface: new desktop-native-select-list group, the list painted last and unclipped, with no motion."
    rationale: "Report Symbols / APIs, Paint; Schema / config on the list's pixels; Reverted names rows as boxes a rejected approach; Outcome (design) rows on clips, `opacity` and no motion; Expected amendments name §Surface: desktop-native."
    basis: packages/blitz-paint/src/render.rs:246-247

### surface#4.1 · lost at classify · .andromeda/design-system-detail/surface.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New item in the `desktop-native-select-list` group: Placement: the list is at least its select's width and at most the viewport's, stays inside the viewport, and opens where its rows fit, else on the side with more room; a row is the select's font on one line tall; `MAX_ROWS` = 20 rows show, the rest reached by key, wheel or the driver's `scroll`; a thumb is painted under the `scrollbars` feature, takes no hit and is not draggable (packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:265-293; packages/blitz-paint/src/render/select_list.rs:130-154)"
    sidecar: "surface: the select list's placement, row height, twenty-row limit and undraggable thumb."
    rationale: "Report Symbols / APIs names `place`, `geometry`, `MAX_ROWS` and `paint_list_thumb`; Deviations give the row height; New text names the placement, long-list and thumb tests; Beyond the plan states the thumb not draggable; Expected amendments carry placement to §Surface: desktop-native."
    basis: packages/blitz-dom/src/node/select_list.rs:218-263

### surface#4.1 · lost at classify · .andromeda/design-system-detail/surface.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New item in the `desktop-native-select-list` group: Row states: a group's `label` is a header row, its options under it, heavier than a row by a synthesized bold on every build, and it chooses nothing; the highlight moves by key, and by pointer to an enabled row; a disabled row changes nothing and leaves the list open; the highlight is in no snapshot and no tree, and a highlighted row's contrast is unread by the lint (packages/blitz-paint/src/render/select_list.rs:85-128; packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689)"
    sidecar: "surface: the select list's header, highlighted and disabled row states, and the highlight's stated limits."
    rationale: "Report Symbols / APIs, Paint, on `RunPaint` and the synthesized bold; Deviations carry the operator's «a header heavier than a row on every build»; New text names the group, highlight and disabled-row tests; Beyond the plan and Coverage state the highlight's limits."
    basis: packages/blitz-paint/src/text.rs:586-593

### surface#4.1 · lost at classify · .andromeda/design-system-detail/surface.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 39
    section: "## Surface: desktop-native › ### Component Patterns"
    change: "New item in the `desktop-native-select-list` group: Opening and closing: a click, Enter, and a Space with no typed text standing open the list on its chosen option, changing nothing; a disabled, inert or hidden select, or one with no row to show, opens nothing, and a label's click focuses a bound single select and opens nothing; Enter, Space, Tab and a click on a row choose and close, Tab before focus moves either way; Escape, a second click, a click outside, the page scrolling and a wheel outside the list close it with the value unchanged; a select made inert, disabled, hidden or removed closes its list; a close clears the typed text; the driver's `type` focuses with no click and leaves no list open (packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:415-420; packages/blitz-dom/src/document.rs:891-907)"
    sidecar: "surface: the select list's opening, choosing and closing states."
    rationale: "Report Symbols / APIs, Pointer, Keys, The resolve and Driver; Deviations on a select with no row to show and a close clearing the typed text; Decisions on the page's scroll closing the list and a label opening nothing; New text names the opening, choosing, closing and `type` tests."
    basis: packages/blitz-dom/src/events/keyboard.rs:215-250

### typography#1.8 · lost at strong · beside a placed fact F4 (near 44, typography#3.4) · .andromeda/design-system-detail/typography.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 17
    section: "## Typography"
    change: "New table row after the blitz-dom default code/pre row: | blitz-dom default `select[multiple][size]` rows | not stated | not stated | rows line-height 1.2; the select's `height` hinted at `size` × 1.2em, `size` capped at 65534, `0` or a bad value no hint | not stated | (packages/blitz-dom/assets/default.css:183-193; packages/blitz-dom/src/stylo.rs:1124-1139) |"
    sidecar: "typography: new row, the line height of a sized multiple select's rows and its height hint, from 2026-10-31-select-popup."
    rationale: "Report, Symbols / APIs, `size`: `stylo.rs:1124-1139` hints `height` of `size × 1.2em` on a `multiple` select and `default.css:183-193` sets `line-height: 1.2` on its rows; Retired names «a `multiple` select's `size` is unread». A new value in the engine's default sheet, one row per value."
    basis: packages/blitz-dom/assets/default.css:183-193

## Held

- color-palette#2.5
- depth-strategy#3.1
- depth-strategy#3.3
- motion#1.5
- spacing#2.6
- surface#1.6
- typography#3.11

## Cascade only

- color-palette#10.1
- depth-strategy#4.1
- depth-strategy#4.4
- motion#1.2
- motion#3.1
- spacing#3.3
- surface#1.4
- surface#7.2
- typography#3.4
