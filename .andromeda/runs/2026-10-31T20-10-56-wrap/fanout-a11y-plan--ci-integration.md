proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "Line 11's «`stand_keyboard` 21 and `stand_contrast` 7 (52)» becomes «`stand_keyboard` 23 and `stand_contrast` 7 (54)», and «14 result lines · 112 passed · 0 failed · 0 ignored, locally and on the ... job of CI#41192433140 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)» becomes «14 result lines · 114 passed · 0 failed · 0 ignored, locally and on the ... job of CI#41235503227 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»; the eight per-file counts summing to 60 stand."
    sidecar: "§9 CI Integration: the a11y leg's log moves to stand_keyboard 23, 14 · 114 · 0 · 0, on CI#41235503227."
    rationale: "Report Counts / qualifiers moved, `kbd-count` (a11y ci-integration:11), `a11y-tally` and `ci-tally` (a11y ci-integration:11); the gate reads `lines 14 passed 114 failed 0`; 52 becomes 54 by the same two tests."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Line 10's «14 files and 112 tests (`stand_keyboard` 21), over the tier-0 witnesses alone, 9 files and 86 tests» becomes «14 files and 114 tests (`stand_keyboard` 23; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md), 112 tests (`stand_keyboard` 21) when he chose, over the tier-0 witnesses alone, 9 files and 86 tests», the fork's 112 and 86 kept as his choice's figures."
    sidecar: "§9 CI Integration: the leg's stated size moves to 114 tests with stand_keyboard 23; the founder's fork figures stay dated to his choice."
    rationale: "Report `kbd-count` names a11y ci-integration:10; `a11y-tally` moves the leg to 114, so a line reading 112 beside `stand_keyboard` 23 would not add up; the report states no new tier-0-alone figure, so 86 stays as the fork's reading."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:10
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Line 9's «the list's fifteen words» becomes «the list's seventeen words, `select_popup` and `select_popup_painted` joining at 2026-10-31-select-popup»."
    sidecar: "§9 CI Integration: stand-repeat's named --test list moves from fifteen words to seventeen."
    rationale: "Report `repeat-names`, fifteen to seventeen, naming a11y ci-integration:9; Harness / gate surface, `ci-leg.sh` stand-repeat's list gains two words and `test_ci_workflows.py:367` asserts them."
    basis: .github/scripts/ci-leg.sh:59-60
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 12
    section: "## 9. CI Integration"
    change: "After line 12, a new record, one leading space as its neighbours: «Since 2026-10-31-select-popup the select list's own checks stand beside the leg and not in it: `select_popup` (25) and `select_popup_painted` (6), named `--test` words of `stand-repeat`, and `stand_act_popup` (12), by its `stand_*` prefix in `run stand` and `stand-repeat`, are among the checks asserting SC 2.1.1, 2.4.3 and 1.4.3 and the tree's reading in the harness, what an assistive technology reads unmeasured and the windowed list read and not run; the `a11y` leg stays fourteen files, its tally moved by `stand_keyboard`'s two new tests (.github/scripts/ci-leg.sh:59-60; .github/scripts/test_ci_workflows.py:367 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»"
    sidecar: "§9 CI Integration: records that the select list's three new checks run in stand-repeat and not in the a11y leg, which stays fourteen files."
    rationale: "Report Harness / gate surface: `stand_act_popup` joins `run stand` and `stand-repeat` by its prefix and the `a11y` leg stays fourteen files; Coverage of new surfaces gives the test counts, the SCs asserted, an assistive technology's reading `unrunnable-here` and the windowed list read and not run."
    basis: .github/scripts/test_ci_workflows.py:367
note: "Line 13's `STATE_ROWS` 18 (now 20, report `state-rows`) is left as a dated record of 2026-10-25-cold-agent-bar, as its neighbours `stand_keyboard` 14 and `stand_contrast` 5 are; D-a11y-surface, D-a11y-obs-schema and D-retired find nothing in this file."
