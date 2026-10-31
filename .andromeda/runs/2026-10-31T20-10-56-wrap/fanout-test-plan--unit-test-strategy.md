proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 26
    section: "## 4. Unit Test Strategy"
    change: "replace «`cli_commands` (14)» with «`cli_commands` (15)»"
    sidecar: "unit-test-strategy: `cli_commands` 14 to 15, the hosted click, shot and row choice of 2026-10-31-select-popup"
    rationale: "The report's Counts / qualifiers moved, `cli-count`: `cli_commands` 14 → 15, naming unit-test-strategy:26 changed; the new test is listed in New text at examples/seven_guis/tests/cli_commands.rs:419-464."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 26
    section: "## 4. Unit Test Strategy"
    change: "replace «the fifteen `stand_act_*` files» with «the sixteen `stand_act_*` files»"
    sidecar: "unit-test-strategy: `stand_act_*` files fifteen to sixteen, `stand_act_popup` added by 2026-10-31-select-popup"
    rationale: "The report's Counts / qualifiers moved, `act-files`: `stand_act_*` files fifteen → sixteen (`git ls-tree`: 15 → 16), naming unit-test-strategy:26 changed; `stand_act_popup` is the new file in Changes → Files."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 39
    section: "## 4. Unit Test Strategy"
    change: "replace «**escher-driver (`type`'s help):** schema_tests.rs pins `type`'s help on a `multiple` select inside a standing test, with no test of its own» with «**escher-driver (the help on a select):** schema_tests.rs pins the fixed help `click`, `type` and `press` state of a select, restated at 2026-10-31-select-popup, inside standing tests, with no test of its own»"
    sidecar: "unit-test-strategy: schema_tests.rs's pin of the help on a select widened from `type` on a `multiple` select to the three texts 2026-10-31-select-popup restated"
    rationale: "The report's Symbols / APIs → Driver: `click`'s, `type`'s and `press`'s fixed help restated, and `type` on any select focuses with no click; Coverage of new surfaces names `schema_tests` as the help's pin; New text lists schema_tests.rs's added ranges with no new test head among them."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 46
    section: "## 4. Unit Test Strategy"
    change: "replace «its case naming the leg's fifteen named targets» with «its case naming the leg's seventeen named targets»"
    sidecar: "unit-test-strategy: `stand-repeat`'s named targets fifteen to seventeen in the leg's test case, `select_popup` and `select_popup_painted` added"
    rationale: "The report's Counts / qualifiers moved, `repeat-names`: named `--test` words fifteen → seventeen, naming unit-test-strategy:46 changed; Harness / gate surface: `test_ci_workflows.py:367` asserts the two new words."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 82
    section: "## 4. Unit Test Strategy"
    change: "add after this line, one row: «- **blitz-dom (node, a select's list):** since 2026-10-31-select-popup `node/select_list.rs`'s inline test module holds six tests of the list's placement, its `size` and its rows' geometry — `a_list_opens_where_its_rows_fit_and_else_on_the_side_with_more_room`, `a_list_is_at_least_its_selects_width_at_most_the_viewports_and_stays_inside_it`, `every_edge_of_a_placed_list_is_finite`, `size_reads_a_whole_number_above_zero_capped` (`size_rows`), `a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement` and `a_point_reads_the_row_it_lies_on_where_the_rows_are_scrolled`; what a document does with the list is read by blitz-tests' `select_popup`, `select_popup_painted` and `stand_act_popup` (§5) (packages/blitz-dom/src/node/select_list.rs:386-539) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "unit-test-strategy: new row, the six unit tests of blitz-dom's `node/select_list.rs` from 2026-10-31-select-popup"
    rationale: "The report's Crates / modules: new module blitz-dom `node::select_list`; Coverage of new surfaces: tests unit (`select_list` 6); New text lists the module at packages/blitz-dom/src/node/select_list.rs:386-539 with the six test heads."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110
note: "D-tests-framework, D-tests-obs-harness and D-retired read no drift in this file; test-plan's body §4 and §1 under blitz-dom (node) name only `node/range.rs` and `node/select.rs` for the rules pinned, so the new `node/select_list.rs` tests are absent there too, outside this file's proposals."
