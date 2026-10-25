# CI — 2026-10-24-a11y-re-read

The run on the pushed sha `4cbf0a4c93e9c60ac192968004daa95c2ba4cb32` (the operator pre-CI commit): **CI#40134371519**,
`verdict: green · checks 17/17 · wall 1092 s` (entry 20, `ci.py conclusion --sha HEAD --wait 1800`, read
2026-10-24T21:47Z).

## Per-job tallies (entry 21, report-only, `<id>` = 40134371519)

lines · passed · failed · ignored, as the entry recorded them (its log `$TMPDIR/andromeda-gate/2026-10-24-a11y-re-read/run-20261024T214728Z/21.log`):

| job | CI#40125511019 (db585b711, the control) | CI#40134371519 (this chunk) | moved |
|---|---|---|---|
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 14 · 84 · 0 · 0 | +8 lines, +41 passed |
| Test [default features] | 206 · 1105 · 0 · 12 | 206 · 1105 · 0 · 12 | — |
| Test (macos) | 187 · 1102 · 0 · 12 | 187 · 1102 · 0 · 12 | — |
| Coverage report | 187 · 1102 · 0 · 12 | 187 · 1102 · 0 · 12 | — |
| Test (windows) | 187 · 1057 · 0 · 9 | 187 · 1057 · 0 · 9 | — |
| Stand checks repeated | 198 · 837 · 0 · 21 | 198 · 837 · 0 · 21 | — |

Every job reads the plan's prediction. The one move is the a11y job's, and it is the eight joining files alone.

## The a11y job's move, per file

Read from the same run's log, the "Accessibility (a11y) tests" job's `Running tests/{name}.rs` and `test result:`
lines (colour codes, in both their escape and `^[` spellings, stripped first) — passed / failed / ignored:

| file | CI#40134371519 | joined |
|---|---|---|
| accessibility_hidden | 6 / 0 / 0 | before |
| accessibility_roles | 6 / 0 / 0 | before |
| focusability_updates | 4 / 0 / 0 | before |
| stand_keyboard | 14 / 0 / 0 | before |
| stand_contrast | 5 / 0 / 0 | before |
| stand_accessibility_ids | 8 / 0 / 0 | before |
| accessibility_states | 5 / 0 / 0 | this chunk |
| accessibility_label | 13 / 0 / 0 | this chunk |
| accessibility_description | 3 / 0 / 0 | this chunk |
| accessibility_merged_states | 3 / 0 / 0 | this chunk |
| accessibility_active_descendant_unread | 1 / 0 / 0 | this chunk |
| inert_subtree | 4 / 0 / 0 | this chunk |
| select_interaction | 7 / 0 / 0 | this chunk |
| stand_act_select | 5 / 0 / 0 | this chunk |

The six that were there before read 43, as on CI#40125511019. The eight joining files read 41
(5 + 13 + 3 + 3 + 1 + 4 + 7 + 5), the same counts as the local leg (evidence/counts.md).
