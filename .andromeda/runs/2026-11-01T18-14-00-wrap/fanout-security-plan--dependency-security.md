proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 24
    section: "## Dependency Security"
    change: "Add after line 24, one bullet: - The upstream sync read at `e42ee1f25` took twelve commits, `fe33b778b` · `8b1a0a3ce` · `dee854c76` · `f6edb9701` · `6fca6308d` · `7a3dd8918` · `91ef65e21` · `a26c6ddeb` · `f76777766` · `6ef4b532d` · `ae774d7f4` · `e42ee1f25`, by no-commit cherry-picks of upstream's text (`45c333f16`), twenty-one left out with their measured reasons, the merge base staying `23b9b26a9`: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every crate manifest are byte-identical to the tree before it, so no package enters, leaves or moves; `url`, which the new `parse_search_params` calls, was already blitz-vibey-script's; the pins stay — Stylo \"0.22.0\", Taffy the registry \"0.15\", Dioxus \"0.7.3\", winit `=0.31.0-beta.3`, no git dependency; the winit shape `a09f46792` · `527394724`, green on the trial, is left out; the audit leg reads `advisories ok` and `licenses ok` with `deny.toml` unchanged — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "Dependency Security gains this take's lock record: twelve commits read at e42ee1f25, twenty-one left out, manifests, lock and deny.toml byte-identical, the pins and the audit unmoved."
    rationale: "The report's Expected amendments name §Dependency Security's lock record for this take (report.md:96); its sync-line row keeps dependency-security.md:24 as the last take's history, with a new sentence beside it (report.md:35); the facts are Dependencies, none (report.md:30), the negative API facts (report.md:84) and the audit gate (report.md:141)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:30
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 52
    section: "## Dependency Security › ### Supply chain integrity"
    change: "Replace line 52's bullet with: - **CI action, the `matrix_test` free-disk step:** removed this chunk, a step that was skipped on every matrix row, its `if:` naming `ubuntu-24.04` and `ubuntu-24.04-arm` where the android row runs `ubuntu-latest`; `Test (android)` read no such step in this take's run, and `Stand checks repeated`'s step, above, stands, pinned by `test_only_stand_repeat_frees_disk` (.github/scripts/test_ci_workflows.py:110-115) — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md"
    sidecar: "Dependency Security retires the matrix_test free-disk step as standing: the step is removed from ci.yml, and Stand checks repeated's step stands."
    rationale: "The report's Retired bullet, item 2, names the matrix_test free-disk step (report.md:33); its free-disk row marks dependency-security.md:52 changed, removed, and :51 no-change (report.md:43); the removal and its pin are at report.md:64 and report.md:144."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
note: "D-security-deps holds: the chunk adds no dependency (report.md:30). D-security-input and D-security-auth find nothing in this file. Line 51's free-disk step is no-change by report.md:43; its WATCH reading, 3 green runs with the cure not shown, is left to P5 (report.md:131, report.md:162)."
