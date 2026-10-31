# Cascade compare — test-plan · wrap · 2026-10-31-engine-debts
E items 61 · held 22 · lost at classify 4 · lost at strong 0 · beside a placed fact 1 · cascade only 6 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### mocking-stubbing-discipline#1.2 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 11
    section: "## 8. Mocking & Stubbing Discipline"
    change: add after line 11 the table row | Shell (event loop) | `CountedWakes`, a test-local `EventLoopProxyProvider` in `blitz-shell`'s first and one unit test since 2026-10-31-engine-debts, stands in for the event loop's proxy and counts its wakes — `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` | (packages/blitz-shell/src/event.rs:138-142; packages/blitz-shell/src/event.rs:144-171) |
    sidecar: 2026-10-31-engine-debts — §8 gains the `CountedWakes` row, the test-local `EventLoopProxyProvider` stand-in of `blitz-shell`'s first unit test.
    rationale: the report's Changes add `blitz-shell/src/event.rs`'s first `#[cfg(test)]` module (Crates / modules; unit tests blitz-shell 0 → 1) and New text, by line lists `impl EventLoopProxyProvider for CountedWakes` in it, a hand-written stand-in this file's table of stand-ins does not yet hold.
    basis: packages/blitz-shell/src/event.rs:138-142

### quality-gates-coverage-targets#5.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 34
    section: "## 10. Quality Gates & Coverage Targets"
    change: "A new continuation line follows line 34: Read against the ceiling on the dev host at 2026-10-31-engine-debts, rust-code-analysis-cli's version unmoved: `findings` 7, `press_multiple_select_key` 6, `click_select_row` 6, `overlaps` 13 and `visible_region` 11, 25 in a first form that did not ship, the helper `padding_box_in_view` taking its lines with the same arithmetic in the same order for a page with no transform — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: the complexity readings of the chunk's new and reworked functions, all under the ceiling of 15."
    rationale: "The report's Counts / qualifiers moved (Cognitive complexity), Reverted and Deviations 5 give readings against this record's ceiling; it marks them as no master site, so they arrive as new text."

### test-data-fixtures#3.21 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 42
    section: "## 7. Test Data & Fixtures"
    change: "In the `Network answer (stand load page)` row, after `read by `stand_load_page` and the stand unit test `the_load_page_click_leaves_a_load_in_flight``, add: `; since 2026-10-31-engine-debts the load page is also booted by `stand_keyboard`'s `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `stand_contrast`'s `the_load_page_meets_its_ratios``"
    sidecar: "2026-10-31-engine-debts: the load page's stand-in network row gains the two new load-page rows of stand_keyboard and stand_contrast."
    rationale: "The report's Coverage of new surfaces names `stand_keyboard`, `stand_contrast` and `stand_load_page` as the load page rows' tests, and New text, by line lists the two new tests, so the row's list of what reaches `boot_load`'s `LoadNet` is short by two checks."

### test-data-fixtures#3.2 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 23
    section: "## 7. Test Data & Fixtures"
    change: "Add a row after the `Shared fixture page (`rsx!`)` row: | Page in its check's file (`rsx!`) | a page one check reads stays in that check's file, as an `rsx!` component of its own: `select_interaction`'s `rows_page`, `seeded_page` and `groups_page`, `stand_act_select`'s own `rows_page`, `stand_lint`'s `selects_page` with its planted overlap of two `multiple` rows reading exactly `fx-row-a` × `fx-row-b`, `inert_subtree`'s `focus_page`, `accessibility_states`' `natives_page`, `stand_act_transformed`'s `zoomed_world_fixture`, and the `page` of `checkable_change_event` and of `text_range_no_change_event` | (tests/blitz-tests/tests/select_interaction.rs:382-423; tests/blitz-tests/tests/select_interaction.rs:714-723; tests/blitz-tests/tests/select_interaction.rs:745-779; tests/blitz-tests/tests/stand_act_select.rs:263-280; tests/blitz-tests/tests/stand_lint.rs:286-317; tests/blitz-tests/tests/inert_subtree.rs:185-201; tests/blitz-tests/tests/accessibility_states.rs:352-361; tests/blitz-tests/tests/stand_act_transformed.rs:106-136; tests/blitz-tests/tests/checkable_change_event.rs:16-44; tests/blitz-tests/tests/text_range_no_change_event.rs:15-40) |"
    sidecar: "2026-10-31-engine-debts: a new seeds row states the rsx! pages this chunk's checks hold in their own files, the planted lint overlap among them."
    rationale: "New text, by line lists these added pages and Deviations 8 states the planted overlap; the seeds table has a row for the shared `rsx!` page and none for a page held in its check's file, the home the body's §7 gives a page with one reader."

### unit-test-strategy#3.1 · lost at classify · beside a placed fact F18 (near 8, unit-test-strategy#1.1) · .andromeda/test-plan-detail/unit-test-strategy.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 27
    section: "## 4. Unit Test Strategy"
    change: "New record after the MCP listing test's helpers: - **escher-driver (`type`'s help):** since 2026-10-31-engine-debts schema_tests.rs pins `type`'s help, moved with `type` on a `multiple` select; no test is added, the crate's unit count staying 100 (packages/escher-driver/src/schema_tests.rs:99-100; packages/escher-driver/src/schema.rs:239-240)"
    sidecar: "unit-test-strategy: a new escher-driver record states the pin of `type`'s help in schema_tests.rs, 2026-10-31-engine-debts"
    rationale: "The report's scope record says `schema_tests.rs` serves `schema.rs`, pinning `type`'s help; Symbols / APIs lists the moved help at `schema.rs:239-240`; Counts holds escher-driver at 100; the file's schema_tests.rs clauses do not state this pin."
    basis: packages/escher-driver/src/schema_tests.rs:99-100

## Held

- ci-integration#1.1
- ci-integration#2.1
- ci-integration#3.1
- ci-integration#6.1
- ci-integration#7.1
- integration-test-strategy#1.1
- integration-test-strategy#2.1
- integration-test-strategy#3.1
- quality-gates-coverage-targets#2.1
- quality-gates-coverage-targets#3.1
- test-scope-summary#11.1
- test-scope-summary#14.1
- test-scope-summary#14.4
- test-scope-summary#2.3
- test-scope-summary#2.4
- test-scope-summary#3.2
- test-scope-summary#3.3
- test-scope-summary#3.4
- test-scope-summary#8.1
- test-strategy#4.1
- test-strategy#8.1
- unit-test-strategy#8.1

## Cascade only

- e2e-test-strategy#2.2
- mocking-stubbing-discipline#1.1
- test-data-fixtures#3.3
- test-scope-summary#13.1
- unit-test-strategy#1.1
- unit-test-strategy#4.1
