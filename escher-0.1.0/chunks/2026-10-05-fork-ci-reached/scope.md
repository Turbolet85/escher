# Scope — 2026-10-05-fork-ci-reached

**Working entry (verbatim):** Fork CI reached — escher build-branch pipeline green, host-reproducible legs run locally, cached builds, fast/slow split, failure artifacts uploaded, signing-secret jobs excluded

**Epoch:** Epoch 1 — Foundation · **Version:** escher-0.1.0 · **Freight on the entry:** none (`route.py pins` — the only freight block on the tail is line 15's CARRY, owned by "CI gate legs")

## What this chunk builds
The fork's GitHub Actions pipeline, made to run on escher's long-lived build branch and to give fast feedback:

- **Build-branch pipeline green** — a push to `build/escher-0.1.0` triggers the fork's CI and the run settles green on the pushed sha (read per-sha through `ci.py conclusion`, never a recency listing).
- **Host-reproducible legs run locally** — every CI leg that can run on the dev host (Linux) has a local invocation that runs the same command the workflow runs, so a red leg is reproduced on the host before a push.
- **Cached builds** — compiled dependencies are restored from cache so a warm run does not recompile the dependency graph.
- **Fast/slow split** — the pipeline separates a fast feedback leg set from the slow legs, so the common push gets its verdict without waiting on the slow ones.
- **Failure artifacts uploaded** — a failing leg uploads the artifacts needed to diagnose it without a re-run.
- **Signing-secret jobs excluded** — jobs that need signing material (macOS signing key, Android keystore, Apple certificate) do not run on the fork's build-branch pushes.

## Detailed scope (P3 premise closure applied — see research.md for each derivation)
- `ci.yml`'s `on:` gains the build-branch pattern (`build/**`). Verified: today it fires only on `pull_request` and on push to `main`/`v0.*` (ci.yml:3-8), and the fork has run 0 workflows (`gh api …/actions/runs` → 0). I1 fact (2) holds (inputs#I1).
- [premise-corrected: the signing workflow is armed by ref, not by branch name — publish-browser.yml:2-7/:40 fires on the fork's `main` and on its existing `ci-test/sign-android-builds` branch, and the fork holds 0 secrets and 0 environments] "Signing-secret jobs excluded" means no FORK ref fires publish-browser.yml's signing jobs: a repository guard (upstream-only), not just keeping it off `build/**`, which already does not trigger it.
- [premise-corrected: wpt.yml runs on the upstream-only `warp-ubuntu-latest-arm64-16x` runner (:26), deploys Pages and dispatches to `DioxusLabs/blitz-wpt-results` on main (:94-118), and diffs against upstream's published report (:75-76); wpt-post-results.yml writes PR bodies with `GITHUB_TOKEN`] WPT on the fork is classified EXCLUDED (upstream-only guard on both WPT workflows), not fast or slow. WPT stays host-reproducible through `cargo run -rp wpt css svg` / `just wpt`.
- [premise-corrected: 7 compiling jobs are uncached, not 5 — build-counter and build-wasm-examples too; and `save-if` is main-only (ci.yml:205), so even matrix_test never saves a build-branch cache] Every compiling job gets a dependency cache whose save rule covers the build branch, so a second push restores what the first saved.
- A reduced debuginfo level for dev/test builds. Verified: there is no `[profile.dev]`/`[profile.test]` (Cargo.toml profiles: profile, production, p2, small, small-panic, tiny), and debuginfo is 82 % of a blitz-tests binary (`accessibility_roles` 402 MB, 331 MB `.debug_*`; 140 executables > 100 MB in target/debug/deps). [inferred] I1's "link I/O-bound" PSI reading ("io full avg60≈48 % with 32 rust-lld at 4–6 % CPU", inputs#I1) is overseer-measured on a past cold run and not reproduced at P3. The byte share supports it. Where the change lives (workspace profile vs CI-only env) is a P4 fork.
- I1 fact (4) verified at baseline.md:72-75: 120.26/5.54 s · 486.45/7.41 s · 1632.88/13.08 s (inputs#I1 rounds to 1633).
- I1 hypothesis "a cache on every job plus a reduced debuginfo level are the two largest levers" (inputs#I1). Partly re-derived: the cache is necessary (no build-branch save means every run is cold), and debuginfo is the dominant byte share. "Largest" was not measured against the other two levers research found: the duplicate linux test compile (test-features-default and matrix linux build the workspace twice, ci.yml:56/:221) and the 59 separate test binaries.
- Consolidating the 59 `tests/blitz-tests` test binaries is NOT in scope by default (I1: "a code-structure call for you and the founder, not a directive", inputs#I1). Research shows 59 binaries × ~400 MB of links, so P4 asks it as a fork.
- The founder's stated concern frames the chunk's success: CI must not rebuild everything on every run ("надеюсь ci не будет все это каждый раз пересобирать", inputs#I1).
- "Green" is proven on a real pushed sha. Verified lifecycle: `gate.py run` never fires a history-moving step (gate-contract.md:88-100). A push plus `ci.py conclusion` read is an `operator` leg, driven at the operator pass before `chore({marker}): operator pre-CI commit` (gate-contract.md:76, :293), with its result recorded in `evidence/`.
- [premise-corrected: `just` is absent on the dev host (`command -v just` → absent), and the justfile has no CI-leg recipe] The local-reproduction surface is one script both the workflow steps and the host call (`.github/scripts/ci-leg.sh {leg}`), so "same command" holds by construction with no new tool. A justfile wrapper is optional sugar. No new env var or workspace crate is minted.
- Workflow invariants are pinned by a `unittest` in `.github/scripts/` (`test_ci_workflows.py`), so the existing CI-scripts leg guards the trigger, the cache, the guards and the leg-script wiring.

## Surfaced at planning (P5 validation-1: intent-incomplete, amended)
- The linux `matrix_test` entry is dropped as a duplicate compile of the fast `test` leg (ci.yml:56 vs :221). Linux stays tested on every push, doctests included.
- The `perl … 's/opt-level = 2/opt-level = 0/g'` steps are removed. They touch only `[profile.p2]` (Cargo.toml:210), so they are a no-op for dev/test builds, and keeping them would make the CI build differ from the host's.
- Every cargo leg passes `--locked` (security-plan §Dependency Security, Pinning; test-plan §9 baseline conditions), except `examples/wasm_hello`, which carries no `Cargo.lock`.
- P4 forks (authority: founder, 2026-10-05, explicit delegation 'сам решай' in his own words, given before the overseer answered; relayed verbatim by the operator at the P5 review — not provisional; the earlier "delegate overseer, provisional" record superseded 2026-10-05): one debuginfo level in `[profile.dev]` for host and CI · slow legs `needs` the fast ones · test-binary consolidation deferred until measured after cache + debuginfo.

## Boundaries (NOT this chunk)
- The "CI gate legs" chunk owns: dependency audit (cargo-audit / cargo-deny), pinned actions (SHA pins), least-privilege tokens, coverage report, the named a11y leg, and the real rustdoc gate (its CARRY — the doc job's bare `cargo doc` and the red `-D warnings` workspace docs). This chunk does not fix rustdoc and does not pin actions beyond what it newly adds.
- "Telemetry bootstrap", "Stand test contract" (`scripts/agent-run.*`) and the headless stand are later chunks.
- No source change to engine crates beyond build configuration (profile / workflow / local recipe).
- Secrets stay only in GitHub Actions secrets/vars; no workflow this chunk adds reads a new secret.

## CI verdict read at Setup (5a)
- `50c13b5910bd` · verdict: none recorded · checks 0/0 · runs 0 · active workflows 4 · commit statuses 0 · no wall-clock (no run) — a no-op by the Setup rule, and itself the observable of I1 fact (2): the build branch triggers nothing.

## Inputs
- inputs#I1 — the overseer PHASE directive (relay, verbatim copy).
