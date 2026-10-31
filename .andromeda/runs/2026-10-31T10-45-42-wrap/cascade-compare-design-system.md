# Cascade compare — design-system · wrap · 2026-10-31-engine-debts
E items 5 · held 2 · lost at classify 0 · lost at strong 1 · beside a placed fact 1 · cascade only 3 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### color-palette#2.5 · lost at classify · beside a placed fact F7 (near 27, color-palette#2.3) · .andromeda/design-system-detail/color-palette.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 29
    section: "## Color Palette › ### Core Colors"
    change: "After line 29 add one table row: | Default `multiple` select chosen row | `#c8c8c8` / `#000000` | Engine default `background-color` / `color` of a `multiple` select's chosen row, directly under the select or under its `optgroup`: Chromium's inactive selected pair, 12.56:1, hardcoded by design as the engine holds no tokens (packages/blitz-dom/assets/default.css:184-187; tests/blitz-tests/tests/select_interaction.rs:691-712) |"
    sidecar: "Color Palette › Core Colors: added the engine-default chosen-row pair of a `multiple` select, `#c8c8c8` / `#000000`, 12.56:1 (2026-10-31-engine-debts, F6)."
    rationale: "The report's Changes › Schema / config names the new engine-default chosen-row pair in the default sheet, measured by `select_interaction`, and Coverage of new surfaces states the same values; the Color Palette focus holds every token a test asserts, and the table holds no row for this pair."
    basis: packages/blitz-dom/assets/default.css:184-187

### spacing#2.5 · lost at strong · .andromeda/design-system-detail/spacing.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 20
    section: "## Spacing"
    change: "- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)"
    sidecar: spacing gains the engine default padding of a multiple select's rows, 0 2px, added to the default sheet by 2026-10-31-engine-debts
    rationale: the report's Changes, Schema / config, adds the row rule with padding 0 2px to the default sheet at default.css:161-188; the file holds the sheet's body margin at line 20 and no value for these new rows, and this chunk's New text lists the rule at 173-177
    basis: packages/blitz-dom/assets/default.css:173-177

## Held

- depth-strategy#3.1
- surface#1.2

## Cascade only

- color-palette#2.3
- color-palette#2.4
- motion#1.3
