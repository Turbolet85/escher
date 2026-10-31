proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "In this line, restate `(the chosen option's value, nothing with none chosen)` as `(the chosen option's value, a `multiple` select's first chosen in tree order, nothing with none chosen)`, and after `each with its select's client rect as its `bounds`` add `, a `multiple` select's rows and groups reading their own since 2026-10-31-engine-debts`; the closed select's options keep their select's client rect."
    sidecar: "a11y-plan §7 sr-tree: the snapshot's reading of a `multiple` select restated, `value` its first chosen option in tree order, its rows and groups their own client rects (2026-10-31-engine-debts F6)."
    rationale: "Report Changes, F6: `select_value` now reads a `multiple` select's first chosen option in tree order and `snapshot.rs` gives a `multiple` select's rows and groups their own client rects; `multiple-select` lists screen-reader-support.md:11 as changed, while `lint-overlap` holds the closed select's reading there unchanged."
    basis: tests/blitz-tests/tests/stand_act_select.rs:311-352
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "New bullet after this line: - Since 2026-10-31-engine-debts a `multiple` select's node states `set_multiselectable`, and on `stand_act_select`'s `multiple` fixture, in both layout modes, it reads its name, its role and its rows, each row with its own bounds; an `optgroup` reads `Role::Group`, named by its non-empty `label` attribute, the label named in the tree and unpainted, and a select's options are its `option` children and its `optgroup` children's, in tree order; what an assistive technology reads of them is `unrunnable-here` (tests/blitz-tests/tests/stand_act_select.rs:311-352; tests/blitz-tests/tests/select_interaction.rs:857-894) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "a11y-plan §7 sr-tree: a `multiple` select's `set_multiselectable` and rows, and `optgroup` as `Role::Group` named by its `label`, added (2026-10-31-engine-debts F6, F7)."
    rationale: "Report Changes, F6 and F7, and Coverage of new surfaces: the `multiple` select's rows and `optgroup` are new interactive surface whose tree reading (`ListBox`, `set_multiselectable`, `Role::Group`, AT `unrunnable-here`) this category does not yet state."
    basis: tests/blitz-tests/tests/stand_act_select.rs:311-352
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 31
    section: "## 7. Screen Reader Support"
    change: "New bullet after this line: - Since 2026-10-31-engine-debts the tree states a native checkbox's or radio's checked state: on a checkable role `toggled` reads `True` or `False` from `checkbox_input_checked()`, neither author attribute read on a native control; under any other role a native control writes nothing, so `pressed` never begins to read on it; the snapshot's `checked` still reads the native state from the element, and what an assistive technology reads of it stays unmeasured (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "a11y-plan §7 sr-snapshot: native `toggled` on checkable roles added, `pressed` unmoved on a native control and the snapshot's `checked` unmoved (2026-10-31-engine-debts F5)."
    rationale: "Report Changes, F5, and Expected amendments, a11y-plan §7 with native `toggled`; `native-toggled` keeps the snapshot's own read unchanged, so the tree's new state is added detail and no reader moves."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 38
    section: "## 7. Screen Reader Support"
    change: "In this line, restate `the elements that react to a click and state no interactive role` as `the elements that react to input and state no interactive role`, and `Cells 676` in that count as `Cells 677 (the 676 data cells and, since 2026-10-31-engine-debts, `cells-grid`, an `onkeydown` and no role)`; the pin's name, `on the 676 data cells` and the history `until then it read 2 · 3 · 676` stand."
    sidecar: "a11y-plan §7 sr-actionable-keys: Cells' reacting-element count 676 to 677, `cells-grid` joining the data cells (2026-10-31-engine-debts F13, pattern `cells-676`)."
    rationale: "Report Spec claims disproved, pattern `cells-676`: measured 677, the count being every element with the listener mark that reads `GenericContainer`, and `cells-grid` gains an `onkeydown` and no role; Counts names screen-reader-support.md:38 under both `cells-676` and `cells-grid`; Deviations 1 has the pin's doc say react to input."
    basis: tests/blitz-tests/tests/stand_non_lean_tasks.rs:102-103
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 39
    section: "## 7. Screen Reader Support"
    change: "In this line, restate `and no Tab-order, keyboard-activation or contrast row (§5)` as `and, since 2026-10-31-engine-debts, its Tab sequence (`LOAD_TAB_ORDER`) and its keyboard-activation and contrast rows in `stand_keyboard` and `stand_contrast` (§5)`."
    sidecar: "a11y-plan §7 sr-actionable-keys: the load page's missing Tab-order, keyboard and contrast rows retired, the rows now standing (2026-10-31-engine-debts F12)."
    rationale: "Report Coverage of new surfaces, Load page rows: tests `stand_keyboard`, `stand_contrast`, `stand_load_page`, SC 2.1.1 and 1.4.3 met; Outcome F12 closes the gap; New text lists `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space` and `the_load_page_meets_its_ratios`, so this line's no-row claim no longer stands."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:157-177
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 39
    section: "## 7. Screen Reader Support"
    change: "New bullet after this line: - Since 2026-10-31-engine-debts Cells' `cells-grid` is one Tab stop, `CELLS_TAB_ORDER` stating its sequence and its keys in §5: it carries `id`, `tabindex` `0`, `aria-label` `Cells` and an `onkeydown` and states no role, so it reads `GenericContainer` named `Cells`; its role and cell semantics are owned by a `.andromeda/residuals.md` line, and a person at a window is `unrunnable-here` (tests/blitz-tests/tests/common/mod.rs:93-98; tests/blitz-tests/tests/stand_keyboard.rs:193-200; tests/blitz-tests/tests/stand_non_lean_tasks.rs:102-103) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "a11y-plan §7 sr-actionable-keys: `cells-grid`, a new focusable element, stated with its name, no role and the owner of its role and cell semantics (2026-10-31-engine-debts F13)."
    rationale: "Report Changes, F13, and Coverage of new surfaces: `cells-grid` is a new Tab stop with SC 2.1.1, 1.4.3 and 2.4.3 asserted, and its role and cell semantics out by a residual line; this category states no such element."
    basis: tests/blitz-tests/tests/common/mod.rs:93-98
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 52
    section: "## 7. Screen Reader Support"
    change: "In this line, restate `and the focused DOM node is reported as tree focus` as `and the focused DOM node is reported as tree focus where it is among the built nodes, else `u64::MAX`, the window, since 2026-10-31-engine-debts, so the tree never names a node it did not build`."
    sidecar: "a11y-plan §7 sr-tree: tree focus gains its window fallback (2026-10-31-engine-debts F4, pattern `tree-focus`)."
    rationale: "Report Changes, F4: `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`; `tree-focus` names screen-reader-support.md:52 as changed, gaining the window fallback."
    basis: packages/blitz-dom/src/accessibility.rs:104-110
note: D-a11y-obs-schema reads no drift, the report moving no log site and no a11y violation schema being defined; the body's a11y-plan.md:142 states the OPEN gap that line 39's edit closes and lies outside this file.
