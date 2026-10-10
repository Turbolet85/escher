
## 2026-10-10-upstream-sync-agent-surfaces — readings on the merged tree: 158 result lines and 719 passed, `run all` 396, a second engine feature split by runner, a 1100 s cold CI run
**Section:** §3 Test Harness Contract → Agent-run contract → Proof · §9 CI Integration → Local baseline, Engine features by runner, Cache
**Change:**
- Local baseline: a re-count clause — `cargo test --workspace --locked` was 154 result lines, 657 passed · 0 failed · 10 ignored; on the merge it reads 158 lines, 719 · 0 · 10: +62 passed and +4 lines, every one upstream's, the workspace's integration-test executables 103 → 107, eight differing rows each attributed to a named upstream file and PR.
- Agent-run Proof: a re-read clause — `run stand` unmoved at passed 110 · failed 0 · ignored 5 over the same 30 files with the script byte-identical; `run all` reads 396 · 0 · 10, the whole package's first stated total.
- Engine features by runner: the workspace build resolves blitz-dom with 16 features and the per-package build with 10, `writing-mode` on in the first only; it selects which of two bodies of the bounds reader `BaseDocument::physical_unrounded_geometry` is compiled, so the same stand check runs over a different reader under each runner; `scroll_into_view_nested` 7, `stand_act_scroll` 4 and `stand_act_obstructed` 3 pass under both.
- Cache: the merge's run, CI#38013740580, 16 of 16 green at its first attempt, took 1100 s wall, a cold run, against 641 s on the chunk start.
**Why:** the merge moved every one of these numbers, and a moved number is restated where the plan states it. The feature split is the standing rule of that bullet made sharper: a green per-package run is not the workspace leg's green, and now the two also differ in the code that reads every snapshot `bounds`.
**Kept:** the earlier re-count clauses stay as their chunks' readings. The per-OS tallies of that CI run — 139 result lines on three jobs, 712 passed and 8 ignored on Windows — are not attributed and are stated in no section: a route entry owns them.
**Ref:** .andromeda/runs/2026-10-10T01-56-50-wrap/
