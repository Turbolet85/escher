# Counts — every reading against step 1 (before.md), 2026-10-24T11:51Z

From this run's gate block (trail `.andromeda/runs/2026-10-24T11-39-00-implement/gate-2026-10-24-upstream-sync-polish-and-ship.json`; first firing, all green).

| reading | base (before.md) | take | moved by |
|---|---|---|---|
| workspace tally (entry 20, `target/ci-logs/test.log` of entry 19's `fast`) | lines 205 · passed 1098 · failed 0 · ignored 12 | lines 206 · passed 1099 · failed 0 · ignored 12 | +1 line, +1 passed: `hit_test_dropped_nodes` (1 test, its own target) |
| `run stand` run.end (entry 29) | passed 195 · failed 0 · ignored 7 | passed 195 · failed 0 · ignored 7 | unmoved |
| `run all` run.end (entry 31) | passed 616 · failed 0 · ignored 12 | passed 617 · failed 0 · ignored 12 | +1 passed: `hit_test_dropped_nodes` |
| a11y leg (entry 27) | lines 6 · passed 43 · failed 0 | lines 6 · passed 43 · failed 0 | unmoved |
| `tests/blitz-tests/tests/*.rs` (entry 22) | 144 | 145 | +1: `hit_test_dropped_nodes.rs` |
| `stand_*.rs` (entry 30) | 48 | 48 | unmoved |
| blitz-tests files not run as their own target (entry 21) | 0 | 0 | unmoved (no `Running tests/all.rs`) |
| CI scripts (entry 23, recorded) | Ran 100 tests · OK | Ran 100 tests · OK | unmoved |
| seven_guis per-package (entry 32) | 12 lines · 36 passed | 12 lines · 36 passed · 0 failed · 0 ignored | unmoved |
| blitz-dom lib (entry 17) | 106 passed | 106 passed · 0 failed | unmoved |
| the integration entry's targets (entry 18) | 19 targets · 106 passed · 0 failed | 19 result lines · 106 passed · 0 failed | unmoved (`stale_node_mapping` not edited by the take) |
| `hit_test_dropped_nodes` (entry 16) | no target | 1 passed · 0 failed | new |
| `Cargo.lock` numstat (entry 8) | — | +15 −14 | the P3 forecast (lock.md) |

No stand figure moved: geometry, Tab order, contrast rows, snapshot sizes, lint figures, shots and the committed Counter checks all read green inside `run stand`, `run all` and `fast`. Nothing was re-measured or loosened.

## The red-before-green control of `hit_test_dropped_nodes`
2026-10-24T11:51Z: the working-tree bytes of `packages/blitz-dom/src/node/node.rs` and `layout/paint_tree.rs` were saved. `f102663b8`'s hunks to those two files were reversed (`git apply -R`, exit 0), and `cargo test -p blitz-tests --locked --test hit_test_dropped_nodes` read **red**: exit 101, `test hit_test_skips_dropped_nodes ... FAILED`, panicked at `packages/blitz-dom/src/node/node.rs:1572:26` (the stale-id index the fix replaces). The saved bytes were then written back (sha256 equal before and after; worktree equal to the index), and the same command read **green**: exit 0, 1 passed.
