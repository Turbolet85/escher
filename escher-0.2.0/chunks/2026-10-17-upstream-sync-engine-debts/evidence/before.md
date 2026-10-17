# Before — the readings the merge can move (untouched tree, HEAD 143b6013)

Read at /implement P1 step 1 (2026-10-17T14:35Z) from the named logs on the untouched tree, each with its write time
(UTC, the file's mtime). No merge had started.

| Reading | Value | Source (mtime, UTC) |
|---|---|---|
| Workspace tally (`fast`'s test leg) | 186 result lines · 958 passed · 0 failed · 12 ignored | `target/ci-logs/test.log` (2026-10-17T08:33:47Z) |
| CI-scripts tally | `Ran 100 tests` · `OK` | `target/ci-logs/ci-scripts.log` (2026-10-17T08:33:56Z) |
| a11y leg | 6 result lines · 43 passed · 0 failed | `target/ci-logs/a11y.log` (2026-10-17T14:24:09Z, P5's baseline run) |
| `run stand` | run.end passed 168 · failed 0 · ignored 7 · cargo_exit 0 · `passed` | `target/agent-run/events.jsonl` run.end 2026-10-17T14:25:58Z (head 143b6013) |
| `run all` | run.end passed 533 · failed 0 · ignored 12 · cargo_exit 0 · `passed` | `target/agent-run/events.jsonl` run.end 2026-10-17T14:27:04Z (head 143b6013) |
| Files under `tests/blitz-tests/tests/*.rs` | 125 (`all.rs` included) → 124 targets | `ls tests/blitz-tests/tests/*.rs \| wc -l` at 14:35Z |
| `stand_*.rs` files | 40 | `ls tests/blitz-tests/tests/stand_*.rs \| wc -l` at 14:35Z |
| Coverage, escher-driver | 4014/4088 = 98.19 % (floor 97 %) | `target/ci-logs/coverage.log` (2026-10-17T14:29:24Z) |
| Coverage, dioxus-native-dom | 2472/2934 = 84.25 % (floor 83 %) | same |
| Coverage, blitz-test-harness | 916/999 = 91.69 % (floor 90 %) | same |
| Coverage summary | `coverage floor: 3 of 3 crates at or above their floors` | same |

## Figures the standing checks assert
- Snapshot-text ceilings (`tests/blitz-tests/tests/stand_snapshot_text.rs:31-38`, `ceiling`): Counter 1024 · FlightBooker
  1536 · Timer 1536 · Crud 2304 bytes, every one under `SNAPSHOT_TEXT_BUDGET` = 10,000
  (`packages/dioxus-native-dom/src/snapshot_text.rs:16`). The check asserts the ceiling, not the exact length.
- `crud-list` bounds (24, 112.796875, 369 × 419.203125) — asserted at `tests/blitz-tests/tests/stand_act_screenshot.rs:130`.
- `task-body` (0, 46, 800 × 554) — a layout-templates reading (`.andromeda/layout-templates.md:45`); no check pins
  the four figures by literal (`command grep -rn -F 554` over `tests/`, `examples/seven_guis/tests`,
  `packages/escher-driver/src`: 0 hits).

The test.log tally the plan quotes at "10:33" is this log's local-time stamp (UTC+2); its UTC mtime is 08:33:47Z.
