# security extract

## Relevance
partial — the chunk rewires CI triggers, caches and artifact uploads around the workflows that hold every recorded secret; the app-level threat model, audit tooling and least-privilege tokens are not in this chunk's scope (they belong to "CI gate legs" or read NOT YET MEASURED).

## Constraints
- Signing material (macOS signing key, Android keystore and its passwords, Apple certificate and its password) is the recorded secret set and lives only in GitHub Actions secrets/vars (per security-plan §Secret Management, Storage and "What counts as secret"). The "signing-secret jobs excluded" deliverable must keep every job that reads these from running on a `build/**` push. Whether a trigger restriction alone does that, or a repository/branch guard is also needed, is P3/P4's call.
- The publish job is gated on environment "Signed Builds" only for main or `ci-test` branches (per security-plan §Authentication & Authorization, RBAC rows; §Dependency Security, Supply chain integrity). The chunk must not widen that gate to include the build branch. Whether the current `on:` trigger plus that gate already keeps publish-browser.yml off `build/escher-0.1.0` is research's question.
- Signing material written to disk in CI must be removed in an `always()` step (per security-plan §Secret Management, Storage). Any reshaping of publish-browser.yml (triggers, job split, `if:` guards) must keep each write/remove pair intact.
- `WPT_GITHUB_TOKEN` (cross-repo dispatch in wpt.yml) and `GITHUB_TOKEN` (wpt-post-results.yml) are recorded secret-bearing tokens (per security-plan §Secret Management, Storage). When the chunk classifies wpt.yml / wpt-post-results.yml as fast, slow or excluded on the fork, it must account for these reads. A leg that runs on the fork's build-branch push must not need a secret the fork does not hold. Whether the fork holds `WPT_GITHUB_TOKEN` is research's question.
- wpt-post-results.yml's trust boundary is to run only for successful `pull_request`-triggered WPT runs and to check out scripts from the default branch ("Checkout trusted scripts") (per security-plan §Threat Model Summary, vector "CI workflow triggered by pull requests"). Any trigger change must preserve both conditions.
- Lockfile and pin discipline must hold on every leg the chunk adds or re-triggers: builds pass `--locked` and git dependencies stay pinned by `rev` (per security-plan §Dependency Security, Pinning; Supply chain integrity, Lockfile verification). Whether ci.yml's compiling jobs already pass `--locked` is research's question.
- No workflow this chunk adds or edits may read a new secret, and source must keep reading no secret (per security-plan §Secret Management, "Never in code" and "Environment values read").

## Patterns to follow
- Swatinem/rust-cache is the dependency-cache action already used in ci.yml's matrix_test job (per security-plan §Dependency Security, Pinning, CI tooling line). Extend that existing action to the other jobs rather than adding a new third-party cache action. Its current reference form (tag or SHA) is research's to read.
- CI tooling is installed at fixed versions/revs: `cross` from a pinned git rev, dioxus-cli 0.7.8, wpt cli 0.0.14 (per security-plan §Dependency Security, Pinning). Any tool a new local-reproduction recipe or leg installs follows the same fixed-version form.
- Secret-bearing steps write material to disk from a secret and remove it in an `always()` step (per security-plan §Secret Management, Storage). This is the template if any job is restructured.
- CI matrix jobs set `fail-fast: false` (per security-plan §Error Handling, Error format). Under a fast/slow split, this keeps one red leg from hiding others' failure artifacts.

## Anti-patterns to avoid
- Do not let a build-branch push reach a job that writes signing material to disk (per security-plan §Secret Management, Storage; §Authentication & Authorization, publish job "Signed Builds" row).
- Do not add a new action at a floating ref like the existing `awalsh128/cache-apt-pkgs-action@latest` (per security-plan §Dependency Security, Pinning). Pinning the existing references is "CI gate legs" scope, but anything this chunk newly adds must not copy the `@latest` form.
- Do not put signing material, a keystore, `Dioxus.toml` with appended passwords, or a token value into an uploaded failure artifact or a cached path (per security-plan §Secret Management, Storage — keystore written to `apps/browser/keystore.jks` and passwords appended to Dioxus.toml). Whether any upload or cache glob the chunk adds could reach those paths is research's question.

## Contract bindings
- security ↔ tests: the build-branch trigger, fast/slow split and failure artifacts bind to the test plan's CI integration (one pipeline, legs as jobs). The dependency-audit CI gate (per security-plan §Bootstrap phases, dep-security-ci-gate / dep-audit-tooling-install) is owned by "CI gate legs", not this chunk.
- security ↔ obs: failure artifacts that carry logs bind to the observability plan. Engine logs record request URLs and resource-failure fields unscrubbed (per security-plan §Logging & Monitoring, What is logged), so uploaded failure logs inherit that exposure. No scrub layer exists to rely on.
- security ↔ arch: a new env var for local CI reproduction joins the recorded environment reads (per security-plan §Secret Management, Environment values read) and is an arch §Occupied Resources registration.
- security ↔ "CI gate legs" chunk: least-privilege tokens rest on the recorded absence of a workflow-level `permissions` block in ci.yml (per security-plan §Authentication & Authorization, RBAC row). That chunk owns them, together with action SHA pins and audit tooling. This chunk must not claim them.

## Acceptance criteria contributions
- On the pushed build-branch sha, no job that reads signing-material secrets ran or was skipped by an expression that could become true on `build/**` (per security-plan §Secret Management, Storage; §Authentication & Authorization, publish job "Signed Builds" row).
- `grep` over `.github/workflows/*.yml` shows every signing-material disk write still paired with an `always()` removal step after the chunk's edits (per security-plan §Secret Management, Storage).
- The chunk's diff adds no new `secrets.*` reference to any workflow, and every `actions/upload-artifact` path it adds excludes `apps/browser/keystore.jks` and Dioxus.toml (per security-plan §Secret Management, Storage).
- wpt-post-results.yml still runs only on successful `pull_request`-triggered WPT runs and still checks out its scripts from the default branch (per security-plan §Threat Model Summary, vector "CI workflow triggered by pull requests").
