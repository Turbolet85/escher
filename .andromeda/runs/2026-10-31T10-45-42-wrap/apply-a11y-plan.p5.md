# Apply — a11y-plan · part 5 of 5

## Verdicts
- fanout-a11y-plan--cascade-g5.md 8 · applied · .andromeda/a11y-plan-detail/ci-integration.md:9#1
- fanout-a11y-plan--cascade-g5.md 9 · applied · .andromeda/a11y-plan-detail/ci-integration.md:10#1
- fanout-a11y-plan--cascade-g5.md 10 · applied · .andromeda/a11y-plan-detail/ci-integration.md:11#1
- fanout-a11y-plan--cascade-g5.md 11 · applied · .andromeda/a11y-plan-detail/ci-integration.md:11+#1

## Edits
- line: .andromeda/a11y-plan-detail/ci-integration.md:9
  old: «the list's ten words pinned by»
  new: «the list's fifteen words pinned by»
- line: .andromeda/a11y-plan-detail/ci-integration.md:10
  old: «14 files and 85 tests (`stand_keyboard` 15), over the tier-0 witnesses alone, 9 files and 59 tests,»
  new: «14 files and 112 tests (`stand_keyboard` 21), over the tier-0 witnesses alone, 9 files and 86 tests (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md),»
- line: .andromeda/a11y-plan-detail/ci-integration.md:11
  old: «Its log reads the six unmoved (43) and accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5 (41): 14 result lines · 84 passed · 0 failed · 0 ignored, locally and on the "Accessibility (a11y) tests" job of CI#40134371519 (6 · 43 on CI#40125511019), every other job's tally unmoved;»
  new: «Its log reads the six with `stand_keyboard` 21 and `stand_contrast` 7 (52) and accessibility_states 6 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 6 · select_interaction 18 · stand_act_select 10 (60): 14 result lines · 112 passed · 0 failed · 0 ignored, locally and on the "Accessibility (a11y) tests" job of CI#41192433140 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md);»
- after: .andromeda/a11y-plan-detail/ci-integration.md:11
  anchor: « Its log reads the six unmoved (43) and accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5 (41): 14 result lines · 84 passed · 0 failed · 0 ignored, locally and on the "Accessibility (a11y) tests" job of CI#40134371519 (6 · 43 on CI#40125511019), every other job's tally unmoved; the job keeps its slow tier, its `needs`, its restore-only `workspace-test` cache, its failure-only log upload and no required status check. Its pin, `LegScriptTest`'s a11y tuple, asserts each of the fourteen target words, beside `--workspace` and `--locked`, by one `assertIn` over the shimmed log, in no order (.github/scripts/ci-leg.sh:39; .github/scripts/ci-leg.sh:54-59; .github/scripts/test_ci_workflows.py:298-302 — as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md and its evidence/counts.md, evidence/ci.md).»
  add: « `accessibility_states` holds a row moving a native control in the tree by a click and a Space, and `inert_subtree` asserts the focus fixup and the tree's window fallback, both read through `&dyn Document`; what an assistive technology reads stays unmeasured (tests/blitz-tests/tests/accessibility_states.rs:386-427; tests/blitz-tests/tests/inert_subtree.rs:229-310 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»

## Patterns
[[pattern]]
id = "a11y5-01"
fixed = 'ten words pinned by'

[[pattern]]
id = "a11y5-02"
fixed = '14 files and 85 tests'

[[pattern]]
id = "a11y5-03"
fixed = 'log reads the six unmoved'
