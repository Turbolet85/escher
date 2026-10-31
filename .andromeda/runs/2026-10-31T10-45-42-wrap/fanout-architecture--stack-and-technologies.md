proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 63
    section: "## Stack and Technologies"
    change: "In the Markdown app's Technology row, directly after `pulldown-cmark 0.13` add: (resolved 0.13.5 since 2026-10-31-engine-debts, past RUSTSEC-2026-0414, a semver-compatible lockfile bump with no coupled pin; reached through rdme, which never sets `ENABLE_WIKILINKS`, so no reader was exposed). The rest of the row stands."
    sidecar: "2026-10-31-engine-debts: Stack, the Markdown app's row gains pulldown-cmark's resolved 0.13.5, past RUSTSEC-2026-0414; the manifest's 0.13 is unmoved."
    rationale: "The report's Dependencies bullet moves pulldown-cmark 0.13.4 to 0.13.5 in the lockfile alone, for RUSTSEC-2026-0414, and adds no crate; the row names only the manifest's 0.13, and the report states that no hit names the advisory. The tokio row beside it already holds this form, a requirement with its resolved version past an advisory."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:43
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 86
    section: "## Stack and Technologies"
    change: "Add after this item, one item: - **CI/CD** beside the actions named above: since 2026-10-31-engine-debts the job `Stand checks repeated` runs `jlumbroso/free-disk-space` at its pinned 40-hex SHA `54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, with `android: true`, `large-packages: false`, `swap-storage: false` — an action site and no crate, read `completed · success` on CI#41192433140, its first measured run in this workflow, since the matrix job's older step of the same action is skipped on every row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` while the android row runs `ubuntu-latest`; the step is not shown to cure the out-of-disk red of CI#41190771843, that runner's free disk unread (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)."
    sidecar: "2026-10-31-engine-debts: Stack, new CI/CD item for the free-disk-space action on Stand checks repeated, its older matrix step skipped on every row and its cure unproven."
    rationale: "The report's Harness / gate surface adds this action site on the operator's word and states that no master holds this step or the older one (the free-disk control unfired); Insufficient fixes and the Addendum keep the cure unproven. The category's list of ci.yml actions does not name it, so it lands as its own item, in the form of the Code quality item beside it."
    basis: escher-0.3.0/chunks/2026-10-31-engine-debts/report.md:85
note: "No Retired item stands in this file and no stack or locked decision is contradicted; .andromeda/architecture.md:247 describes the stand-repeat job and is silent on the free-disk step, which is the body's own to carry and outside this file."
