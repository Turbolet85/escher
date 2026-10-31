proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 28
    section: '## 2. Test Strategy'
    change: 'Replace `and states each limit as pinned by its check or as asserted by no check (the `select` model''s limits)` with `and states each limit as pinned by its check or as asserted by no check; the `select` model''s limits, read as asserted by no check until 2026-10-31-engine-debts, now stand in its two bullets `The select model''s limits` and `The select model''s other limits`, the model''s keys, rows and groups pinned by `select_interaction` and `stand_act_select``.'
    sidecar: 'test-strategy: the `select` model''s limits are no longer the example of limits asserted by no check.'
    rationale: 'Report, Symbols / APIs F6 to F8 and Outcome: `select_interaction` 18 and `stand_act_select` 10 pin the model, and Expected amendments carries "the select''s limits now asserted"; New text, by line, lists the two capability bullets. The report does not say which of the other limits, `size` among them, a check pins, so the edit claims the keys, rows and groups alone.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 53
    section: '## 2. Test Strategy'
    change: 'Replace `read through `mod fixtures;` by 24 checks — 22 `stand_*.rs` (` with `read through `mod fixtures;` by 28 checks, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md; the 24 until that chunk — 22 `stand_*.rs` (`, the rest of the list standing as that 24.'
    sidecar: 'test-strategy: `mod fixtures` readers 24 to 28, the named split kept as the earlier 24.'
    rationale: 'Report, Counts / qualifiers moved, `fixture-readers`: readers 24 to 28, changed at `test-strategy.md:53`. The report gives the total alone, so the four new readers are not named and the 22 and two split is dated, not re-split.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 54
    section: '## 2. Test Strategy'
    change: 'Replace `148 `.rs` files, 147 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md (145 and 144` with `153 `.rs` files, 152 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md (148 and 147, as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md, before `visible_region_transformed.rs`, `hit_non_invertible_transform.rs`, `open_state.rs`, `checkable_change_event.rs` and `text_range_no_change_event.rs`, each its own auto-discovered target with no manifest line; 145 and 144`.'
    sidecar: 'test-strategy: blitz-tests `.rs` files 148 to 153 and test targets 147 to 152, the five new targets named.'
    rationale: 'Report, Counts / qualifiers moved, `rs-files`: 148 to 153 files and 147 to 152 targets, changed at `test-strategy.md:54`; Harness / gate surface: `BlitzTestsTargetsTest`''s manifest is unmoved.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 58
    section: '## 2. Test Strategy'
    change: 'Add after this line: `- **Shared tab tables:** since 2026-10-31-engine-debts `tests/common/mod.rs` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` beside the shared tables (tests/blitz-tests/tests/common/mod.rs:93-98)`.'
    sidecar: 'test-strategy: `tests/common/mod.rs` gains the load page''s and Cells'' tab tables.'
    rationale: 'Report, Symbols / APIs F13: `common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`.'
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 8
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `- **blitz-shell''s first unit test:** since 2026-10-31-engine-debts `packages/blitz-shell/src/event.rs` holds an inline `#[cfg(test)]` module with the crate''s first unit test, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read (packages/blitz-shell/src/event.rs:128-172)`.'
    sidecar: 'unit-test-strategy: blitz-shell''s first inline unit test recorded.'
    rationale: 'Report, Crates / modules: `blitz-shell/src/event.rs` (`:128-172`), its first unit test; Outcome, CARRY 2: `blitz-shell`''s unit test stating what it cannot read.'
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 34
    section: '## 4. Unit Test Strategy'
    change: 'Replace `(the three rounds; the stop at the first red)` with `(the three rounds, its case naming the leg''s fifteen named targets since 2026-10-31-engine-debts; the stop at the first red)`, and break the line before ` — the `a11y` row asserting`, which opens a continuation line, so neither part stands over 2,000 chars.'
    sidecar: 'unit-test-strategy: the `stand-repeat` three-rounds case names the leg''s fifteen targets.'
    rationale: 'Report, Harness / gate surface: `test_stand_repeat_leg_runs_three_rounds` names the five more targets; Counts / qualifiers moved, `ten-named`: ten to fifteen, `test_ci_workflows.py:365-367`. `Ran 108 tests` is unmoved, and the report already lists `unit-test-strategy.md:34` among its lines over 2,000 chars.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 63
    section: '## 4. Unit Test Strategy'
    change: 'Add after this line: `- **blitz-dom (node, a select''s steps, spans and groups):** since 2026-10-31-engine-debts `node/select.rs`'' tests also pin a step moving from the row it is given, a span listing the enabled rows either way round, and the option list running through a group in tree order (packages/blitz-dom/src/node/select.rs:438-457; packages/blitz-dom/src/node/select.rs:459-477; packages/blitz-dom/src/node/select.rs:560-589)`.'
    sidecar: 'unit-test-strategy: `node/select.rs` tests of steps, spans and groups recorded.'
    rationale: 'Report, Symbols / APIs F6 and F7 and New text, by line: `a_step_moves_from_the_row_it_is_given`, `a_span_lists_the_enabled_rows_between_two_either_way_round`, `the_list_of_options_runs_through_a_group_in_tree_order`. The report gives no per-file total, so the standing four are not recounted.'
    fact: F20
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 8
    section: '## 5. Integration Test Strategy'
    change: 'Restate the claim: `- **Layout assertions ↔ both layout modes:** since 2026-10-31-engine-debts upstream''s `pseudo_element_flow_classification` (3 tests — every box-generating `::before` or `::after` voting in flow classification) runs `for incremental in [false, true]` as other pipeline tests do, ending the exception this line stated until then; `incremental_oracle`, green beside it, stays the layout-identity witness (§2 Differential oracle) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`.'
    sidecar: 'integration-test-strategy: the both-modes exception of `pseudo_element_flow_classification` retired.'
    rationale: 'Report, Retired: `upstream''s both-modes exception`, the three tests now running `for incremental in [false, true]`; `flow-class` and `no-assert-gap` both change `integration-test-strategy.md:8`.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 14
    section: '## 5. Integration Test Strategy'
    change: 'Add after this line: `- **Script ↔ DOM, a radio''s `change`:** since 2026-10-31-engine-debts dom.rs holds `radio_click_fires_change_only_where_it_was_not_checked`, a first click hearing `input,change` and a second adding `input` alone; a page''s script hears one `change`, from the engine, the crate''s own synthesis of it gone (packages/blitz-vibey-script/tests/dom.rs:372-400)`.'
    sidecar: 'integration-test-strategy: the radio `change` test in blitz-vibey-script''s `dom` target recorded.'
    rationale: 'Report, Symbols / APIs F10: `runtime/events.rs` loses its `change` synthesis, so a page''s script hears one `change`, from the engine; New text, by line: the test at dom.rs `372-400`, with `["input,change", "input,change,input"]` at `394-399`.'
    fact: F21
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 24
    section: '## 5. Integration Test Strategy'
    change: 'After the clause on a click on a Cells cell right after `scroll`, which an `advance` past the fade precedes, add: `since 2026-10-31-engine-debts the keyboard walks to a cell and commits a value read in the first part, `press` walking and opening the editor, `type` supplying the characters and `press` committing (`stand_cells_served`), and a click inside a cell''s own open editor returns early, so typed text stays where it was reset to the stored text before;`.'
    sidecar: 'integration-test-strategy: Cells walked by keys in a held session, and the one pointer path moved.'
    rationale: 'Report, Spec claims disproved, step 16: `press` walks and opens the editor, `type` supplies the characters, `press` commits; New text, by line: `the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part`; Deviations 6a: the pointer path moved, needed for a keyboard-opened editor to keep its return to the grid.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 260
    section: '## 5. Integration Test Strategy'
    change: 'Replace `Cells walked in parts through the driver (`stand_cells_served`, a cell past the viewport read in the last part,` with `Cells walked in parts through the driver and, since 2026-10-31-engine-debts, by keys to a cell whose committed value reads in the first part (`stand_cells_served`, a cell past the viewport read in the last part,`.'
    sidecar: 'test-plan §5: the broad Session line names Cells walked by keys.'
    rationale: 'The broad line the group details, moved with it. Report, Coverage of new surfaces: `cells-grid` Tab stop and keys are tested by `stand_cells_served` among others; New text, by line: its keyboard test.'
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 25
    section: '## 5. Integration Test Strategy'
    change: 'Add after this line: `- **Session ↔ held instance, a `multiple` select:** since 2026-10-31-engine-debts `stand_act_select` drives a `multiple` select through the driver, in both layout modes — its name, role and rows read each with its own bounds, an accepted row click choosing the row, a Shift press spanning rows, `type` choosing its match alone with no stray click, and an option under a closed select''s group refused `covered` (tests/blitz-tests/tests/stand_act_select.rs:262-457)`.'
    sidecar: 'integration-test-strategy: `stand_act_select` drives a `multiple` select through the driver.'
    rationale: 'Report, Outcome: `stand_act_select` 10; Symbols / APIs F6 and F8: `type` on a `multiple` select focuses with no click, and a closed select''s option stays refused `covered`, under a group too; New text, by line: its five new tests in `262-457`, each `for incremental in [false, true]`.'
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 19
    section: '## 6. E2E Test Strategy'
    change: 'Append to this entry: `; since 2026-10-31-engine-debts Cells'' synchronous `MountedData` handle calls are measured headlessly and their windowed path is read from the same lines and not run, the async form tried first having panicked with `RefCell already borrowed` at `events.rs:162``.'
    sidecar: 'e2e-test-strategy: Cells'' handle calls measured headlessly, their windowed path not run.'
    rationale: 'Report, Reverted / negative API facts: the async form panicked at `events.rs:162`; Deviations 2: measured headlessly, the windowed path read from the same lines and not run.'
    fact: F22
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 24
    section: '## 7. Test Data & Fixtures'
    change: 'Replace `47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`,` with `47,949 bytes in 740 lines as read before 2026-10-31-engine-debts gave `cells-grid` an `id` and an `aria_label`, that chunk''s report stating the 5 parts of `SNAPSHOT_TEXT_BUDGET` unmoved and no new reading of the bytes or lines,`.'
    sidecar: 'test-data-fixtures: Cells'' byte and line figures dated at the `cells-grid` change; its 5 parts hold.'
    rationale: 'Report, Symbols / APIs F13: `cells-grid` takes `id`, `tabindex` and `aria_label`; Counts / qualifiers moved: `PARTS` 5 is measured unmoved, and no pattern or reading names the 47,949 bytes or 740 lines. Those figures may have moved with `cells.rs`, so a re-read is the owner''s and this edit only dates them.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 10
    section: '## 8. Mocking & Stubbing Discipline'
    change: 'Add after this row: `| Event loop (blitz-shell) | a test-local `CountedWakes` implements `EventLoopProxyProvider`, read by `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` for a loop woken once | (packages/blitz-shell/src/event.rs:138-142) |`.'
    sidecar: 'mocking-stubbing-discipline: blitz-shell''s `CountedWakes` stand-in recorded.'
    rationale: 'Report, New text, by line: `impl EventLoopProxyProvider for CountedWakes` at `138-142` of `blitz-shell/src/event.rs`, inside the new test module. The table holds a row per stand-in concern and has none for it.'
note: F19 is left to the new-detail pass, no picked entity carrying `form.rs`; §3 body sites `test-plan.md:151`, `:190` and §1's `:17`, `:19` are outside group 3, and `test-plan.md:237` (with `:104`) says slice s03 holds no unit test, which the new-detail or group owner should read against blitz-shell 0 to 1 since the files read here do not say which slice holds that crate; D-tests-coverage, D-tests-framework and D-tests-obs-harness find no drift.
