# Apply — a11y-plan · part 4 of 4

## Verdicts
- fanout-a11y-plan--cascade-g5.md 8 · applied · .andromeda/a11y-plan-detail/ci-integration.md:9#1
- fanout-a11y-plan--cascade-g5.md 9 · applied · .andromeda/a11y-plan-detail/ci-integration.md:10#1
- fanout-a11y-plan--cascade-g5.md 10 · applied · .andromeda/a11y-plan-detail/ci-integration.md:11#1, .andromeda/a11y-plan-detail/ci-integration.md:11#2
- fanout-a11y-plan--cascade-g5.md 11 · applied · .andromeda/a11y-plan-detail/ci-integration.md:12+#1

## Edits
- line: .andromeda/a11y-plan-detail/ci-integration.md:9
  old: «the list's fifteen words pinned by»
  new: «the list's seventeen words pinned by»
- line: .andromeda/a11y-plan-detail/ci-integration.md:10
  old: «14 files and 112 tests (`stand_keyboard` 21), over the tier-0 witnesses alone, 9 files and 86 tests (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «14 files and 114 tests (`stand_keyboard` 23), over the tier-0 witnesses alone, 9 files and 88 tests (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- line: .andromeda/a11y-plan-detail/ci-integration.md:11
  old: «`stand_keyboard` 21 and `stand_contrast` 7 (52)»
  new: «`stand_keyboard` 23 and `stand_contrast` 7 (54)»
- line: .andromeda/a11y-plan-detail/ci-integration.md:11
  old: «14 result lines · 112 passed · 0 failed · 0 ignored, locally and on the "Accessibility (a11y) tests" job of CI#41192433140 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «14 result lines · 114 passed · 0 failed · 0 ignored, locally and on the "Accessibility (a11y) tests" job of CI#41235503227 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/a11y-plan-detail/ci-integration.md:12
  anchor: « `accessibility_states` holds a row moving a native control in the tree by a click and a Space, and `inert_subtree` asserts the focus fixup and the tree's window fallback, both read through `&dyn Document`; what an assistive technology reads stays unmeasured (tests/blitz-tests/tests/accessibility_states.rs:386-427; tests/blitz-tests/tests/inert_subtree.rs:229-310 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»
  add: « `select_popup`, `select_popup_painted` and `stand_act_popup` run in the workspace `test` leg and three `stand-repeat` rounds, the first two by its named `--test` list and `stand_act_popup` by its `stand_*` glob; none is among the a11y leg's fourteen files, though `stand_keyboard`, one of them, carries the list's walk there (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`, `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`) (.github/scripts/ci-leg.sh:59-60; tests/blitz-tests/tests/stand_keyboard.rs:1082-1134 — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»

## Patterns
[[pattern]]
id = "a11y4-01"
fixed = 'fifteen words pinned by'

[[pattern]]
id = "a11y4-02"
fixed = '14 files and 112 tests'

[[pattern]]
id = "a11y4-03"
fixed = '`stand_keyboard` 21 and `stand_contrast` 7 (52)'

[[pattern]]
id = "a11y4-04"
fixed = '14 result lines · 112 passed'
