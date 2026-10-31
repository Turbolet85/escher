proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 53
    section: "## Conventions"
    change: "In the both-modes rule, delete the clause «, except upstream's `pseudo_element_flow_classification`, which does not assert both layout modes, `incremental_oracle` green beside it», so the rule reads «scenarios run in both layout modes `for incremental in [false, true]`» with no exception standing."
    sidecar: "conv-tests: upstream's both-modes exception ended; `pseudo_element_flow_classification`'s three tests now run in both layout modes (2026-10-31-engine-debts)."
    rationale: "The report's Retired names `upstream's both-modes exception`, `pseudo_element_flow_classification`'s three tests now running `for incremental in [false, true]`; its Expected amendments and the `flow-class` row name `conventions.md:53` as changed."
    basis: .andromeda/architecture-detail/conventions.md:53
note: "Body architecture.md:121 already reads scenarios run in both layout modes with no exception, so no body edit is proposed; the fixture-reader count 24 to 28 sits at test-plan.md:151, test-strategy.md:53 and obs-plan.md:92, not in conventions."
