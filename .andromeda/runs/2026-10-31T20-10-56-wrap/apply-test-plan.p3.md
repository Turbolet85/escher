# Apply — test-plan · part 3 of 5

## Verdicts
- fanout-test-plan--cascade-g4.md 1 · applied · .andromeda/test-plan-detail/test-strategy.md:28#1
- fanout-test-plan--cascade-g4.md 2 · applied · .andromeda/test-plan-detail/test-strategy.md:52#1
- fanout-test-plan--cascade-g4.md 3 · applied · .andromeda/test-plan-detail/test-strategy.md:53#1
- fanout-test-plan--cascade-g4.md 4 · applied · .andromeda/test-plan-detail/test-strategy.md:53#2, .andromeda/test-plan-detail/test-strategy.md:53#3
- fanout-test-plan--cascade-g4.md 5 · applied · .andromeda/test-plan-detail/test-strategy.md:54#1
- cascade-compare-test-plan.md 15 · applied · .andromeda/test-plan-detail/test-strategy.md:37+#1
- fanout-test-plan--cascade-g4.md 6 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:26#1
- fanout-test-plan--cascade-g4.md 7 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:26#2
- fanout-test-plan--cascade-g4.md 8 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:30+#1
- fanout-test-plan--cascade-g4.md 9 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:39#1
- fanout-test-plan--cascade-g4.md 10 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:46#1
- fanout-test-plan--cascade-g4.md 11 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:85+#1
- fanout-test-plan--cascade-g4.md 13 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:30#1
- fanout-test-plan--cascade-g4.md 14 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:31#1
- fanout-test-plan--cascade-g4.md 15 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:47+#1
- fanout-test-plan--cascade-g4.md 16 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:35#1
- fanout-test-plan--cascade-g4.md 17 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:41+#1
- cascade-compare-test-plan.md 3 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:41+#2
- cascade-compare-test-plan.md 5 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:47+#2
- cascade-compare-test-plan.md 6 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:47+#3

## Edits
- line: .andromeda/test-plan-detail/test-strategy.md:28
  old: «the `select` model's limits in its two bullets (docs/capabilities.md:205-207; docs/capabilities.md:223-226), its keys, rows and groups pinned by `select_interaction` and `stand_act_select`;»
  new: «the `select` model's limits in its four bullets (docs/capabilities.md:205-213; docs/capabilities.md:223-226), its keys, rows and groups pinned by `select_interaction` and `stand_act_select`, the list a click, Enter and Space open read by `select_popup`, `select_popup_painted` and `stand_act_popup`;»
