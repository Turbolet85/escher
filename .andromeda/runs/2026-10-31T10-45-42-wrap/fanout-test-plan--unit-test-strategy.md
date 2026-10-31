proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 63
    section: "## 4. Unit Test Strategy"
    change: "Append to the blitz-dom (node) record, after its four `node/select.rs` tests: ; since 2026-10-31-engine-debts further `node/select.rs` tests pin the rules of a `multiple` select and of `optgroup` — a step moves from the row it is given (`a_step_moves_from_the_row_it_is_given`), a span lists the enabled rows between two, either way round (`a_span_lists_the_enabled_rows_between_two_either_way_round`), and the list of options runs through a group in tree order (`the_list_of_options_runs_through_a_group_in_tree_order`) (packages/blitz-dom/src/node/select.rs:438-457; packages/blitz-dom/src/node/select.rs:459-477; packages/blitz-dom/src/node/select.rs:560-589)"
    sidecar: "unit-test-strategy: the blitz-dom (node) record gains the `multiple` and `optgroup` rule tests of `node/select.rs`, 2026-10-31-engine-debts"
    rationale: "The report's Symbols / APIs F6 and F7 add the rule types and rules to `node/select.rs`, its Coverage of new surfaces names unit `node::select`, and its New text, by line lists these three tests; the record states only the four tests from before this chunk."
    basis: packages/blitz-dom/src/node/select.rs:560-589
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 63
    section: "## 4. Unit Test Strategy"
    change: "New record after the blitz-dom (node) record: - **blitz-dom (form):** since 2026-10-31-engine-debts two tests, joining form.rs's standing test module, pin a select's form entries through the helpers `select_entries` and `entries_of` — a `multiple` select submits its set less its disabled rows (`a_multiple_select_submits_its_set_less_its_disabled_rows`), and a single select submits its choice through a group (`a_single_select_submits_its_choice_through_a_group`) (packages/blitz-dom/src/form.rs:558-592; packages/blitz-dom/src/form.rs:594-617)"
    sidecar: "unit-test-strategy: a new blitz-dom (form) record states the two select entry tests of form.rs, 2026-10-31-engine-debts"
    rationale: "The report's Crates / modules says form.rs's test module gained two tests, Outcome (F6, F7) cites `form.rs` +2, Deviations 7 says they joined the standing module, and New text, by line names them; the category file holds no record of them."
    basis: packages/blitz-dom/src/form.rs:558-617
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 63
    section: "## 4. Unit Test Strategy"
    change: "New record after the blitz-dom records: - **blitz-shell:** since 2026-10-31-engine-debts the crate's first unit test, an inline `#[cfg(test)]` module in `event.rs`, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, over the module's `CountedWakes`, an `EventLoopProxyProvider`; the test states what it cannot read (packages/blitz-shell/src/event.rs:128-172; packages/blitz-shell/src/event.rs:138-142; packages/blitz-shell/src/event.rs:144-171)"
    sidecar: "unit-test-strategy: a new blitz-shell record states the crate's first unit test, 2026-10-31-engine-debts"
    rationale: "The report's Crates / modules adds the `#[cfg(test)]` module at `blitz-shell/src/event.rs:128-172`, its first unit test; Counts moves blitz-shell 0 to 1; Outcome CARRY 2 cites the test as stating what it cannot read; the category file names no blitz-shell unit test."
    basis: packages/blitz-shell/src/event.rs:128-172
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 27
    section: "## 4. Unit Test Strategy"
    change: "New record after the MCP listing test's helpers: - **escher-driver (`type`'s help):** since 2026-10-31-engine-debts schema_tests.rs pins `type`'s help, moved with `type` on a `multiple` select; no test is added, the crate's unit count staying 100 (packages/escher-driver/src/schema_tests.rs:99-100; packages/escher-driver/src/schema.rs:239-240)"
    sidecar: "unit-test-strategy: a new escher-driver record states the pin of `type`'s help in schema_tests.rs, 2026-10-31-engine-debts"
    rationale: "The report's scope record says `schema_tests.rs` serves `schema.rs`, pinning `type`'s help; Symbols / APIs lists the moved help at `schema.rs:239-240`; Counts holds escher-driver at 100; the file's schema_tests.rs clauses do not state this pin."
    basis: packages/escher-driver/src/schema_tests.rs:99-100
note: "Outside this file, for test-plan §1's reader: its blitz-dom (document, mutator, net, form) bullet still says a named select's entries have no test of their own, and its covered-behaviours bullet says the select's limits are asserted by no check, each against this report's form.rs tests and select_interaction 18; the report's blitz-dom count leaves the total of new node/select.rs tests unstated, so the first proposal states no new total."
