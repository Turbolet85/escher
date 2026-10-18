
## 2026-10-17-upstream-sync-engine-debts — the coverage leg excludes `keyboard_compat.rs` by filename (§9, §10)
**Section:** §9 CI Integration (Coverage) · §10 Quality Gates & Coverage Targets (Coverage floors)
**Change:**
- Coverage: was «`cargo llvm-cov --workspace --locked --lcov …`, then `cargo llvm-cov report --workspace --locked`»; now both lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`.
- Coverage floors: the exclusion of dioxus-native-dom's crate-private `keyboard_compat.rs` — upstream's temporary keyboard-types 0.7/0.8 bridge, 564 lines whose 530 arms no test of ours reaches — until upstream retires it; the floors and `coverage_floor.py` unchanged; the merged tree reads dioxus-native-dom 84.25 % (equal to the baseline), escher-driver 98.19 %, blitz-test-harness 91.69 %, `3 of 3` with no `keyboard_compat` record in the lcov; pinned by `test_coverage_leg_reaches_the_floor_on_each_crate`.
**Why:** the operator's answer to P4 fork 3 — a named filename exclusion over lowering the floor or testing the bridge's arms.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
