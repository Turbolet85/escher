proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 19
    section: "## 1. Test Scope Summary"
    change: "after «the apps named still hold no tests» add: «; since 2026-10-24-a11y-re-read the stand's normal dependency graph is measured to hold neither platform accessibility adapter, `accesskit_xplat` nor `accesskit_winit` (`cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` → `0`), so no check in the `a11y` leg reaches the adapter crossing — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md»"
    sidecar: "test-scope-summary: the apps entry gains the measured fact that the stand's normal graph holds neither platform adapter, so no a11y-leg check reaches the adapter crossing (2026-10-24-a11y-re-read, F5)"
    rationale: "the report's Dependencies bullet and its Outcome gate both read the `cargo tree` adapter probe at `0`; the entity names `accesskit_xplat` among the apps holding no tests, the one clause F5's adapter-free stand graph can sit beside"
    basis: escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md:11
    fact: F5
note: test-scope-summary#1.1's cite `.github/scripts/test_ci_workflows.py:381-495` moved +3 to `:384-498` (lines 299-302 added, one removed) — left to `cites.py apply` as the report states; confirm the sweep re-pointed it.
