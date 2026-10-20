# CI — 2026-10-20-select-by-id

Operator pass, run on the operator's word in this session (2026-10-20).

## The push and the run
- Entry 20, `gate.py hygiene` fired as written: `hygiene: clean — read 51 (runs 42 · evidence 5 · inputs 4) · trails 14
  not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
- Pre-CI commit `14818317c`: `chore(2026-10-20-select-by-id): operator pre-CI commit, for the run this chunk's verdict
  reads`.
- Entry 21, `gate.py run --operator 21`:
  ` 21 probe       green · exit 0 · 99.89s · 112162 B → 21.log · history moved: refs/remotes/origin/build/escher-0.2.0 2f099735→14818317 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)`
  · `entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22`.
  The history move is the push the entry exists to make.
- Entry 22, `ci.py conclusion --sha HEAD --wait 1800` fired as written:
  `14818317c397 verdict: green · checks 17/17 · wall 1022 s · runs CI#39371468367 completed/success`.

## Per-OS tallies (entry 23, `--id 39371468367`)
Line: `23 probe recorded · exit 0 · 5.86s · 266 B → 23.log · history: unmoved`. Its output, as job · lines · passed ·
failed · ignored, against the previous chunk's CI#39365557179:

| job | CI#39365557179 | CI#39371468367 | moved |
|---|---|---|---|
| Test [default features] | 202 · 1062 · 0 · 12 | 204 · 1083 · 0 · 12 | +2 lines, +21 passed |
| Test (macos) | 183 · 1059 · 0 · 12 | 185 · 1080 · 0 · 12 | +2 lines, +21 passed |
| Coverage report | 183 · 1059 · 0 · 12 | 185 · 1080 · 0 · 12 | +2 lines, +21 passed |
| Test (windows) | 183 · 1017 · 0 · 9 | 185 · 1038 · 0 · 9 | +2 lines, +21 passed |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | unmoved |
| Stand checks repeated | 162 · 645 · 0 · 21 | 165 · 660 · 0 · 21 | +3 lines, +15 passed |

Each move, by name:
- **+2 lines** on every job that runs blitz-tests: the two new targets, `select_interaction` and `stand_act_select`.
  This is the prediction.
- **+21 passed:** `node::select::tests` 4 (blitz-dom), `snapshot::tests` 2 (dioxus-native-dom), `select_interaction`
  7, `stand_act_select` 5 and `accessibility_label` 3. The same 21 as the local tally (`evidence/counts.md`).
- **Stand checks repeated, +3 lines and +15 passed:** `stand_act_select`, the one new `stand_*` file, runs three
  rounds of 5 tests each under the zero-failure budget. `select_interaction` and `accessibility_label` are not
  `stand_*` files.
- No job moved its failed or ignored count. The coverage job's floors hold: the run's verdict is green, with 17 of 17
  checks.
