# Counts — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Measured by /andromeda-implement, run `.andromeda/runs/2026-10-19T19-16-25-implement/`, 2026-10-19, against
`evidence/before.md`. Lines quoted from that run's trail `gate-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`.

## The two new targets
```
 10 unit        green · exit 0 · 4.06s · 1720 B → 10.log · cargo test -p blitz-tests --locked --test focus_within 2>&… (59 chars)
 11 unit        green · exit 0 · 0.8s · 559 B → 11.log · cargo test -p blitz-tests --locked --test focus_redraw 2>&… (59 chars)
```
- `Running tests/focus_within.rs` — `test result: ok. 7 passed; 0 failed; 0 ignored`.
- `Running tests/focus_redraw.rs` — `test result: ok. 4 passed; 0 failed; 0 ignored`.

## The workspace tally (entry 15, over this run's `target/ci-logs/test.log`)
```
 15 probe       green · exit 0 · 0.0s · 42 B → 15.log · grep -E '^test result:' target/ci-logs/test.log | awk '{p+… (146 chars)
```
- before: `lines 197 passed 1034 failed 0 ignored 12`
- after: `lines 199 passed 1045 failed 0 ignored 12`
- moved: +2 lines, +11 passed — `focus_within` (7) and `focus_redraw` (4), each one target. As predicted.
- `Running tests/all.rs` lines in the log: 0 (`all.rs` stays `test = false`).

## The blitz-tests directory
```
 16 probe       green · exit 1 · 0.24s · 2 B → 16.log · for f in tests/blitz-tests/tests/*.rs; do n=$(basename "$f… (184 chars)
 17 probe       green · exit 0 · 0.01s · 4 B → 17.log · ls tests/blitz-tests/tests/*.rs | wc -l
```
- `.rs` files: 136 → 138 (the two upstream files); targets 135 → 137. Entry 16 reads `0` absent: every file but
  `all.rs` ran as its own target in the workspace leg.

## The CI-scripts tally (entry 18, recorded)
- `Ran 100 tests in 8.679s` · `OK` — unmoved (before: `Ran 100 tests`).

## The a11y leg (entries 21, 22)
- `lines 6 passed 43 failed 0` — unmoved: `accessibility_hidden` 6 · `accessibility_roles` 6 · `focusability_updates` 4 ·
  `stand_accessibility_ids` 8 · `stand_contrast` 5 · `stand_keyboard` 14.

## agent-run
```
 24 integration green · exit 0 · 69.29s · 24139 B → 24.log · bash scripts/agent-run.sh run stand
 25 probe       green · exit 0 · 0.0s · 3 B → 25.log · ls tests/blitz-tests/tests/stand_*.rs | wc -l
 26 integration green · exit 0 · 79.15s · 73647 B → 26.log · bash scripts/agent-run.sh run all
```
- `run stand`: run.end `passed 178 failed 0 ignored 7` — unmoved from the base (178 · 0 · 7). The stand selection
  lists 44 files, unmoved.
- `run all`: run.end `passed 586 failed 0 ignored 12` — before 575 · 0 · 12; +11 passed, the `focus_within` (7
  `test` events) and `focus_redraw` (4) files.

## Focused checks (entries 12, 13)
- `changed_set_tests`: 7 passed, `changed_set_focus_move_marks_both_nodes_and_hover_marks_nothing ... ok`.
- `inert_subtree` 4 · `interaction_state_teardown` 4 · `stale_interaction_state` 5 · `stand_accessibility_ids` 8 ·
  `stand_keyboard` 14 — all passed, unmoved from P5's untouched-tree reading.

## seven_guis (entry 27)
- 12 result lines · 33 passed · 0 failed — unmoved.
