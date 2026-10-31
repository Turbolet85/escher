# Apply — test-plan · part 4 of 4

## Verdicts
- fanout-test-plan--cascade-g4.md 1 · applied · .andromeda/test-plan-detail/ci-integration.md:8#1
- fanout-test-plan--cascade-g4.md 2 · applied · .andromeda/test-plan-detail/ci-integration.md:9#1, .andromeda/test-plan-detail/ci-integration.md:9#2, .andromeda/test-plan-detail/ci-integration.md:9#3
- fanout-test-plan--cascade-g4.md 3 · applied · .andromeda/test-plan-detail/ci-integration.md:10#1
- fanout-test-plan--cascade-g4.md 4 · applied · .andromeda/test-plan-detail/ci-integration.md:16#1
- fanout-test-plan--cascade-g4.md 5 · applied · .andromeda/test-plan-detail/ci-integration.md:16#2
- fanout-test-plan--cascade-g4.md 6 · applied · .andromeda/test-plan-detail/ci-integration.md:24+#1
- fanout-test-plan--cascade-g4.md 7 · applied · .andromeda/test-plan-detail/ci-integration.md:24+#2
- fanout-test-plan--cascade-g4.md 8 · applied · .andromeda/test-plan-detail/ci-integration.md:24+#3
- fanout-test-plan--cascade-g4.md 9 · applied · .andromeda/test-plan-detail/ci-integration.md:24+#4
- fanout-test-plan--cascade-g4.md 10 · applied · .andromeda/test-plan-detail/ci-integration.md:50+#1
- fanout-test-plan--cascade-g4.md 11 · applied · .andromeda/test-plan-detail/ci-integration.md:71#1, .andromeda/test-plan-detail/ci-integration.md:71+#1
- fanout-test-plan--cascade-g4.md 12 · applied · .andromeda/test-plan-detail/ci-integration.md:73#1
- fanout-test-plan.md 10 · refused · its line is the body's .andromeda/test-plan.md:377, no target of this part, and the proposal it was merged into states the local a11y tally, not this fact, so a fold would lose it: test-plan.md:377 still reads `ten named blitz-tests targets` and needs its own edit; the detail's own statement is written by fanout-test-plan--cascade-g4.md 2
- fanout-test-plan--cascade-g4.md 13 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16#1
- fanout-test-plan.md 11 · refused · its line is the body's .andromeda/test-plan.md:413, no target of this part, and the proposal it was merged into states the coverage reads, not this fact, so a fold would lose it: test-plan.md:413 still reads `the ten named blitz-tests targets` and needs its own edit; the detail's own statement is written by fanout-test-plan--cascade-g4.md 14
- fanout-test-plan--cascade-g4.md 14 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22#1, .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22#2
- fanout-test-plan.md 5 · applied · .andromeda/registries/contracts/test-plan/session-lifecycle.md:11#1
- fanout-test-plan.md 6 · applied · .andromeda/registries/contracts/test-plan/session-lifecycle.md:11#2

## Edits
- line: .andromeda/test-plan-detail/ci-integration.md:8
  old: «14 files and 85 tests»
  new: «14 files and 112 tests»
- line: .andromeda/test-plan-detail/ci-integration.md:9
  old: «ten named targets as literal `--test` words»
  new: «fifteen named targets as literal `--test` words»
- line: .andromeda/test-plan-detail/ci-integration.md:9
  old: «`select_interaction` and `text_extent`, each file»
  new: «`select_interaction`, `text_extent`, `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event`, each file»
- line: .andromeda/test-plan-detail/ci-integration.md:9
  old: «naming the glob and the ten words»
  new: «naming the glob and the fifteen words»
- line: .andromeda/test-plan-detail/ci-integration.md:10
  old: «pinning both words beside the ten;»
  new: «pinning both words beside the fifteen;»
- line: .andromeda/test-plan-detail/ci-integration.md:16
  old: «`stand-repeat`'s ten named `--test` words»
  new: «`stand-repeat`'s fifteen named `--test` words»
- line: .andromeda/test-plan-detail/ci-integration.md:16
  old: «with no line of its script naming it;»
  new: «with no line of its script naming it; its three tests run `for incremental in [false, true]` (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md);»
- after: .andromeda/test-plan-detail/ci-integration.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **Red on `Stand checks repeated`:** CI#41190771843 at `42eed99a6` ended red on `Stand checks repeated` alone (job 124508189681, linker bus errors, `Free space left: 5 MB`, no test ran) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **Free-disk step of `Stand checks repeated`:** the job runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`, with `android: true`, `large-packages: false`, `swap-storage: false` (.github/workflows/ci.yml:311-316) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **The free-disk step's cure is unproven:** the step in `Stand checks repeated` ran `completed · success` on CI#41192433140, taking `/dev/root` from 86 G free to 109 G free, and is not shown to be the cure: that runner had 86 GB free before the step, CI#41190771843's failed run has no `df` line, and on that run the job alone among the `ubuntu-24.04` jobs stood on the older image `20261018.96.1`, whose free disk stays unread — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · ci -->»
  add: «»
  add: «- **The matrix job's free-disk step never fires:** the free-disk step at .github/workflows/ci.yml:392-394 is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest` (`completed · skipped` on CI#41190771843's `Test (android)`) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:50
  anchor: « Re-read with upstream's eleven commits on CI#40487650912, the pre-CI commit `e795a088c` (green 17/17): linux 209 · 1125 · 0 · 12; macos and coverage 190 · 1122 · 0 · 12; windows 190 · 1076 · 0 · 9; the a11y job 14 · 85 · 0 · 0 and the stand repeat 204 · 864 · 0 · 21, unmoved — +1 result line and +8 passed on each of the four test jobs over CI#40434241601, every move the take's by name — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/ci.md»
  add: « Re-read on CI#41192433140, the final HEAD `40ac6ddac` (green 17/17): linux 214 · 1181 · 0 · 12; macos and coverage 195 · 1178 · 0 · 12; windows 195 · 1132 · 0 · 9; the a11y job 14 · 112 · 0 · 0; the stand repeat 219 · 1008 · 0 · 21 — +5 result lines and +56 passed on each of the four test jobs over CI#40487650912, no new result line and +27 passed on the a11y job, +15 · +144 on the stand repeat — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md»
