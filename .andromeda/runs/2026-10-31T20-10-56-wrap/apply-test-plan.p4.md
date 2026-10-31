# Apply — test-plan · part 4 of 5

## Verdicts
- fanout-test-plan--cascade-g4.md 18 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:8#1
- fanout-test-plan--cascade-g4.md 19 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:10+#1
- fanout-test-plan--cascade-g4.md 20 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:12#1
- cascade-compare-test-plan.md 2 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:20+#1
- fanout-test-plan--cascade-g5.md 1 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:11+#1
- fanout-test-plan--cascade-g5.md 2 · applied · .andromeda/test-plan-detail/ci-integration.md:8#1, .andromeda/test-plan-detail/ci-integration.md:8#2
- fanout-test-plan--cascade-g5.md 3 · applied · .andromeda/test-plan-detail/ci-integration.md:8#3
- fanout-test-plan--cascade-g5.md 4 · applied · .andromeda/test-plan-detail/ci-integration.md:9#1, .andromeda/test-plan-detail/ci-integration.md:9#2, .andromeda/test-plan-detail/ci-integration.md:9#3
- fanout-test-plan--cascade-g5.md 5 · applied · .andromeda/test-plan-detail/ci-integration.md:10#1
- fanout-test-plan--cascade-g5.md 6 · applied · .andromeda/test-plan-detail/ci-integration.md:16#1
- fanout-test-plan--cascade-g5.md 7 · applied · .andromeda/test-plan-detail/ci-integration.md:30+#1
- fanout-test-plan--cascade-g5.md 8 · applied · .andromeda/test-plan-detail/ci-integration.md:30+#2
- fanout-test-plan--cascade-g5.md 9 · applied · .andromeda/test-plan-detail/ci-integration.md:75+#1
- fanout-test-plan--cascade-g5.md 10 · applied · .andromeda/test-plan-detail/ci-integration.md:97#1
- fanout-test-plan--cascade-g5.md 11 · applied · .andromeda/test-plan-detail/ci-integration.md:99#1, .andromeda/test-plan-detail/ci-integration.md:99#2
- fanout-test-plan--cascade-g5.md 12 · applied · .andromeda/test-plan-detail/ci-integration.md:99#3

