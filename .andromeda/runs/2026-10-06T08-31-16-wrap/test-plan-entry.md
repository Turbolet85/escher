
## 2026-10-06-upstream-sync-element-identity — workspace count 430 → 431; wpt/runner attr_test unit test; citation re-point
**Section:** §1 Test Scope Summary (wpt/runner) · §4 Unit Test Strategy (What unit tests cover) · §9 CI Integration (Local baseline) · every section citing a merged upstream file's lines
**Change:**
- §9 Local baseline: `cargo test --workspace` re-counted after the upstream merge — 120 result lines, 431 passed · 0 failed · 4 ignored (was 430 · 0 · 4 at 2026-10-06-headless-stand, kept as history); the +1 is wpt/runner's `subtest_names_include_nonempty_root_titles`.
- §1 wpt/runner unit tests: 8 in fuzzy.rs, 3 in js_wrapper.rs, 2 in harness_test.rs, 1 in attr_test.rs, 1 in mod.rs (was without attr_test.rs).
- §4 wpt/runner: attr_test.rs tests that native checkLayout subtest names include a non-empty root title.
- 19 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no other claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`, which brought one wpt/runner unit test (upstream `3aa87bc1`).
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/
