# Apply — test-plan · part 2 of 5

## Verdicts
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:14#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:15#1
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:15#2
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:17#1
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:24#1
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/test-scope-summary.md:24#2
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/test-scope-summary.md:24#3
- fanout-test-plan--cascade-g1.md 11 · applied · .andromeda/test-plan-detail/test-scope-summary.md:51+#1
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan-detail/test-scope-summary.md:51+#2
- fanout-test-plan--cascade-g1.md 14 · applied · .andromeda/test-plan-detail/test-scope-summary.md:94#1
- fanout-test-plan--cascade-g2.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:151#1
- fanout-test-plan--cascade-g2.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:151#2
- fanout-test-plan--cascade-g2.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:154#1
- fanout-test-plan--cascade-g2.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:152#1
- fanout-test-plan--cascade-g2.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:158#1
- fanout-test-plan--cascade-g2.md 6 · applied · .andromeda/test-plan-detail/test-scope-summary.md:192#1
- fanout-test-plan--cascade-g2.md 7 · applied · .andromeda/test-plan-detail/test-scope-summary.md:161#1
- fanout-test-plan--cascade-g2.md 8 · applied · .andromeda/test-plan-detail/test-scope-summary.md:161#2
- fanout-test-plan--cascade-g2.md 9 · applied · .andromeda/test-plan-detail/test-scope-summary.md:188+#1
- fanout-test-plan--cascade-g2.md 10 · applied · .andromeda/test-plan-detail/test-scope-summary.md:206+#1
- fanout-test-plan--cascade-g2.md 11 · applied · .andromeda/test-plan-detail/test-scope-summary.md:206+#2
- fanout-test-plan--cascade-g3.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:228#1, .andromeda/test-plan-detail/test-scope-summary.md:228#2
- fanout-test-plan--cascade-g3.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:237#1
- cascade-compare-test-plan.md 10 · applied · .andromeda/test-plan-detail/test-scope-summary.md:60+#1
- cascade-compare-test-plan.md 14 · applied · .andromeda/test-plan-detail/test-scope-summary.md:239+#1

## Edits
- line: .andromeda/test-plan-detail/test-scope-summary.md:14
  old: «— called by `host_log` and `mcp_log` alone (examples/seven_guis/tests/common/mod.rs:371-544);»
  new: «— called by `host_log` and `mcp_log` alone (examples/seven_guis/tests/common/mod.rs:371-544), and `option_needles()`, the needle list of a select's options read from a snapshot, called by the same two (examples/seven_guis/tests/common/mod.rs:425-453);»
