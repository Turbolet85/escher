proposals:
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "In the Flakiness budget item, in place of 'run the 48 `stand_*.rs` files and seven_guis' eight process checks three times with no retry', read 'run the 48 `stand_*.rs` files, ten named blitz-tests targets outside the `stand_` prefix (`inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `transformed_client_rect`, `accessibility_active_descendant_unread`, `virtual_clock_engine`, `select_interaction`, `text_extent` — since 2026-10-24-stand-requirement-sweep a fixed `--test` list after the glob on the same `cargo test -p blitz-tests --locked` line every round, the files keeping their names, on the founder's «Список в скрипте (Рекомендую)» (2026-10-24), pinned by `test_stand_repeat_leg_runs_three_rounds` and read from the script by `stand_requirement_sweep`'s leg check) and seven_guis' eight process checks three times with no retry'; add .github/scripts/ci-leg.sh:54-58 and .github/scripts/test_ci_workflows.py:359-362 to the item's citations"
    sidecar: "test-plan quality-gates: the flakiness budget's stand-repeat selection gains the ten named blitz-tests targets (2026-10-24-stand-requirement-sweep, the founder's word of 2026-10-24)"
    rationale: "Report Changes → Schema / config: `ci-leg.sh` `stand-repeat` (54-58) adds ten `--test` words after the `stand_*.rs` glob and `test_ci_workflows.py` (359-362) pins them; Counts fact 1 lists `test-plan-detail/quality-gates-coverage-targets.md:22` as Moved; Decisions & corrections: the founder's word, inputs#I1. The file states the leg's selection as the stand files and seven_guis' eight alone, which no longer matches the runner the chunk ships."
    basis: .github/scripts/ci-leg.sh:54-58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "In the same item's §9 Legs parenthetical, before 'with `stand_snapshot_parts` CI#39410865604 read 168 · 684 · 0 · 21, +3 · +12;', add 'with the ten named targets CI `Stand checks repeated` on CI#40114977936 read 198 · 837 · 0 · 21, +30 · +153, the local leg ending `stand-repeat: 3 of 3 rounds green` at the same counts (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md);'"
    sidecar: "test-plan quality-gates: stand-repeat's latest reading 198 · 837 · 0 · 21 on CI#40114977936 prepended to the flakiness budget's reading chain"
    rationale: "Report Counts fact 11 moves `quality-gates-coverage-targets.md:22` ('168 · 684 · 0 · 21' → 198 · 837 · 0 · 21); Harness / gate surface and Outcome read CI#40114977936's `Stand checks repeated` 198 · 837 · 0 · 21 and the local leg `3 of 3 rounds green` at the same counts. The item keeps its readings as a dated chain, newest first, so the new reading is prepended rather than replacing the dated one."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:196
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "After line 16, a continuation line inside the same gate-coverage-floor block: ' At 2026-10-24-stand-requirement-sweep, a chunk of test files and one CI leg only, the three floors and `coverage_floor.py` are unchanged and the `coverage` leg reads escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %, ending `coverage floor: 3 of 3 crates at or above their floors`, equal to CI#40105170961 (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md, gate entry 16)'"
    sidecar: "test-plan quality-gates: the per-crate coverage reading at 2026-10-24-stand-requirement-sweep (97.99 · 85.57 · 94.15 %, floors unchanged) added after the load-wait reading"
    rationale: "Report Counts closing paragraph: 'The coverage floors are unmoved: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %, equal to CI#40105170961'; Outcome: `coverage` ends `3 of 3`, the floors unchanged. Line 16's latest stated reading is escher-driver 4695/4794 = 97.93 % since 2026-10-20-load-wait, so the measured later reading is new detail; the floors themselves do not move."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:192
note: "test-plan.md:410 (body §10 Flakiness budget: 'the `stand_*.rs` files and seven_guis' process checks') is moved by the same fact 1 and lies outside this file; line 22's `.github/scripts/ci-leg.sh:48-65` cite spans the five lines added at 54-58 and is left to the cites re-point."
