proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "In the sentence on the three a11y witnesses outside the `stand_` prefix, replace «their membership held by `stand_requirement_sweep`'s leg check» with «their names read, in `ci-leg.sh`'s whole text, by `stand_requirement_sweep`'s leg check»; the rest of the sentence unchanged."
    sidecar: "ci-integration: the stand-repeat list's membership no longer credited to `stand_requirement_sweep`'s leg check alone (it reads ci-leg.sh whole) — 2026-10-26-codebase-health"
    rationale: "The report's Spec claims disproved by measurement names `a11y-plan-detail/ci-integration.md:9` and measures that the leg check reads `ci-leg.sh` whole by `contains`, so `--test select_interaction` dropped from stand-repeat alone (ci-leg.sh:57) stays green while the a11y leg (ci-leg.sh:39) still names it; the clause overstates its reach."
    basis: ".andromeda/a11y-plan-detail/ci-integration.md:9"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Follow line 9 with: «`stand_requirement_sweep`'s leg check reads `ci-leg.sh`'s text whole, which names `--test select_interaction` in both the a11y leg (`ci-leg.sh:39`) and the stand-repeat list (`ci-leg.sh:57`), so it cannot see a target dropped from the stand-repeat list alone while the a11y leg still names it — only both sites removed read red; a measured reach gap that predates 2026-10-26-codebase-health, its owner placed at that chunk's wrap (P5) — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md.»"
    sidecar: "ci-integration: the sweep leg check's reach gap over a name the a11y leg and stand-repeat both carry recorded — 2026-10-26-codebase-health"
    rationale: "The report's Spec claims disproved by measurement and its Deviations (Step 4 control (a)) measure the gap; Owners names P5 for its owner; the task's F1."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Follow the F1 line with: «Stand-repeat membership of the a11y-leg files is held jointly by two checks — `stand_requirement_sweep`'s leg check and `.github/scripts/test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds` (`:361-366`), which asserts `select_interaction` among the stand-repeat leg's own logged words — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md.»"
    sidecar: "ci-integration: stand-repeat membership of the a11y-leg files held jointly by the sweep leg check and test_stand_repeat_leg_runs_three_rounds — 2026-10-26-codebase-health"
    rationale: "The report's Spec claims disproved by measurement reads `test_ci_workflows.py:361-366` asserting `select_interaction` among the stand-repeat leg's logged words, so membership is held by the two checks jointly, not the sweep test alone; the task's F2."
    fact: F2
note: "The same disproved sweep-reach claim stands in other masters, not a11y-plan's — architecture `existing-scopes.md:115` and `standard-contracts.md:273` (test-plan `test-scope-summary.md:120` states it exactly); D-a11y-surface and D-a11y-obs-schema fire nothing (no interactive element, no schema change); ci-integration's counts (a11y 14 · 85, stand-repeat eight files) re-read unmoved."
