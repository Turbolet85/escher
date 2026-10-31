proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 210
    section: "## 1. Test Scope Summary"
    change: "Add after this line: - **tests/blitz-tests — `stand_keyboard`'s closed-arrow test with a list to open:** it passes untouched, yet its Enter and Space rows now open the list and then choose, so it reads true and no longer reads what its row order suggests — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: records that `stand_keyboard`'s closed-arrow test stays green while its Enter and Space rows now open and then choose."
    rationale: "The report's Decisions & corrections states this of the closed-arrow test, and task fact F5 places it in this category; the entity picked as add-to is this category's `stand_keyboard` row."
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 228
    section: "## 1. Test Scope Summary"
    change: "«and five more of the targets the leg names» now reads «and seven more of the targets the leg names, `select_popup` and `select_popup_painted` among them»; the entity's measured tail adds «, the count at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»; the rest stands."
    sidecar: "test-scope-summary: `test_stand_repeat_leg_runs_three_rounds` asserts seven further `stand-repeat` targets beside `select_interaction`, up from five."
    rationale: "The report's Harness / gate surface: `stand-repeat`'s list gains two words at `ci-leg.sh:59-60` and `test_ci_workflows.py:367` asserts them; the `repeat-names` pattern counts the leg's fifteen → seventeen and does not reach this wording."
    basis: .github/scripts/test_ci_workflows.py:367
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 237
    section: "## 1. Test Scope Summary"
    change: "The line now reads: - **The lint's select-option reading:** no unit test pins it — escher-driver's `overlaps` (`packages/escher-driver/src/lint.rs`) skips, on either side, an entry in a select's box, `Entry.in_select_box`: true inside a closed select, read through `closed_select_of`, the element and never the role, the snapshot still giving each such option its select's client rect, and true also for an open list's row or group, an open row's `clip_region` coming from `visible_region` alone with no `contain: paint` cut; it lints a `multiple` select's rows as usual; `stand_lint` pins the stand at `[0, 0, 0, 0]` at boot (§1 above), its `an_open_lists_rows_read_no_overlap_on_the_stand` reads an open list's rows, and `stand_act_popup`'s `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads` reads a row with the planted pair still reading — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "test-scope-summary: the lint's select exclusion restated on `Entry.in_select_box`, retiring `Entry.in_closed_select`, with an open row's clip and the two checks that read it."
    rationale: "The report's Retired bullet names `Entry.in_closed_select`; its Symbols / APIs gives `in_select_box` and `clip_region`, its `closed-option` and `lint-exclude` rows list test-scope-summary:237 as changed, and its Outcome's (lint) row names both checks."
    basis: packages/escher-driver/src/lint.rs:339-342
note: test-scope-summary's F2, F3, F4, F6 and F7 fit no picked entity here and go to the new-detail pass; `stand_keyboard`'s two new tests are flight-type walks with no F{n}, beside the count at test-scope-summary:151 and not on the Cells row picked.
