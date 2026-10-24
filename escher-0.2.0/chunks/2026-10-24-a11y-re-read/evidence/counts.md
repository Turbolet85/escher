# Counts — 2026-10-24-a11y-re-read

Measured by /implement run `.andromeda/runs/2026-10-24T21-09-21-implement/` (gate trail
`gate-2026-10-24-a11y-re-read.json`), 2026-10-24T21:10Z–21:21Z, on db585b711 plus the chunk's two script edits.

## The widened `a11y` leg (entry 1, `bash .github/scripts/ci-leg.sh a11y`, workspace build)

Tally probe (entry 2), as printed: `lines 14 passed 84 failed 0 ignored 0` — the prediction exactly.

Per file, passed / failed / ignored, read from `target/ci-logs/a11y.log` (fresh: entry 1's `artifact` key):

| file | measured | predicted | joined |
|---|---|---|---|
| accessibility_hidden | 6 / 0 / 0 | 6 | before |
| accessibility_roles | 6 / 0 / 0 | 6 | before |
| focusability_updates | 4 / 0 / 0 | 4 | before |
| stand_keyboard | 14 / 0 / 0 | 14 | before |
| stand_contrast | 5 / 0 / 0 | 5 | before |
| stand_accessibility_ids | 8 / 0 / 0 | 8 | before |
| accessibility_states | 5 / 0 / 0 | 5 | this chunk |
| accessibility_label | 13 / 0 / 0 | 13 | this chunk |
| accessibility_description | 3 / 0 / 0 | 3 | this chunk |
| accessibility_merged_states | 3 / 0 / 0 | 3 | this chunk |
| accessibility_active_descendant_unread | 1 / 0 / 0 | 1 | this chunk |
| inert_subtree | 4 / 0 / 0 | 4 | this chunk |
| select_interaction | 7 / 0 / 0 | 7 | this chunk |
| stand_act_select | 5 / 0 / 0 | 5 | this chunk |

Moved counts: lines 6 → 14 and passed 43 → 84. The +8 lines and +41 passed are the eight joining files above
(5 + 13 + 3 + 3 + 1 + 4 + 7 + 5 = 41). No file of the six moved.

## The same fourteen, per-package build (entry 7)

`cargo test -p blitz-tests --locked --test …` over the fourteen: 14 result lines, 84 passed, 0 failed, 0 ignored,
the same per-file counts as the leg.

## Other tallies

- `bash scripts/agent-run.sh run stand` (entry 13), its `run.end` event: `"passed": 197, "failed": 0, "ignored": 7`,
  `"outcome": "passed"` — the prediction, unchanged.
- `bash .github/scripts/ci-leg.sh stand-repeat` (entry 14): 198 result lines, 837 passed, 0 failed, 21 ignored, last
  line `stand-repeat: 3 of 3 rounds green` — as CI#40125511019's Stand checks repeated.
- `bash .github/scripts/ci-leg.sh fast` (entry 15), the workspace tally probe (entry 16): `lines 206 passed 1105
  failed 0 ignored 12` — the prediction, unchanged (no test added).
- `python3 -m unittest discover -s .github/scripts -p test_ci_workflows.py` (entry 5): `Ran 33 tests`, `OK`.
