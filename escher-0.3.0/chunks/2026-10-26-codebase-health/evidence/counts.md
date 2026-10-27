# Gate counts — 2026-10-26-codebase-health (plan step 6)

Implement run `.andromeda/runs/2026-10-27T01-14-55-implement/`, one firing of the whole block on the refactored tree
(`gate-2026-10-26-codebase-health.json`). Summary line: `entries 22 · green 18 · red 0 · recorded 0 · timeout 0 ·
not-run 4` (the four `leg = 'operator'` entries 19–22). Logs under
`$TMPDIR/andromeda-gate/2026-10-26-codebase-health/implement-2026-10-27T01-14-55/`. Counts are libtest
`test result:` lines summed from each entry's log, or the line the entry's own tool prints.

| entry | what | baseline / prediction | measured |
|---|---|---|---|
| 1 | complexity reading | 88 at base → predicted 85 | `over 15: 85`, files read 429 (`evidence/complexity.md`) |
| 2 | `cargo test -p escher-driver` | 100 passed · 0 failed · 0 ignored | 2 result lines · 100 passed · 0 failed · 0 ignored |
| 3 | seven blitz-tests targets, per-package build | 27 passed · 0 failed (2 ignored) | 7 result lines · 27 passed · 0 failed · 2 ignored |
| 4 | four targets, workspace build | 18 passed · 0 failed · 2 ignored | 4 result lines · 18 passed · 0 failed · 2 ignored |
| 5 | five seven_guis targets | 30 passed · 0 failed · 0 ignored | 5 result lines · 30 passed · 0 failed · 0 ignored |
| 6 | silence census, three files | exit 1, last line 0 | exit 1 · `0` |
| 7 | `env!` in `mcp_tests.rs` | 2 | `2` |
| 8 | `info_span!` sites in escher-driver | 2 | `2` |
| 9 | tracing event / `#[instrument]` files in escher-driver | 0 | exit 1 · `0` |
| 10 | preservation guard | `guard: clean` | `guard: allowed 3 — packages/escher-driver/src/lint.rs, packages/escher-driver/src/mcp_tests.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs · companions excluded 0 · changed 3 · excluded 0` · `guard: clean` |
| 11 | `agent-run.sh boot` | outcome ready | `"outcome": "ready"`, cargo_exit 0 |
| 12 | `agent-run.sh run stand` | 205 · 0 · 7 | run.end passed 205 · failed 0 · ignored 7 · outcome passed |
| 13 | `ci-leg.sh a11y` | 14 · 85 · 0 · 0 | 14 result lines · 85 passed · 0 failed · 0 ignored |
| 14 | `ci-leg.sh stand-repeat` | 204 · 864 · 0 · 21, 3 of 3 rounds | 204 result lines · 864 passed · 0 failed · 21 ignored · `stand-repeat: 3 of 3 rounds green` (420.91 s) |
| 15 | `ci-leg.sh coverage` | escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 % | escher-driver 4930/5031 = 97.99 % (floor 97 %) · dioxus-native-dom 2741/3202 = 85.60 % (floor 83 %) · blitz-test-harness 952/1008 = 94.44 % (floor 90 %) · `coverage floor: 3 of 3 crates at or above their floors`; 189 result lines · 1114 passed · 0 failed · 12 ignored; `lint.rs` 394 lines, 14 missed (96.45 %) |
| 16 | `ci-leg.sh fast` | green | green; 208 result lines · 1117 passed · 0 failed · 12 ignored; CI scripts `Ran 108 tests` · `OK`; artifact `target/ci-logs/test.log` fresh |
| 17 | workspace tally over `test.log` | 208 · 1117 · 0 · 12 | `lines 208 passed 1117 failed 0 ignored 12` |
| 18 | `ci-leg.sh doc` | green | green, exit 0 |

Every count equals its baseline or prediction. The one moved figure is escher-driver's counted lines: +12 lines,
+12 covered, so its percentage reads 97.99 % as before. The factoring's new lines in `lint.rs` are all covered
by the workspace run.