- line: .andromeda/test-plan-detail/test-strategy.md:52
  old: «read through `mod session_common;` by 31 checks (`stand_cells_served.rs` and `stand_load_page.rs` the latest; 29 with»
  new: «read through `mod session_common;` by 32 checks, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md (`stand_act_popup.rs`, `stand_cells_served.rs` and `stand_load_page.rs` the latest; 29 with»
- line: .andromeda/test-plan-detail/test-strategy.md:53
  old: «the fifteen `stand_act_*.rs`»
  new: «the sixteen `stand_act_*.rs`»
- line: .andromeda/test-plan-detail/test-strategy.md:53
  old: «read through `mod fixtures;` by 28 checks, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md — 22 `stand_*.rs` (`stand_snapshot_parts`,»
  new: «read through `mod fixtures;` by 31 checks, counted anchored at line start, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md — 23 `stand_*.rs` (`stand_act_popup`, `stand_snapshot_parts`,»
- line: .andromeda/test-plan-detail/test-strategy.md:53
  old: «`accessibility_states.rs`, `checkable_change_event.rs` and `text_range_no_change_event.rs` (tests/blitz-tests/tests/fixtures/mod.rs:17-130)»
  new: «`accessibility_states.rs`, `checkable_change_event.rs`, `text_range_no_change_event.rs`, `select_popup.rs` and `select_popup_painted.rs` (tests/blitz-tests/tests/fixtures/mod.rs:17-130)»
- line: .andromeda/test-plan-detail/test-strategy.md:54
  old: «153 `.rs` files, 152 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the latest `visible_region_transformed.rs`,»
  new: «156 `.rs` files, 155 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the latest `select_popup.rs`, `select_popup_painted.rs` and `stand_act_popup.rs`, then `visible_region_transformed.rs`,»
- after: .andromeda/test-plan-detail/test-strategy.md:37
  anchor: «»
  next: «<!-- detail: procedure · strategy -->»
  add: «<!-- detail: procedure · strategy -->»
  add: «»
  add: «- **Painted select list:** `select_popup_painted` reads an open list on a frame's `Raster` against the engine's constants, in both layout modes (§5); no local gate builds `blitz-paint/font-embolden`, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reads on linux the header and row numbers `Test (macos)` reads through `apple-font-embolden` (tests/blitz-tests/tests/select_popup_painted.rs:119-159; tests/blitz-tests/tests/select_popup_painted.rs:282-297; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:26
  old: «seven_guis' `cli_commands` (14)»
  new: «seven_guis' `cli_commands` (15)»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:26
  old: «while the fifteen `stand_act_*` files drive it»
  new: «while the sixteen `stand_act_*` files drive it»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:30
  anchor: «»
  next: «<!-- detail: procedure · unit -->»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **escher-driver (the list's help and `covered`):** `schema_tests`, `refusal` and `json` pin the help of `click`, `type` and `press` and `covered`'s clause, an option of a closed select lying under it, each fixed `'static` text, with `VERBS` at 12, causes at 8 and `describe`'s count row `[12, 13, 12, 3, 8, 4, 15]` (packages/escher-driver/src/schema_tests.rs:98-110; packages/escher-driver/src/schema_tests.rs:119-123; packages/escher-driver/src/schema_tests.rs:136; packages/escher-driver/src/refusal.rs:261-266; packages/escher-driver/src/json.rs:868-869; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:39
  old: «schema_tests.rs pins `type`'s help on a `multiple` select inside a standing test»
  new: «schema_tests.rs pins `type`'s help on any select, which `type` focuses with no click, inside a standing test»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:46
  old: «its case naming the leg's fifteen named targets»
  new: «its case naming the leg's seventeen named targets»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:85
  anchor: «»
  next: «<!-- detail: procedure · unit -->»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **blitz-dom (node, a select's list):** `node/select_list.rs` holds six inline unit tests: a list opening where its rows fit and else on the side with more room, its width at least its select's and at most the viewport's, every edge of a placed list finite, `size` read as a whole number above zero and capped, a scroll staying within the rows and bringing a row in by the least movement, and a point reading the row it lies on where the rows are scrolled (packages/blitz-dom/src/node/select_list.rs:386-539; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:30
  old: «each stated in `covered`'s fixed meaning:»
  new: «each stated in `covered`'s fixed meaning, which also says an option of a closed select lies under it and to click the select first, then the option:»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:31
  old: «(the fifteen `stand_act_*` files, §1)»
  new: «(the sixteen `stand_act_*` files, §1)»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:47
  anchor: «»
  next: «<!-- detail: absence · integration -->»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Session ↔ held instance, a select's list:** `stand_act_popup` (12) drives the list through a held session, in both layout modes — a `click` opening it and a `click` on a row choosing, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, two `click`s on one point opening then closing, each focusing, a closed option and a button under the list refused `covered`, a row off screen until `scroll`, `type` leaving no list open, a row's shot leaving the list open, the lint reading no `overlap` on a row, the open screen under its budget, and a screen read with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:35
  old: «`host_log` holds 4 tests and `mcp_log` 2»
  new: «`host_log` holds 5 tests and `mcp_log` 3»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:41
  anchor: «»
  next: «<!-- detail: procedure · integration -->»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Session host ↔ what it writes, a select's list:** seven_guis' `host_log` holds a `#[cfg(unix)]` test, `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, that opens and chooses in a list through the host at `trace` in the state directory `hl-popup` under `CARGO_TARGET_TMPDIR` with no option logged, and `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` does so through the role and records nothing, no `ml-popup` directory existing, since recording there would need a `cfg!(unix)` site (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:41
  anchor: «»
  next: «<!-- detail: procedure · integration -->»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Session host ↔ session socket, a select's list:** `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in the state directory `cc-popup` under `CARGO_TARGET_TMPDIR`, opens the list through a held host, reads its shot equal to the in-process one and chooses a row by a click (examples/seven_guis/tests/cli_commands.rs:419-464; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:47
  anchor: «»
  next: «<!-- detail: absence · integration -->»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Harness ↔ event pipeline, a single select's list:** `select_popup` (25) reads the engine's list through harness input, in both layout modes — a click, Enter and Space opening it on its chosen option and firing nothing; the keys moving the highlight over enabled rows and choosing nothing, the pointer moving it to an enabled row; Enter, Space, Tab and a click choosing a row, one `input` then one `change`, Tab before focus moves either way, the chosen option firing nothing and a disabled row changing nothing; Escape, a second click, a click outside and the page scrolling closing with no event; a label's click focusing the select and opening nothing; a disabled, inert, hidden, singular or empty select opening nothing, and one made inert, disabled, hidden or removed with its list open closing; an option dropped with the list open no row; the tree reading the list open on the select, its focus staying there, an author's `aria-expanded` and `open` steering nothing; a box that clips the select clipping no row; a pointer at a row's centre landing on it, scrolled or not; groups read as headers, a header choosing nothing; a long list showing twenty rows and scrolling by key and wheel; hostile markup (`size`, 2,000 options, a 5,500-character label) keeping every box finite; `size` on a `multiple` select making it that many rows tall; each open, move, scroll and close asking a redraw. One row skips its key press: a key pressed after an ancestor gained `hidden` panics at `packages/blitz-dom/src/node/node.rs:1775`, for any control, with no list opened, the test's comment saying why (tests/blitz-tests/tests/select_popup.rs:202-1186; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:47
  anchor: «»
  next: «<!-- detail: absence · integration -->»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Paint ↔ harness, a single select's list:** `select_popup_painted` (6), in both layout modes, reads the list on frames — the highlight and a disabled row against `SelectListView`'s constants, the plain pair measured at the default sheet and at the stand, a list painted whole past an `opacity` ancestor and a box that clips its select, a closed select's frame byte-equal before opening and after a close with no choice, a group's header darker than a row by weight, and a scrolled list's thumb painted and taking no hit; the header's weight under `blitz-paint/font-embolden` has the local witness §2 Painted select list names (tests/blitz-tests/tests/select_popup_painted.rs:119-351; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
  add: «»
  add: «<!-- /detail -->»
  add: «»

## Patterns
[[pattern]]
id = "tp3-01"
fixed = "limits in its two bullets"

[[pattern]]
id = "tp3-02"
fixed = "`mod session_common;` by 31 checks"

[[pattern]]
id = "tp3-03"
fixed = "the fifteen `stand_act_*.rs`"

[[pattern]]
id = "tp3-04"
fixed = "`mod fixtures;` by 28 checks"

[[pattern]]
id = "tp3-05"
fixed = "153 `.rs` files, 152 test targets"

[[pattern]]
id = "tp3-06"
fixed = "`cli_commands` (14)"

[[pattern]]
id = "tp3-07"
fixed = "the fifteen `stand_act_*` files drive"

[[pattern]]
id = "tp3-08"
fixed = "pins `type`'s help on a `multiple` select"

[[pattern]]
id = "tp3-09"
fixed = "the leg's fifteen named targets"

[[pattern]]
id = "tp3-10"
fixed = "the fifteen `stand_act_*` files, §1"

[[pattern]]
id = "tp3-11"
fixed = "`host_log` holds 4 tests"
