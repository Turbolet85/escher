proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "Three clause edits, the stand's readings left as they are: «keeps its `option` children in the tree» reads «keeps its `option` children, and those under an `optgroup`, in the tree»; «(the chosen option's value, nothing with none chosen)» reads «(the chosen option's value, a `multiple` select's first chosen option in tree order, nothing with none chosen)»; «and its options as its children, each with its select's client rect as its `bounds`» reads «and a closed select's options as its options, each with its select's client rect as its `bounds`»."
    sidecar: "screen-reader-support: the select's tree and snapshot line restated for a `multiple` select and `optgroup`, a closed select's options keeping its client rect (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved, `multiple-select` changes `screen-reader-support.md:11` and `lint-overlap` keeps its closed select's client rect there; Symbols / APIs F6 (`select_value` reads a `multiple` select's first chosen option in tree order; `snapshot.rs` rows read their own rects) and F7 (`options` and `is_select_option` hold under a group)."
    basis: .andromeda/a11y-plan-detail/screen-reader-support.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "New line after it: - A `multiple` select's rows and groups read their own client rects as their `bounds` in the snapshot, unlike a closed select's options, in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:311-352) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "screen-reader-support: a `multiple` select's rows and groups read their own client rects (2026-10-31-engine-debts)."
    rationale: "Report Symbols / APIs F6, `snapshot.rs`: a `multiple` select's rows and groups read their own client rects; New text lists `a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds`."
    basis: tests/blitz-tests/tests/stand_act_select.rs:311
    fact: F21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "New line after it: - `evt.values()` reads a `multiple` select's set through `multiple_select_values`, one `(name, FormValue::Text(value))` form entry per chosen option on `Input` and `Change`, while `value` stays `select_value` (packages/dioxus-native-dom/src/dioxus_document.rs:31-45) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "screen-reader-support: `evt.values()` reads a `multiple` select's set, `value` staying `select_value` (2026-10-31-engine-debts)."
    rationale: "Report Symbols / APIs F6, `dioxus_document.rs`: `multiple_select_values` fills `values` on `Input` and `Change`, one entry per chosen option, so `evt.values()` reads the set; `value` stays `select_value`."
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:31
    fact: F22
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 38
    section: "## 7. Screen Reader Support"
    change: "«and the elements that react to a click and state no interactive role are counted» reads «and the elements that react to input and state no interactive role are counted»; «Circle Drawer 1, Cells 676» reads «Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid`»; the test's name, «until then it read 2 · 3 · 676» and «on the 676 data cells» stand."
    sidecar: "screen-reader-support: Cells' reacting-element count 676 to 677, `cells-grid`'s `onkeydown` joining the data cells (2026-10-31-engine-debts)."
    rationale: "Report Spec claims disproved by measurement, Cells' count: measured 677, `cells-grid` takes an `onkeydown` and no role and joins the 676 data cells; Counts `cells-676` and `cells-grid` both change `screen-reader-support.md:38`; Deviations 1: the pin's doc now says react to input."
    basis: .andromeda/a11y-plan-detail/screen-reader-support.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 38
    section: "## 7. Screen Reader Support"
    change: "New line after it: - Cells' `cells-grid` takes `aria_label` `Cells` and no role; role and cell semantics stay out by `.andromeda/residuals.md` line 24, and what a person reads at a window is `unrunnable-here` — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "screen-reader-support: `cells-grid` named `Cells` with no role, cell semantics left to the residual line (2026-10-31-engine-debts)."
    rationale: "Report Symbols / APIs F13: `cells-grid` takes `aria_label` Cells with no role; Coverage of new surfaces, `cells-grid`: role and cell semantics out by residual line 24, a person at a window `unrunnable-here`."
    fact: F23
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 39
    section: "## 7. Screen Reader Support"
    change: "«out of the tree and the snapshot while it is hidden, and no Tab-order, keyboard-activation or contrast row (§5)» reads «out of the tree and the snapshot while it is hidden; since 2026-10-31-engine-debts it holds Tab-order, keyboard-activation and contrast rows (§5), in both layout modes: `stand_keyboard::the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` walks `LOAD_TAB_ORDER` and takes Enter and Space on `load-fetch`, and `stand_contrast::the_load_page_meets_its_ratios` reads its ratios»."
    sidecar: "screen-reader-support: the load page's missing Tab-order, keyboard-activation and contrast rows now stand, the gap closed (2026-10-31-engine-debts)."
    rationale: "Report Coverage of new surfaces, Load page rows: tests `stand_keyboard`, `stand_contrast`, `stand_load_page`, SC 2.1.1 and 1.4.3 met; Symbols / APIs F13: `common/mod.rs` states `LOAD_TAB_ORDER`; Outcome (F12, a11y): rows met; `load-rows` changes the same gap in §5. New text lists both test functions."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:157
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 52
    section: "## 7. Screen Reader Support"
    change: "«and the focused DOM node is reported as tree focus» reads «and the focused DOM node is reported as tree focus: since 2026-10-31-engine-debts `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build»."
    sidecar: "screen-reader-support: the tree's focus gains the window fallback (2026-10-31-engine-debts)."
    rationale: "Report Symbols / APIs F4 (`accessibility.rs:104-110`, `:122`); Counts `tree-focus`: changed `screen-reader-support.md:52`, gaining the window fallback."
    basis: packages/blitz-dom/src/accessibility.rs:104
    fact: F20
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "«the list's ten words pinned by `test_stand_repeat_leg_runs_three_rounds`» reads «the list's fifteen words since 2026-10-31-engine-debts, ten until then, pinned by `test_stand_repeat_leg_runs_three_rounds`»."
    sidecar: "a11y ci-integration: the stand-repeat list's words ten to fifteen (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: `stand-repeat`'s named targets ten to fifteen (`ci-leg.sh:58-59`, `test_ci_workflows.py:365-367`); Harness / gate surface: `test_stand_repeat_leg_runs_three_rounds` names them too. The line's «ten words» form is outside `ten-named`'s listed sites, the same moved fact."
    basis: .github/scripts/ci-leg.sh:58
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "«14 files and 85 tests (`stand_keyboard` 15), over the tier-0 witnesses alone, 9 files and 59 tests» reads «14 files and 85 tests (`stand_keyboard` 15) at that word, 14 files and 112 tests (`stand_keyboard` 21) since 2026-10-31-engine-debts, its fourteen files unmoved, over the tier-0 witnesses alone, 9 files and 59 tests at that word»."
    sidecar: "a11y ci-integration: the leg's tally 14 · 85 to 14 · 112, `stand_keyboard` 15 to 21, fourteen files unmoved (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: `a11y-tally` changes `a11y-plan-detail/ci-integration.md:10`, 14 · 85 to 14 · 112, fourteen files unmoved; `n-keyboard` changes the same line, `stand_keyboard` 15 to 21."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:10
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "«Its log reads the six unmoved (43)» reads «Its log read the six unmoved (43)», that CI#40134371519 reading standing as history, and after «(6 · 43 on CI#40125511019), every other job's tally unmoved;» joins «since 2026-10-31-engine-debts it reads the six with `stand_keyboard` 21 and `stand_contrast` 7 (52) and accessibility_states 6 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 6 · select_interaction 18 · stand_act_select 10 (60): 14 result lines · 112 passed · 0 failed · 0 ignored, locally and on the job of CI#41192433140;»."
    sidecar: "a11y ci-integration: the leg's per-file log re-read at 14 · 112, `accessibility_states` 6, `inert_subtree` 6, `select_interaction` 18, `stand_act_select` 10 (2026-10-31-engine-debts)."
    rationale: "Report Counts / qualifiers moved: `n-inert` 4 to 6, `n-a11y-states` 5 to 6, `n-select` 7 to 18 and `n-act-select` 5 to 10 each change `a11y-plan-detail/ci-integration.md:11`; `n-keyboard` 21 and `n-contrast` 7; Outcome: `lines 14 passed 112 failed 0` and CI#41192433140 Accessibility `14 · 112 · 0 · 0`. The per-file figures sum to 112."
    basis: .andromeda/a11y-plan-detail/ci-integration.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: "A sentence closing the line: Since 2026-10-31-engine-debts `accessibility_states` holds a row moving a native control in the tree by a click and a Space, read through `&dyn Document`, and `inert_subtree` asserts the focus fixup and the tree's window fallback; what an assistive technology reads stays unmeasured (tests/blitz-tests/tests/accessibility_states.rs:386-427; tests/blitz-tests/tests/inert_subtree.rs:229-310) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md."
    sidecar: "a11y ci-integration: the leg's `accessibility_states` and `inert_subtree` rows for native `toggled`, the fixup and the window fallback (2026-10-31-engine-debts)."
    rationale: "Report Outcome (F3, a11y), (F4, a11y), (F5, a11y): `inert_subtree` 6 and `accessibility_states` 6, what an assistive technology reads unmeasured; Coverage of new surfaces, the focus fixup · tree focus · native `toggled`: tests through `&dyn Document`; Deviations 9 names `a_click_and_a_space_move_a_native_control_in_the_tree`."
    basis: tests/blitz-tests/tests/accessibility_states.rs:386
    fact: F27
note: "No D-a11y-surface or D-a11y-obs-schema drift in this group (the `multiple` select and `cells-grid` carry the report's coverage, no schema moved). Body §9's «eight of them» at `a11y-plan.md:237` is left unproposed: the report names stand-repeat's five new targets nowhere, so that count is unverified and the orchestrator should read `ci-leg.sh:58-59` against it. `.andromeda/residuals.md:23`'s 676 is P5's."
