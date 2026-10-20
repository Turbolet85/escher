# Counts measured — 2026-10-20-select-by-id

Read by /andromeda-implement from the second full firing of the block (run dir
`.andromeda/runs/2026-10-20T04-14-31-implement/`, trail `gate-2026-10-20-select-by-id.json`), each from its entry's
log under `$TMPDIR/andromeda-gate/2026-10-20-select-by-id/implement-2026-10-20T04-14-31/`.

| reading | predicted (plan) | measured | entry |
|---|---|---|---|
| workspace tally (`target/ci-logs/test.log`, fresh from entry 17) | lines 202 → 204, passed 1062 → more, failed 0, ignored 12 | `lines 204 passed 1083 failed 0 ignored 12` | 18 (18.2.log) |
| `run stand` | 46 → 47 files, 185 → 185 + `stand_act_select`'s tests, failed 0, ignored 7 | 47 files, `passed 190 failed 0 ignored 7` (`run.end` 2026-10-20T04:37:20Z) | 15 (15.2.log) |
| a11y leg per file | 6 · 6 · 4 · 8 · 5 · 14, 43 tests | 6 · 6 · 4 · 8 · 5 · 14, 43 tests | 16 (16.2.log) |
| `accessibility_label` | 10 → more | 13 (3 rows added); `accessibility_roles` 6, unmoved | 6 (6.2.log) |

The +21 passed in the workspace tally, by name: `node::select::tests` 4 (blitz-dom), `snapshot::tests` 2
(`a_select_reads_its_chosen_options_value`, `a_selects_options_are_its_children_named_by_their_labels`),
`select_interaction` 7, `stand_act_select` 5, `accessibility_label` 3. The +2 result lines are the two new targets.
