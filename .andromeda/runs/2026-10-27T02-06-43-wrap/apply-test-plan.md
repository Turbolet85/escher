# Apply — test-plan

## Verdicts
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:129+#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:130+#1
- fanout-test-plan--cascade-g2.md 1 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:23+#1
- fanout-test-plan--cascade-g2.md 2 · applied · .andromeda/test-plan-detail/ci-integration.md:42+#1
- fanout-test-plan--cascade-g2.md 3 · applied · .andromeda/test-plan-detail/ci-integration.md:65#1
- fanout-test-plan--cascade-g2.md 4 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16#1
- fanout-test-plan--cascade-g2.md 5 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:30+#1, .andromeda/test-plan.md:427#1
- fanout-test-plan--cascade-g2.md 6 · folded into fanout-test-plan--cascade-g2.md 5

## Edits
- after: .andromeda/test-plan-detail/test-scope-summary.md:129
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **The 0.2.0 sweep test's helpers:** `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` keeps its table-order and leg checks and asserts each requirement row through the private `#[track_caller]` helper `assert_0_2_0_row`, which asserts each of the row's witnesses through the private `#[track_caller]` `assert_0_2_0_witness`; its target predicate is the private `inside_stand_repeat`, holding the two path-prefix constants `BLITZ_TESTS` and `SEVEN_GUIS`; the helpers' messages name the row and the index only (tests/blitz-tests/tests/stand_requirement_sweep.rs:1050-1073; tests/blitz-tests/tests/stand_requirement_sweep.rs:1075-1093; tests/blitz-tests/tests/stand_requirement_sweep.rs:1046-1107) — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/test-scope-summary.md:130
  anchor: «»
  next: «<!-- detail: procedure · scope-entities -->»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **The 0.2.0 sweep test's reach:** its leg check reads `.github/scripts/ci-leg.sh` as one whole text by `contains`, so a named target dropped from the `stand-repeat` leg alone reads green while the `a11y` leg still names it — only both sites removed read red (`--test select_interaction` at .github/scripts/ci-leg.sh:39 and .github/scripts/ci-leg.sh:57) — and stand-repeat membership is held jointly with `.github/scripts/test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds`, which asserts `select_interaction` among that leg's logged words (.github/scripts/test_ci_workflows.py:361-366); the gap's fix is owned by a `CARRY:` on the 0.3.0 working route's Stand requirement sweep entry — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md (evidence/row-controls.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:23
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **escher-driver (the MCP listing test's helpers):** mcp_tests.rs's `the_listing_is_the_twelve_verbs_of_the_table` keeps its outer assertions and its row walk, each row's name asserted inline, and asserts each listed tool through three private `#[track_caller]` helpers — `assert_described` (the tool's description and its result fields), `assert_schema` (the input schema's properties and their order) and `assert_property` (one argument's kind, enum and maximum) — whose messages name the row and the index only; the file holds 16 `#[test]`, 15 on unix (packages/escher-driver/src/mcp_tests.rs:415-416; packages/escher-driver/src/mcp_tests.rs:420-432; packages/escher-driver/src/mcp_tests.rs:434-466; packages/escher-driver/src/mcp_tests.rs:468-496) — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/ci-integration.md:42
  anchor: « re-read with upstream's `hit_test_dropped_nodes` on CI#40092092468, the pre-CI commit `34f6f051b` (green 17/17): linux 206 · 1099 · 0 · 12; macos and coverage 187 · 1096 · 0 · 12; windows 187 · 1052 · 0 · 9; the a11y job 6 · 43 · 0 · 0 and the stand repeat 168 · 684 · 0 · 21, unmoved — +1 result line and +1 passed on the four test jobs over CI#39410865604 (`hit_test_dropped_nodes` 1, not unix-gated) — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/ci.md; re-read with `stand-repeat`'s ten named targets on CI#40114977936, the pre-CI commit `18b50a2a2` (green 17/17): linux 206 · 1105 · 0 · 12; macos and coverage 187 · 1102 · 0 · 12; windows 187 · 1057 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 198 · 837 · 0 · 21 — no new result line and +6 passed on linux, macos and coverage over CI#40092092468 (`text_extent` 3, `stand_requirement_sweep` 1, `stand_lint` 1, `host_log` 1), +5 on windows (`host_log`'s new test unix-gated), +30 result lines and +153 passed on the stand repeat (the ten named targets, 48 passed per round, and `stand_requirement_sweep`, `stand_lint` and `host_log` 1 each per round) — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/evidence/ci.md; re-read with the `a11y` leg's fourteen files on CI#40134371519, the pre-CI commit `4cbf0a4c9` (green 17/17): linux 206 · 1105 · 0 · 12; macos and coverage 187 · 1102 · 0 · 12; windows 187 · 1057 · 0 · 9; the a11y job 14 · 84 · 0 · 0; the stand repeat 198 · 837 · 0 · 21 — every job but the a11y one equal to the base's CI#40125511019 (`db585b711`), the a11y job +8 result lines and +41 passed over its 6 · 43 there, the eight joining files read per file from that job's log (accessibility_states 5, accessibility_label 13, accessibility_description 3, accessibility_merged_states 3, accessibility_active_descendant_unread 1, inert_subtree 4, select_interaction 7, stand_act_select 5) — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/ci.md»
  add: « Re-read on CI#40434241601, the pre-CI commit `3c04042ed` (green 17/17): linux 208 · 1117 · 0 · 12; macos and coverage 189 · 1114 · 0 · 12; windows 189 · 1068 · 0 · 9; the a11y job 14 · 85 · 0 · 0; the stand repeat 204 · 864 · 0 · 21 — every job equal to the base's CI#40389488295 (`19f9ff91ca`), its chunk adding no test — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md»
- line: .andromeda/test-plan-detail/ci-integration.md:65
  old: «`ci-leg.sh coverage` exit 0, its instrumented test run 404 · 0 · 3 (cargo-llvm-cov runs no doctests), before 2026-10-06-telemetry-bootstrap's +9 tests / +1 ignored, not re-run locally since — as measured at escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/gates.md»
  new: «`ci-leg.sh coverage` exit 0 on the dev host, its floor's verdict `coverage floor: 3 of 3 crates at or above their floors` (cargo-llvm-cov runs no doctests; the instrumented run's tally not recorded there — the coverage job's stands in the per-OS tallies above) — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
  old: «read again with the served Cells and load page, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, its `ci-leg.sh coverage` gate)»
  new: «read now under the same floors and `coverage_floor.py`: escher-driver 4930/5031 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %, `coverage floor: 3 of 3 crates at or above their floors` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, its `ci-leg.sh coverage` gate, evidence/counts.md row 15)»
- after: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:30
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · gate-complexity -->»
  add: «»
  add: «- **Complexity ceiling:** the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone with rust-code-analysis-cli 0.0.25, a host tool no lockfile resolves, test functions counted — no CI leg runs it and it is no ceiling on every function: escher's three functions the audit named read under it (escher-driver's `findings` 7, mcp_tests.rs's `the_listing_is_the_twelve_verbs_of_the_table` 3 and `stand_requirement_sweep`'s `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` 3), while 85 functions read over it, among them blitz-dom's `clear_interaction_state_for_removed_node` (17.0) and `get_text_selection_ranges` (32.0), the engine's functions left as they are; the reading's `cognitive.sum` adds nested spaces into their parent, so a closure or a nested `fn` keeps its branches counted in its enclosing function and only splitting a function into module-level sibling items lowers its reading — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md (evidence/complexity.md; research.md §Patterns detected)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan.md:427
  old: «gate-flakiness · gate-mutation -->»
  new: «gate-flakiness · gate-mutation · gate-complexity -->»

## Patterns
[[pattern]]
id = "tp-01"
fixed = 'not re-run locally since'

[[pattern]]
id = "tp-02"
fixed = '4918/5019 = 97.99 %'
