proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the a11y leg's membership clause, replace `14 files and 85 tests` with `14 files, 112 tests since 2026-10-31-engine-debts and 85 until then`; the fourteen files, the `--test` list and `9 files and 59 tests` stand."
    sidecar: "2026-10-31-engine-debts: the a11y leg's fourteen files read 112 tests, 85 until then."
    rationale: "Report, Counts / qualifiers moved: `a11y` leg 14 · 85 → 14 · 112, fourteen files unmoved, with `a11y-tally` changed at `ci-integration.md:8` (@c1256)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Replace `ten named targets as literal --test words` with `fifteen named targets as literal --test words (ten until 2026-10-31-engine-debts)`, add `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` to the list after `text_extent`, and replace `naming the glob and the ten words` with `naming the glob and the fifteen words`."
    sidecar: "2026-10-31-engine-debts: stand-repeat names fifteen targets, the five new ones joining the ten."
    rationale: "Report: `stand-repeat`'s named targets ten → fifteen (`ci-leg.sh:58-59`, `test_ci_workflows.py:365-367`), `ten-named` changed at `ci-integration.md:9`; Harness / gate surface: `test_stand_repeat_leg_runs_three_rounds` names them too. The report's Expected amendments says the five targets, and the stand repeat's +15 result lines are five targets over three rounds."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Replace `pinning both words beside the ten` with `pinning both words beside the fifteen`."
    sidecar: "2026-10-31-engine-debts: the leg's pin holds seven_guis' two words beside fifteen named targets."
    rationale: "Report, Harness / gate surface: `test_stand_repeat_leg_runs_three_rounds` names the five more targets. The `ten-named` pattern does not list this line, whose wording is `the ten`; the same count moved."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "Replace `none of stand-repeat's ten named --test words` with `none of stand-repeat's fifteen named --test words`; the rest of the claim stands, the target joining neither `stand-repeat` nor the `a11y` leg."
    sidecar: "2026-10-31-engine-debts: pseudo_element_flow_classification stays outside stand-repeat's fifteen named targets."
    rationale: "Report: `ten-named` changed at `ci-integration.md:16`. The target is an edited check and none of the five new targets, and the stand repeat's +15 result lines are those five over three rounds, so it stays outside the leg."
    dependent-of: D-cascade
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "After `with no line of its script naming it`, add `; since 2026-10-31-engine-debts its three tests run `for incremental in [false, true]`, upstream's both-modes exception ended`."
    sidecar: "2026-10-31-engine-debts: pseudo_element_flow_classification's three tests run in both layout modes, upstream's exception ended."
    rationale: "Report, Retired: `upstream's both-modes exception`, the three tests now running `for incremental in [false, true]`; Counts / qualifiers moved: `flow-class` changed at `ci-integration.md:16`. The entity as picked states no exception, so this adds the moved fact and removes nothing."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 22
    section: "## 9. CI Integration"
    change: "Add after the Cache entity: `- **Red on `Stand checks repeated`, 2026-10-31:** CI#41190771843 at `42eed99a6` ended red on `Stand checks repeated` alone (job 124508189681, linker bus errors, `Free space left: 5 MB`, no test ran) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md`"
    sidecar: "2026-10-31-engine-debts: records CI#41190771843's red on Stand checks repeated, a runner out of disk."
    rationale: "Report, Cross-project / external claims, CI runs: CI#41190771843 at `42eed99a6` ended `failure`, the one failed job, `Free space left: 5 MB`, no test ran."
    fact: F26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 22
    section: "## 9. CI Integration"
    change: "Add after the Cache entity: `- **Free-disk step of `Stand checks repeated`:** since 2026-10-31-engine-debts the job runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`, with `android: true`, `large-packages: false`, `swap-storage: false`, on the operator's word (.github/workflows/ci.yml:311-316) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`"
    sidecar: "2026-10-31-engine-debts: Stand checks repeated gains a pinned free-disk step before rust-cache."
    rationale: "Report, Harness / gate surface: `.github/workflows/ci.yml:311-316`, the action, its place and its three `with` values, a widening on the operator's word."
    fact: F23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 22
    section: "## 9. CI Integration"
    change: "Add after the Cache entity: `- **The free-disk step's cure is unproven:** the step in `Stand checks repeated` ran `completed · success`, taking `/dev/root` from 86 G to 109 G free on the green runner, and is not shown to be the cure: that runner had 86 GB free before it, the failed run has no `df` line, and the red job alone among the `ubuntu-24.04` jobs stood on the older image `20261018.96.1`, whose free disk stays unread — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md`"
    sidecar: "2026-10-31-engine-debts: records that the free-disk step ran and is not shown to be the cure."
    rationale: "Report, Insufficient fixes: the step's run and the two disk figures; its Addendum, inputs#I14: the red job alone on image `20261018.96.1`, that image's free disk unread, the cure unproven."
    fact: F24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 22
    section: "## 9. CI Integration"
    change: "Add after the Cache entity: `- **The older free-disk step never fires:** the free-disk step at `ci.yml:392-394` is skipped on every matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest` (`completed · skipped` on CI#41190771843's `Test (android)`) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md`"
    sidecar: "2026-10-31-engine-debts: records that the matrix job's older free-disk step is skipped on every row."
    rationale: "Report, Harness / gate surface: the older step, now `:392` to `:394`, skipped on every matrix row, `completed · skipped` on `Test (android)`; Spec claims disproved corrects the first report's `already runs`."
    fact: F25
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 50
    section: "## 9. CI Integration"
    change: "Add after this row: ` Re-read at 2026-10-31-engine-debts on CI#41192433140, `40ac6ddac` (green 17/17): linux 214 · 1181 · 0 · 12; macos and coverage 195 · 1178 · 0 · 12; windows 195 · 1132 · 0 · 9; the a11y job 14 · 112 · 0 · 0; the stand repeat 219 · 1008 · 0 · 21; ios and android build only — +5 result lines and +56 passed on each of the four test jobs over CI#40487650912, +0 · +27 on the a11y job, +15 · +144 on the stand repeat — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md`"
    sidecar: "2026-10-31-engine-debts: per-OS tallies re-read on CI#41192433140; the earlier rows stand as history."
    rationale: "Report, Counts / qualifiers moved: the per-OS table against CI#40487650912 with each job's new row, `ci-tallies` and `ws-tally` naming :50 as history gaining a row beside it; Outcome's per-job tally states the same figures. The deltas are the differences between those rows."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 71
    section: "## 9. CI Integration"
    change: "Add after this line, continuing the chain: ` re-counted at 2026-10-31-engine-debts: 214 result lines, 1181 passed · 0 failed · 12 ignored — +5 result lines (`visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2) and +56 passed (those 17, 31 more in the edited checks, blitz-dom's lib +6, blitz-shell's lib +1, blitz-vibey-script's `dom` +1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (the workspace tally probe)`"
    sidecar: "2026-10-31-engine-debts: workspace re-count 214 · 1181 · 0 · 12."
    rationale: "Report: workspace tally `209 · 1125 · 0 · 12` → `214 · 1181 · 0 · 12`, `ws-tally` changed at `ci-integration.md:71`. The 31 is the sum of the report's moved target counts: `stand_act_transformed` +1, `inert_subtree` +2, `accessibility_states` +1, `select_interaction` +11, `stand_act_select` +5, `stand_lint` +1, `stand_keyboard` +6, `stand_contrast` +2, `stand_load_page` +1, `stand_cells_served` +1, `stand_non_lean_tasks` no count moved."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 73
    section: "## 9. CI Integration"
    change: "In the `ci-leg.sh a11y` clause, read `14 result lines · 112 passed · 0 failed · 0 ignored at 2026-10-31-engine-debts (accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 7 · stand_keyboard 21 · accessibility_states 6 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 6 · select_interaction 18 · stand_act_select 10, `target/ci-logs/a11y.log`), 85 passed until then`, the old reading's own `as measured at` clause kept on the 85."
    sidecar: "2026-10-31-engine-debts: the local a11y leg reads 14 · 112, with six per-target counts moved."
    rationale: "Report: `a11y-tally`, `n-inert`, `n-a11y-states`, `n-select`, `n-act-select`, `n-keyboard` and `n-contrast` all changed at `ci-integration.md:73`; Outcome's gate block holds the a11y tally `lines 14 passed 112 failed 0`, and the fourteen per-file counts sum to 112."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Replace `read now under the same floors` with `read at 2026-10-26-codebase-health under the same floors`, and after that reading's closing parenthesis add `; read at 2026-10-31-engine-debts on CI#41192433140, under the same floors and `coverage_floor.py`: escher-driver 4950/5049 = 98.04 %, dioxus-native-dom 2786/3235 = 86.12 %, blitz-test-harness 956/1008 = 94.84 %, `coverage floor: 3 of 3 crates at or above their floors``."
    sidecar: "2026-10-31-engine-debts: coverage reads 98.04, 86.12 and 94.84 %, the floors unmoved."
    rationale: "Report: coverage reads on CI#41192433140 with floors unmoved, `coverage-read` changed at `quality-gates-coverage-targets.md:16`, its reads; Outcome (CI) holds the verdict line."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Replace `the ten named blitz-tests targets of its --test list` with `the fifteen named blitz-tests targets of its --test list`, and open the parenthesis `(§9 Legs; with the fifteen CI#41192433140 read 219 · 1008 · 0 · 21, +15 · +144, `stand-repeat: 3 of 3 rounds green`; with the served Cells and load page ...`, the older readings standing."
    sidecar: "2026-10-31-engine-debts: the flakiness budget covers fifteen named targets, read on CI#41192433140."
    rationale: "Report: `ten-named` changed at `quality-gates-coverage-targets.md:22`; the stand repeat `219 · 1008 · 0 · 21` against CI#40487650912's 204 · 864; Outcome (CI): `stand-repeat: 3 of 3 rounds green`."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 377
    section: "## 9. CI Integration"
    change: "Replace `ten named blitz-tests targets outside the stand_ prefix` with `fifteen named blitz-tests targets outside the stand_ prefix`."
    sidecar: "2026-10-31-engine-debts: §9 Legs' broad line reads fifteen named targets."
    rationale: "Report: `ten-named` changed at `test-plan.md:377`."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 413
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Replace `the ten named blitz-tests targets` with `the fifteen named blitz-tests targets`."
    sidecar: "2026-10-31-engine-debts: §10 Flakiness budget's broad line reads fifteen named targets."
    rationale: "Report: `ten-named` changed at `test-plan.md:413`."
    dependent-of: D-cascade
note: The task and the report place pulldown-cmark at `ci-integration.md:73`, but that line as picked does not name it, so no proposal carries the bump; the report's five `stand-repeat` names are inferred from its `the five targets` and the +15 · +144 tally, and the orchestrator should confirm them against `ci-leg.sh:58-59`.
