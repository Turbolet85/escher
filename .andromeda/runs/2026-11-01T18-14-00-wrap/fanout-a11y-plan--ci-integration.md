proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "New record after this line. After the upstream take's twelve engine behaviour changes and the panic's fix the a11y leg keeps its fourteen files and the a11y job of CI#41333172440 reads 14 · 114 · 0 · 0, unmoved against CI#41235503227; `stand_keyboard` and `stand_contrast` pass with every table and row unedited, `text_contrast`'s new `None` past a blend moving no row, and `inert_subtree` holds no hunk and reads green (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)."
    sidecar: "2026-11-01-upstream-sync-ui-graph: §9 records the a11y job's reading on CI#41333172440, 14 · 114 · 0 · 0, unmoved past the take, with no stand table or row edited."
    rationale: "The report's job-log tally reads a11y 14 · 114 · 0 · 0 on CI#41333172440 against CI#41235503227 (report.md:36, :159), and its a11y criteria hold the stand tables and inert_subtree unedited (report.md:140, :146); this file records the leg's latest reading at line 11 and readings past engine changes at line 14."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:159
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 13
    section: "## 9. CI Integration"
    change: "New record after this line. `select_popup_painted` also runs, with `--features blitz-paint/font-embolden`, as the `embolden` leg, last in the local `fast` leg, with no CI job of its own, macOS exercising the feature in CI; it is still none of the a11y leg's fourteen files (.github/scripts/ci-leg.sh:8-9; .github/scripts/ci-leg.sh:28-29 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)."
    sidecar: "2026-11-01-upstream-sync-ui-graph: §9 adds the `embolden` step of `fast` to the legs `select_popup_painted` runs in."
    rationale: "Line 13 lists the legs that run `select_popup_painted`, the list's SC 1.4.3 witness per a11y-plan §6, and the report adds one: a new leg `embolden` in `LEGS` and last in `FAST_LEGS`, no CI job, macOS exercising the feature there (report.md:64)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:64
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 13
    section: "## 9. CI Integration"
    change: "New record after this line. `key_press_after_container_lost_its_box`, 3 tests in both layout modes, asserts that a key on a focused control whose anonymous block was dropped does not panic and still moves a select's choice; it runs in the workspace `test` leg, has no `stand_` prefix, and is in neither the a11y leg's fourteen files nor the `stand-repeat` list's seventeen words (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md)."
    sidecar: "2026-11-01-upstream-sync-ui-graph: §9 places the new key-press check in the workspace `test` leg, outside the a11y and `stand-repeat` legs."
    rationale: "The report adds this focus and key check with no `stand_` prefix, among the workspace tally's +2 result lines, and says it joins no leg, the stand-repeat's seventeen and the a11y leg's 14 · 114 unmoved (report.md:27, :36, :60, :114); line 13 records the same placement for the select list's checks."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:60
note: D-a11y-surface, D-a11y-obs-schema and D-retired find no violated passage in this file; the report adds no interactive UI element or schema change and no Retired item stands here.