- line: .andromeda/test-plan-detail/test-scope-summary.md:15
  old: «`tests/host_log.rs` (4 tests, unix only, the fourth below;»
  new: «`tests/host_log.rs` (5 tests, unix only, the fourth and the open list's below;»
- line: .andromeda/test-plan-detail/test-scope-summary.md:15
  old: «`tests/cli_commands.rs` (13 tests, unix only:»
  new: «`tests/cli_commands.rs` (15 tests, unix only:»
- line: .andromeda/test-plan-detail/test-scope-summary.md:17
  old: «`tests/mcp_log.rs` (2 tests — 1 until»
  new: «`tests/mcp_log.rs` (3 tests, the open list's below — 1 until»
- line: .andromeda/test-plan-detail/test-scope-summary.md:24
  old: «no popup is painted and Enter and Space open nothing on a single select (`enter_and_space_open_nothing_on_a_single_select`,»
  new: «a closed single select opens a list the engine paints over the page on a click, Enter or Space, read by blitz-tests' `select_popup` (below) and `select_interaction` (`enter_and_space_open_the_list_and_a_released_space_does_nothing_more`,»
- line: .andromeda/test-plan-detail/test-scope-summary.md:24
  old: «`size` is unread, and a group's label is named in the tree and unpainted;»
  new: «a `multiple` select's `size` hints a height of `size × 1.2em` through `size_rows`, capped at 65534, `0` or a bad value hinting nothing, read by `select_popup`'s `size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in` and `node::select_list`'s `size_reads_a_whole_number_above_zero_capped` (tests/blitz-tests/tests/select_popup.rs:995-1032; packages/blitz-dom/src/node/select_list.rs:459-475), while a single select's `size` is unread; a group's label is named in the tree and painted as a header over its options, read by `select_popup`'s `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing` (tests/blitz-tests/tests/select_popup.rs:805-855);»
- line: .andromeda/test-plan-detail/test-scope-summary.md:24
  old: «pinned by `accessibility_label`, while a click on a select's label re-dispatched to the select is asserted by no check;»
  new: «pinned by `accessibility_label`; a click on a single select's label focuses the select and opens nothing, with no re-dispatch, asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing` (tests/blitz-tests/tests/select_popup.rs:428-441), while a click on a `multiple` select's label re-dispatched to it is asserted by no check;»
- after: .andromeda/test-plan-detail/test-scope-summary.md:51
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-dom (select) — the key press `select_popup` skips:** `select_popup` skips the key press in its one row where an ancestor gains `hidden`, because a key press there panics at `position_in_containing_block` for any focused control, list or no list, the control's anonymous block dropped; no check reads that panic (tests/blitz-tests/tests/select_popup.rs:478-520; packages/blitz-dom/src/node/node.rs:1775) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; owner: a CARRY on «Upstream sync ahead of the UI graph»»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/test-scope-summary.md:51
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-dom (select) — the list's unmeasured limits:** the windowed list is read and not run, and what an assistive technology reads is `unrunnable-here`; stated limits: the thumb is not draggable, a highlighted row's contrast is unread by the lint, a larger authored font clips, and the highlight is in no snapshot and no tree — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; owner of the four stated limits: each limit's line in `.andromeda/residuals.md`»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/test-scope-summary.md:60
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **blitz-dom (node) — `node/select_list.rs`:** 6 inline tests of the list's state — a list opens where its rows fit and else on the side with more room; it is at least its select's width, at most the viewport's, and stays inside it; every edge of a placed list is finite; `size` reads a whole number above zero, capped; a scroll stays within the rows and brings a row in by the least movement; a point reads the row it lies on where the rows are scrolled (packages/blitz-dom/src/node/select_list.rs:386-539); `BaseDocument`'s public `open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout` and `close_select_list` are read through the checks `select_popup`, `select_popup_painted` and `stand_act_popup` (below) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:94
  old: «the fifteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth)»
  new: «the sixteen `stand_act_*` integration files (`stand_act_screenshot` the thirteenth, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth, `stand_act_popup` the sixteenth)»
- line: .andromeda/test-plan-detail/test-scope-summary.md:151
  old: «(`STATE_ROWS`, 18 reached states — return-chosen 2 (`flight-type` and `flight-return-date`, each 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — 17 pass and 1 exempt, the lowest pass 5.11)»
  new: «(`STATE_ROWS`, 20 reached states — return-chosen 2 (`flight-type` and `flight-return-date`, each 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 and two more (tests/blitz-tests/tests/stand_contrast.rs:304-316) — 19 pass and 1 exempt as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the lowest pass of the eighteen in those six classes 5.11)»
- line: .andromeda/test-plan-detail/test-scope-summary.md:151
  old: «`stand_keyboard` 21 (run by the `a11y` leg)»
  new: «`stand_keyboard` 23 (run by the `a11y` leg)»
- line: .andromeda/test-plan-detail/test-scope-summary.md:152
  old: «the flight type select moving its choice by its arrows (tests/blitz-tests/tests/stand_keyboard.rs:461-521);»
  new: «the flight type select moving its choice by its arrows, that test's Enter and Space rows opening the list and then choosing, so it reads true and not what its row order suggests (tests/blitz-tests/tests/stand_keyboard.rs:461-521);»
- line: .andromeda/test-plan-detail/test-scope-summary.md:154
  old: «tests/blitz-tests/tests/stand_keyboard.rs:1027-1054);»
  new: «tests/blitz-tests/tests/stand_keyboard.rs:1027-1054), the flight type's walk holding with its list open and after each close (`the_flight_type_walk_holds_with_its_list_open_and_after_each_close`) and a click opening the list, focus staying on the select and the walk going on from it (`a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`) (tests/blitz-tests/tests/stand_keyboard.rs:1082-1111; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134);»
- line: .andromeda/test-plan-detail/test-scope-summary.md:158
  old: «`stand_lint` 8, declaring»
  new: «`stand_lint` 9, declaring»
- line: .andromeda/test-plan-detail/test-scope-summary.md:161
  old: «choosing an enabled option over a 14-row table;»
  new: «choosing an enabled option over a 14-row table, `KEYS`, holding `PageDown` and no Enter, which opens the list;»
- line: .andromeda/test-plan-detail/test-scope-summary.md:161
  old: «a click then a key choosing an option;»
  new: «a click then a key choosing an option, the click opening the select's list and the key moving or choosing in it;»
- after: .andromeda/test-plan-detail/test-scope-summary.md:188
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **tests/blitz-tests — `stand_act_popup`:** 12 tests, the sixteenth `stand_act_*` file, each in both layout modes, driving the select list through a held driver session: a click opening it and a click on a row choosing, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option and a button under the list refused `covered`, a row past the scrollport `off-screen` until `scroll` brings it in, `type` leaving no list open, a shot of a row cropping to it and leaving the list open, the lint, the open screen under its budget with every actionable element keyed, two clicks on one point opening then closing, and a read after a step with no `advance`; picked up by `run stand` and the `stand-repeat` leg by its prefix, not among the `a11y` leg's fourteen (tests/blitz-tests/tests/stand_act_popup.rs:58-423) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:192
  old: «(tests/blitz-tests/tests/stand_lint.rs:282-332) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  new: «(tests/blitz-tests/tests/stand_lint.rs:282-332), and an open list's rows read no overlap on the stand, in both layout modes (`an_open_lists_rows_read_no_overlap_on_the_stand`, tests/blitz-tests/tests/stand_lint.rs:493-519), `stand_act_popup`'s `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads` reading a row the same through a held session, the planted pair still reading (tests/blitz-tests/tests/stand_act_popup.rs:318-350) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the open list's at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:206
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **tests/blitz-tests — `select_popup`:** 25 tests, its own target with no `stand_` prefix, each in both layout modes, reading the select list's engine behaviour: opening, key and pointer moves, choosing, closing, inert, disabled, hidden and removed selects, the tree's `expanded`, clips, groups, the twenty-row cap, hostile markup, `size` on a `multiple` select, redraw requests, and a label's click focusing a single select and opening nothing; run by the workspace `test` leg, `run all`, by name and as a named `--test` of the `stand-repeat` leg, not by `run stand` or the `a11y` leg (tests/blitz-tests/tests/select_popup.rs:202-1186; .github/scripts/ci-leg.sh:59-60) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/test-scope-summary.md:206
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **tests/blitz-tests — `select_popup_painted`:** 6 tests, its own target with no `stand_` prefix, each in both layout modes, reading the select list on frames: the highlight and a disabled row against their constants, the plain contrast pair, painting past clips and `opacity`, a closed frame byte-equal after a close with no choice, a header darker than a row by weight, and a thumb taking no hit; run as `select_popup` is, a named `--test` of the `stand-repeat` leg; blitz-paint's `render/select_list.rs` holds no unit test of its own (tests/blitz-tests/tests/select_popup_painted.rs:119-351; .github/scripts/ci-leg.sh:59-60) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-scope-summary.md:228
  old: «and five more of the targets the leg names»
  new: «and seven more of the targets the leg names, `select_popup` and `select_popup_painted` among them»
- line: .andromeda/test-plan-detail/test-scope-summary.md:228
  old: «escher-0.3.0/chunks/2026-10-26-codebase-health/report.md (evidence/row-controls.md)»
  new: «escher-0.3.0/chunks/2026-10-26-codebase-health/report.md (evidence/row-controls.md), the count at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- line: .andromeda/test-plan-detail/test-scope-summary.md:237
  old: «an entry inside a closed select, `Entry.in_closed_select` read through `closed_select_of`, the element and never the role, the snapshot still giving each such option its select's client rect, and it lints a `multiple` select's rows as usual; `stand_lint` pins the stand at `[0, 0, 0, 0]` at boot (§1 above)»
  new: «an entry in a select's box, `Entry.in_select_box`: true inside a closed select, read through `closed_select_of`, the element and never the role, the snapshot still giving each such option its select's client rect, and true for an open list's row or group, an open row's `clip_region` coming from `visible_region` alone with no `contain: paint` cut; it lints a `multiple` select's rows as usual; `stand_lint` pins the stand at `[0, 0, 0, 0]` at boot and, with `stand_act_popup`, reads an open list's rows no overlap (§1 above) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:239
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · scope-entities -->»
  add: «»
  add: «- **The open list through the process:** seven_guis' `cli_commands` `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in `cc-popup`, the hosted shot equal to the in-process one (examples/seven_guis/tests/cli_commands.rs:419-464); `host_log` `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, in `hl-popup`, `#[cfg(unix)]` as every host-driving test there (examples/seven_guis/tests/host_log.rs:282-393); `mcp_log` `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing and holds no state directory (examples/seven_guis/tests/mcp_log.rs:68-113) — the two log checks reading 0 needles at `trace` — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "tp2-01"
fixed = 'host_log.rs` (4 tests, unix only'

[[pattern]]
id = "tp2-02"
fixed = 'cli_commands.rs` (13 tests, unix only'

[[pattern]]
id = "tp2-03"
fixed = 'mcp_log.rs` (2 tests'

[[pattern]]
id = "tp2-04"
fixed = 'no popup is painted and Enter and Space open nothing'

[[pattern]]
id = "tp2-05"
fixed = 'label is named in the tree and unpainted'

[[pattern]]
id = "tp2-06"
fixed = 'label re-dispatched to the select is asserted by no check'

[[pattern]]
id = "tp2-07"
fixed = 'the fifteen `stand_act_*` integration files'

[[pattern]]
id = "tp2-08"
fixed = '`STATE_ROWS`, 18 reached states'

[[pattern]]
id = "tp2-09"
fixed = '`stand_keyboard` 21 (run by'

[[pattern]]
id = "tp2-10"
fixed = '`stand_lint` 8, declaring'

[[pattern]]
id = "tp2-11"
fixed = 'five more of the targets the leg names'

[[pattern]]
id = "tp2-12"
fixed = '`Entry.in_closed_select` read through'
