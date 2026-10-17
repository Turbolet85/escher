# Counts — the merged tree against the baseline

Gate run: `.andromeda/runs/2026-10-17T14-34-19-implement/gate-2026-10-17-upstream-sync-engine-debts.json`
(`entries 47 · green 42 · red 0 · recorded 1 · timeout 0 · not-run 4`, the four `leg = 'operator'` entries).
Baseline: `evidence/before.md`.

| Reading | Before | After | Delta, by name |
|---|---|---|---|
| Workspace tally (`fast`, entry 23) | 186 lines · 958 passed · 0 failed · 12 ignored | 187 lines · 971 passed · 0 failed · 12 ignored | +1 line, +13 passed — see below |
| Files under `tests/blitz-tests/tests/*.rs` | 125 (124 targets) | 127 (126 targets) | +2: upstream's `accessibility_description.rs`, `accessibility_label.rs` |
| Absent by name in the test leg (entry 24) | 0 | 0 | — |
| Upstream's two files, `Running` once each (entry 25) | 0 | 2 | +2 |
| `Running tests/all.rs` (entry 26) | 0 | 0 | — |
| Workspace integration-test executables (entry 27) | 135 · all-target 0 | 137 · all-target 0 | +2, the two files above |
| CI-scripts (entry 28) | `Ran 100 tests` · `OK` | `Ran 100 tests` · `OK` | 0 — step 7 adds an assertion to an existing case, no case |
| a11y leg (entry 32) | 6 lines · 43 passed · 0 failed | 6 lines · 43 passed · 0 failed | — |
| `run stand` (entry 34) | 168 passed · 0 failed · 7 ignored | 168 passed · 0 failed · 7 ignored (run.end 2026-10-17T14:46:29Z) | — |
| `stand_*.rs` files (entry 35) | 40 | 40 | — |
| `run all` (entry 36) | 533 passed · 0 failed · 12 ignored | 546 passed · 0 failed · 12 ignored (run.end 2026-10-17T14:47:34Z) | +13: the two upstream files |

## The workspace tally's delta, per target
Per-target libtest tallies of the pre-merge `fast` log (the 2026-10-16-fast-failing-mutants wrap's light-gate
`fast` firing, written 2026-10-17T08:33:56Z — the run whose `target/ci-logs/test.log` the baseline read, 186 · 958 ·
0 · 12) against this run's `target/ci-logs/test.log`, keyed by `Running`/`Doc-tests` target (a scratch parser over the
merged stream; every other target reads the same three numbers):

- `Running tests/accessibility_description.rs`: absent → 3 passed · 0 failed · 0 ignored (upstream's new file)
- `Running tests/accessibility_label.rs`: absent → 10 passed · 0 failed · 0 ignored (upstream's new file)
- `Running unittests src/main.rs (…/bump)`: 0 · 0 · 0 → absent (upstream deleted `apps/bump`)

So +2 −1 result lines = +1 (187, where the plan predicted 188 without the bump target's empty line), and +13
passed. No ours-side count moved: the key migration changed spellings only.
