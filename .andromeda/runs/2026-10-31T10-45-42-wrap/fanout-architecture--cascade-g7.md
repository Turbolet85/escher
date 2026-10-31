proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 92
    section: "## Occupied Resources"
    change: "Add after the CI infrastructure line: - Since 2026-10-31-engine-debts the `Stand checks repeated` job runs `jlumbroso/free-disk-space` at the pinned sha `54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false`, a widening on the operator's word after that job went red on CI#41190771843 with `Free space left: 5 MB`; it ran `completed · success` on CI#41192433140, taking `/dev/root` from 86 G free to 109 G free, and is not shown to be the cure: that runner had 86 GB free before the step, the failed run has no `df` line, and the red job alone among the `ubuntu-24.04` jobs stood on the older image `20261018.96.1`, whose free disk stays unread (.github/workflows/ci.yml:311-316; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md)"
    sidecar: "res-ci-infrastructure gains the `Stand checks repeated` free-disk step, its pinned sha and inputs, and that its cure stays unproven."
    rationale: "The report's Harness / gate surface names the new action site with its sha, place and inputs and says no master states it (`free-disk` control unfired); Insufficient fixes holds the unproven cure, and the Addendum's inputs#I14 adds the older image."
    basis: .github/workflows/ci.yml:311-316
    fact: F23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 92
    section: "## Occupied Resources"
    change: "Add after the CI infrastructure line: - The older free-disk step of the `matrix_test` job is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest`, read `completed · skipped` on `Test (android)` in CI#41190771843, so this workflow held no measured run of the action before `Stand checks repeated` gained its own (.github/workflows/ci.yml:392-394; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md)"
    sidecar: "res-ci-infrastructure records that the matrix job's older free-disk step is skipped on every row."
    rationale: "The report's Harness / gate surface and Spec claims disproved state the step stands and does not fire, with the job state read from CI#41190771843; `free-disk`'s unfired control means no master holds it."
    basis: .github/workflows/ci.yml:392-394
    fact: F24
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 40
    section: "## Cross-cutting Patterns"
    change: "In the changed set's fourth kind of site, `BaseDocument::apply_select_choice` now reads: which marks a select where a key or typeahead or, on a `multiple` select, a click on a row changed its chosen option or set, and nothing for a choice of the chosen option; the rest of the line stands."
    sidecar: "crosscut-invalidation-and-state-integrity widens `apply_select_choice`'s trigger to a `multiple` select's row click and its chosen set."
    rationale: "The report's F6 has `apply_select_choice` take a set and stay the one write path, and adds `click_select_row` behind a new `option` arm of `handle_click`, so a pointer now reaches that write; the report's `multiple-select` row reads this line no change and the task says the statement stands, but neither shows a key or typeahead is still the whole trigger, so this edit is put to the orchestrator against that disposition."
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
note: "`BaseDocument`'s new `select_cursor` meets cross-cutting-patterns' rule that interaction state on a removed node is cleared or retargeted, and the report states only that a stale id reads none, not whether the cursor is cleared at removal, so that reading stays open beside dec-dom-semantics and no edit is proposed here."
