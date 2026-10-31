# Apply — test-plan · part 3 of 4

## Verdicts
- fanout-test-plan--cascade-g3.md 1 · applied · .andromeda/test-plan-detail/test-strategy.md:28#1
- fanout-test-plan--cascade-g3.md 2 · applied · .andromeda/test-plan-detail/test-strategy.md:53#1, .andromeda/test-plan-detail/test-strategy.md:53#2
- fanout-test-plan--cascade-g3.md 3 · applied · .andromeda/test-plan-detail/test-strategy.md:54#1
- fanout-test-plan--cascade-g3.md 4 · applied · .andromeda/test-plan-detail/test-strategy.md:60+#1
- fanout-test-plan--cascade-g3.md 5 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:10+#1
- fanout-test-plan--cascade-g3.md 6 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:34#1, .andromeda/test-plan-detail/unit-test-strategy.md:34#2, .andromeda/test-plan-detail/unit-test-strategy.md:34+#1
- fanout-test-plan--cascade-g3.md 7 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:63#1, .andromeda/test-plan-detail/unit-test-strategy.md:65+#1
- fanout-test-plan--cascade-new-unit-test-strategy.md 1 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:65+#2
- cascade-compare-test-plan.md 5 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:29+#1
- fanout-test-plan--cascade-g3.md 8 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:8#1
- fanout-test-plan--cascade-g3.md 9 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:16+#1
- fanout-test-plan--cascade-g3.md 10 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:24#1
- fanout-test-plan--cascade-g3.md 12 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:34+#1
- fanout-test-plan--cascade-g3.md 13 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:14+#1
- fanout-test-plan--cascade-g3.md 14 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:24#1
- cascade-compare-test-plan.md 3 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:42#1
- fanout-test-plan--cascade-g3.md 15 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:11+#1

