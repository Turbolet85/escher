proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 53
    section: "## Conventions"
    change: The tests group's sentence on layout modes reads that scenarios run in both layout modes `for incremental in [false, true]` and ends at its own citations, with no exception standing; the clause naming upstream's `pseudo_element_flow_classification` as not asserting both layout modes, with `incremental_oracle` green beside it, is removed.
    sidecar: Conventions, Tests - upstream's both-modes exception ended, since `pseudo_element_flow_classification`'s three tests now run in both layout modes (2026-10-31-engine-debts).
    rationale: The report's **Retired** names `upstream's both-modes exception`, the three tests now running `for incremental in [false, true]`, and its **Expected amendments** names the exception at conventions.md:53 as ended, pattern `flow-class`.
    basis: tests/blitz-tests/tests/pseudo_element_flow_classification.rs:44-54
note: D-arch-resources and D-arch-decisions find nothing in this category, and body line .andromeda/architecture.md:121 already states both layout modes with no exception, so it needs no edit for this retirement.
