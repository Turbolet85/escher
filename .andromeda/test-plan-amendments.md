# test-plan — amendments

One entry per amendment to `test-plan.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-05-as-built-baseline — the local baseline reading
**Section:** §9 CI Integration (Pipeline facts)
**Change:** new bullet "Local baseline" — the reference the fork's CI is compared against: dev profile, workspace default features, `--locked`, no `opt-level` rewrite; cold after `cargo clean`, warm on an immediate re-run, on the dev host (32 CPUs, 2026-10-05): `cargo build --workspace` 120.26 s / 5.54 s; `cargo test -p blitz-tests` 486.45 s / 7.41 s, 61 result lines, 255 passed · 0 failed · 3 ignored (`paint_tree_bench`), no font-skip line; `cargo test --workspace` 1632.88 s / 13.08 s, 108 result lines, 407 · 0 · 3, the test-profile compile dominating; fmt and clippy exit 0; workspace rustdoc `-D warnings` exit 101 (3 crates, 9 errors).
**Why:** the as-built baseline chunk exists to fix this starting point; "Fork CI reached" reads its CI leg against it.
**Ref:** .andromeda/runs/2026-10-05T20-35-18-wrap/

## 2026-10-05-fork-ci-reached — CI on the build branch, legs through ci-leg.sh, upstream-only WPT
**Section:** §1 Test Scope Summary (Workspace (CI) · CI Python scripts · Native platforms · Web Platform Tests) · §4 Unit Test Strategy (What unit tests cover) · §6 E2E Test Strategy (Drivers per surface — WPT) · §9 CI Integration (Platform · Pipeline facts)
**Change:** was CI on PRs and pushes to main/v0.*, inline `cargo test --workspace`, a matrix testing windows/macos/linux, WPT in this repository's CI; now:
- CI runs on PRs and pushes to main, v0.* and build/**; each linux leg runs `bash .github/scripts/ci-leg.sh {leg}` (every cargo leg `--locked` but `examples/wasm_hello`), the same command the host runs; the workspace tests are the fast `test` leg, `cargo test --workspace --locked`; the CI-script tests the `ci-scripts` leg (PyYAML ensured);
- fast/slow: fmt, clippy, test-features-default, ci-scripts carry no `needs`, every other job needs all four; rust-cache on every compiling job saved on main and build/*; each leg's log uploaded on failure, 7 days; local pre-push gate `ci-leg.sh fast`;
- the matrix tests windows and macos and builds ios and android, `--locked`; linux is the `test` leg's;
- WPT and its PR-results posting run only on upstream `DioxusLabs/blitz` (repository guard); the fork runs WPT on the host;
- §4 adds `test_ci_workflows.py` (CiWorkflowTest · UpstreamGuardTest · LegScriptTest with a cargo shim) — the CI-scripts leg runs 16 tests;
- the first build-branch run took 1255 s cold and its same-sha re-run 475 s warm (the fast `test` leg 315 → 169 s), as measured on GitHub-hosted runners.
**Why:** fork CI reached on escher's build branch with host-reproducible legs, caches and a fast/slow split (the chunk's acceptance; CI run green cold and warm).
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — local baseline re-measured under line-tables-only debuginfo
**Section:** §9 CI Integration (Local baseline)
**Change:** was the full-debuginfo dev profile's figures, per "2026-10-05-as-built-baseline — the local baseline reading" — build 120.26 / 5.54 s, blitz-tests 486.45 / 7.41 s, workspace tests 1632.88 / 13.08 s cold/warm; now the dev profile carries `debug = "line-tables-only"` and the baseline reads build 63.90 / 2.07 s, blitz-tests 45.36 / 6.51 s (61 lines, 255 · 0 · 3, font-skip line absent), workspace tests 52.10 / 11.44 s (108 lines, 407 · 0 · 3), cold total 161.36 s against 2239.59 s, `target/debug` 32G; fmt and clippy legs exit 0. The workspace rustdoc reading (exit 101, 3 crates, 9 errors) keeps its earlier evidence, not re-measured.
**Why:** the §9 baseline's "dev profile" condition moved; the counts held, so the profile change cost no test.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/

## 2026-10-05-fork-ci-reached — file:line citations re-pointed after the CI files moved
**Section:** every section citing `ci.yml` or `wpt.yml` lines
**Change:** 13 citations re-pointed — `wpt.yml` from line 26 on +1 (the repository guard), `ci.yml` by a range map over the rewritten file (e.g. the test job 72-89, the CI-scripts job 175-188, the matrix platforms 220-252); no claim text changed by the re-point.
**Why:** this chunk moved the cited lines.
**Ref:** .andromeda/runs/2026-10-05T21-59-55-wrap/
