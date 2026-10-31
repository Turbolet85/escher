# Apply — test-plan · part 1 of 5

## Verdicts
- fanout-test-plan--cascade-g1.md 8 · applied · .andromeda/test-plan.md:19#1, .andromeda/test-plan.md:19#2
- fanout-test-plan--cascade-g1.md 9 · applied · .andromeda/test-plan.md:25#1
- fanout-test-plan--cascade-g4.md 12 · applied · .andromeda/test-plan.md:227#1
- fanout-test-plan--cascade-g5.md 18 · applied · .andromeda/test-plan.md:377#1
- fanout-test-plan--cascade-g5.md 19 · applied · .andromeda/test-plan.md:413#1
- fanout-test-plan.md 2 · refused · the key file, main writes it
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:135#1
- fanout-test-plan.md 4 · applied · .andromeda/test-plan.md:146#1
- fanout-test-plan.md 5 · applied · .andromeda/test-plan.md:149#1
- fanout-test-plan.md 6 · applied · .andromeda/test-plan.md:151#1
- fanout-test-plan.md 7 · applied · .andromeda/test-plan.md:176#1, .andromeda/test-plan.md:176+#1
- fanout-test-plan.md 8 · applied · .andromeda/test-plan.md:190#1, .andromeda/test-plan.md:190#2

## Edits
- line: .andromeda/test-plan.md:19
  old: «with `optgroup`, and hit testing»
  new: «with `optgroup` and a single select's painted list, and hit testing»
- line: .andromeda/test-plan.md:19
  old: «the select's by `select_interaction` and `stand_act_select`»
  new: «the select's by `select_interaction` and `stand_act_select`, and its list's by `select_popup`, `select_popup_painted` and `stand_act_popup`»
- line: .andromeda/test-plan.md:25
  old: «over stated tables, the range value rules in `node/range.rs` and the select's rules in `node/select.rs`»
  new: «over stated tables, the range value rules in `node/range.rs` and the select's rules in `node/select.rs`, and, in `node/select_list.rs`, where a list opens, its bounds, `size` read as a capped whole number and a point reading its row under scroll»
- line: .andromeda/test-plan.md:227
  old: «pin over stated tables the range value rules in `node/range.rs` and the select's rules in `node/select.rs`»
  new: «pin over stated tables the range value rules in `node/range.rs` and the select's rules in `node/select.rs`, and pin the list's placement, scroll and row hit in `node/select_list.rs`»
- line: .andromeda/test-plan.md:377
  old: «fifteen named blitz-tests targets outside the `stand_` prefix»
  new: «seventeen named blitz-tests targets outside the `stand_` prefix»
- line: .andromeda/test-plan.md:413
  old: «the fifteen named blitz-tests targets»
  new: «the seventeen named blitz-tests targets»
- line: .andromeda/test-plan.md:135
  old: «every scrolling box that holds the node, then the viewport —»
  new: «every scrolling box that holds the node, then the viewport, a row of an open select list moving the list's `scroll_y` alone —»
- line: .andromeda/test-plan.md:146
  old: «six of the fifteen driver-action checks»
  new: «six of the sixteen driver-action checks»
- line: .andromeda/test-plan.md:149
  old: «the fifteen driver-action checks `stand_act_*`»
  new: «the sixteen driver-action checks `stand_act_*`»
- line: .andromeda/test-plan.md:151
  old: «28 checks declare `mod fixtures;` — 22 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`), `font_face_sniffed_source`, `select_interaction`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md»
  new: «31 checks declare `mod fixtures;` — 23 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`; `stand_act_popup`), `font_face_sniffed_source`, `select_interaction`, `select_popup`, `select_popup_painted`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- line: .andromeda/test-plan.md:176
  old: «picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  new: «picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md;»
- after: .andromeda/test-plan.md:176
  anchor: « — as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md; re-counted with the paged snapshot, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 195 · failed 0 · ignored 7 over 48 files — +3 passed (`stand_snapshot_parts`, a new file its `stand_` prefix picks up); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md; re-counted with upstream's `hit_test_dropped_nodes`, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` unmoved at passed 195 · failed 0 · ignored 7 over 48 files, and `run all` reads passed 617 · failed 0 · ignored 12 — `hit_test_dropped_nodes` 1, a new file with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md; re-counted with `stand-repeat`'s ten named targets, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 197 · failed 0 · ignored 7 over 48 files — +2 passed (`stand_requirement_sweep` +1 and `stand_lint` +1, in files its `stand_` prefix already picks up); the ten carry no `stand_` prefix, so the agent harness reaches them through `run all` and `run {name}` alone; `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md; re-counted with the served Cells and load page, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 205 · failed 0 · ignored 7 over 50 files — +8 passed (`stand_cells_served` 4 and `stand_load_page` 2, two new files picked up by their `stand_` prefix with no script change, `stand_keyboard` +1 and `stand_snapshot` +1); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md; re-counted with upstream's `pseudo_element_flow_classification`, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` unmoved at passed 205 · failed 0 · ignored 7 over 50 files, and `run all` reads passed 633 · failed 0 · ignored 12 — `pseudo_element_flow_classification` 3, a new file with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  add: « re-counted with the select list's three new targets, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files — +15 passed (`stand_act_popup` 12, a new file its `stand_` prefix picks up, `stand_keyboard` +2 and `stand_lint` +1) — and `run all` reads passed 727 · failed 0 · ignored 12, `select_popup` and `select_popup_painted` carrying no `stand_` prefix and picked up by it with no script change — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
- line: .andromeda/test-plan.md:190
  old: «the 22 `stand_*` readers of the shared module, `run {file}` the six non-stand readers (`run font_face_sniffed_source`, which holds a load, `run select_interaction`, `run inert_subtree`»
  new: «the 23 `stand_*` readers of the shared module, `run {file}` the eight non-stand readers (`run font_face_sniffed_source`, which holds a load, `run select_interaction`, `run select_popup`, `run select_popup_painted`, `run inert_subtree`»
- line: .andromeda/test-plan.md:190
  old: «`run stand` 222 · 0 · 7 over 50 files, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
  new: «`run stand` 237 · 0 · 7 over 51 files, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = 'fifteen named blitz-tests targets outside the `stand_` prefix'

[[pattern]]
id = "tp1-02"
fixed = 'the fifteen named blitz-tests targets'

[[pattern]]
id = "tp1-03"
fixed = 'six of the fifteen driver-action checks'

[[pattern]]
id = "tp1-04"
fixed = 'the fifteen driver-action checks `stand_act_*`'

[[pattern]]
id = "tp1-05"
fixed = '28 checks declare `mod fixtures;`'

[[pattern]]
id = "tp1-06"
fixed = 'the 22 `stand_*` readers of the shared module'

[[pattern]]
id = "tp1-07"
fixed = '`run stand` 222 · 0 · 7 over 50 files'
