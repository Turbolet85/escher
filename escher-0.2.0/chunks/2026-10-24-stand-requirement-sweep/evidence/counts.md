# Counts — /implement P2, run `.andromeda/runs/2026-10-24T16-43-19-implement/` (2026-10-24)

The gate block's summary line: `entries 23 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 4` (the four `leg = 'operator'`
entries, the operator pass's). Each reading below is copied from that run's entry log (`gate.py show … --n {n}` names it);
no raw stderr is copied.

| Entry | Reading | Plan's prediction / baseline |
|---|---|---|
| 1 · `-p blitz-tests` C1 | `stand_lint` 7 passed · `text_extent` 6 passed | 6 → 7 · 3 → 6 |
| 2 · `--workspace` C1 | `stand_lint` 7 · `text_extent` 6 (the vertical test green with `writing-mode` compiled in) | same |
| 3 · the record, spans, parts | `stand_requirement_sweep` 3 · `stand_act_spans` 2 passed 2 ignored (36 calls per mode, asserted in the child) · `stand_snapshot_parts` 3 | 2 → 3 · unchanged · unchanged |
| 4 · the ten, per package | 1 · 5 · 13 · 4 · 2 · 1 · 7 · 6 · 4 · 5 = 48 passed, 0 failed | 45 + `text_extent`'s 3 |
| 5 · `-p seven_guis` needles | `host_log` 3 · `mcp_log` 2 · `recording` 4 | 2 → 3 · 2 · 4 |
| 6 · `--workspace` needles | `host_log` 3 · `mcp_log` 2 | 2 → 3 · 2 |
| 7 · windows check | exit 0 | exit 0 |
| 8 · silence census | `0`, exit 1 | last line 0 |
| 9 · platform adapter in the stand graph | `0`, exit 1 | last line 0 |
| 10 · manifest / source guard | `0` | last line 0 |
| 11 · `gate.py guard` | `guard: allowed 9 … · companions excluded 0 · changed 9 · excluded 0` / `guard: clean` | clean |
| 12 · `agent-run.sh boot` | `"outcome": "ready"`, head `4b4804ba5` | ready |
| 13 · `agent-run.sh run stand` | `run.end` selection stand · passed 197 · failed 0 · ignored 7 · cargo_exit 0 | 195 → 197 (+`stand_lint` 1, +`stand_requirement_sweep` 1) |
| 14 · `a11y` leg | 6 · 6 · 4 · 8 · 5 · 14 = 43 passed, 0 failed | 43, unmoved |
| 15 · `stand-repeat` | rounds 1, 2, 3 then `stand-repeat: 3 of 3 rounds green`; 198 result lines · 837 passed · 0 failed · 21 ignored (360 s) | 168 → 198 lines (the ten × 3); passed 684 + 3 × 51 (the ten's 48, `stand_lint` +1, `stand_requirement_sweep` +1, `host_log` +1) = 837 |
| 16 · `coverage` | `packages/escher-driver 4918/5019 = 97.99 % (floor 97 %)` · `packages/dioxus-native-dom 2740/3202 = 85.57 % (floor 83 %)` · `packages/blitz-test-harness 949/1008 = 94.15 % (floor 90 %)` · `coverage floor: 3 of 3 crates at or above their floors` | equal to CI#40105170961; floors unmoved |
| 17 · `fast` | exit 0 (fmt · clippy both lines · workspace tests · CI scripts) | green |
| 18 · workspace tally | `lines 206 passed 1105 failed 0 ignored 12` | 206 · 1099 + 6 · 0 · 12 |
| 19 · `doc` | exit 0 | green |

C1's lint co-reading measured as predicted in both layout modes: `fx-cut-text` clipped · `fx-cut-text` truncated ·
`fx-padded-text` clipped · `fx-off-text` truncated · `fx-off-text` off-screen. No difference from the prediction to
record.
