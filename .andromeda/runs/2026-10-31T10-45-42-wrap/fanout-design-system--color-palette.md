proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 29
    section: "## Color Palette › ### Core Colors"
    change: "After line 29 add one table row: | Default `multiple` select chosen row | `#c8c8c8` / `#000000` | Engine default `background-color` / `color` of a `multiple` select's chosen row, directly under the select or under its `optgroup`: Chromium's inactive selected pair, 12.56:1, hardcoded by design as the engine holds no tokens (packages/blitz-dom/assets/default.css:184-187; tests/blitz-tests/tests/select_interaction.rs:691-712) |"
    sidecar: "Color Palette › Core Colors: added the engine-default chosen-row pair of a `multiple` select, `#c8c8c8` / `#000000`, 12.56:1 (2026-10-31-engine-debts, F6)."
    rationale: "The report's Changes › Schema / config names the new engine-default chosen-row pair in the default sheet, measured by `select_interaction`, and Coverage of new surfaces states the same values; the Color Palette focus holds every token a test asserts, and the table holds no row for this pair."
    basis: packages/blitz-dom/assets/default.css:184-187
note: D-design-tokens is not violated, as no Coverage flag reads `hardcoded✗` (the `multiple` select's pair is flagged hardcoded by design, the `cells-grid` outline `#3a5ce5✓` is the stand's held accent and no new token); the report's Expected amendments lands this pair under §Surface: desktop-native, so that detector's row should point to this literal rather than copy it.
