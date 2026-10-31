proposals:
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Keyboard handling"
    change: "A focused single-choice `select` that is no disabled form control takes ArrowDown / ArrowUp to the next / previous enabled option, Home / End to the first / last, and a character with no Ctrl, Alt or Meta as typeahead kept for 1 s of the document's clock, its list running through `optgroup` children in tree order; every other key does nothing, Enter and Space opening nothing; each change dispatches one `input` then one `change`, a key that changes nothing none. A focused `multiple` select takes ArrowDown, ArrowUp, Home and End alone to choose that row alone, with Shift to span from the anchor, with Ctrl or Meta to move the current row and change nothing; a Space with no typed text standing flips the current row and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen and Ctrl+A is not modelled. No popup is painted."
    sidecar: "§5 Keyboard handling: `multiple` select and `optgroup` keys are stated; the not-modelled limit is retired, no popup is painted staying true (2026-10-31-engine-debts F6, F7, F8)."
    rationale: "Symbols / APIs F6 and F7 build the `multiple` select's keys and `optgroup`; Counts names `a11y-plan.md:123` under `multiple-select` and `optgroup`; Expected amendments names `multiple` and groups' keys. The body still says neither is modelled, so a new interactive element reads as uncovered."
    basis: "packages/blitz-dom/src/events/keyboard.rs:213-244"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Focus restoration"
    change: "After the click on a `select` that focuses it, add: a click on an `option` row of a `multiple` select focuses the select and clears its typeahead and text selection, then, plain, chooses the row alone, with Ctrl or Meta flips it and with Shift spans, a disabled select stopping it and a disabled row changing nothing; an option of a closed single select has no click arm. Keep: a click on any other element is unchanged, a CRUD row or the list still clearing focus."
    sidecar: "§5 Focus restoration: `handle_click`'s new `option` arm on a `multiple` select's rows is stated; any other element stays unchanged (2026-10-31-engine-debts F6, F8)."
    rationale: "Symbols / APIs F6: `click_select_row` behind a new `option` arm of `handle_click`; F8: an option of a closed single select has no click arm. The body's claim that a click on any other element is unchanged no longer holds for these rows."
    basis: "packages/blitz-dom/src/events/pointer.rs:1013-1054"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Per-surface keyboard shortcuts"
    change: "Add beside the CRUD bullet: seven_guis Cells: `cells-grid`, named `Cells` by `aria_label` with no role, is one Tab stop; arrows move `current`, clamped, and bring the cell into view; Enter opens the editor on `current`, focused, and the editor's Enter commits and returns focus to the grid; Tab leaving an open editor commits it; `.cells-grid:focus .cell.current` draws a 2px `#3a5ce5` outline; role and cell semantics stay out, owned by a `.andromeda/residuals.md` line."
    sidecar: "§5 Per-surface keyboard shortcuts: Cells' `cells-grid` Tab stop, keys and current-cell outline are stated (2026-10-31-engine-debts F13)."
    rationale: "Symbols / APIs F13 adds the Tab stop and its keys; Coverage of new surfaces records SC 2.1.1, 1.4.3 and 2.4.3 asserted with role and cell semantics out by a residual line; Expected amendments names Cells' row. The §5 body states no Cells keys."
    basis: "examples/seven_guis/src/tasks/cells.rs:551-554"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Test harness pattern"
    change: "Replace the OPEN bullet with a closed statement: the load page and Cells hold Tab-order, keyboard-activation and contrast rows — `stand_keyboard` walks `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, the load page's `load-fetch` Button takes Enter and Space, and `stand_contrast` asserts both pages' ratios — so SC 2.1.1 and 1.4.3 are asserted on them; the owner clause naming a `.andromeda/residuals.md` line goes."
    sidecar: "§5 Test harness pattern: the OPEN gap on the served pages' keyboard and contrast coverage is closed for the load page and Cells (2026-10-31-engine-debts F12, F13)."
    rationale: "Outcome (F12, a11y): The OPEN gap closes is `a11y-plan.md:142`, this wrap's P2 amendment; Counts names it under `load-rows` and `no-assert-gap`; Coverage of new surfaces records Load page rows at SC 2.1.1 and 1.4.3."
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:157-177"
  - detector: D-a11y-surface
    severity: warning
    section: "§5 Keyboard Navigation → Test harness pattern"
    change: "After the lean tasks' walk, add: and it walks the load page's `LOAD_TAB_ORDER` and Cells' `CELLS_TAB_ORDER` the same way, so the two orders named for the lean tasks are no longer the only stated orders; every lean-task control lies in one of those two."
    sidecar: "§5 Test harness pattern: `stand_keyboard`'s stated orders gain `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` (2026-10-31-engine-debts F12, F13)."
    rationale: "Symbols / APIs F13: `common/mod.rs` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`; New text lists `stand_keyboard`'s load-page and Cells walks. The body's every stand control lying in one of the two orders restates the gap the OPEN bullet retires."
    basis: "tests/blitz-tests/tests/common/mod.rs:93-98"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§3 A11y Assertion Harness Contract → Keyboard test harness"
    change: "`stand_keyboard` drives Tab, Shift+Tab, Enter, Space and the CRUD list's keys headlessly on the four lean tasks and, since 2026-10-31-engine-debts, walks the load page's sequence with its button taking Enter and Space and Cells' sequence with its arrows, Enter and committing edits, through the same `press` / `press_with`, reading the focused node back per step."
    sidecar: "§3 Keyboard test harness: `stand_keyboard`'s reach grows from the four lean tasks to the load page and Cells (2026-10-31-engine-debts F12, F13)."
    rationale: "Counts: `stand_keyboard` 15 → 21; New text lists its load-page and Cells tests. The body limits the check to the four lean tasks."
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:157-335"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§6 Visual Design Verification → Color contrast"
    change: "`stand_contrast` asserts three tables and, since 2026-10-31-engine-debts, the load page's and Cells' ratios (`the_load_page_meets_its_ratios`, `cells_meets_its_ratios`, the latter over 5 rows) in both layout modes, a row exempt exactly where the snapshot's `enabled` reads `Some(false)`, so SC 1.4.3 is met on every measured non-exempt stand row; a `multiple` select's engine-default chosen-row pair `#c8c8c8` on `#000000` text reads 12.56:1, measured by `select_interaction`."
    sidecar: "§6 Color contrast: the load page's and Cells' ratios and the engine-default chosen-row pair join the measured set (2026-10-31-engine-debts F6, F12, F13)."
    rationale: "Counts: `stand_contrast` 5 → 7; New text lists the two new tests and Cells' 5-row table; Schema / config states the chosen-row pair at 12.56:1 measured by `select_interaction`. The body's three tables no longer bound what the check asserts."
    basis: "tests/blitz-tests/tests/stand_contrast.rs:647-704"
  - detector: D-a11y-surface
    severity: warning
    section: "§3 A11y Assertion Harness Contract → Contrast verification harness"
    change: "After the three tables `stand_contrast` states, add: and, since 2026-10-31-engine-debts, it reads the load page's and Cells' ratios through the same reader in both layout modes (§6)."
    sidecar: "§3 Contrast verification harness: `stand_contrast` beyond its three tables, the load page and Cells (2026-10-31-engine-debts F12, F13)."
    rationale: "The same claim as §6's three tables, restated in §3; Coverage of new surfaces: tokens read through the shared `read`, no hex copied; New text lists the two new tests."
    basis: "tests/blitz-tests/tests/stand_contrast.rs:647-704"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§7 Screen Reader Support → Accessibility tree output"
    change: "Add a bullet: a `multiple` select carries `set_multiselectable` in the tree and its rows read their own client rects in the snapshot, its `value` staying `select_value`, its first chosen option in tree order; an `optgroup` reads `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted; what an assistive technology reads of them is not measured."
    sidecar: "§7 Accessibility tree output: the tree and snapshot reading of a `multiple` select and an `optgroup` are stated (2026-10-31-engine-debts F6, F7)."
    rationale: "Symbols / APIs F6: `accessibility.rs` `set_multiselectable`, `snapshot.rs` rows' rects and `value`; F7: `optgroup` → `Role::Group`; Coverage of new surfaces: `ListBox`, `set_multiselectable`, `Role::Group`, label unpainted, AT `unrunnable-here`. The §7 body states none of this for the new elements."
    basis: "packages/blitz-dom/src/accessibility.rs:517"
  - detector: D-a11y-surface
    severity: warning
    section: "§2 A11y Strategy → Role derivation"
    change: "Replace nothing else writes `toggled`, so the tree never states a native checkbox or radio checked, and its owner clause, with: a native checkbox or radio under a checkable role writes `toggled` true or false from its own checked state, reading neither author attribute, and under any other role writes nothing, so `pressed` never begins to read on it; the windowed AT witness stays owned by the `.andromeda/residuals.md` line that reads `re-carried:0.2.0`, an active assistive technology this version does not reach."
    sidecar: "§2 Role derivation: native `toggled` is written on checkable roles; the never-states limit and its next-target owner clause are retired (2026-10-31-engine-debts F5)."
    rationale: "Symbols / APIs F5: `build_accessibility_node` writes `Toggled` from `checkbox_input_checked()` under `is_native_checkable && is_checkable(role)`; Counts: `native-toggled` 9 changed; Coverage of new surfaces lists native `toggled` with AT `unrunnable-here`; Expected amendments names native `toggled`."
    basis: "packages/blitz-dom/src/accessibility.rs:199-224"
  - detector: D-a11y-surface
    severity: warning
    section: "§3 A11y Assertion Harness Contract → A11y testing tool pick"
    change: "`accessibility_states` reads 6 tests, not 5: the sixth, `a_click_and_a_space_move_a_native_control_in_the_tree`, moves a native control by a click and a Space and reads it in the tree, in both layout modes."
    sidecar: "§3 A11y testing tool pick: `accessibility_states` 5 → 6, its sixth test on native controls (2026-10-31-engine-debts F5)."
    rationale: "Counts: `accessibility_states` 5 → 6; Outcome (F5, a11y): met, `accessibility_states` 6; Deviations 9 names the row. The report warns that a 'N tests' form escapes `n-a11y-states`, the detectors' sweeps being the control, and this body sentence holds that form."
    basis: "tests/blitz-tests/tests/accessibility_states.rs:386-427"
    dependent-of: D-a11y-surface
  - detector: D-retired
    severity: warning
    section: "§5 Keyboard Navigation → Focus order per layout"
    change: "Replace HTML's focus fixup is not built, so a node focused as it becomes inert keeps focus until focus moves, and its owner clause, with: a node holding focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up as the mutator flushes, through `clear_focus`, no DOM `blur` or `focusout` firing."
    sidecar: "§5 Focus order: the focus-fixup limit and its next-target owner clause are retired; the fixup fires no `blur` (2026-10-31-engine-debts F3)."
    rationale: "Retired: `inert_subtree::removing_inert_restores_focus_hit_and_tree`'s focus-fixup limit lines; Symbols / APIs F3: `DocumentMutator::flush` calls `clear_focus`; Counts: `focus-fixup` 8 changed including `a11y-plan.md:107`."
    basis: "packages/blitz-dom/src/mutator.rs:1051-1063"
  - detector: D-retired
    severity: warning
    section: "§3 A11y Assertion Harness Contract → A11y testing tool pick"
    change: "`inert_subtree` reads (6), not (4), and its list adds: a node made inert losing focus and taking no key, and the tree's focus falling back to the window where the focused node was not built."
    sidecar: "§3 A11y testing tool pick: `inert_subtree` 4 → 6, asserting the fixup and the tree's window fallback (2026-10-31-engine-debts F3, F4)."
    rationale: "Counts: `inert_subtree` 4 → 6 with `n-inert` naming `a11y-plan.md:55`; Outcome (F3, a11y) and (F4, a11y): met, `inert_subtree` 6; F4 writes `focus` only where `focus_node_id` is among the built nodes, else the window."
    basis: "tests/blitz-tests/tests/inert_subtree.rs:229-310"
    dependent-of: D-retired
note: "D-a11y-obs-schema holds: the report changes neither the a11y violation schema nor the obs log schema (no log site added). The report's P2 escalation on F13's unmet pointer-editor clause concerns the matrix criterion's text, and no a11y-plan body claim states it, so it is the orchestrator's and no proposal here."
