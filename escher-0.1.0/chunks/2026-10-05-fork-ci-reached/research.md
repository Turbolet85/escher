# Codebase Research — 2026-10-05-fork-ci-reached

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 8
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — 0 Session Additions applied (its 5-command contract is NOT YET MEASURED; it has no live leg relevant to CI, and this chunk touches none of its `paths:`)
- **Platform issues consulted:** none. No CI verdict was folded (Setup 5a read `none recorded`), so there is no runner-only bullet. The two GitHub facts the plan relies on are read from the fork's own API below, not from a tracker.
- **External inputs:** `inputs#I1` — the overseer PHASE directive (four measured facts, the cache/debuginfo hypothesis and the founder's fast-feedback concern)

## Files inspected
- `.github/workflows/ci.yml` (full) — triggers `pull_request` plus push to `main`/`v0.*` (:3-8). Per-ref concurrency with cancel-in-progress (:10-12). 10 jobs (:26-221). Only `matrix_test` caches (:201-206), with `save-if: github.ref == 'refs/heads/main'` (:205). No cargo command passes `--locked` (re-derived: `grep -c -- '--locked' .github/workflows/ci.yml` → 0).
- `.github/workflows/publish-browser.yml` (full) — push to `main` and `ci-test/*` plus `workflow_dispatch` (:2-7). Environment "Signed Builds" applies only on main/ci-test refs (:40). Apple key written at :105-107 and removed `always()` at :167-169. Android keystore written at :109-119 and removed `always()` at :171-173. The keystore removal leaves the passwords appended to `apps/browser/Dioxus.toml` in the workspace. Bundle step at :153-160.
- `.github/workflows/wpt.yml` (full) — `pull_request` plus push to `main` (:3-7). Runner `warp-ubuntu-latest-arm64-16x` (:26). 15-min timeout (:27). Pages deploy and the `repository-dispatch` to `DioxusLabs/blitz-wpt-results` run on main only (:94-118). It fetches upstream's `dioxuslabs.github.io/blitz/wptreport.json` (:75-76).
- `.github/workflows/wpt-post-results.yml` (full) — `workflow_run` of "WPT". Runs only for successful `pull_request` runs (:15). Scripts are checked out from the default branch (:18-21).
- `Cargo.toml` (:195-235, profiles) — the only profiles are `profile`, `production`, `p2`, `small`, `small-panic` and `tiny`. There is no `[profile.dev]` or `[profile.test]` (re-derived: `grep -n '^\[profile' Cargo.toml`). The CI `perl … 's/opt-level = 2/opt-level = 0/g'` rewrite hits only `[profile.p2]` (:210), so it does nothing for dev/test builds (re-derived: `grep -n 'opt-level' Cargo.toml` lists 9 lines, and only :210 reads `= 2`; agrees with baseline.md:57-59).
- `justfile` (head) — has `check`, `clippy` (without `-D warnings`), `fmt`, `wpt` and app recipes. It has no CI-leg recipe.
- `escher-0.1.0/chunks/2026-10-05-as-built-baseline/evidence/baseline.md` (:23-92 spot-checks) — cold/warm figures at :72-75. The baseline's conditions add `--locked` to every cargo form, while "CI passes none" (:60-61).
- `.claude/docs/commands.md` (grep) — the CI legs are documented as bare commands, for example `cargo test --workspace` (:20) and the CI-scripts leg (:25).
- `/home/turbolet/.claude/skills/andromeda-phase/references/gate-contract.md` (grep) — `gate.py run` never fires a `leg` entry, and a `run` that moves history (push) is forbidden in a round (:88-100). The operator pass fires before `chore({marker}): operator pre-CI commit` (:76, :293). So a CI verdict on a pushed sha belongs to an operator leg.

## Graph impact
- No Rust symbol is in scope. The chunk edits workflow YAML, a Cargo profile stanza and a shell/Python CI script. The code-graph query is skipped: there is no symbol, caller or crate edge to enumerate (`derived-without-graph` does not apply, because the rust plane is indexed but nothing here is a Rust symbol).

## Patterns detected
- **Upstream-ref gating by expression** (publish-browser.yml:40, :106, :110, :168, :172; wpt.yml:95, :98, :103, :108): upstream gates secret and Pages steps with `github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/ci-test')`. The pattern gates by ref, not by repository, so on the fork the same refs re-arm it.
- **rust-cache with a target key** (ci.yml:201-206): `key: target`, `cache-all-crates: "true"`, `save-if` on main, and `cache-bin` disabled on macOS (Swatinem/rust-cache#341).
- **Failure-diagnosis artifact** (wpt.yml:59-63, :86-93): `actions/upload-artifact@v7` with a named artifact. The PR-only diff upload is guarded `if: github.event_name == 'pull_request'`.
- **Matrix `fail-fast: false`** (ci.yml:131-132): one red platform does not cancel the others.

## Conventions to follow
- **Job naming**: human-readable `name:` per job (ci.yml:27, :39, :49) — `"Test [default features]"` form.
- **Ubuntu apt deps**: `sudo apt-get install -y libfontconfig1-dev` in plain jobs (ci.yml:35), `awalsh128/cache-apt-pkgs-action@latest` in the matrix (ci.yml:208-213). New steps must not add a `@latest` action (security extract).
- **Toolchain action**: `dtolnay/rust-toolchain@stable` or `@master` with `toolchain:` (ci.yml:31-33, :43).

## Measured facts (re-derived at HEAD)
- **The fork has never run CI.** `gh api repos/Turbolet85/escher/actions/runs --jq .total_count` → 0. Cache usage `active_caches_count` 0. Actions enabled, `allowed_actions: all`, `sha_pinning_required: false`. 4 workflows are `active`: CI, Publish Browser, Post WPT results and WPT (`gh api …/actions/workflows`).
- **The fork holds no secrets and no environments.** `gh secret list -R Turbolet85/escher` printed none, and `gh api …/environments` listed none. A "Signed Builds"/"WPT" environment job on the fork would create the environment unprotected and run with empty secrets.
- **The fork carries a `ci-test/sign-android-builds` branch** (`gh api …/branches`, first page). A push to it, or to the fork's `main`, would fire publish-browser.yml as it stands today.
- **Cache is cold on every build-branch run even where it exists.** `save-if` is main-only (ci.yml:205), so a `build/**` run restores at most the default branch's cache and never saves its own. GitHub caches are scoped per branch, readable by that branch and from the default branch.
- **Uncached compiling jobs: 7, not 5.** build-msrv, build-features-default, test-features-default, build-counter, build-wasm-examples, clippy and doc (re-derived: every `cargo` job in ci.yml:26-124 has no `Swatinem/rust-cache` step; fmt and ci-scripts do not compile workspace crates). I1(1) named 5 and omitted build-counter and build-wasm-examples. Corrected here.
- **Duplicate linux test compile.** `test-features-default` runs `cargo test --workspace` (:56). `matrix_test` linux runs `cargo test --all --tests --target x86_64-unknown-linux-gnu` (:151-158, :221). The explicit `--target` puts it in its own `target/x86_64-unknown-linux-gnu/` tree, so the two compile the same workspace twice in separate jobs. They differ in that the matrix form excludes doctests (`--tests`) and the plain form includes them.
- **Debuginfo dominates the test binaries.** `target/debug/deps/accessibility_roles-74392d167e10ca1e` is 402 155 872 B, and its `.debug_*` sections total 331 175 198 B (82 %) (`readelf -S -W`, summed). `find target/debug/deps -maxdepth 1 -type f -executable -size +100M | wc -l` → 140. `du -sh target/debug` → 85G, `target/debug/deps` → 65G. tests/blitz-tests has 59 test files (`ls tests/blitz-tests/tests/*.rs | wc -l`), each its own binary, since `tests/blitz-tests/Cargo.toml` declares no `[[test]]`/`autotests` (grep → 0).
- **I1(3)'s PSI reading is not reproduced here.** "io full avg60≈48 % with 32 rust-lld at 4–6 % CPU" is the overseer's observation of the predecessor's cold run, and baseline.md does not record it (`grep -n -iE 'pressure|rust-lld' baseline.md` → 0). Re-measuring needs a ~27-min cold run. The byte measurement above supports the mechanism: each link writes ~400 MB, 82 % of it debuginfo. The I/O-bound claim stays overseer-measured.
- **I1(4)'s figures match baseline.md:72-75**: 120.26/5.54 s · 486.45/7.41 s · 1632.88/13.08 s (I1 rounds the last to 1633).
- **Dead free-disk condition.** `matrix_test`'s Free Disk Space step (:193-199) checks `matrix.platform.os == 'ubuntu-24.04'`, but the matrix uses `ubuntu-latest`, so it never runs.
- **Host tooling for local reproduction:** `just` absent, `actionlint` absent, `act` absent, `zizmor` present (`~/.cargo/bin/zizmor`), python3 with PyYAML 6.0.3, toolchains stable/nightly/1.95/1.96/1.98.1 (no 1.91 — `rustup toolchain list`).

## New files to create
- `.github/scripts/ci-leg.sh` — the one definition of each linux CI leg's command (fmt, clippy, test, build, msrv-build, counter, wasm, doc, ci-scripts). The workflow steps and the local run both call it, so "host-reproducible" is one command by construction.
- `.github/scripts/test_ci_workflows.py` — `unittest` (picked up by the existing CI-scripts leg) asserting the workflow invariants: the build-branch trigger, every compiling job cached with the save rule, the upstream-only guard on publish/WPT jobs, linux legs calling `ci-leg.sh`, and failure-artifact steps present.

## Files to modify
- `.github/workflows/ci.yml` — build-branch trigger, cache on every compiling job with a save rule covering the build branch, legs through `ci-leg.sh`, fast/slow split, failure-artifact upload, `--locked`
- `.github/workflows/publish-browser.yml` — upstream-only guard so no fork ref fires the signing jobs
- `.github/workflows/wpt.yml` — upstream-only guard (warp runner, Pages deploy and upstream dispatch are upstream-owned)
- `.github/workflows/wpt-post-results.yml` — upstream-only guard, consistent with wpt.yml
- `Cargo.toml` — a `[profile.dev]` debuginfo stanza (provisional: the P4 fork on where the debuginfo change lives)

## Open questions
- Where the debuginfo reduction lives: a workspace `[profile.dev]` stanza (local and CI) vs a CI-only `CARGO_PROFILE_DEV_DEBUG` env (CI only, a new env var registration). → blocks: plan-decision
- The shape of the fast/slow split: slow legs (windows/macos/ios/android matrix, wasm, msrv) on every push in parallel, gated behind the fast legs, or off the build-branch push path (PR/schedule/dispatch). → blocks: plan-decision
- Whether to consolidate the 59 `tests/blitz-tests` binaries into one test binary, a link-count lever (I1: "a code-structure call for you and the founder"). → blocks: plan-decision
