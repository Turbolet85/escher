proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 21
    section: "## 1. Test Scope Summary"
    change: "Replace `36 `#[test]` functions` with `38 `#[test]` functions`; replace `1 in form.rs's first test module, `a_range_input_submits_its_current_value`` with `3 in form.rs's test module, `a_range_input_submits_its_current_value` and, since 2026-10-31-engine-debts, `a_multiple_select_submits_its_set_less_its_disabled_rows` and `a_single_select_submits_its_choice_through_a_group``; replace `have no test of their own` with `are pinned by those two since that chunk`."
    sidecar: "test-scope-summary: blitz-dom's named tests 36 → 38, form.rs 1 → 3, a named select's entries now tested."
    rationale: "Report Outcome (F6, F7) states `form.rs` +2 and Coverage of new surfaces names `form` for the `multiple` select and `optgroup`; the line's no-test claim no longer holds. A site the report's patterns do not name, found by this detector's sweep."
    basis: "packages/blitz-dom/src/form.rs:558-617"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "Replace `the four `node::select` unit tests` with `the eight `node::select` unit tests`."
    sidecar: "test-scope-summary: the select rules' unit tests four → eight, the blitz-dom covered behaviours line."
    rationale: "Second occurrence of the count moved at `node/select.rs`'s own line; report Unit tests row, blitz-dom 106 → 112, `form.rs` +2."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 24
    section: "## 1. Test Scope Summary"
    change: "Replace `the single-choice `select` model` with `the `select` model`, and replace `its stated limits (a `multiple` select and an `optgroup`'s options not modelled, no popup painted, an option with no author key reading in `unkeyed_actionable`) asserted by no check` with `since 2026-10-31-engine-debts a `multiple` select and an `optgroup`'s options are modelled, read by `select_interaction`, `stand_act_select` and `form`; of its limits Enter and Space opening nothing on a single select is asserted (`enter_and_space_open_nothing_on_a_single_select`) and a closed select's option under a group stays refused `covered` (`an_option_under_a_closed_selects_group_is_refused_covered`); no popup is painted, and an option with no author key reading in `unkeyed_actionable` is still asserted by no check`."
    sidecar: "test-scope-summary: the select's limits moved, `multiple` and `optgroup` modelled and asserted, no popup painted."
    rationale: "Report Symbols F6, F7, F8 and Counts `multiple-select` and `optgroup`, each naming test-scope-summary.md:24; Expected amendments: test-plan §1, the select's limits now asserted."
    basis: "tests/blitz-tests/tests/select_interaction.rs:355-380"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 32
    section: "## 1. Test Scope Summary"
    change: "After `grid_row_grows_by_baseline_shim_of_pseudo_only_items` add `, each running in both layout modes since 2026-10-31-engine-debts ended upstream's both-modes exception`."
    sidecar: "test-scope-summary: `pseudo_element_flow_classification`'s three tests run in both layout modes."
    rationale: "Report Retired (`upstream's both-modes exception`) and Counts `flow-class`, which names test-scope-summary.md:32; Outcome (CARRY 2)."
    basis: "tests/blitz-tests/tests/pseudo_element_flow_classification.rs:44-71"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 33
    section: "## 1. Test Scope Summary"
    change: "Replace ``node/select.rs` holds 4 inline table tests of the select's rules` with ``node/select.rs` holds 8 inline tests of the select's rules (4 until 2026-10-31-engine-debts, whose four include a step moving from the row it is given, a span listing the enabled rows between two either way round and the list of options running through a group in tree order)`."
    sidecar: "test-scope-summary: `node/select.rs`'s inline tests 4 → 8."
    rationale: "Report Unit tests row, blitz-dom 106 → 112, less `form.rs`' two (Outcome F6, F7): four, and New text lists no test added in another blitz-dom module. The 8 is that subtraction, no figure the report prints; New text names three of the four."
    basis: "packages/blitz-dom/src/node/select.rs:438-589"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 34
    section: "## 1. Test Scope Summary"
    change: "Replace `it has no test of its own — `details_element` and the a11y leg read green beside it` with `since 2026-10-31-engine-debts `open_state` 4 reads it, each in both layout modes: at construction from the attribute (`at_construction_open_is_read_from_the_attribute`, four rows), a write and a removal through the mutator (`a_write_and_a_removal_through_the_mutator_move_it`), a summary's click and its Enter flipping its details (`a_summary_click_flips_its_details`, `a_summary_enter_flips_its_details`); its own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg or `run stand``; add `, `open_state` at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md` after the line's as-measured."
    sidecar: "test-scope-summary: the `:open` gap closed, `open_state` 4."
    rationale: "Report Counts `open-state` names test-scope-summary.md:34 changed; New targets `open_state` 4; Outcome (CARRY 2)."
    basis: "tests/blitz-tests/tests/open_state.rs:54-143"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 71
    section: "## 1. Test Scope Summary"
    change: "Replace `tests/dom.rs holds 30 `#[test]` functions covering the JS DOM APIs` with `tests/dom.rs holds 31 `#[test]` functions covering the JS DOM APIs, since 2026-10-31-engine-debts `radio_click_fires_change_only_where_it_was_not_checked` among them, a page's script hearing one `change`, from the engine`, and read that count as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md."
    sidecar: "test-scope-summary: blitz-vibey-script `dom` 30 → 31."
    rationale: "Report Unit tests row `unit-counts` names test-scope-summary.md:71, the 30; Symbols F10; Outcome (F10), (F11)."
    basis: "packages/blitz-vibey-script/tests/dom.rs:372-400"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 101
    section: "## 1. Test Scope Summary"
    change: "After `no CSS selecting by them and no other markup line moved` add `; since 2026-10-31-engine-debts Cells' grid carries an `id` too, `cells-grid`, its one Tab stop, with `tabindex` 0, `aria_label` Cells and an `onkeydown`, and no role`; the 676 data cells stay."
    sidecar: "test-scope-summary: `cells-grid` joins the keyed ids of Cells."
    rationale: "Report Counts `cells-grid` names test-scope-summary.md:101 as gaining it, and `cells-676` holds its 676 data cells true; Symbols F13."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 102
    section: "## 1. Test Scope Summary"
    change: "Replace ``accessibility_states` 5` with ``accessibility_states` 6 (5 until 2026-10-31-engine-debts)`."
    sidecar: "test-scope-summary: `accessibility_states` 5 → 6."
    rationale: "Report Target counts `n-a11y-states` names test-scope-summary.md:102 changed; Outcome (F5, a11y)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 102
    section: "## 1. Test Scope Summary"
    change: "Replace `a native checkbox or radio reading neither `aria-checked` nor `aria-pressed`` with `a native checkbox or radio reading neither author attribute, since 2026-10-31-engine-debts its own checked state written as the toggle on a checkable role alone (`a_click_and_a_space_move_a_native_control_in_the_tree`)`."
    sidecar: "test-scope-summary: a native control's toggle now reads its own checked state, in the tree."
    rationale: "Report Symbols F5: `is_native_checkable && is_checkable(role)` writes `Toggled` from `checkbox_input_checked()`, neither author attribute read; Counts `native-toggled`; Deviations 9."
    basis: "tests/blitz-tests/tests/accessibility_states.rs:386-427"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 104
    section: "## 1. Test Scope Summary"
    change: "Replace `a route that holds no check of its own, the windowed `seven_guis_native` boot smoke its one reading and `focus_redraw` green beside it` with `a route read since 2026-10-31-engine-debts by blitz-shell's first unit test, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` (packages/blitz-shell/src/event.rs:144-171), which states what it cannot read, the windowed `seven_guis_native` boot smoke its one windowed reading and `focus_redraw` green beside it`."
    sidecar: "test-scope-summary: the redraw route's gap closed, blitz-shell's first unit test."
    rationale: "Report Counts `redraw-route` names test-scope-summary.md:104 changed; Unit tests row blitz-shell 0 → 1; Crates / modules; Outcome (CARRY 2)."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 105
    section: "## 1. Test Scope Summary"
    change: "Replace ``inert_subtree` 4` with ``inert_subtree` 6 (4 until 2026-10-31-engine-debts)`, and replace `restores focus, hit and tree, the focus-fixup limit (a node made inert keeps focus) pinned there,` with `restores focus, hit and tree; a node made inert loses focus and takes no key over five rows (`a_node_made_inert_loses_focus_and_takes_no_key`), the fixup firing no DOM `blur` or `focusout`, and the tree's focus reads the window where the focused node was not built;`."
    sidecar: "test-scope-summary: the focus-fixup limit retired, `inert_subtree` 4 → 6, the tree's focus falls back to the window."
    rationale: "Report Retired (`inert_subtree::removing_inert_restores_focus_hit_and_tree`'s focus-fixup limit lines, the test's name and first assertions standing); Symbols F3, F4; Counts `n-inert` and `focus-fixup`, each naming test-scope-summary.md:105."
    basis: "tests/blitz-tests/tests/inert_subtree.rs:229-310"
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "Replace `a target inside a translated scrolling box reading outside its `visible_region`, the measured `off-screen` limit` with `a target inside a translated scrolling box reading inside its `visible_region`, the pin renamed and inverted at 2026-10-31-engine-debts, the `off-screen` limit it measured retired`."
    sidecar: "test-scope-summary: `transformed_client_rect`'s outside-its-visible-region pin retired, inverted to inside."
    rationale: "Report Retired (the pin name `…reads_outside_its_visible_region`, and `off-screen`'s transformed-box clause); Symbols F1; Counts `visible-region` names test-scope-summary.md:118."
    basis: "tests/blitz-tests/tests/transformed_client_rect.rs:183-216"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "Replace ``stand_act_transformed` 2` with ``stand_act_transformed` 3 (2 until 2026-10-31-engine-debts added `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`)`."
    sidecar: "test-scope-summary: `stand_act_transformed` 2 → 3."
    rationale: "Report Target counts `n-act-transf` names test-scope-summary.md:118 changed; Outcome (F1, arch, design)."
    basis: "tests/blitz-tests/tests/stand_act_transformed.rs:138-176"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 120
    section: "## 1. Test Scope Summary"
    change: "Replace ``stand_contrast` 5 (2 until 2026-10-14-stand-a11y-assertions)` with ``stand_contrast` 7 (5 until 2026-10-31-engine-debts, 2 until 2026-10-14-stand-a11y-assertions)`."
    sidecar: "test-scope-summary: `stand_contrast` 5 → 7."
    rationale: "Report Target counts `n-contrast` names test-scope-summary.md:120 changed."
    basis: "tests/blitz-tests/tests/stand_contrast.rs:647-704"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: "## 1. Test Scope Summary"
    change: "Replace ``stand_keyboard` 15 (11 until 2026-10-14-stand-a11y-assertions;` with ``stand_keyboard` 21 (15 until 2026-10-31-engine-debts, 11 until 2026-10-14-stand-a11y-assertions;`."
    sidecar: "test-scope-summary: `stand_keyboard` 15 → 21."
    rationale: "Report Target counts `n-keyboard` names test-scope-summary.md:121 changed."
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:157-335"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 125
    section: "## 1. Test Scope Summary"
    change: "Replace `the elements that react to a click and state no role` with `the elements that react to input and state no role`, and replace `Cells 676` with `Cells 677 (676 until 2026-10-31-engine-debts gave `cells-grid` an `onkeydown` and no role, so it joins the 676 data cells)`."
    sidecar: "test-scope-summary: `stand_non_lean_tasks`' Cells 676 → 677, reacting to input."
    rationale: "Report Spec claims disproved (Cells' count, measured 677), Counts Cells' reacting-element count 676 → 677, Deviations 1 (the doc now says react to input). `cells-676`'s dispositions name only :101 in this file; this is a hit its row list leaves out, the reacting-element count and not the data cells."
    basis: "tests/blitz-tests/tests/stand_non_lean_tasks.rs:102-103"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 128
    section: "## 1. Test Scope Summary"
    change: "Replace ``stand_lint` 7, declaring` with ``stand_lint` 8 (7 until 2026-10-31-engine-debts), declaring`; `7 findings on the planted page` stays."
    sidecar: "test-scope-summary: `stand_lint` 7 → 8."
    rationale: "Report Target counts `n-lint` names test-scope-summary.md:128 changed; Outcome (F9)."
    basis: "tests/blitz-tests/tests/stand_lint.rs:282-332"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 129
    section: "## 1. Test Scope Summary"
    change: "Replace ``[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line` with ``[0, 0, 0, 0]` — `[0, 1, 0, 0]` until 2026-10-31-engine-debts, whose `overlaps` skips a closed select's option on either side, so the flight booker's `flight-type-one-way` × `flight-type-return` reads no `overlap``."
    sidecar: "test-scope-summary: the lean tasks' boot findings `[0, 1, 0, 0]` → `[0, 0, 0, 0]`."
    rationale: "Report Counts `stand_lint`'s `at_boot`, and `lint-overlap` naming test-scope-summary.md:129 changed; Symbols F9."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 131
    section: "## 1. Test Scope Summary"
    change: "Replace ``select_interaction` 7, declaring` with ``select_interaction` 18 (7 until 2026-10-31-engine-debts, whose eleven read a `multiple` select and groups, stated in their own record below), declaring`; the seven on the controlled single-choice page stay as stated."
    sidecar: "test-scope-summary: `select_interaction` 7 → 18."
    rationale: "Report Target counts `n-select` names test-scope-summary.md:131 changed; Counts `option-click` holds the line's `covered` reading true; Outcome (F6, a11y), (F7)."
    basis: "tests/blitz-tests/tests/select_interaction.rs:354-894"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 131
    section: "## 1. Test Scope Summary"
    change: "Replace ``stand_act_select` 5, declaring` with ``stand_act_select` 10 (5 until 2026-10-31-engine-debts, whose five read a `multiple` select and a closed select's group, stated in their own record below), declaring`."
    sidecar: "test-scope-summary: `stand_act_select` 5 → 10."
    rationale: "Report Target counts `n-act-select` names test-scope-summary.md:131 changed; Outcome (F6, the click, driver)."
    basis: "tests/blitz-tests/tests/stand_act_select.rs:262-457"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 150
    section: "## 1. Test Scope Summary"
    change: "Replace ``stand_cells_served` 4, on the stand's own Cells` with ``stand_cells_served` 5 (4 until 2026-10-31-engine-debts added `the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part`), on the stand's own Cells`."
    sidecar: "test-scope-summary: `stand_cells_served` 4 → 5."
    rationale: "Report Target counts `n-cells` names test-scope-summary.md:150 among its two changed; Coverage of new surfaces, the `cells-grid` row."
    basis: "tests/blitz-tests/tests/stand_cells_served.rs:179-214"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 150
    section: "## 1. Test Scope Summary"
    change: "Replace ``stand_load_page` 2, on the load page` with ``stand_load_page` 3 (2 until 2026-10-31-engine-debts added the load page's rows, tests/blitz-tests/tests/stand_load_page.rs:55-96, SC 2.1.1 and 1.4.3 read beside them by `stand_keyboard` and `stand_contrast`), on the load page`."
    sidecar: "test-scope-summary: `stand_load_page` 2 → 3."
    rationale: "Report Target counts `n-load-page` and Text limits `load-rows`, each naming test-scope-summary.md:150 changed; Coverage of new surfaces, the Load page rows row."
    basis: "tests/blitz-tests/tests/stand_load_page.rs:55-96"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "Replace the record's text after `**The lint's select-option reading:**` with `no unit test pins it — since 2026-10-31-engine-debts escher-driver's `overlaps` (`packages/escher-driver/src/lint.rs`) skips an entry `Entry.in_closed_select` holds, on either side, reading the element and never the role, so CRUD's `option`-role rows are not read as a select's, the snapshot still giving a closed select's options its client rect; a `multiple` select's rows are linted as usual; `stand_lint` pins both on its `selects_page`, two overlapping `multiple` rows reading exactly `fx-row-a` × `fx-row-b`, and the stand's boot reading `[0, 0, 0, 0]` (§1 above)`."
    sidecar: "test-scope-summary: the lint's select-option defect retired, `overlaps` skips a closed select's options."
    rationale: "Second occurrence of the moved reading; report Counts `lint-overlap` names test-scope-summary.md:153 changed; Symbols F9; Deviations 8."
    basis: "packages/escher-driver/src/lint.rs:337-338"
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "New record: - **The visible region through transforms:** since 2026-10-31-engine-debts `visible_region_transformed` 6, each in both layout modes, its own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg or `run stand` — a world under an untransformed viewport showing a node and panning one out (`a_world_under_an_untransformed_viewport_shows_a_node_and_pans_one_out`), a target in a node body under a transformed world reading where it shows, a page with no transform reading its padding box in the client rect's frame, a translate moving the region by the translation, a box mapped to a point leaving nothing visible inside it, and under a rotation the region reading the rotated box's bounding box (tests/blitz-tests/tests/visible_region_transformed.rs:132-276) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "test-scope-summary: new target `visible_region_transformed` 6."
    rationale: "Report New targets and Symbols F1; Outcome (F1, arch, design), (F1, security); Coverage of new surfaces."
    basis: "tests/blitz-tests/tests/visible_region_transformed.rs:132-276"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "New record: - **A non-invertible transform's hit:** since 2026-10-31-engine-debts `hit_non_invertible_transform` 2, each in both layout modes, its own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg or `run stand` — a box under a transform whose determinant is 0 or not finite takes no hit, the whole subtree answering none (`a_box_under_a_non_invertible_transform_takes_no_hit`), and its invertible twin is hit where it shows (`an_invertible_twin_is_hit_where_it_shows`) (tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "test-scope-summary: new target `hit_non_invertible_transform` 2."
    rationale: "Report New targets (a master holds no statement of F2) and Symbols F2; Outcome (F2)."
    basis: "tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "New record: - **`change` on a checkbox and a radio:** since 2026-10-31-engine-debts `checkable_change_event` 3, each in both layout modes, its own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg or `run stand` — a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone, and a label's click reading as its control's (tests/blitz-tests/tests/checkable_change_event.rs:68-136) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "test-scope-summary: new target `checkable_change_event` 3."
    rationale: "Report New targets and Symbols F10; Outcome (F10), (F11); Coverage of new surfaces."
    basis: "tests/blitz-tests/tests/checkable_change_event.rs:68-136"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "New record: - **No `change` on a text input or a range:** since 2026-10-31-engine-debts `text_range_no_change_event` 2, each in both layout modes, its own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg or `run stand` — typing in a text input firing `input` and no `change`, and a range's key and pointerdown firing `input` and no `change` (tests/blitz-tests/tests/text_range_no_change_event.rs:59-97) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "test-scope-summary: new target `text_range_no_change_event` 2."
    rationale: "Report New targets; Outcome (F10), (F11); Coverage of new surfaces, the `change` row."
    basis: "tests/blitz-tests/tests/text_range_no_change_event.rs:59-97"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "New record: - **A `multiple` select and groups, at the engine:** `select_interaction`'s eleven added at 2026-10-31-engine-debts, each in both layout modes — Enter and Space opening nothing on a single select; a `multiple` select's rows laid out with boxes of their own; its keys choosing, spanning and flipping rows over a 17-row table; typed text choosing its row alone, a Space joining it while the text stands; a click choosing, flipping or spanning over a 10-row table; a disabled `multiple` select taking no click and no key; `multiple` removed settling the set to one option; a chosen row and a plain one meeting their ratio at the user agent's pair; a set the markup seeds stepped and spanned from its last chosen row; a group's options chosen by a single select's keys and shown alone; a `multiple` select's groups laid out with a disabled group's rows not chosen (tests/blitz-tests/tests/select_interaction.rs:355-894) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "test-scope-summary: `select_interaction`'s eleven `multiple` and group rows stated."
    rationale: "Report Symbols F6, F7, F8; Target counts `n-select`; Outcome (F6, a11y), (F7); New text lists the eleven heads, 7 + 11 = 18."
    basis: "tests/blitz-tests/tests/select_interaction.rs:355-894"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "New record: - **A `multiple` select, through the driver:** `stand_act_select`'s five added at 2026-10-31-engine-debts, through `Session::run` in both layout modes — a `multiple` select reading its name, role and rows, each with its own bounds; a click on a row accepted and choosing it; a `press` with Shift spanning the rows; `type` choosing its match alone with no stray click; an option under a closed select's group refused `covered` (tests/blitz-tests/tests/stand_act_select.rs:311-457) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "test-scope-summary: `stand_act_select`'s five `multiple` rows stated."
    rationale: "Report Symbols F6 (`type` on a `multiple` select), F8; Target counts `n-act-select`; Outcome (F6, the click, driver); New text lists the five heads, 5 + 5 = 10."
    basis: "tests/blitz-tests/tests/stand_act_select.rs:311-457"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "New record: - **Cells' grid and the load page, by keyboard and colour:** since 2026-10-31-engine-debts, each in both layout modes, `stand_keyboard`'s six — the load page walking its stated sequence with its button taking Enter and Space (`common::LOAD_TAB_ORDER`), Cells walking its stated sequence (`common::CELLS_TAB_ORDER`), arrows and Enter opening the moved-to cell with its value committed by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and arrows clamping at the grid's ends and bringing a cell into view — and `stand_contrast`'s `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`; the grid's role and cell semantics are out by residual line 24, and a person at a window stays `unrunnable-here` (tests/blitz-tests/tests/stand_keyboard.rs:157-335; tests/blitz-tests/tests/stand_contrast.rs:647-704; tests/blitz-tests/tests/common/mod.rs:93-98) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "test-scope-summary: Cells' grid and the load page's keyboard and contrast checks stated."
    rationale: "Report Symbols F13, Target counts `n-keyboard` and `n-contrast`, Coverage of new surfaces (the `cells-grid` and Load page rows), Deviations 9; New text lists the six heads, 15 + 6 = 21."
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:157-335"
note: "D-tests-framework and D-tests-obs-harness hold (runners and harness unmoved); D-tests-coverage is not proposed but two paths name no test in the report, Deviations 6a (cells.rs:425-433, read at Outcome F13's P2 escalation) and 6c (execute.rs:235-239); test-plan.md:17's select-entries no-test claim ends too and no pattern names it, and test-plan.md is not this file's to edit; :125's `STAND_REPEAT_NAMED` (10) and 18 stay as `stand_requirement_sweep` is not among the edited checks though the leg now names fifteen; the report names neither stand-repeat's five added targets nor the four new fixture readers, so the new records name no stand-repeat membership and :150's no-`mod fixtures;` is not re-asserted."
