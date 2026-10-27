proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 42
    section: "## 9. CI Integration"
    change: "Append to the Per-OS test tallies record, after its last reading (CI#40134371519): re-read at 2026-10-26-codebase-health on CI#40434241601, the pre-CI commit `3c04042ed` (green 17/17): linux 208 · 1117 · 0 · 12; macos and coverage 189 · 1114 · 0 · 12; windows 189 · 1068 · 0 · 9; the a11y job 14 · 85 · 0 · 0; the stand repeat 204 · 864 · 0 · 21 — every job equal to the base run CI#40389488295 (`19f9ff91ca`), the chunk moving no tally; the move since CI#40134371519 (+2 result lines and +12 passed on linux, macos and coverage, +2 · +11 on windows, +1 passed on the a11y job, +6 · +27 on the stand repeat) predates this chunk's base and was never written here — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md"
    sidecar: "test-plan §9 CI Integration · Per-OS test tallies: the 2026-10-26-codebase-health re-read on CI#40434241601 recorded, equal to its base run, the unwritten move since CI#40134371519 stated"
    rationale: "The report's Cross-project / external claims state CI#40434241601's six job tallies on 3c04042ed096, green 17/17, each equal to the base run CI#40389488295; the category's Per-OS test tallies record ends at CI#40134371519 (linux 206 · 1105, a11y 14 · 84, stand repeat 198 · 837), so the standing reading is stale. No detector invariant is violated: the chunk's new private helpers carry integration and unit controls (Coverage of new surfaces), no runner or framework changed, and the harness, CI legs and log format are unchanged (Harness / gate surface: none); Retired: none."
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:116
note: The report's disproved sweep-reach claim names other masters' files; this category states stand-repeat membership as pinned by test_stand_repeat_leg_runs_three_rounds (lines 9-10), which the report confirms, so no edit is owed here.
