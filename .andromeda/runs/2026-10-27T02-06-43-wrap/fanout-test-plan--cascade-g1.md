proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 120
    section: ## 1. Test Scope Summary
    change: after «holding the `tests/blitz-tests/tests/stand_*.rs` glob and `--test {name}` for all 18» add «; since 2026-10-26-codebase-health that third test asserts each requirement row through the private `#[track_caller]` helpers `assert_0_2_0_row` and `assert_0_2_0_witness`, its target predicate the private `inside_stand_repeat` holding the path prefixes `BLITZ_TESTS` and `SEVEN_GUIS`, its rows, witnesses, table-order and leg checks and every message unchanged — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md»
    sidecar: test-scope-summary — the 0.2.0 sweep test's per-row and per-witness assertions now in private `#[track_caller]` helpers, its target predicate `inside_stand_repeat` (2026-10-26-codebase-health, F4)
    rationale: the report's Symbols / APIs bullet for stand_requirement_sweep.rs (`assert_0_2_0_row` :1050-1073, `assert_0_2_0_witness` :1075-1093, `inside_stand_repeat` inside :1046-1107, every message unchanged) and New facts F4; the entity's line 120 describes this test's assertions and is the line that carries it
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:39
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 120
    section: ## 1. Test Scope Summary
    change: beside the same sentence add «; that leg check reads `ci-leg.sh` as one whole text by `contains`, so a named target dropped from the `stand-repeat` leg alone stays green while the `a11y` leg still names it — only both sites removed read red — and stand-repeat membership is held jointly with `.github/scripts/test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds`, which asserts `select_interaction` among that leg's logged words; a gap predating 2026-10-26-codebase-health, its owner placed at that chunk's wrap P5 — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md»
    sidecar: test-scope-summary — the 0.2.0 sweep test's leg check holds stand-repeat membership only jointly with `test_stand_repeat_leg_runs_three_rounds` (measured at 2026-10-26-codebase-health, F5)
    rationale: the report's Spec claims disproved by measurement (row-guard control (a): `--test select_interaction` dropped at `ci-leg.sh:57` alone leaves the test green while `ci-leg.sh:39` names it) and New facts F5; line 120 states the read exactly, so the reach limit is an addition beside it, not a correction
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:134
    fact: F5
note: no line of this entity credits stand-repeat membership to the sweep test alone (the report rules line 120 exact); the three over-reaching sentences live in architecture existing-scopes.md:115, standard-contracts.md:273 and a11y-plan ci-integration.md:9, and test-plan's §9 Legs / §10 Flakiness-budget lines (ci-integration, quality-gates-coverage-targets) are another group's — the cascade-check's or their categories' to dispose; no detector of D-tests-coverage · D-tests-framework · D-tests-obs-harness fires (the new private paths are covered by `stand_lint` 7 and controls (a)–(e), runners unchanged, no harness or log change)
