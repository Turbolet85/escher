proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "| Hostile `select` markup | `hostile_html`, a page built in code with `format!`: 2,000 options, a 5,500-character label and a hostile `size`, read by `hostile_markup_keeps_every_box_finite_and_panics_nowhere` in both layout modes | (tests/blitz-tests/tests/select_popup.rs:913-930; tests/blitz-tests/tests/select_popup.rs:938-975) |"
    sidecar: "Test Data & Fixtures: new seed row, the hostile select page `select_popup` generates in code."
    rationale: "The report's Coverage of new surfaces (`size on a multiple select`: hostile `size`) and Outcome (security: hostile `size`, 2,000 options, a 5,500-character label) bring a generated page this table's seeds do not hold; its lines are in New text, by line."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 23
    section: "## 7. Test Data & Fixtures"
    change: "| Select list pages (`rsx!`, each in its check's own file) | `select_popup`'s `page`, `stated_page` (an author's `aria-expanded` and `open`), `clipped_page`, `long_page`, `sized_page` (`size` on a `multiple` select), `focus_moved_page` (a `:focus` rule moving the select's box) and `chosen_disabled_page` (a disabled chosen option); `select_popup_painted`'s `page`, `clipped_page`, `grouped_page` and `long_page`; `stand_act_popup`'s `long_page`, held through `hold_long` | (tests/blitz-tests/tests/select_popup.rs:32-107; tests/blitz-tests/tests/select_popup.rs:616-626; tests/blitz-tests/tests/select_popup.rs:653-675; tests/blitz-tests/tests/select_popup.rs:857-868; tests/blitz-tests/tests/select_popup.rs:977-993; tests/blitz-tests/tests/select_popup.rs:1092-1103; tests/blitz-tests/tests/select_popup.rs:1124-1134; tests/blitz-tests/tests/select_popup_painted.rs:22-34; tests/blitz-tests/tests/select_popup_painted.rs:189-209; tests/blitz-tests/tests/select_popup_painted.rs:255-267; tests/blitz-tests/tests/select_popup_painted.rs:299-310; tests/blitz-tests/tests/stand_act_popup.rs:168-196) |"
    sidecar: "Test Data & Fixtures: new seed row, the `rsx!` pages the three new select list checks state in their own files."
    rationale: "The report's Changes name three new check files and its New text, by line lists their pages; the table holds no row for them. Deviations names `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`, the reader of `focus_moved_page`."
note: "No retired item and no count of the report stands in this file; `option_needles` (examples/seven_guis/tests/common/mod.rs:425-453) is a helper of §3, not a row here, and the `mod fixtures;` count moves at the body's §3 line 151."
