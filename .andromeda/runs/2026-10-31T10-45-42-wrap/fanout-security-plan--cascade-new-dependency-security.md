proposals:
  - detector: D-new-detail
    severity: warning
    fact: F14
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 49
    section: "## Dependency Security › ### Supply chain integrity"
    change: "- **CI action, the free-disk step:** since 2026-10-31-engine-debts job `Stand checks repeated` runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be`, after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false` and `swap-storage: false` (.github/workflows/ci.yml:311-316) — a CI action site, not a crate, a widening on the operator's word (2026-10-31) after a runner out of disk (`Free space left: 5 MB`) turned the job red on CI#41190771843; it ran, `completed · success` on CI#41192433140, taking `/dev/root` from 86 G free to 109 G free, and it is not shown to be the cure: that runner had 86 GB free before the step, the failed run has no `df` line, and the red job stood on an older image whose free disk stays unread — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: Supply chain integrity gains the pinned free-disk action of job `Stand checks repeated`, its cure unproven."
    rationale: "Report, Changes, Harness / gate surface and Insufficient fixes, and its Addendum on the runner images: the step is added on the operator's word, its pinned site validated, and no standing text of the file states it."
    basis: .github/workflows/ci.yml:311-316
  - detector: D-new-detail
    severity: warning
    fact: F15
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 49
    section: "## Dependency Security › ### Supply chain integrity"
    change: "- **CI action, the older free-disk step:** the step on the `matrix_test` job (.github/workflows/ci.yml:392-394) is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, and the android row runs `ubuntu-latest` (`completed · skipped` on `Test (android)`, CI#41190771843), so this workflow held no measured run of the action before 2026-10-31-engine-debts — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: Supply chain integrity records that the older free-disk step is skipped on every matrix row."
    rationale: "Report, Changes, Harness / gate surface and Spec claims disproved (pattern `free-disk`): the step stands and does not fire, and no standing text of the file states it."
    basis: .github/workflows/ci.yml:392-394
note: F14 and F15 both follow line 49, F14 first; the owners the report asks for, a WATCH on the free-disk step and an owner of the skipped step, are P5's and are not written here.
