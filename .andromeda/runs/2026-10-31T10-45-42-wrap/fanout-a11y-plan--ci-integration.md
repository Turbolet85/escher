proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Replace `the list's ten words pinned by` with `the list's fifteen words (ten until 2026-10-31-engine-debts named five more targets) pinned by`; the rest of the line stands."
    sidecar: "2026-10-31-engine-debts: stand-repeat's named list, ten words to fifteen, pinned by test_stand_repeat_leg_runs_three_rounds."
    rationale: "Report, Counts / qualifiers moved: stand-repeat's named targets ten to fifteen; Harness / gate surface: `ci-leg.sh` stand-repeat names five more targets and `test_stand_repeat_leg_runs_three_rounds` names them too. This line still states ten as standing."
    basis: .github/scripts/test_ci_workflows.py:365-367
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Replace `14 files and 85 tests` with `14 files and 112 tests, 85 before 2026-10-31-engine-debts`, the parenthetical and the comparison with 9 files and 59 tests standing."
    sidecar: "2026-10-31-engine-debts: the a11y leg's tally, 14 files, 85 tests to 112."
    rationale: "Report, Counts / qualifiers moved, `a11y-tally`: `a11y` leg 14 · 85 to 14 · 112, fourteen files unmoved, this line named as changed; CI#41192433140 Accessibility reads 14 · 112 · 0 · 0."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Replace `(`stand_keyboard` 15)` with `(`stand_keyboard` 21)`."
    sidecar: "2026-10-31-engine-debts: stand_keyboard in the a11y leg's tally, 15 tests to 21."
    rationale: "Report, Counts / qualifiers moved, `n-keyboard`: `stand_keyboard` 15 to 21, this line named as changed."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "In the leg's log replace `accessibility_states 5` with `accessibility_states 6`."
    sidecar: "2026-10-31-engine-debts: accessibility_states in the a11y leg's log, 5 tests to 6."
    rationale: "Report, Counts / qualifiers moved, `n-a11y-states`: 5 to 6, this line named as changed."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "In the leg's log replace `inert_subtree 4` with `inert_subtree 6`."
    sidecar: "2026-10-31-engine-debts: inert_subtree in the a11y leg's log, 4 tests to 6."
    rationale: "Report, Counts / qualifiers moved, `n-inert`: 4 to 6, this line named as changed."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "In the leg's log replace `select_interaction 7` with `select_interaction 18`."
    sidecar: "2026-10-31-engine-debts: select_interaction in the a11y leg's log, 7 tests to 18."
    rationale: "Report, Counts / qualifiers moved, `n-select`: 7 to 18, this line named as changed."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "In the leg's log replace `stand_act_select 5` with `stand_act_select 10`."
    sidecar: "2026-10-31-engine-debts: stand_act_select in the a11y leg's log, 5 tests to 10."
    rationale: "Report, Counts / qualifiers moved, `n-act-select`: 5 to 10, this line named as changed."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "Keep the log's totals with its moved rows and its run: replace `the six unmoved (43)` with `the six at 52 (stand_contrast 7, stand_keyboard 21)`, `(41)` with `(60)`, `84 passed` with `112 passed`, and `CI#40134371519 (6 · 43 on CI#40125511019), every other job's tally unmoved` with `CI#41192433140, the fourteen files unmoved (84 on CI#40134371519 with every other job's tally unmoved, 6 · 43 on CI#40125511019)`; end the line's measured-at clause with `, and for the 112 at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md and its evidence/counts.md, evidence/ci.md`."
    sidecar: "2026-10-31-engine-debts: the a11y leg's log restated at 14 · 112 on CI#41192433140, the earlier reads kept beside their runs."
    rationale: "Second occurrence of the tally the report moves (`a11y-tally`, 14 · 112, local gate and CI#41192433140). The moved rows make the line's own subtotals false: the six 43 + 2 (`stand_contrast` 5 to 7) + 6 (`stand_keyboard` 15 to 21, 14 in this line's 43 being one more) reads 52 by per-file sum, the eight 60, and 52 + 60 = 112 closes against the report. Replacing the rows alone would attribute them to CI#40134371519, a run that did not read them."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:11
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 12
    section: "## 9. CI Integration"
    change: "Add after this line: `Since 2026-10-31-engine-debts the leg's fourteen files, unmoved, carry that chunk's new rows: `stand_keyboard` and `stand_contrast` the load page's (SC 2.1.1 · 1.4.3) and Cells' (SC 2.1.1 · 1.4.3 · 2.4.3) rows, a person at a window `unrunnable-here`; `inert_subtree` the focus fixup and the tree's window fallback; `accessibility_states` a native control's `toggled`; `select_interaction` and `stand_act_select` a `multiple` select's rows and `optgroup`; green in the leg and on CI#41192433140 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md.`"
    sidecar: "2026-10-31-engine-debts: what the a11y leg's fourteen unmoved files now witness: the served pages' rows, the fixup, tree focus, native toggled and the multiple select."
    rationale: "Report, Coverage of new surfaces and Outcome (F3 to F7, F12, F13): each named check holds the new surface, and `the a11y leg stays fourteen files`, so the tally's rise has one record saying what it runs; expected amendments list a11y-plan §9's leg tally."
note: "Lines 8 and 12 keep `stand_keyboard` 14 and `stand_contrast` 5 as history at their stated runs (report: `n-keyboard`, `n-contrast` history); line 9's `ten` and line 11's totals are this detector's own sweep hits, the report's bare `ci-integration.md:9` not distinguishing this file from test-plan-detail's."