## Edits
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:8
  old: «`cli_commands` (14)»
  new: «`cli_commands` (15)»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:10
  anchor: « since 2026-10-19-session-recording `start <task> --record <dir>` run as a process keeps one `0600` check in a `0700` directory, the state directory gone after `stop`, a group-open recording directory refused status `3` with nothing spawned (seven_guis `recording`), and `cli_commands` reads the usage line naming `--record` on `start`, the host role and the `mcp` role, a misplaced `--record` a usage error, status `2`, and since 2026-10-19-recorded-check-replay `recording`'s `the_committed_checks_are_what_the_binary_records` runs `record_checks.sh` — a command-line session (`start counter --record` · `snapshot` · `click counter-increment` · `advance 600` · `click counter-increment` · `snapshot` · `stop`) and an MCP one (`initialize` … `scroll counter-increment` · `snapshot` · `stop`) — through the binary and reads both kept checks byte-equal to `tests/blitz-tests/checks/counter-{cli,mcp}.json`, the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS (CI#39225375577) (examples/seven_guis/tests/recording.rs:205-262; examples/seven_guis/tests/recording.rs:264-304; examples/seven_guis/tests/cli_commands.rs:628-638)»
  add: «- **Command line, a select's list:** `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, under the state directory `cc-popup`, reads a hosted shot of an open list equal to the in-process one (examples/seven_guis/tests/cli_commands.rs:419-464; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:12
  old: «`mcp_log` (2) reads a recording role's stderr at `trace` too»
  new: «`mcp_log` (2) reads a recording role's stderr at `trace` too, `mcp_log` (3) reads a select list opened and chosen through the role logging no option at `trace`, its test recording nothing (examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:20
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · e2e -->»
  add: «»
  add: «- **The select list, headless:** the list paints through `paint_scene` alone, no shell or renderer code of its own, and the harness, the session and the binary drive it, measured headlessly; its windowed path read and not run (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:11
  anchor: «| Shell | `RecordingShell` implements `ShellProvider` and records `set_ime_enabled` calls | (tests/blitz-tests/tests/interaction_state_teardown.rs:27-35) |»
  add: «| Shell | `select_popup`'s own hand-written `CountingShell`, a `ShellProvider` counting redraw requests, reads that opening, moving, scrolling and closing a select's list each ask a redraw | (tests/blitz-tests/tests/select_popup.rs:1034-1043; tests/blitz-tests/tests/select_popup.rs:1045-1090) |»
- line: .andromeda/test-plan-detail/ci-integration.md:8
  old: «— 50 files,»
  new: «— 51 files,»
- line: .andromeda/test-plan-detail/ci-integration.md:8
  old: «`stand_cells_served` and `stand_load_page`»
  new: «`stand_cells_served`, `stand_load_page` and `stand_act_popup`»
- line: .andromeda/test-plan-detail/ci-integration.md:8
  old: «14 files and 112 tests, over the tier-0 witnesses alone, 9 files and 59 tests»
  new: «14 files and 114 tests, over the tier-0 witnesses alone, 9 files and 88 tests»
- line: .andromeda/test-plan-detail/ci-integration.md:9
  old: «fifteen named targets as literal `--test` words»
  new: «seventeen named targets as literal `--test` words»
- line: .andromeda/test-plan-detail/ci-integration.md:9
  old: «`checkable_change_event` and `text_range_no_change_event`, each file keeping its name (.github/scripts/ci-leg.sh:54-58)»
  new: «`checkable_change_event`, `text_range_no_change_event`, `select_popup` and `select_popup_painted`, each file keeping its name (.github/scripts/ci-leg.sh:54-60)»
- line: .andromeda/test-plan-detail/ci-integration.md:9
  old: «naming the glob and the fifteen words (.github/scripts/test_ci_workflows.py:361-365)»
  new: «naming the glob and the seventeen words (.github/scripts/test_ci_workflows.py:361-367)»
- line: .andromeda/test-plan-detail/ci-integration.md:10
  old: «pinning both words beside the fifteen»
  new: «pinning both words beside the seventeen»
- line: .andromeda/test-plan-detail/ci-integration.md:16
  old: «none of `stand-repeat`'s fifteen named `--test` words»
  new: «none of `stand-repeat`'s seventeen named `--test` words»
- after: .andromeda/test-plan-detail/ci-integration.md:30
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **Red on `Test (macos)`, and its local witness:** CI#41233234207 at `8cd9c1286` ended red on `Test (macos)` alone, sixteen jobs `success`: `apple-font-embolden` made it red on macOS alone, and no local gate builds `blitz-paint/font-embolden` (owner: a CARRY on Upstream sync ahead of the UI graph), so a local gate block reads green over a defect only macOS shows; a paint change under that feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:30
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **A red job's later targets:** `cargo` stops at the first failing target, so a red job's later targets go unread on that run — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:75
  anchor: « Re-read on CI#41192433140, the final HEAD `40ac6ddac` (green 17/17): linux 214 · 1181 · 0 · 12; macos and coverage 195 · 1178 · 0 · 12; windows 195 · 1132 · 0 · 9; the a11y job 14 · 112 · 0 · 0; the stand repeat 219 · 1008 · 0 · 21 — +5 result lines and +56 passed on each of the four test jobs over CI#40487650912, no new result line and +27 passed on the a11y job, +15 · +144 on the stand repeat — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md»
  add: « Re-read on CI#41235503227, the final HEAD `715798ce2` (green 17/17): linux 217 · 1236 · 0 · 12; macos and coverage 198 · 1233 · 0 · 12; windows 198 · 1185 · 0 · 9; the a11y job 14 · 114 · 0 · 0; the stand repeat 228 · 1155 · 0 · 21 — +3 result lines and +55 passed on linux, macos and coverage over CI#41192433140, +3 · +53 on windows, no new result line and +2 passed on the a11y job, +9 · +147 on the stand repeat — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md»
- line: .andromeda/test-plan-detail/ci-integration.md:97
  old: «escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (a `ci-leg.sh fast` run's tally probe)»
  new: «escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (a `ci-leg.sh fast` run's tally probe); re-counted with the select list's three new targets: 217 result lines, 1236 passed · 0 failed · 12 ignored — +3 result lines (`select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12) and +55 passed (those 43, blitz-dom's lib `select_list` 6, `stand_keyboard` +2, `stand_lint` +1, `cli_commands`, `host_log` and `mcp_log` +1 each), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md (CI#41235503227's linux job)»
- line: .andromeda/test-plan-detail/ci-integration.md:99
  old: «14 result lines · 112 passed»
  new: «14 result lines · 114 passed»
- line: .andromeda/test-plan-detail/ci-integration.md:99
  old: «`target/ci-logs/a11y.log`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  new: «`target/ci-logs/a11y.log`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- line: .andromeda/test-plan-detail/ci-integration.md:99
  old: «stand_keyboard 21»
  new: «stand_keyboard 23»

## Patterns
[[pattern]]
id = "tp4-01"
fixed = "`cli_commands` (14)"

[[pattern]]
id = "tp4-02"
fixed = "`tests/blitz-tests/tests/stand_*.rs` — 50 files"

[[pattern]]
id = "tp4-03"
fixed = "`stand_cells_served` and `stand_load_page`"

[[pattern]]
id = "tp4-04"
fixed = "14 files and 112 tests"

[[pattern]]
id = "tp4-05"
fixed = "9 files and 59 tests"

[[pattern]]
id = "tp4-06"
fixed = "fifteen named targets as literal"

[[pattern]]
id = "tp4-07"
fixed = "`checkable_change_event` and `text_range_no_change_event`, each file keeping its name"

[[pattern]]
id = "tp4-08"
fixed = "naming the glob and the fifteen words"

[[pattern]]
id = "tp4-09"
fixed = "pinning both words beside the fifteen"

[[pattern]]
id = "tp4-10"
fixed = "fifteen named `--test` words"

[[pattern]]
id = "tp4-11"
fixed = "14 result lines · 112 passed"

[[pattern]]
id = "tp4-12"
fixed = "stand_keyboard 21"
