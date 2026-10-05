# security-plan — amendments

One entry per amendment to `security-plan.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-fork-ci-reached — signing and PR-token jobs repository-guarded; CI legs locked
**Section:** §Threat Model Summary (CI workflow triggered by pull requests) · §Authentication & Authorization (RBAC — the publish job row) · §Dependency Security (Pinning · Supply chain integrity — Signed artifacts, Lockfile verification) · §Logging & Monitoring (CI publish)
**Change:** was the publish job using "Signed Builds" only on main or `ci-test` branches — a ref-only gate — and post-results running for successful `pull_request` WPT runs; now `release-cli` and `post-results` (and the WPT jobs) carry `github.repository == 'DioxusLabs/blitz'`, so no fork ref — the fork's `main`, `ci-test/*`, `build/**` — reaches the environment, its secrets, a signed artifact or the PR-writing token; publish-build trace logging is upstream-only too. `--locked` now covers every ci.yml cargo leg (the leg script and the matrix) beside the flake and `dx bundle`, `examples/wasm_hello` excepted (no `Cargo.lock`). ci.yml references no `secrets.`, pinned by a unit test.
**Why:** the fork holds no secrets or environments, and a ref-only `if:` re-arms signing on the fork's own `main` and its existing `ci-test/sign-android-builds` branch; a repository guard keeps upstream byte-equal.
**Kept:** the write / `always()`-remove pairs for the signing key and keystore are unchanged.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `ci.yml`, `wpt.yml` or `publish-browser.yml` lines
**Change:** 23 citations re-pointed — `publish-browser.yml` from line 37 on +1 and `wpt.yml` from line 26 on +1 (the repository guards), `ci.yml` by a range map over the rewritten file; no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/
