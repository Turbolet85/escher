proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Append to the per-crate reading, after its last clause: `; read again at 2026-10-25-cold-agent-bar, the floors and coverage_floor.py unchanged: escher-driver 4918/5019 = 97.99 % (unchanged), dioxus-native-dom 2741/3202 = 85.60 % (was 2740/3202 = 85.57 %), blitz-test-harness 952/1008 = 94.44 % (was 949/1008 = 94.15 %), the leg green with 3 of 3 crates at or above their floors (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, F12 and its ci-leg.sh coverage gate)`. Keep the 2026-10-24 reading as the earlier one."
    sidecar: "test-plan quality-gates: per-crate coverage re-read at 2026-10-25-cold-agent-bar — dioxus-native-dom 85.57 → 85.60 %, blitz-test-harness 94.15 → 94.44 %, escher-driver 97.99 % unchanged; floors unchanged, 3 of 3."
    rationale: "Report Counts F12 names quality-gates-coverage-targets.md:16 as the one standing coverage row and marks it stale: dioxus-native-dom 2740/3202 = 85.57 % → 2741/3202 = 85.60 %, blitz-test-harness 949/1008 = 94.15 % → 952/1008 = 94.44 %, escher-driver 4918/5019 = 97.99 % unchanged; Outcome gate `ci-leg.sh coverage` green (3 of 3 crates at or above their floors)."
    basis: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Change `run the 48 stand_*.rs files` to `run the 50 stand_*.rs files`, and open the parenthesis's history with `with stand_cells_served and stand_load_page, picked up by their stand_ prefix with no CI step change, the local leg read stand-repeat: 3 of 3 rounds green at 2026-10-25-cold-agent-bar; 48 until then; ` before the existing `with the ten CI#40114977936 read …` clause; the ten named targets and seven_guis' eight process checks unchanged."
    sidecar: "test-plan quality-gates: the stand-repeat leg's stand_*.rs file count 48 → 50 (stand_cells_served, stand_load_page, by prefix, no CI step change)."
    rationale: "Report Counts F10 marks quality-gates-coverage-targets.md:22 (\"48 files\") stale: `agent-run.sh run stand` 48 → 50 files; Harness / gate surface: `stand-repeat` takes the two new `stand_*` files by prefix, no CI step changes; Outcome gate `ci-leg.sh stand-repeat` green (`stand-repeat: 3 of 3 rounds green`); Files: two new stand files, no seven_guis process-check file added (host_log gains a test in its own file)."
    basis: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22
note: the report states no per-round totals for CI#40214713710's `Stand checks repeated` job and does not re-read the workspace-wide coverage of line 8, so neither proposal adds a CI tally or moves line 8; the mutation tier (line 28) is untouched by this chunk.
