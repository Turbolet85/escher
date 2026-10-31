proposals:
  - detector: D-new-detail
    severity: warning
    fact: F12
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 24
    section: "## 7. Test Data & Fixtures"
    change: "| Select option needles | `option_needles()` in seven_guis' `tests/common/mod.rs`: the needle list of a select's options, read from a snapshot | (examples/seven_guis/tests/common/mod.rs:425-453) |"
    sidecar: "test-data-fixtures: new data-seeds row for seven_guis' `option_needles()`, added by 2026-10-31-select-popup"
    rationale: "The report's New text, by line lists `pub fn option_needles() -> Vec<String>` at seven_guis' common/mod.rs 425-453, reading `select` from `snapshot`, and its Changes name common/mod among the checks; no row of this file holds it."
    basis: examples/seven_guis/tests/common/mod.rs:425-453
note: "The siblings `stand_needles`, `cells_needles` and `recorded_needles` stand at test-plan-detail/test-scope-summary.md:14, so the orchestrator may prefer that home; I saw this through one read-only grep over test-plan-detail/*.md, beyond this task's named paths, whose glob also covered the category file outside detail.py (no hit there)."
