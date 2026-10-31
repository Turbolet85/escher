proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 101
    section: "## 1. Test Scope Summary"
    change: "After `cell-input` in the list of added ids, add: and, since 2026-10-31-engine-debts, `cells-grid`, the grid's own id, taken with `tabindex` 0, `aria_label` `Cells` and an `onkeydown`, with no role; the 676 data cells stay 676."
    sidecar: "test-scope-summary: `cells-grid` joins the keyed ids of Cells; the 676 data cells hold."
    rationale: "Report, Counts / qualifiers moved: `cells-grid` 4 hits, all gaining `cells-grid`, this site among them, and `cells-676` keeps the 676 data cells here as a true count; Symbols / APIs F13 gives `cells-grid` its id, tabindex, label and key handler."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 102
    section: "## 1. Test Scope Summary"
    change: "`accessibility_states` 5 reads `accessibility_states` 6 (5 until 2026-10-31-engine-debts)."
    sidecar: "test-scope-summary: `accessibility_states` 5 to 6."
    rationale: "Report, Counts / qualifiers moved, `n-a11y-states`: `accessibility_states` 5 to 6, `test-scope-summary.md:102` changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 102
    section: "## 1. Test Scope Summary"
    change: "Replace `a native checkbox or radio reading neither `aria-checked` nor `aria-pressed`` with: a native checkbox or radio reading neither author attribute, and since 2026-10-31-engine-debts writing `toggled` from its own checked state under a checkable role."
    sidecar: "test-scope-summary: a native checkable control now writes `toggled`; neither author attribute is read on it."
    rationale: "Report, Symbols / APIs F5: `is_native_checkable && is_checkable(role)` writes `Toggled` from `checkbox_input_checked()`, neither author attribute read on a native control; `native-toggled` 10 hits, 9 changed, the one no-change site being the snapshot's own read in `standard-contracts.md`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 104
    section: "## 1. Test Scope Summary"
    change: "Replace `a route that holds no check of its own, the windowed `seven_guis_native` boot smoke its one reading and `focus_redraw` green beside it` with: a route read since 2026-10-31-engine-debts by blitz-shell's first unit test, `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once` (packages/blitz-shell/src/event.rs:128-172), which states what it cannot read, the windowed `seven_guis_native` boot smoke and `focus_redraw` green beside it."
    sidecar: "test-scope-summary: the redraw route's gap closes through blitz-shell's unit test."
    rationale: "Report, Counts / qualifiers moved: `redraw-route` 15 hits, 2 changed, `test-scope-summary.md:104` among them; Outcome, CARRY 2: `blitz-shell`'s unit test stating what it cannot read; Crates / modules: its first unit test at `event.rs:128-172`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 105
    section: "## 1. Test Scope Summary"
    change: "`inert_subtree` 4 reads `inert_subtree` 6 (4 until 2026-10-31-engine-debts)."
    sidecar: "test-scope-summary: `inert_subtree` 4 to 6."
    rationale: "Report, Counts / qualifiers moved, `n-inert`: `inert_subtree` 4 to 6, `test-scope-summary.md:105` changed; Outcome (F3, a11y) and (F4, a11y)."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 105
    section: "## 1. Test Scope Summary"
    change: "Replace `the focus-fixup limit (a node made inert keeps focus) pinned there` with: its focus-fixup limit lines gone since 2026-10-31-engine-debts, a node coming under `inert` giving focus up with no `blur` or `focusout`."
    sidecar: "test-scope-summary: the focus-fixup limit retired; focus clears under `inert` with no `blur` or `focusout`."
    rationale: "Report, Retired: `inert_subtree::removing_inert_restores_focus_hit_and_tree`'s focus-fixup limit lines, the test's name and first assertions standing; Symbols / APIs F3; `focus-fixup` 8 hits, 8 changed, `test-scope-summary.md:105` among them."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "Replace `a target inside a translated scrolling box reading outside its `visible_region`, the measured `off-screen` limit` with: a target inside a translated scrolling box reading inside its `visible_region` (the pin renamed `…reads_inside_its_visible_region` and inverted at 2026-10-31-engine-debts), the region exact under a translate and a scale, the rotated padding box's bounding box under a rotation, and `None` on a non-finite edge."
    sidecar: "test-scope-summary: `transformed_client_rect`'s outside-its-region pin retired, renamed and inverted; `visible_region`'s limit moved."
    rationale: "Report, Retired: the pin name `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`, renamed and inverted; Symbols / APIs F1; `visible-region` 13 hits, 7 changed, `test-scope-summary.md:118` among them. The target's count of 4 does not move."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "`stand_act_transformed` 2 reads `stand_act_transformed` 3 (2 until 2026-10-31-engine-debts)."
    sidecar: "test-scope-summary: `stand_act_transformed` 2 to 3."
    rationale: "Report, Counts / qualifiers moved, `n-act-transf`: `stand_act_transformed` 2 to 3, `test-scope-summary.md:118` changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 120
    section: "## 1. Test Scope Summary"
    change: "`stand_contrast` 5 (2 until 2026-10-14-stand-a11y-assertions) reads `stand_contrast` 7 (5 until 2026-10-31-engine-debts; 2 until 2026-10-14-stand-a11y-assertions)."
    sidecar: "test-scope-summary: `stand_contrast` 5 to 7."
    rationale: "Report, Counts / qualifiers moved, `n-contrast`: `stand_contrast` 5 to 7, `test-scope-summary.md:120` changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: "## 1. Test Scope Summary"
    change: "`stand_keyboard` 15 reads `stand_keyboard` 21, and its parenthesis opens with `15 until 2026-10-31-engine-debts;` before its `11 until 2026-10-14-stand-a11y-assertions`, the rest of it as it stands."
    sidecar: "test-scope-summary: `stand_keyboard` 15 to 21."
    rationale: "Report, Counts / qualifiers moved, `n-keyboard`: `stand_keyboard` 15 to 21, `test-scope-summary.md:121` changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 125
    section: "## 1. Test Scope Summary"
    change: "In `stand_non_lean_tasks`' 7-row table, `the elements that react to a click and state no role` reads `the elements that react to input and state no role`, and `Cells 676` reads `Cells 677` (676 until 2026-10-31-engine-debts: the 676 data cells and `cells-grid`, whose `onkeydown` and no role join them)."
    sidecar: "test-scope-summary: `stand_non_lean_tasks`' Cells row 676 to 677, `cells-grid` joining the data cells."
    rationale: "Report, Spec claims disproved by measurement, `cells-676`: measured 677, the count being every element with the listener mark that reads `GenericContainer`, and `cells-grid` joining the 676 data cells; Deviations 1: the pin moved with its reason and its doc now says «react to input». The report's `cells-676` bullet names only the data-cell site of this file, so this row is a hit its bullet leaves out."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 128
    section: "## 1. Test Scope Summary"
    change: "`stand_lint` 7, declaring reads `stand_lint` 8 (7 until 2026-10-31-engine-debts), declaring."
    sidecar: "test-scope-summary: `stand_lint` 7 to 8."
    rationale: "Report, Counts / qualifiers moved, `n-lint`: `stand_lint` 7 to 8, `test-scope-summary.md:128` changed; Outcome (F9)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 129
    section: "## 1. Test Scope Summary"
    change: "Replace `the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line` with: the lean tasks at boot read their measured findings, `[0, 0, 0, 0]` since 2026-10-31-engine-debts (`[0, 1, 0, 0]` until then, the flight booker's overlap of its two options), the lint skipping a closed select's options."
    sidecar: "test-scope-summary: `stand_lint`'s `at_boot` `[0, 1, 0, 0]` to `[0, 0, 0, 0]`; the flight booker's option overlap is gone."
    rationale: "Report, Counts / qualifiers moved, `lint-overlap`: `at_boot` to `[0, 0, 0, 0]`, `test-scope-summary.md:129` changed; Symbols / APIs F9: `overlaps` skips such an entry on either side."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 131
    section: "## 1. Test Scope Summary"
    change: "`select_interaction` 7, declaring reads `select_interaction` 18 (7 until 2026-10-31-engine-debts), declaring."
    sidecar: "test-scope-summary: `select_interaction` 7 to 18."
    rationale: "Report, Counts / qualifiers moved, `n-select`: `select_interaction` 7 to 18, `test-scope-summary.md:131` changed; `option-click` keeps this site's statement of a closed select's option as true."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 131
    section: "## 1. Test Scope Summary"
    change: "`stand_act_select` 5, declaring reads `stand_act_select` 10 (5 until 2026-10-31-engine-debts), declaring."
    sidecar: "test-scope-summary: `stand_act_select` 5 to 10."
    rationale: "Report, Counts / qualifiers moved, `n-act-select`: `stand_act_select` 5 to 10, `test-scope-summary.md:131` changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 144
    section: "## 1. Test Scope Summary"
    change: "`which asserts `select_interaction` among that leg's logged words` reads: which asserts `select_interaction` among that leg's logged words and, since 2026-10-31-engine-debts, the five more targets the leg names."
    sidecar: "test-scope-summary: the stand-repeat pin of test_ci_workflows.py names the leg's five more targets."
    rationale: "Report, Harness / gate surface: `ci-leg.sh` `stand-repeat` names five more targets and `test_stand_repeat_leg_runs_three_rounds` names them too (`ci-leg.sh:58-59`, `test_ci_workflows.py:365-367`). The reach of the sweep's own `contains` read and the owner of its gap do not move: the report's `owner-generic` row holds this site as standing and not naming this chunk."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 150
    section: "## 1. Test Scope Summary"
    change: "`stand_cells_served` 4, on the stand's own Cells reads `stand_cells_served` 5 (4 until 2026-10-31-engine-debts), on the stand's own Cells."
    sidecar: "test-scope-summary: `stand_cells_served` 4 to 5."
    rationale: "Report, Counts / qualifiers moved, `n-cells`: `stand_cells_served` 4 to 5, `test-scope-summary.md:150` changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 150
    section: "## 1. Test Scope Summary"
    change: "`stand_load_page` 2, on the load page reads `stand_load_page` 3 (2 until 2026-10-31-engine-debts), on the load page; and after its readings add: the load page's rows are asserted since that chunk beside its own load wait, which is unchanged, its Tab sequence and button keys by `stand_keyboard`, its ratios by `stand_contrast`, with the third test of this file at tests/blitz-tests/tests/stand_load_page.rs:55-96."
    sidecar: "test-scope-summary: `stand_load_page` 2 to 3; the load page's rows are asserted."
    rationale: "Report, Counts / qualifiers moved: `n-load-page` 2 to 3 and `load-rows` 11 hits, 2 changed, `test-scope-summary.md:150` among them; Coverage of new surfaces, Load page rows: `stand_keyboard`, `stand_contrast`, `stand_load_page`. The report lists no head for the third test, so this text names its lines alone."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 153
    section: "## 1. Test Scope Summary"
    change: "Replace the sentence after `no unit test pins it` with: escher-driver's `overlaps` (`packages/escher-driver/src/lint.rs`) skips, on either side, an entry inside a closed select, `Entry.in_closed_select` through `closed_select_of`, reading the element and never the role, the snapshot still giving each such option its select's client rect, and it lints a `multiple` select's rows as usual; `stand_lint` pins the stand at `[0, 0, 0, 0]` at boot (§1 above)."
    sidecar: "test-scope-summary: the lint's select-option defect closed; a closed select's options are skipped by element."
    rationale: "Report, Symbols / APIs F9 and Counts / qualifiers moved, `lint-overlap`: `test-scope-summary.md:153` changed; `no unit test pins it` holds, `unit-counts` keeping escher-driver at 100."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "After `stand_act_transformed`'s clause add: since 2026-10-31-engine-debts `visible_region_transformed` 6, its own target, each test in both layout modes: a world panned under an untransformed viewport showing a node and panning one out, a target in a node body under a transformed world reading where it shows, a page with no transform reading its padding box in the client rects' frame, a translate moving the region by the translation, a box mapped to a point leaving nothing visible inside it, and a rotation reading the rotated box's bounding box (tests/blitz-tests/tests/visible_region_transformed.rs:132-276)."
    sidecar: "test-scope-summary: `visible_region_transformed` 6 stated."
    rationale: "Report, Counts / qualifiers moved, New targets: `visible_region_transformed` 6; Outcome (F1, arch, design) and (F1, security)."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "Beside `visible_region_transformed` add: `hit_non_invertible_transform` 2, its own target, in both layout modes: a box under a transform with determinant 0 or not finite takes no hit, and an invertible twin is hit where it shows (tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71)."
    sidecar: "test-scope-summary: `hit_non_invertible_transform` 2 stated, the rule's first statement in this file."
    rationale: "Report, Symbols / APIs F2 and New targets: `non-invertible`'s control is unfired, so no standing text states the rule; Outcome (F2)."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 105
    section: "## 1. Test Scope Summary"
    change: "After `inert_subtree`'s clause add: since 2026-10-31-engine-debts `open_state` 4, its own target, in both layout modes: `:open` read from the attribute at construction, moved by a mutator write and removal, and flipped by a summary's click and its Enter (tests/blitz-tests/tests/open_state.rs:54-143)."
    sidecar: "test-scope-summary: `open_state` 4 stated."
    rationale: "Report, Counts / qualifiers moved, New targets: `open_state` 4; Gaps CARRY 2 closes, `open-state`; Outcome (CARRY 2)."
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 105
    section: "## 1. Test Scope Summary"
    change: "Beside `open_state` add: `checkable_change_event` 3, its own target, in both layout modes: a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone, and a label's click reading as its control's (tests/blitz-tests/tests/checkable_change_event.rs:68-136)."
    sidecar: "test-scope-summary: `checkable_change_event` 3 stated."
    rationale: "Report, Symbols / APIs F10 and New targets: `checkable_change_event` 3; Outcome (F10), (F11)."
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 105
    section: "## 1. Test Scope Summary"
    change: "Beside `checkable_change_event` add: `text_range_no_change_event` 2, its own target, in both layout modes: typing in a text input, and a range's key and pointerdown, firing `input` and no `change` (tests/blitz-tests/tests/text_range_no_change_event.rs:59-97)."
    sidecar: "test-scope-summary: `text_range_no_change_event` 2 stated."
    rationale: "Report, Counts / qualifiers moved, New targets: `text_range_no_change_event` 2; Outcome (F10), (F11)."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "In `stand_act_transformed`'s clause add: since 2026-10-31-engine-debts `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`, the driver reaching a control under a zoomed world (tests/blitz-tests/tests/stand_act_transformed.rs:138-176)."
    sidecar: "test-scope-summary: `stand_act_transformed`'s third test stated."
    rationale: "Report, New text by line, `stand_act_transformed.rs`; Outcome (F1, arch, design): `stand_act_transformed` 3."
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 131
    section: "## 1. Test Scope Summary"
    change: "After `select_interaction`'s single-choice readings add: since 2026-10-31-engine-debts, in both layout modes, Enter and Space opening nothing on a single select, a `multiple` select's rows having boxes of their own, a 17-row key table, typed text choosing its row alone and a Space joining it while it stands, a 10-row click table, a disabled `multiple` select, removal of `multiple` settling the set to one option, a seeded set stepped and spanned from its last chosen row, and groups on both select kinds (tests/blitz-tests/tests/select_interaction.rs:355-380; tests/blitz-tests/tests/select_interaction.rs:447-894)."
    sidecar: "test-scope-summary: `select_interaction`'s `multiple` and group readings stated."
    rationale: "Report, Symbols / APIs F6, F7, F8 and New text by line, `select_interaction.rs`; Outcome (F6, a11y), (F7)."
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 131
    section: "## 1. Test Scope Summary"
    change: "Beside `select_interaction`'s `multiple` readings add: the chosen-row pair `#c8c8c8` on `#000000`, an engine default in `default.css`, measured at 12.56:1 (tests/blitz-tests/tests/select_interaction.rs:691-712)."
    sidecar: "test-scope-summary: the chosen-row pair's measured ratio stated."
    rationale: "Report, Schema / config: the new engine-default chosen-row pair, 12.56:1, measured by `select_interaction`."
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 131
    section: "## 1. Test Scope Summary"
    change: "After `stand_act_select`'s clause add: the driver cannot build a non-adjacent `multiple` set, `press` holding no Ctrl, and Ctrl+A is not modelled, so no driver check reaches either."
    sidecar: "test-scope-summary: the `multiple` set's reach through the driver stated as a limit."
    rationale: "Report, Symbols / APIs F6: Ctrl+A is not modelled; Fixed text moved: `press`'s help at `schema.rs:261-263`; Decisions & corrections: «Ограничение и residual (Рекомендую)» on `press`."
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 129
    section: "## 1. Test Scope Summary"
    change: "After the boot reading add: its planted overlap case since 2026-10-31-engine-debts is two overlapping rows of a `multiple` select, reading exactly `fx-row-a` × `fx-row-b`, and a closed select's options are excluded by element, never by role (tests/blitz-tests/tests/stand_lint.rs:282-332)."
    sidecar: "test-scope-summary: `stand_lint`'s planted `multiple` overlap and the closed-select exclusion stated."
    rationale: "Report, Deviations 8 and Symbols / APIs F9: `Entry.in_closed_select`, reading the element and never the role, so CRUD's `role=option` rows are not read as a select's."
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 102
    section: "## 1. Test Scope Summary"
    change: "After `aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere` add: and since 2026-10-31-engine-debts `a_click_and_a_space_move_a_native_control_in_the_tree`; a native control writes `toggled` under a checkable role alone, so `pressed` never reads on it (tests/blitz-tests/tests/accessibility_states.rs:386-427)."
    sidecar: "test-scope-summary: `accessibility_states`' native-control test stated."
    rationale: "Report, Deviations 9 and Symbols / APIs F5; Outcome (F5, a11y): `accessibility_states` 6."
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 105
    section: "## 1. Test Scope Summary"
    change: "In `inert_subtree`'s clause add: since 2026-10-31-engine-debts `a_node_made_inert_loses_focus_and_takes_no_key` over five ways of being made inert, and a check that the tree's focus reads the window when the focused node was not built (tests/blitz-tests/tests/inert_subtree.rs:229-310; tests/blitz-tests/tests/inert_subtree.rs:312-357)."
    sidecar: "test-scope-summary: `inert_subtree`'s focus-loss and window-fallback checks stated."
    rationale: "Report, Symbols / APIs F3, F4 and New text by line, `inert_subtree.rs`; Outcome (F3, a11y) and (F4, a11y): `inert_subtree` 6."
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 122
    section: "## 1. Test Scope Summary"
    change: "After the flight type select's clause add: since 2026-10-31-engine-debts Cells' Tab sequence, arrows clamped at the grid's ends and bringing a cell into view, Enter opening the cell's editor and committing by keyboard, Tab leaving an open editor committing it, a click on another cell committing to its own cell, and the load page's sequence with its button taking Enter and Space (tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_keyboard.rs:193-335)."
    sidecar: "test-scope-summary: `stand_keyboard`'s Cells and load page readings stated."
    rationale: "Report, Symbols / APIs F13, Deviations 9 and New text by line, `stand_keyboard.rs`; Outcome (F13, a11y): the keyboard path met."
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: "## 1. Test Scope Summary"
    change: "After `stand_contrast`'s SC 1.4.3 tests add: since 2026-10-31-engine-debts `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`, the `.cell.current` focus outline using the stand accent `#3a5ce5` (tests/blitz-tests/tests/stand_contrast.rs:647-704; examples/seven_guis/src/tasks/cells.rs:551-554)."
    sidecar: "test-scope-summary: `stand_contrast`'s load page and Cells tests stated."
    rationale: "Report, Symbols / APIs F13: the CSS rule, the accent's held value; New text by line, `stand_contrast.rs`; Coverage of new surfaces, `cells-grid`."
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 150
    section: "## 1. Test Scope Summary"
    change: "After `stand_cells_served`'s lint reading add: since 2026-10-31-engine-debts `the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part`, where `press` walks and opens the editor, `type` supplies the characters and `press` commits, the twelve key names holding no character (tests/blitz-tests/tests/stand_cells_served.rs:179-214)."
    sidecar: "test-scope-summary: `stand_cells_served`'s keyboard walk stated."
    rationale: "Report, Spec claims disproved by measurement, the plan's step 16, and Deviations 3: `press` walks and opens, `type` supplies, `press` commits."
    fact: F16
note: "The report states changed sites in this file that no picked entity holds, none of them proposed here: `test-scope-summary.md:24` (`multiple-select`, `optgroup`), `:32` (`flow-class`), `:34` (`open-state`) and `:71` (`unit-counts`); the proposal for `stand_non_lean_tasks`' Cells count rests on a hit the report's `cells-676` bullet leaves out."
