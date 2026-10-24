proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 28
    section: "## 4. Unit Test Strategy"
    change: "in the `LegScriptTest` clause, `two \`stand-repeat\` cases (the three rounds; the stop at the first red)` → `two \`stand-repeat\` cases (the three rounds — since 2026-10-24-stand-requirement-sweep \`test_stand_repeat_leg_runs_three_rounds\` also pinning the ten named \`--test\` words the leg runs after the \`stand_*.rs\` glob (.github/scripts/test_ci_workflows.py:359-362); the stop at the first red)`"
    sidecar: "unit-test-strategy: the stand-repeat three-rounds case of test_ci_workflows.py now also pins the leg's ten named --test words (2026-10-24-stand-requirement-sweep)"
    rationale: "The report's Changes › Schema / config states `.github/scripts/test_ci_workflows.py` `test_stand_repeat_leg_runs_three_rounds` (359-362) asserts the ten words, and Counts fact 2 names the leg pin gaining them; this file's line 28 is where that case is described (`the three rounds`) and still names only the rounds. The sweep's leg-select no-change on :28 reads the line's leg naming, not what the case now pins — new detail for the unit-test record, one item in the existing clause."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:72
note: "No drift under D-tests-coverage (test code only; the new stand-repeat selection is pinned by test_ci_workflows.py and stand_requirement_sweep's leg check), D-tests-framework (cargo test / unittest, on spec) or D-tests-obs-harness (agent-run selections unchanged). The 4 lines added at test_ci_workflows.py:359-362 shift this file's later citations of that file (line 28's `:323-371` end, line 29's `:378-492`, `:422-456`, `:103-496`) by +4 — a citation re-point for cites.py, not proposed here."