- line: .andromeda/test-plan-detail/ci-integration.md:71
  old: «2026-10-27-upstream-sync-engine-debts/evidence/counts.md (a `ci-leg.sh fast` run's tally probe)»
  new: «2026-10-27-upstream-sync-engine-debts/evidence/counts.md (a `ci-leg.sh fast` run's tally probe);»
- after: .andromeda/test-plan-detail/ci-integration.md:71
  anchor: « re-counted with upstream's `hit_test_dropped_nodes`: 206 result lines, 1099 passed · 0 failed · 12 ignored — +1 result line and +1 passed (`hit_test_dropped_nodes` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/counts.md (a `ci-leg.sh fast` run's `target/ci-logs/test.log`); re-counted with `stand-repeat`'s ten named targets: 206 result lines, 1105 passed · 0 failed · 12 ignored — no new result line and +6 passed (`text_extent` 3, `stand_requirement_sweep` 1, `stand_lint` 1, `host_log` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md (a `ci-leg.sh fast` run's `target/ci-logs/test.log`); re-counted with the served Cells and load page: 208 result lines, 1117 passed · 0 failed · 12 ignored — +2 result lines (`stand_cells_served`, `stand_load_page`) and +12 passed (`stand_cells_served` 4, `stand_load_page` 2, seven_guis' three stand unit tests, `host_log` 1, `stand_keyboard` 1, `stand_snapshot` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md (a `ci-leg.sh fast` run's tally probe); re-counted with upstream's eleven commits: 209 result lines, 1125 passed · 0 failed · 12 ignored — +1 result line (`pseudo_element_flow_classification`) and +8 passed (`pseudo_element_flow_classification` 3, blitz-vibey-script's `tests/dom.rs` 3, dioxus-native-dom 1, `apps/browser/src/tab.rs` 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/counts.md (a `ci-leg.sh fast` run's tally probe)»
  add: « re-counted with the five new targets: 214 result lines, 1181 passed · 0 failed · 12 ignored — +5 result lines (`visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2) and +56 passed (those 17, 31 more in the edited checks, blitz-dom's lib 6, blitz-shell's lib 1, blitz-vibey-script's `dom` 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (a `ci-leg.sh fast` run's tally probe)»
- line: .andromeda/test-plan-detail/ci-integration.md:73
  old: «14 result lines · 85 passed · 0 failed · 0 ignored (accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 5 · stand_keyboard 15 · accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5, `target/ci-logs/a11y.log`, as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  new: «14 result lines · 112 passed · 0 failed · 0 ignored (accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 7 · stand_keyboard 21 · accessibility_states 6 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 6 · select_interaction 18 · stand_act_select 10, `target/ci-logs/a11y.log`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
  old: «escher-driver 4930/5031 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, its `ci-leg.sh coverage` gate, evidence/counts.md row 15)»
  new: «escher-driver 4950/5049 = 98.04 %, dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at the `Coverage report` job of CI#41192433140, read in escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22
  old: «the ten named blitz-tests targets of its `--test` list»
  new: «the fifteen named blitz-tests targets of its `--test` list»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22
  old: «(§9 Legs; with the served Cells and load page»
  new: «(§9 Legs; with the fifteen CI#41192433140 read 219 · 1008 · 0 · 21, +15 · +144, `stand-repeat: 3 of 3 rounds green`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md; with the served Cells and load page»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
  old: «`stand_act_transformed` 2 (50; 48 before `stand_act_transformed`,»
  new: «`stand_act_transformed` 3 (51; 48 before `stand_act_transformed`,»
- line: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11
  old: «`stand_act_select` 5, the fifteenth (55 in all)»
  new: «`stand_act_select` 10, the fifteenth (61 in all)»

## Patterns
[[pattern]]
id = "tp4-01"
fixed = '14 files and 85 tests'

[[pattern]]
id = "tp4-02"
fixed = 'ten named targets as literal'

[[pattern]]
id = "tp4-03"
fixed = 'naming the glob and the ten words'

[[pattern]]
id = "tp4-04"
fixed = 'pinning both words beside the ten'

[[pattern]]
id = "tp4-05"
fixed = 'ten named `--test` words'

[[pattern]]
id = "tp4-06"
fixed = '14 result lines · 85 passed'

[[pattern]]
id = "tp4-07"
fixed = 'escher-driver 4930/5031 = 97.99'

[[pattern]]
id = "tp4-08"
fixed = 'the ten named blitz-tests targets'

[[pattern]]
id = "tp4-09"
fixed = '`stand_act_transformed` 2 (50'

[[pattern]]
id = "tp4-10"
fixed = '`stand_act_select` 5, the fifteenth (55 in all'