## Edits
- line: .andromeda/test-plan-detail/test-strategy.md:28
  old: «or as asserted by no check (the `select` model's limits)»
  new: «or as asserted by no check, the `select` model's limits in its two bullets (docs/capabilities.md:196-198; docs/capabilities.md:208-211), its keys, rows and groups pinned by `select_interaction` and `stand_act_select`»
- line: .andromeda/test-plan-detail/test-strategy.md:53
  old: «read through `mod fixtures;` by 24 checks — 22 `stand_*.rs` (»
  new: «read through `mod fixtures;` by 28 checks, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md — 22 `stand_*.rs` (»
- line: .andromeda/test-plan-detail/test-strategy.md:53
  old: «`font_face_sniffed_source.rs` and `select_interaction.rs` (tests/blitz-tests/tests/fixtures/mod.rs:17-130)»
  new: «`font_face_sniffed_source.rs`, `select_interaction.rs`, `inert_subtree.rs`, `accessibility_states.rs`, `checkable_change_event.rs` and `text_range_no_change_event.rs` (tests/blitz-tests/tests/fixtures/mod.rs:17-130)»
- line: .andromeda/test-plan-detail/test-strategy.md:54
  old: «148 `.rs` files, 147 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md (145 and 144»
  new: «153 `.rs` files, 152 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the latest `visible_region_transformed.rs`, `hit_non_invertible_transform.rs`, `open_state.rs`, `checkable_change_event.rs` and `text_range_no_change_event.rs`, each its own auto-discovered target with no manifest line (145 and 144»
- after: .andromeda/test-plan-detail/test-strategy.md:60
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · naming -->»
  add: «»
  add: «- **Shared tab tables:** `tests/common/mod.rs` states the load page's and Cells' Tab sequences, `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` (tests/blitz-tests/tests/common/mod.rs:93-98)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:10
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **blitz-shell:** `packages/blitz-shell/src/event.rs` holds the crate's one unit test in an inline `#[cfg(test)]` module, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read (packages/blitz-shell/src/event.rs:128-172)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:34
  old: «(the three rounds; the stop at the first red)»
  new: «(the three rounds, its case naming the leg's fifteen named targets; the stop at the first red)»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:34
  old: «(.github/scripts/test_ci_workflows.py:326-376) — the `a11y` row asserting each of the leg's fourteen target words, beside `--workspace` and `--locked`, by one `assertIn` over the shimmed log, in no order — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` (the last three added at 2026-10-14-stand-a11y-assertions), then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` (.github/scripts/test_ci_workflows.py:298-302); `InstallScriptTest` runs the CI install script `.github/scripts/apt-install.sh` in a session of its own, against a stand-in `sudo`, `apt-get` and, since 2026-10-14-quality-gates, `dpkg` first on PATH with the real `timeout`, and kills that process group when the test's own wait bound fires — eight tests (five until that chunk): a healthy install runs one update and one install, and no repair, a failed attempt is tried again from `update`, every attempt failing exits 1 after three attempts and never 124 or 137, a stalled call ends at its bound and is tried again, an install failed by its bound is repaired before the next attempt, a failed repair fails its attempt in phase `repair`, and a bad call»
  new: «(.github/scripts/test_ci_workflows.py:326-376)»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:34
  anchor: « — a failing leg exits non-zero and writes its log, the log is truncated at the leg's start, an unknown leg exits 2, the `doc` leg documents the whole workspace (`--workspace --no-deps --locked`), and the `audit`, `a11y` and `coverage` legs dispatch to cargo and write their log — since 2026-10-14-quality-gates the `audit` row asserting `--workspace`, `--all-features`, `advisories` and `licenses`, the `coverage` row two cases over a seeded lcov (the crates at their floors exit 0 with `3 of 3`, escher-driver one line under exits 1 with `1 of 3 … under`), a `clippy` case (the workspace line, then the blitz-dom line) and two `stand-repeat` cases (the three rounds; the stop at the first red) (.github/scripts/test_ci_workflows.py:326-376) — the `a11y` row asserting each of the leg's fourteen target words, beside `--workspace` and `--locked`, by one `assertIn` over the shimmed log, in no order — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` (the last three added at 2026-10-14-stand-a11y-assertions), then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` (.github/scripts/test_ci_workflows.py:298-302); `InstallScriptTest` runs the CI install script `.github/scripts/apt-install.sh` in a session of its own, against a stand-in `sudo`, `apt-get` and, since 2026-10-14-quality-gates, `dpkg` first on PATH with the real `timeout`, and kills that process group when the test's own wait bound fires — eight tests (five until that chunk): a healthy install runs one update and one install, and no repair, a failed attempt is tried again from `update`, every attempt failing exits 1 after three attempts and never 124 or 137, a stalled call ends at its bound and is tried again, an install failed by its bound is repaired before the next attempt, a failed repair fails its attempt in phase `repair`, and a bad call»
  add: « — the `a11y` row asserting each of the leg's fourteen target words, beside `--workspace` and `--locked`, by one `assertIn` over the shimmed log, in no order — `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`, then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` (.github/scripts/test_ci_workflows.py:298-302); `InstallScriptTest` runs the CI install script `.github/scripts/apt-install.sh` in a session of its own, against a stand-in `sudo`, `apt-get` and `dpkg` first on PATH with the real `timeout`, and kills that process group when the test's own wait bound fires — eight tests: a healthy install runs one update and one install, and no repair, a failed attempt is tried again from `update`, every attempt failing exits 1 after three attempts and never 124 or 137, a stalled call ends at its bound and is tried again, an install failed by its bound is repaired before the next attempt, a failed repair fails its attempt in phase `repair`, and a bad call»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:63
  old: «four `node/select.rs` tests pin the select's rules»
  new: «eight `node/select.rs` tests pin the select's rules»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:65
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **blitz-dom (node, a select's steps, spans and groups):** `node/select.rs`'s tests also pin a step moving from the row it is given, a span listing the enabled rows either way round, and the list of options running through a group in tree order (packages/blitz-dom/src/node/select.rs:438-457; packages/blitz-dom/src/node/select.rs:459-477; packages/blitz-dom/src/node/select.rs:560-589)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:65
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **blitz-dom (form):** `form.rs`'s test module pins a select's submitted entries by two tests — `a_multiple_select_submits_its_set_less_its_disabled_rows`, a `multiple` set less its disabled rows, and `a_single_select_submits_its_choice_through_a_group`, a single choice through a group — and these use neither `.unwrap()` nor `.expect(` (packages/blitz-dom/src/form.rs:558-592; packages/blitz-dom/src/form.rs:594-617)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:29
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · unit -->»
  add: «»
  add: «- **escher-driver (`type`'s help):** schema_tests.rs pins `type`'s help on a `multiple` select inside a standing test, with no test of its own (packages/escher-driver/src/schema_tests.rs:99-100; packages/escher-driver/src/schema.rs:239-240)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:8
  old: «is an exception to the rule that pipeline tests run `for incremental in [false, true]`: the file asserts neither mode pair, and `incremental_oracle`, green beside it, is the layout-identity witness (§2 Differential oracle) (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:11-22; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:39-107) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  new: «runs `for incremental in [false, true]` as the pipeline tests do, and `incremental_oracle`, green beside it, is the layout-identity witness (§2 Differential oracle) (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:44-54; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:61-71; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:88-106) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:16
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Script ↔ DOM, a radio's `change`:** dom.rs holds `radio_click_fires_change_only_where_it_was_not_checked`, a first click hearing `input,change` and a second adding `input` alone; a page's script hears one `change`, from the engine, `runtime/events.rs` synthesizing none (packages/blitz-vibey-script/tests/dom.rs:372-400)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:24
  old: «so an `advance` past the fade comes before the click (`stand_cells_served`; tests/blitz-tests/tests/stand_cells_served.rs:216-232);»
  new: «so an `advance` past the fade comes before the click (`stand_cells_served`; tests/blitz-tests/tests/stand_cells_served.rs:216-232); the keyboard walks to a cell and commits a value read in the first part, `press` walking and opening the editor, `type` supplying the characters and `press` committing (`stand_cells_served`; tests/blitz-tests/tests/stand_cells_served.rs:179-214), and a click inside a cell's own open editor returns early, so the text typed there stays (examples/seven_guis/src/tasks/cells.rs:425-433);»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:34
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Session ↔ held instance, a `multiple` select:** `stand_act_select` drives a `multiple` select through the driver, in both layout modes — its name, role and rows read each with its own bounds, an accepted row click choosing the row, a Shift press spanning rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered` (tests/blitz-tests/tests/stand_act_select.rs:262-457)»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:14
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · e2e -->»
  add: «»
  add: «- **Cells' handle calls, headless:** Cells calls its `MountedData` handles synchronously, measured headlessly, their windowed path read from the same lines and not run; a `MountedData` call in a task spawned from a key handler panics with `RefCell already borrowed` at `events.rs:162` (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md)»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:24
  old: «47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`,»
  new: «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/counts.md,»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:42
  old: «read by `stand_load_page` and the stand unit test `the_load_page_click_leaves_a_load_in_flight` | (examples/seven_guis/src/stand.rs:154-175; tests/blitz-tests/tests/stand_load_page.rs:35-111) |»
  new: «read by `stand_load_page` and the stand unit test `the_load_page_click_leaves_a_load_in_flight`; the load page is also booted by `stand_keyboard`'s `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `stand_contrast`'s `the_load_page_meets_its_ratios` | (examples/seven_guis/src/stand.rs:154-175; tests/blitz-tests/tests/stand_load_page.rs:35-111; tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_contrast.rs:647-677) |»
- after: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:11
  anchor: «| Shell | `RecordingShell` implements `ShellProvider` and records `set_ime_enabled` calls | (tests/blitz-tests/tests/interaction_state_teardown.rs:27-35) |»
  add: «| Event loop (blitz-shell) | a test-local `CountedWakes` implements `EventLoopProxyProvider`, read by `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` for a loop woken once | (packages/blitz-shell/src/event.rs:138-142) |»

## Patterns
[[pattern]]
id = "tp3-01"
fixed = "asserted by no check (the `select` model's limits)"

[[pattern]]
id = "tp3-02"
fixed = "by 24 checks"

[[pattern]]
id = "tp3-03"
fixed = "148 `.rs` files, 147 test targets"

[[pattern]]
id = "tp3-04"
fixed = "four `node/select.rs` tests pin"

[[pattern]]
id = "tp3-05"
fixed = "is an exception to the rule that pipeline tests run"

[[pattern]]
id = "tp3-06"
fixed = "47,949 bytes in 740 lines, 5 parts"
