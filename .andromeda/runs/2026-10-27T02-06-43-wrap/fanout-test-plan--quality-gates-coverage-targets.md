proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: '## 10. Quality Gates & Coverage Targets'
    change: 'Append to the line, after its last reading: «; since 2026-10-26-codebase-health, which factored the lint findings walk into the private `readings` and `placement`, escher-driver reads 4930/5031 = 97.99 % (+12 counted lines, +12 covered), dioxus-native-dom 2741/3202 = 85.60 % and blitz-test-harness 952/1008 = 94.44 % unchanged, the floors and `coverage_floor.py` unchanged, `coverage floor: 3 of 3 crates at or above their floors` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, its `ci-leg.sh coverage` gate)»'
    sidecar: 'test-plan §10 coverage floors: escher-driver reading 4918/5019 → 4930/5031 = 97.99 % at 2026-10-26-codebase-health, the other two crates and the floors unchanged'
    rationale: 'The report (Counts / qualifiers moved) moves escher-driver coverage 4918/5019 = 97.99 % → 4930/5031 = 97.99 % (+12 counted, +12 covered), the floor 97 % holding, the other two crates unchanged, and names this line (quality-gates-coverage-targets.md:16, last reading 4918/5019) as the one changed site; Owners names it for P2'
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:53
note: D-tests-coverage, D-tests-framework and D-tests-obs-harness hold for this category — the new private helpers carry integ/unit tests (Coverage of new surfaces), no runner changes, no harness/gate surface changed; the stand-repeat tallies (204 · 864 · 0 · 21) and the body §10 lines are unmoved; the sweep-reach disproved claim names no line of this file.
