
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
