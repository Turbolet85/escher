proposals:
  - detector: D-design-tokens
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 29
    section: "## Color Palette › ### Core Colors"
    change: "Add after this row: | blitz-dom select list highlighted row | `#c8c8c8` background, `#000000` text | An open select list's highlighted row, the same pair as the chosen row above: `SelectListView`'s constants `HIGHLIGHT_BACKGROUND` and `HIGHLIGHT_TEXT`, hardcoded as the engine holds no tokens and not set in `default.css`; read as painted by `select_popup_painted`, a highlighted row's contrast unread by the lint (packages/blitz-dom/src/node/select_list.rs:110-123; packages/blitz-paint/src/render/select_list.rs:85-128; tests/blitz-tests/tests/select_popup_painted.rs:119-159) |"
    sidecar: "Color Palette › Core Colors: new row for the open select list's highlighted row, `#c8c8c8` on `#000000` text, engine constants."
    rationale: "Report Coverage, `select list, engine overlay`: tokens hardcoded✗, engine constants on the founder's F6, new rows owed in §Color Palette; `select-colours` places the change at color-palette:29's neighbour and leaves the `multiple` row's pair standing."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 71
    section: "## Color Palette › ### Semantic Colors"
    change: "Add after this row: | Disabled select list row | `#6d6d6d` text | An open select list's disabled row: `SelectListView`'s constant `DISABLED_TEXT`, hardcoded as the engine holds no tokens; read as painted by `select_popup_painted` (packages/blitz-dom/src/node/select_list.rs:110-123; packages/blitz-paint/src/render/select_list.rs:85-128; tests/blitz-tests/tests/select_popup_painted.rs:119-159) |"
    sidecar: "Color Palette › Semantic Colors: new row for the open select list's disabled row text, `#6d6d6d`, an engine constant."
    rationale: "Report Symbols / APIs names `DISABLED_TEXT` `#6d6d6d` among `SelectListView`'s constants, and Coverage owes new rows in §Color Palette; the file's one row per state holds no select list state."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 97
    section: "## Color Palette › ### Border Progression"
    change: "Add after this row: | blitz-dom select list box border | `#999999` | An open select list's border: `SelectListView`'s constant `BORDER`, hardcoded as the engine holds no tokens (packages/blitz-dom/src/node/select_list.rs:110-123; packages/blitz-paint/src/render/select_list.rs:54-83) |"
    sidecar: "Color Palette › Border Progression: new row for the open select list's border, `#999999`, an engine constant."
    rationale: "Report Symbols / APIs names `BORDER` `#999999` among `SelectListView`'s constants, and Coverage owes new rows in §Color Palette; `select-colours` keeps the sheet's border as it stands, so this is a new row and no edit to one."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:10
note: "The report states no value for the open list's plain row pair, box background or thumb, so no row is proposed for them; no retired item stands in this file, and color-palette:120's absence of tokens holds."
