proposals:
  - detector: D-design-tokens
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 21
    section: "## Spacing"
    change: Add item after this line - "- blitz-dom single select list, engine-painted, takes its row padding `ROW_PADDING_X` and `ROW_PADDING_Y`, group indent `GROUP_INDENT` and border width `BORDER_WIDTH` from engine constants, not tokens, the engine holding none (packages/blitz-dom/src/node/select_list.rs)"
    sidecar: Spacing gains the select list's row padding, group indent and border width as untokenized engine constants (2026-10-31-select-popup)
    rationale: Report Coverage reads the select list's tokens as hardcoded✗, engine constants on his F6 with the engine holding no tokens, and Symbols / APIs names these constants; the Expected amendments list §Spacing rows as new text with no master site standing
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 28
    section: "## Spacing"
    change: Add item after this line - "- `size` on a `multiple` select hints `height` of `size` × 1.2em, parsed by `size_rows` and capped at 65534, `0` or a bad value hinting nothing; its rows take `line-height` 1.2 and the select `overflow-y` auto; a single select's `size` stays unread (packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193)"
    sidecar: Spacing gains the `size` height hint and row line-height on a `multiple` select (2026-10-31-select-popup)
    rationale: Report Symbols / APIs under `size` states the hint, the cap and the sheet rule, and Retired names «a `multiple` select's `size` is unread»; the file's attribute-to-spacing mappings hold no row for it
    basis: packages/blitz-dom/src/stylo.rs:1124-1139
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 32
    section: "## Spacing"
    change: Add item after this line - "- A single select's open list shows at most `MAX_ROWS` = 20 rows and scrolls past them, each row the height of the select's font on one line (`row_height`, packages/blitz-dom/src/node/select_list.rs:28-46)"
    sidecar: Spacing gains the select list's row cap and row height (2026-10-31-select-popup)
    rationale: Report Symbols / APIs states `MAX_ROWS` = 20 and the `row_height` field, and Deviations states the row height is the select's font on one line; the Expected amendments list §Spacing rows as new text with no master site standing
    basis: packages/blitz-dom/src/node/select_list.rs:28-46
note: the report names ROW_PADDING_X, ROW_PADDING_Y, GROUP_INDENT and BORDER_WIDTH without values or lines, so the first proposal states no pixel figures and no line in its citation; the file holds no passage of any Retired item.
