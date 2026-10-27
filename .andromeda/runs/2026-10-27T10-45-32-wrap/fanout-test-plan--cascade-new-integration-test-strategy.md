proposals:
  - detector: D-new-detail
    fact: F2
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 5
    section: "## 5. Integration Test Strategy"
    change: "a new group before the Script ↔ DOM group, in its own `<!-- detail: procedure · integration -->` … `<!-- /detail -->` block: - **Layout assertions ↔ both layout modes:** since 2026-10-27-upstream-sync-engine-debts upstream's `pseudo_element_flow_classification` (3 tests — every box-generating `::before`/`::after` votes in flow classification) is an exception to the rule that pipeline tests run `for incremental in [false, true]`: the file asserts neither mode pair, and `incremental_oracle`, green beside it, is the layout-identity witness (§2 Differential oracle) (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:11-22; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78; as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)"
    sidecar: "integration-test-strategy: a layout-assertions group records `pseudo_element_flow_classification` as an exception to the both-layout-modes rule, `incremental_oracle` its layout-identity witness (2026-10-27-upstream-sync-engine-debts)"
    rationale: "the report's Changes reads 'pseudos voting in flow classification → tests integ (`pseudo_element_flow_classification` 3, both layout modes not asserted by the file; `incremental_oracle` green)' (report.md:87); no group of the file details the body's Layout assertions line, so the fact opens one, binding with §2's Differential oracle (test-plan.md:87)"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:87
note: the new group binds with §2 Test Strategy's Differential oracle (test-plan.md:87); the body's §5 Layout assertions line may want a pointer clause to it, which is the strong readers' call
