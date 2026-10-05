# arch extract

## Relevance
relevant — the chunk is CI infrastructure, which this plan owns (per architecture §Infrastructure Patterns → CI/CD, §Occupied Resources → CI infrastructure). It also touches engine rustdoc in blitz-dom, blitz-vibey-script and the example `transparent` (§Conventions → Documentation), and the `blitz` filename collision (§Occupied Resources → Names).

## Constraints
- Every new leg (audit, coverage, a11y, the reworked `doc`) must run in CI as `bash .github/scripts/ci-leg.sh {leg}`, the same command the dev host runs. It inherits the leg-script contract: output tee'd to `target/ci-logs/{leg}.log`, exit with the leg command's status, exit 2 on an unknown leg, and every cargo leg `--locked`. Per architecture §Infrastructure Patterns → CI/CD and §Standard Contracts → CI contracts.
- Each new ci.yml leg job must keep the failure-log contract: `actions/upload-artifact` `if: failure()` named `ci-log-{job id}`, uploading only its own log, `retention-days: 7`, `if-no-files-found: ignore`. That contract is pinned in `test_ci_workflows.py`, so any change to the workflow invariants is pinned there too. Per architecture §Standard Contracts → CI contracts.
- The fast/slow split must stay intact. Only `fmt`, `clippy`, `test-features-default` and `ci-scripts` carry no `needs`, and every other job `needs` all four. Whether a new leg (audit, a11y) joins the fast tier or sits behind `needs` is a decision this split constrains. Per architecture §Infrastructure Patterns → CI/CD.
- The fork's Actions cache is a registered occupied resource: a 10 GB budget with LRU eviction, which held about 97 % after one run (one `v0-rust-*` entry per compiling job, plus apt). A new compiling leg such as the instrumented coverage build adds a cache entry that can evict. The leg's cache key and save policy (`Swatinem/rust-cache` saves on `main` and `build/*`), and any change to cache occupancy, are recorded under §Occupied Resources → CI infrastructure. Per architecture §Occupied Resources → CI infrastructure and §Infrastructure Patterns → CI/CD.
- The upstream-only jobs (`release-cli`, `wpt`, `trigger-archive`, `post-results`) must keep `github.repository == 'DioxusLabs/blitz'` in their `if`. A `permissions:` or SHA-pin edit to those workflows must not weaken the guard. Per architecture §Occupied Resources → CI infrastructure and §Infrastructure Patterns → Deployment model.
- Dependency pins are coupled: html5ever family ↔ stylo web_atoms, skrifa ↔ parley/vello, svgtypes ↔ usvg, taffy/parley git `rev`s, winit exact beta, and the examples' `idna_adapter = "=1.0.0"`. An advisory remedy may not move one side alone. Per architecture §Established Decisions → [Dependency pinning].
- A CI tool installed from outside the toolchain (audit or coverage CLI) follows the existing pinned-install precedent: `cross` from a git rev, dioxus-cli "0.7.8", wpt cli "0.0.14". Per architecture §Established Decisions → [Cross-compilation] and §Infrastructure Patterns → CI/CD.

## Patterns to follow
- One leg script for CI and host. The `doc` leg today runs bare `cargo doc --locked` with `RUSTDOCFLAGS` exported by the leg script. The real gate replaces that leg's command in place, not with a parallel ad-hoc step. Per architecture §Stack and Technologies → Code quality and §Standard Contracts → CI contracts.
- Workflow invariants are pinned by `.github/scripts/test_ci_workflows.py` (PyYAML), run by the `ci-scripts` leg. SHA pins, `permissions:` blocks and the new jobs extend that test file. Per architecture §Stack and Technologies → CI/CD and §Standard Contracts → CI contracts.
- Documentation convention for the rustdoc fixes: modules open with `//!`, public items carry `///`, and spec URLs are cited next to implementations. Fixes stay doc-only. Per architecture §Conventions → Documentation.
- The rustdoc `-D warnings` setting is workflow-wide (`env` in ci.yml and the leg script). The gate should make that existing setting effective rather than add a second flag source. Per architecture §Conventions → Formatting and lints and §Inherited Defaults → Code quality.

## Anti-patterns to avoid
- No new workspace crate, binary rename, env var, port or listener as a side effect of the gates. The `blitz` filename collision (bin `blitz` of `browser` vs lib `blitz`) is resolved without renaming registered names, through doc attributes or doc-target selection. Any rename becomes a §Occupied Resources → Names amendment, never a silent change. Per architecture §Occupied Resources → Names and §Inherited Defaults.
- No `cargo` leg without `--locked`. The only exception is `examples/wasm_hello`, which has no lockfile. Per architecture §Standard Contracts → CI contracts.
- Do not re-arm upstream-only jobs on the fork through a ref-only `if:` while editing workflows. Per architecture §Occupied Resources → CI infrastructure.

## Contract bindings
- arch ↔ security: SHA-pinned `uses:`, least-privilege `permissions:` and the audit leg land in workflows whose shape arch owns (§Infrastructure Patterns → CI/CD). The security-plan defines which actions and scopes must be pinned. The repository guard (§Occupied Resources → CI infrastructure) is shared surface.
- arch ↔ tests: the coverage leg runs the workspace tests that §Infrastructure Patterns → CI/CD places in the fast `test` leg. Coverage tooling and reporting belong to test-plan §9. Its cache cost binds to §Occupied Resources → CI infrastructure.
- arch ↔ a11y: the named a11y leg runs blitz-tests accessibility files (§Existing Scopes → blitz-tests). `accessibility` is a default feature of blitz-dom and blitz-shell (§Established Decisions → [Default features]), so the leg must build with default features to exercise it.
- arch ↔ ci-scripts: every new invariant is pinned in `test_ci_workflows.py` (§Standard Contracts → CI contracts).

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh doc` runs a workspace-wide `--no-deps` rustdoc under `-D warnings`, documents the library crates (blitz-dom and blitz-vibey-script included), and exits 0 with no `target/doc` output-filename collision warning. ci.yml's docs job invokes that same leg. Per architecture §Stack and Technologies → Code quality and §Infrastructure Patterns → CI/CD.
- Each new CI leg (audit, coverage, a11y) is reachable as `bash .github/scripts/ci-leg.sh {leg}` on the host. In ci.yml it has an `if: failure()` `ci-log-{job id}` upload with 7-day retention, and `test_ci_workflows.py` pins its presence. The `ci-scripts` leg passes. Per architecture §Standard Contracts → CI contracts.
- The fast/slow `needs` split stays green under `test_ci_workflows.py`. The upstream-only jobs still carry the `github.repository == 'DioxusLabs/blitz'` guard. Per architecture §Infrastructure Patterns → CI/CD and §Occupied Resources → CI infrastructure.
- The coverage leg's cache entry and save policy are measured after the first fork run, and §Occupied Resources → CI infrastructure is amended with the new occupancy. No `Cargo.toml` coupled-pin pair is moved on one side only. Per architecture §Occupied Resources → CI infrastructure and §Established Decisions → [Dependency pinning].
