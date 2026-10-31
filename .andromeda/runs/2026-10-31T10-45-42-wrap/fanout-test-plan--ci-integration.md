proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "In the stand-repeat clause replace `ten named targets as literal` with `fifteen named targets as literal`, and end the list `..., `select_interaction`, `text_extent`, `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event`, each file keeping its name`."
    sidecar: "CI Integration, Legs: stand-repeat's named targets ten to fifteen, the chunk's five new checks joining (2026-10-31-engine-debts)."
    rationale: "Report, Counts moved: `stand-repeat`'s named targets ten to fifteen, `ten-named` changed at this line; Harness surface: `ci-leg.sh` names five more. Their identity is the report's New checks (5), and the stand repeat's own move agrees (+15 result lines, 5 targets by 3 rounds; +144 passed, 48 per round). Binds with test-plan.md:377."
    basis: .andromeda/test-plan-detail/ci-integration.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Replace `naming the glob and the ten words` with `naming the glob and the fifteen words`."
    sidecar: "CI Integration, Legs: the leg's pin names the glob and fifteen words."
    rationale: "Report, Harness surface: `test_stand_repeat_leg_runs_three_rounds` names the five more targets too. A second occurrence on the same line, its own edit."
    basis: .andromeda/test-plan-detail/ci-integration.md:9
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Replace `pinning both words beside the ten` with `pinning both words beside the fifteen`."
    sidecar: "CI Integration, Legs: the seven_guis words stand beside the fifteen."
    rationale: "Report, Harness surface: the pin names the five more targets. This hit is outside the `ten-named` pattern's row list, found by this detector's own sweep, which the report keeps as the control."
    basis: .andromeda/test-plan-detail/ci-integration.md:10
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 16
    section: "## 9. CI Integration"
    change: "Replace `is none of `stand-repeat`'s ten named `--test` words` with `is none of `stand-repeat`'s fifteen named `--test` words`; the rest of the line stands."
    sidecar: "CI Integration, Legs reaching `pseudo_element_flow_classification`: none of the fifteen named words."
    rationale: "Report, Counts moved: `ten-named` and `flow-class` each list this line as changed; the target is not among the five new checks, so the negative holds at the new count."
    basis: .andromeda/test-plan-detail/ci-integration.md:16
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the a11y leg's membership clause replace `14 files and 85 tests` with `14 files and 112 tests`; the fourteen files stand."
    sidecar: "CI Integration, Legs: the a11y leg's tally 14 files and 85 tests to 112 tests, its membership unmoved."
    rationale: "Report, Counts moved: `a11y` leg 14 · 85 to 14 · 112, fourteen files unmoved, `a11y-tally` @c1256 changed at this line."
    basis: .andromeda/test-plan-detail/ci-integration.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 73
    section: "## 9. CI Integration"
    change: "Replace the `ci-leg.sh a11y` reading with `14 result lines · 112 passed · 0 failed · 0 ignored (accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 · stand_contrast 7 · stand_keyboard 21 · accessibility_states 6 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 6 · select_interaction 18 · stand_act_select 10, `target/ci-logs/a11y.log`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)`; the fmt, clippy, doc, audit and coverage readings stand."
    sidecar: "CI Integration, Local baseline: `ci-leg.sh a11y` reads 14 · 112, six per-file counts moved."
    rationale: "Report, Counts moved: `a11y-tally`, `n-inert` 4 to 6, `n-a11y-states` 5 to 6, `n-select` 7 to 18, `n-act-select` 5 to 10, `n-keyboard` 15 to 21 and `n-contrast` 5 to 7 each change this line; Outcome's gate block reads `lines 14 passed 112 failed 0`. The fourteen rows sum to 112."
    basis: .andromeda/test-plan-detail/ci-integration.md:73
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 50
    section: "## 9. CI Integration"
    change: "After this line add the continuation row: ` Re-read with the engine debts on CI#41192433140, the fix commit `40ac6ddac` (green 17/17): linux 214 · 1181 · 0 · 12; macos and coverage 195 · 1178 · 0 · 12; windows 195 · 1132 · 0 · 9; the a11y job 14 · 112 · 0 · 0; the stand repeat 219 · 1008 · 0 · 21; ios and android build only — +5 result lines and +56 passed on each of the four test jobs over CI#40487650912 (the five new targets 17, thirteen edited checks' 31 in existing lines, blitz-dom's lib 6, blitz-shell's lib 1, blitz-vibey-script's `dom` 1), +27 passed on the a11y job, no new result line, and +15 result lines and +144 passed on the stand repeat (the five new targets and those checks' moves, 48 passed per round) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md`."
    sidecar: "CI Integration, Per-OS test tallies: a new row at CI#41192433140; the earlier rows stand as history."
    rationale: "Report, Counts moved: `ci-tallies` and `ws-tally` keep lines 49 and 50 as history with the new row added beside; Outcome's per-job tally names each job's figures. The moves are this detector's differences against line 50 and reconcile with the report's per-target and unit counts."
    basis: .andromeda/test-plan-detail/ci-integration.md:50
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 71
    section: "## 9. CI Integration"
    change: "Close this line with `;` and add after it the continuation row: ` re-counted with the engine debts: 214 result lines, 1181 passed · 0 failed · 12 ignored — +5 result lines (`visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event`, `text_range_no_change_event`) and +56 passed (those five 17, thirteen edited checks' 31 in existing lines, blitz-dom's lib 6, blitz-shell's lib 1, blitz-vibey-script's `dom` 1), timings not re-measured — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (a `ci-leg.sh fast` run's tally probe)`."
    sidecar: "CI Integration, Local baseline: the workspace re-count 209 · 1125 to 214 · 1181."
    rationale: "Report, Counts moved: workspace tally `209 · 1125 · 0 · 12` to `214 · 1181 · 0 · 12`, `ws-tally` changed at this line; Outcome's gate block reads `lines 214 passed 1181 failed 0 ignored 12`. A new row, since this line's own earlier rows are history, and line 71 stays under the 2 000-char limit."
    basis: .andromeda/test-plan-detail/ci-integration.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 22
    section: "## 9. CI Integration"
    change: "After this line, in its group, add the record: `- **Free-disk step in `Stand checks repeated`:** since 2026-10-31-engine-debts the job runs `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` after the toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false`, a widening on the operator's word (.github/workflows/ci.yml:311-316). CI#41190771843 on `42eed99a6` ended `failure`, this job the one failed, linker bus errors, `Free space left: 5 MB`, no test run; on CI#41192433140 on `40ac6ddac` the step read `completed · success`, its first witness in this workflow, taking `/dev/root` from 86 G free to 109 G free. Written, kept and not shown to be the cure: that green runner held 86 GB free before the step, the failed run holds no `df` line, and the red job alone among the `ubuntu-24.04` jobs stood on image `20261018.96.1`, agent `20261015.610`, the others and the same job on the green run on `20261025.353.1`, agent `20261021.618`, that older image's free disk unread — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md`."
    sidecar: "CI Integration: a new record, the free-disk step in `Stand checks repeated`, its witness and its unproven cure."
    rationale: "Report, Harness surface states the step and that `free-disk`'s control is unfired, so no master holds it; Insufficient fixes and the Addendum bearing on it state the witness, the unproven cure and the runner images. Binds with test-plan.md:381, the body's Cache line naming the same job."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 22
    section: "## 9. CI Integration"
    change: "After the free-disk record add the record: `- **The matrix job's free-disk step, skipped:** the older step of the same action (.github/workflows/ci.yml:392; .github/workflows/ci.yml:394) fires on no matrix row: its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm` and the android row runs `ubuntu-latest`, reading `completed · skipped` on CI#41190771843's `Test (android)`, so this workflow held no measured run of the action before 2026-10-31-engine-debts; a first report that it already ran, read from the YAML alone, stood wrong until the job's state was read — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/ci.md`."
    sidecar: "CI Integration: a new record, the matrix job's older free-disk step reads skipped on every row."
    rationale: "Report, Harness surface: that older step is skipped on every matrix row, owner P5; Spec claims disproved: the operator pass's first report of it, pattern `free-disk`, no master site, its control unfired."
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness read no violation in this file and no Retired item stands in it. Lines 48 and 71's `ten named` wording and lines 47 to 50's counts stay history as the report disposes them; test-plan.md:377 and :413 hold the same ten for the body's own pass; the `9 files and 59 tests` at line 8 is no figure the report re-measures, so it is left as written."
