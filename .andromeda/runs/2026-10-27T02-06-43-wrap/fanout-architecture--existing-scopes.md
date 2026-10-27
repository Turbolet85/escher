proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: "## Existing Scopes"
    change: "In the `WITNESSES_020` item, replace «every other witness found inside `stand-repeat` by the `stand_` prefix or by the named `--test` list of `ci-leg.sh`'s own text» with «every other witness's target found by the `stand_` prefix or by its `--test` name anywhere in `ci-leg.sh`'s own text, read whole by `contains` — so a target dropped from the `stand-repeat` list alone while another leg still names it reads green (measured on `select_interaction`, which the `a11y` leg also names), its `stand-repeat` membership held jointly with `.github/scripts/test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds`, which asserts `select_interaction` among that leg's own logged words»; the clause's citations unchanged."
    sidecar: "existing-scopes: the 0.2.0 sweep test's reach restated — it reads `ci-leg.sh` whole, so it does not see a target dropped from `stand-repeat` alone while another leg names it; membership held jointly with `test_stand_repeat_leg_runs_three_rounds` (2026-10-26-codebase-health)."
    rationale: "The report's Spec claims disproved by measurement names `existing-scopes.md:115`'s sentence as disproved — implement's row-guard control (a) dropped `--test select_interaction` from `stand-repeat` alone (`ci-leg.sh:57`) and the sweep test stayed green while the `a11y` leg (`ci-leg.sh:39`) named it; red only with both sites removed; the property predates the chunk; disposition Validate check 6 (P2), the gap's owner P5's."
    basis: escher-0.3.0/chunks/2026-10-26-codebase-health/report.md:134
note: D-arch-resources and D-arch-decisions hold for this category (no public symbol, crate, module, dependency or resource added; private helpers only, none of which the file names); the sweep test's reach gap itself still needs its P5 owner, so the amended sentence names no residuals line.
