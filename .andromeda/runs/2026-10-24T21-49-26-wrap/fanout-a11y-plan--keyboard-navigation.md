proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 100
    section: "## 5. Keyboard Navigation"
    change: "In the select key and typeahead check's row, replace the clause \"— neither file in the a11y leg's six (§9), both three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list (.github/scripts/ci-leg.sh:54-58)\" with \"— both files in the `a11y` leg since 2026-10-24-a11y-re-read, among its fourteen `--test` targets (§9; .github/scripts/ci-leg.sh:39), and both still three rounds in the `stand-repeat` leg, `stand_act_select` by its `stand_` prefix and `select_interaction` by the leg's named `--test` list (.github/scripts/ci-leg.sh:54-58) — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md\"; every other clause of the row (the 7 and 5 tests, the tables, the cites) unchanged."
    sidecar: "keyboard-navigation.md:100 — select_interaction and stand_act_select now run in the a11y leg (fourteen targets), stand-repeat placement kept; the retired \"neither file in the a11y leg's six\" corrected (2026-10-24-a11y-re-read)."
    rationale: "The report's Symbols / APIs bullet: the `a11y` leg now runs fourteen `--test` targets, `select_interaction` and `stand_act_select` among the eight joining, each keeping its `stand-repeat` placement (named list / `stand_*` glob); Counts / qualifiers moved, site sweep `a11y-leg` and `outside-leg` both list `a11y-plan-detail/keyboard-navigation.md:100` (\"neither file in the a11y leg's six\") as changed; Expected amendments name this line (\"`select_interaction` and `stand_act_select` in the leg, `stand-repeat`'s placement kept\"). Harness / gate surface cites the leg at `.github/scripts/ci-leg.sh:39`."
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:9
note: D-a11y-surface and D-a11y-obs-schema unfired (no interactive UI element added, no violation or log schema changed; Retired none); every other leg-naming row of this file (:97-:99) names no a11y-leg count, and :106's "17 listed s06 files" is the a11y-tally pattern's false match, no change.
