# security extract

## Relevance
relevant: three of the six legs (dependency audit, pinned actions, least-privilege tokens) are security-plan gaps, and the coverage, a11y and rustdoc legs inherit the plan's lockfile and secret rules.

## Constraints
- A dependency-audit tool must be installed, and it must run as a CI step that fails on an unignored advisory. Per security-plan §Bootstrap phases, both `dep-audit-tooling-install` and `dep-security-ci-gate` are open, and §Dependency Security records the audit tool, the update policy and the CI integration as observed absent. Choosing cargo-audit or cargo-deny is P4's decision. The plan names neither as the target.
- Every third-party `uses:` in the fork-active workflows must be pinned to an immutable full commit SHA. security-plan §Dependency Security → Pinning names `awalsh128/cache-apt-pkgs-action@latest` as a mutable reference at three call sites (ci.yml, publish-browser.yml, wpt.yml). Research must check whether other tag-pinned actions (`actions/*`, rust-toolchain, rust-cache) exist at HEAD. The plan does not enumerate them.
- `GITHUB_TOKEN` must get an explicit `permissions:` scope. security-plan §Authentication & Authorization (RBAC / permissions (CI tokens) rows) records a workflow-level `permissions` block in ci.yml as observed absent. Research must answer whether any job-level block exists today.
- A new CI tool (audit, coverage) must be installed from a pinned version, matching the pinned-tool pattern recorded in security-plan §Dependency Security → Pinning (the CI tooling bullet).
- Every new cargo invocation must keep `--locked`. These are the audit, coverage, a11y and workspace-rustdoc legs, both in `ci-leg.sh` and in ci.yml. security-plan §Dependency Security → Supply chain integrity (Lockfile verification) records `--locked` on every ci.yml cargo leg except the lockfile-less `examples/wasm_hello`.
- The new legs must not read secrets. Secrets stay in GitHub Actions secrets/vars, and the only token any fork job touches is `GITHUB_TOKEN`, per security-plan §Secret Management → Storage and → What counts as secret.
- Editing the upstream-only workflows must keep the `github.repository == 'DioxusLabs/blitz'` guard and the "Signed Builds" environment gating. This applies both to pinning them (a P4 lean) and to adding `permissions:` to them. Per security-plan §Authentication & Authorization (publish row) and §Dependency Security → Supply chain integrity (Signed artifacts), no fork ref may reach the signing environment or its secrets.

## Patterns to follow
- Scope token grants per workflow with the plan's own examples: wpt-post-results.yml (`pull-requests: write`, `actions: read`, `contents: read`) and wpt.yml (`contents: read`, `pages: write`, `id-token: write`). Per security-plan §Authentication & Authorization, grant only what each job uses.
- Pin tool versions the way existing CI tools are pinned: `cross` from a git rev, dioxus-cli `0.7.8`, the wpt cli `0.0.14`. Per security-plan §Dependency Security → Pinning, the audit and coverage tools follow the same pattern.
- Keep the signing-material pairing: material written from secrets is removed in an `always()` step. Per security-plan §Secret Management → Storage, any edit to publish-browser.yml must preserve it.
- Keep the trusted-scripts boundary. wpt-post-results checks out its scripts from the default branch ("Checkout trusted scripts") for PR-triggered runs, per security-plan §Threat Model Summary (attack surface: CI workflow triggered by pull requests). A permissions or pin edit there must not move script checkout to the PR ref.

## Anti-patterns to avoid
- Do not give a guarded job a ref-only `if:` in place of the `github.repository` check. That re-arms the publish, WPT and post-results jobs on the fork. Per security-plan §Authentication & Authorization (publish row).
- Do not add a blanket advisory allow, and do not bump one side of a coupled pin alone to clear an advisory. Each advisory is either an upgrade within the coupled-pin rules or a recorded, justified per-ID ignore. This follows security-plan §Dependency Security (the audit gate it calls for, which a blanket allow would empty) and the chunk's own boundary.
- Do not use `permissions: write-all` and do not leave a job on the repository-default token scope. Per security-plan §Authentication & Authorization (RBAC / permissions (CI tokens)).

## Contract bindings
- security ↔ tests: the audit leg is a CI security gate. It lives in `ci-leg.sh` beside the other legs, runs as a ci.yml job on `build/**`, and its invariants (SHA pins, `permissions:` blocks, the audit leg's presence) are guarded in `.github/scripts/test_ci_workflows.py` (test-plan §9 CI integration).
- security ↔ arch: the audit tool fetches an advisory database on the CI runner, and the coverage leg is a compiling leg. Both are CI infrastructure under arch §Occupied Resources → CI infrastructure, which is under cache-budget pressure. Neither adds a workspace listener or env var.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh {audit-leg}` exits 0 at HEAD, and ci.yml runs the leg on `build/**`. Every ignored advisory is listed by ID with a written reason, and no blanket allow exists (per security-plan §Bootstrap phases `dep-security-ci-gate`; §Dependency Security).
- A grep over the fork-active workflows finds no third-party `uses:` whose ref is not a 40-hex commit SHA (no `@latest`, no `@vN`), and `test_ci_workflows.py` asserts it (per security-plan §Dependency Security → Pinning).
- Every workflow and job in ci.yml declares an explicit `permissions:` block, with no `write-all`. The fork-run jobs grant nothing beyond `contents: read` unless a named step needs more. The upstream-only jobs keep their `github.repository` guard (per security-plan §Authentication & Authorization).
- Each new leg's cargo commands carry `--locked`, and each new CI tool installs at a pinned version (per security-plan §Dependency Security → Pinning; → Supply chain integrity, Lockfile verification).
