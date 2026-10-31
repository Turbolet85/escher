proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 20
    section: "## Spacing"
    change: "- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)"
    sidecar: spacing gains the engine default padding of a multiple select's rows, 0 2px, added to the default sheet by 2026-10-31-engine-debts
    rationale: the report's Changes, Schema / config, adds the row rule with padding 0 2px to the default sheet at default.css:161-188; the file holds the sheet's body margin at line 20 and no value for these new rows, and this chunk's New text lists the rule at 173-177
    basis: packages/blitz-dom/assets/default.css:173-177
note: D-design-tokens reads no hardcoded-cross flag in the report's Coverage; the only spacing literal among new engine UI is this default-sheet padding, and the `.cell.current` outline-offset is a stand's own value, so it is not here.
