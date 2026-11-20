# P5 controls — 2026-11-20-stand-test-setup-shared

Each control is the entry's own `run` with one input operand changed (a base sha, a report path, the bar's two numbers),
fired as `bash -o pipefail -c '…'` from the project root at HEAD `1b0125c103721081449288aff17bf977fa74dfe7`, the
untouched tree. The instruments are `.andromeda/cache/chunks/2026-11-20-stand-test-setup-shared/evidence/{fold_guard.py,
stand_reading.py, sum_dup.py}`; the 0.3.0 Epoch 4 archive is `git archive 9464034d4` extracted under
`$TMPDIR/{scratchpad}/arch-9464034d4`, its jscpd report taken by the audit's recipe (scalars reproduced: pct 3.8,
5,493 / 144,543, 495 clones, test 266 / 3,086).

| entry | baseline on the untouched tree | control (operand) | reading |
|---|---|---|---|
| 4 fold guard | green — `fold guard: PASS` (files 197 · tests 930 · literals 3636) | base `9464034d4` (must fail) | exit 1, `fold guard: FAIL` — files +17, twelve files' test lists differ, literals +202 −7 |
| 5 no other path | green — exit 0 | base `9464034d4` (must fail) | exit 1 |
| 6 stand reading | red — `stand reading: 75 pairs / 870 lines · bar under 47 / 550 · stand reading bar: FAIL` (the subject this chunk builds) | the `9464034d4` report, bar 48 / 551 (must pass) | exit 0, `47 pairs / 550 lines · … · stand reading bar: PASS` |
| 6 stand reading | — | the `9464034d4` report, bar 47 / 550 (must fail at the boundary) | exit 1, `stand reading bar: FAIL` |

The fold guard's test count is exact: `git grep -c '#\[test\]'` reads 931 over the same files, the one more a doc
comment (`stand_requirement_sweep.rs:241`), which the guard's lexer strips.
