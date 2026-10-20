# Counts — 2026-10-19-visual-lint

Measured by `/andromeda-implement`: one firing of the block, `.andromeda/runs/2026-10-20T00-14-31-implement/`
(trail `gate-2026-10-19-visual-lint.json`), summary `entries 22 · green 15 · red 0 · recorded 0 · timeout 0 · not-run 7`.
The 7 not run are the four `operator` entries (16–19) and the three `gated:` defers (20–22).

| reading | forecast | measured | entry |
|---|---|---|---|
| `text_extent` tests | 3 passed · 0 failed | 3 passed · 0 failed | 1 |
| `lint.rs` unit tests | the two test lines | 2 passed (92 filtered out) | 2 |
| `stand_lint` tests | 5 test lines | 5 passed · 0 failed | 3 |
| escher-driver `info_span!` sites | 2 | 2 | 5 |
| silence census over the four new files | 0 | 0 (exit 1) | 6 |
| tracked files changed outside the six | 0 | 0 (exit 1) | 7 |
| untracked code files beyond the four | 0 | 0 (exit 1) | 8 |
| `agent-run.sh run stand` | 183 · 0 · 7 over 45 files | 183 · 0 · 7 (`run.end`, `outcome: passed`) | 10 |
| `agent-run.sh run text_extent` | 3 · 0 · 0 | 3 · 0 · 0 | 11 |
| a11y leg | 43 tests over 6 files | 43 passed over 6 targets (6 · 6 · 4 · 8 · 5 · 14) | 12 |
| workspace tally (`target/ci-logs/test.log`, fresh from entry 13) | 201 lines · 1055 · 0 · 12 | 201 lines · 1055 passed · 0 failed · 12 ignored | 14 |
| `ci-leg.sh fast` · `ci-leg.sh doc` | green | green | 13 · 15 |
| blitz-tests `tests/*.rs` files | 138 → 140 | 140 | — |
| blitz-tests `stand_*.rs` files | 44 → 45 | 45 | — |

Every forecast read as predicted on the first firing.
