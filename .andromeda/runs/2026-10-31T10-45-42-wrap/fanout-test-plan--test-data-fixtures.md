proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 42
    section: "## 7. Test Data & Fixtures"
    change: "In the `Network answer (stand load page)` row, after `read by `stand_load_page` and the stand unit test `the_load_page_click_leaves_a_load_in_flight``, add: `; since 2026-10-31-engine-debts the load page is also booted by `stand_keyboard`'s `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `stand_contrast`'s `the_load_page_meets_its_ratios``"
    sidecar: "2026-10-31-engine-debts: the load page's stand-in network row gains the two new load-page rows of stand_keyboard and stand_contrast."
    rationale: "The report's Coverage of new surfaces names `stand_keyboard`, `stand_contrast` and `stand_load_page` as the load page rows' tests, and New text, by line lists the two new tests, so the row's list of what reaches `boot_load`'s `LoadNet` is short by two checks."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 23
    section: "## 7. Test Data & Fixtures"
    change: "Add a row after the `Shared fixture page (`rsx!`)` row: | Page in its check's file (`rsx!`) | a page one check reads stays in that check's file, as an `rsx!` component of its own: `select_interaction`'s `rows_page`, `seeded_page` and `groups_page`, `stand_act_select`'s own `rows_page`, `stand_lint`'s `selects_page` with its planted overlap of two `multiple` rows reading exactly `fx-row-a` × `fx-row-b`, `inert_subtree`'s `focus_page`, `accessibility_states`' `natives_page`, `stand_act_transformed`'s `zoomed_world_fixture`, and the `page` of `checkable_change_event` and of `text_range_no_change_event` | (tests/blitz-tests/tests/select_interaction.rs:382-423; tests/blitz-tests/tests/select_interaction.rs:714-723; tests/blitz-tests/tests/select_interaction.rs:745-779; tests/blitz-tests/tests/stand_act_select.rs:263-280; tests/blitz-tests/tests/stand_lint.rs:286-317; tests/blitz-tests/tests/inert_subtree.rs:185-201; tests/blitz-tests/tests/accessibility_states.rs:352-361; tests/blitz-tests/tests/stand_act_transformed.rs:106-136; tests/blitz-tests/tests/checkable_change_event.rs:16-44; tests/blitz-tests/tests/text_range_no_change_event.rs:15-40) |"
    sidecar: "2026-10-31-engine-debts: a new seeds row states the rsx! pages this chunk's checks hold in their own files, the planted lint overlap among them."
    rationale: "New text, by line lists these added pages and Deviations 8 states the planted overlap; the seeds table has a row for the shared `rsx!` page and none for a page held in its check's file, the home the body's §7 gives a page with one reader."
note: "Line 24 holds Cells' 47,949 bytes in 740 lines: the report states `PARTS` 5 unmoved and gives no re-read of those two figures although F13 gives `cells-grid` an id and a name, so no value is proposed and the figures stand unconfirmed by the report."
