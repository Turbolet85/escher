proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 8
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨a known limit stands: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves⟩ with ⟨HTML's focus fixup is built since 2026-10-31-engine-debts, in every document: at `DocumentMutator::flush`, where `focus_node_id` resolves to a node reading `is_inert()`, `BaseDocument::clear_focus` runs, no DOM `blur` or `focusout` fires and no `inert` arm joins the attribute writes, so a subtree moved under an inert box is covered too, `disabled` and `hidden` reading as before — ratified by the founder (2026-10-31, «Снимать фокус без blur (Рекомендую)», by dialog in the overseer session, relayed verbatim by the operator) (packages/blitz-dom/src/mutator.rs:1051-1063; tests/blitz-tests/tests/inert_subtree.rs:229-310)⟩, and drop ⟨, its fix owned by a `.andromeda/residuals.md` line (target: next)⟩."
    sidecar: "keyboard-focus-order: HTML's focus fixup stated as built, with no DOM blur; the known limit and its residual owner leave the record."
    rationale: "Report Retired names inert_subtree's focus-fixup limit lines and Symbols / APIs F3 builds the fixup; Counts / qualifiers moved lists focus-fixup as changed at keyboard-navigation.md:8."
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨a `multiple` select and an `optgroup`'s options are not modelled (a `multiple` select takes no key) and no popup is painted⟩ with ⟨since 2026-10-31-engine-debts an `optgroup`'s options are in the select's list, its `option` children and its `optgroup` children's in tree order, and these keys and typeahead choose them, a disabled group's rows skipped by `option_disabled`, the one rule `readings` and `form.rs` share — ratified by the founder (2026-10-31, «Построить сейчас», by dialog in the overseer session, relayed verbatim by the operator) (packages/blitz-dom/src/node/select.rs:175-190; tests/blitz-tests/tests/select_interaction.rs:781-855); a `multiple` select takes the keys of the next record; no popup is painted (tests/blitz-tests/tests/select_interaction.rs:355-380)⟩."
    sidecar: "keyboard-handling: a single select's keys and typeahead reach an optgroup's options; a multiple select no longer reads as taking no key."
    rationale: "Report Symbols / APIs F6, F7 and F8 build the multiple select and optgroup and keep the popup out; Counts / qualifiers moved lists multiple-select and optgroup as changed at keyboard-navigation.md:38."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:38
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 25
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨while an option is not clicked (a driver `click` on one is refused `covered` — on the stand the flight booker's two options among them)⟩ with ⟨while an option of a closed single select is not clicked (a driver `click` on one is refused `covered`, under a group too — on the stand the flight booker's two options among them)⟩."
    sidecar: "keyboard-focus-restoration: the unclicked option narrowed to a closed single select's option, under a group too."
    rationale: "Report Symbols / APIs F8 keeps a closed single select's option refused covered and F6 gives a multiple select's rows a click; Counts / qualifiers moved narrows option-click at keyboard-navigation.md:25."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:25
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 25
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under keyboard-focus-restoration: - A click on a row of a `multiple` select runs `click_select_row`, behind an `option` arm of `handle_click`, since 2026-10-31-engine-debts, in every document: a disabled select stops it; it focuses the select and clears the typeahead and the text selection; plain chooses the row alone, Ctrl or Meta flips it, Shift spans; a disabled row changes nothing; through the driver a `click` on a row is accepted and chooses it — ratified by the founder (2026-10-31, «Построить сейчас», by dialog in the overseer session, relayed verbatim by the operator) (packages/blitz-dom/src/events/pointer.rs:1013-1054; tests/blitz-tests/tests/select_interaction.rs:601-640; tests/blitz-tests/tests/select_interaction.rs:642-664; tests/blitz-tests/tests/stand_act_select.rs:354-379) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-focus-restoration: new record, a multiple select row's click and its focus."
    rationale: "Report Symbols / APIs F6 events/pointer.rs and Coverage of new surfaces add a clickable row the file does not cover."
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 28
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨no test asserts `:open`⟩ with ⟨`open_state` (4 tests) asserts `:open` in both layout modes: read from the attribute at construction, moved by a write and a removal through the mutator, flipped by a summary's click and by its Enter (tests/blitz-tests/tests/open_state.rs:54-73; tests/blitz-tests/tests/open_state.rs:75-104; tests/blitz-tests/tests/open_state.rs:106-123; tests/blitz-tests/tests/open_state.rs:125-143)⟩."
    sidecar: "keyboard-focus-restoration: the no-test gap on :open closed by open_state."
    rationale: "Report Counts / qualifiers moved, Gaps CARRY 2 closes, lists no-assert-gap and open-state as changed at keyboard-navigation.md:28; New targets holds open_state 4."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 29
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨dispatch `input` and move focus to the control⟩ with ⟨dispatch `input`, then, since 2026-10-31-engine-debts, in every document, a `change` with the same value — a checkbox always, a radio only where it was not checked before the click, so a page's script hears one `change`, from the engine — and move focus to the control, a click and a Space on a checkbox each reading so — ratified by the founder (2026-10-31, «Шлёт движок (Рекомендую)», by dialog in the overseer session, relayed verbatim by the operator) (tests/blitz-tests/tests/checkable_change_event.rs:68-98; tests/blitz-tests/tests/checkable_change_event.rs:100-119)⟩."
    sidecar: "keyboard-focus-restoration: a checkbox's and a radio's click states the engine's change after input."
    rationale: "Report Symbols / APIs F10 adds DomEventData::Change to handle_click's checkbox and radio arms; the record still names input alone."
    basis: packages/blitz-dom/src/events/pointer.rs:827-836
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under keyboard-handling: - A focused `multiple` `select` that is no disabled form control takes keys of its own, since 2026-10-31-engine-debts, in every document: ArrowDown, ArrowUp, Home and End alone choose that row alone; with Shift they span from the anchor, with no cursor yet from the last chosen row; with Ctrl or Meta they move the current row and change nothing; a Space with no typed text standing flips the current row, and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen; Ctrl+A is not modelled and `size` is unread, so every row is listed; each change goes through `apply_select_choice`, the one write path, `evt.values()` on `input` and `change` reading the set; removing `multiple` settles the set to one option — ratified by the founder (2026-10-31, «Построить сейчас», by dialog in the overseer session, relayed verbatim by the operator) (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:334-349; packages/blitz-dom/src/events/keyboard.rs:351-380; packages/blitz-dom/src/node/select.rs:317-358; packages/dioxus-native-dom/src/dioxus_document.rs:31-45; docs/capabilities.md:208-211; tests/blitz-tests/tests/select_interaction.rs:523-556; tests/blitz-tests/tests/select_interaction.rs:558-588; tests/blitz-tests/tests/select_interaction.rs:642-664; tests/blitz-tests/tests/select_interaction.rs:666-689; tests/blitz-tests/tests/select_interaction.rs:725-743) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: new record, a multiple select's keys, SC 2.1.1."
    rationale: "Report Symbols / APIs F6 events/keyboard.rs and Deviations 6b add a keyboard surface; Coverage of new surfaces reads SC 2.1.1 on it, and the file's keyboard handling does not cover it."
    basis: packages/blitz-dom/src/events/keyboard.rs:213-244
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under keyboard-handling: - The driver's `type` on a `multiple` select focuses it with no click, which would land on the row at the centre, and clears the typeahead (`Target` gains `multiple`, `focus_multiple_select`), so it chooses its match alone; a text opening on a space types nothing, no label matching and the Space flipping a row; no verb, argument, key name, result field, cause, session error or exit is added (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239; tests/blitz-tests/tests/stand_act_select.rs:410-446) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: new record, type on a multiple select through the driver."
    rationale: "Report Symbols / APIs (driver and stand) F6 and Deviations 6c; Counts / qualifiers moved lists type-on-select as changed at keyboard-navigation.md:38."
    basis: packages/escher-driver/src/execute.rs:557-563
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 61
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under keyboard-framework: - A hazard for a focus or scroll call on a mounted handle, measured since 2026-10-31-engine-debts: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics `RefCell already borrowed`; called synchronously from the handler the native handle acts at the call, Cells' form; measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines read and not run; any app meets it — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-framework: new record, the MountedData hazard from a task spawned by a key handler."
    rationale: "Report Decisions & corrections states the hazard, Reverted / negative API facts the form that did not ship; Counts / qualifiers moved reads mounted-hazard as a new fact beside its 2 unmoved hits."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:61
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 76
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under keyboard-shortcuts: - Cells, since 2026-10-31-engine-debts: `cells-grid` is one Tab stop, taking an `id`, `tabindex` 0, the `aria_label` `Cells` and an `onkeydown`, with no role; focused, the arrows move `current`, clamped at the grid's ends, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a Tab leaving an open editor commits it and a click on another cell commits it to its own cell; a click inside a cell's own open editor returns early, so typed text stays where it was reset to the stored text before; `.cells-grid:focus .cell.current` draws the current cell's outline, 2px solid `#3a5ce5`; SC 2.1.1, 1.4.3 and 2.4.3 asserted, its role and cell semantics out by a `.andromeda/residuals.md` line, a person at a window `unrunnable-here` — ratified by the founder (2026-10-31, «Одна Tab-остановка (Рекомендую)», by dialog in the overseer session, relayed verbatim by the operator) (examples/seven_guis/src/tasks/cells.rs:362-393; examples/seven_guis/src/tasks/cells.rs:425-433; examples/seven_guis/src/tasks/cells.rs:551-554) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-shortcuts: new record, Cells' grid Tab stop, its keys and current-cell outline."
    rationale: "Report Symbols / APIs F13, Deviations 6a and Coverage of new surfaces add cells-grid as a new interactive element; the file's Cells record names only an edit's commit."
    basis: examples/seven_guis/src/tasks/cells.rs:551-554
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 100
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under the keyboard-harness procedure: - Since 2026-10-31-engine-debts `stand_keyboard` walks the two served pages' stated sequences, `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` in the shared module, forward and back in both layout modes: the load page's button takes Enter and Space; Cells' arrows and Enter open the moved-to cell and its value commits by keyboard, a Tab leaving an open editor and a click on another cell commit it, and the arrows clamp at the grid's ends and bring a cell into view; through the driver `stand_cells_served` walks to a cell by keyboard and commits a value (tests/blitz-tests/tests/common/mod.rs:93-98; tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_keyboard.rs:193-200; tests/blitz-tests/tests/stand_keyboard.rs:202-249; tests/blitz-tests/tests/stand_keyboard.rs:251-268; tests/blitz-tests/tests/stand_keyboard.rs:270-291; tests/blitz-tests/tests/stand_keyboard.rs:293-335; tests/blitz-tests/tests/stand_cells_served.rs:179-214) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-harness: new record, the load page's and Cells' Tab sequences and keys asserted."
    rationale: "Report Symbols / APIs F13, Coverage of new surfaces and Outcome F12 and F13 assert keyboard coverage on Cells and the load page; the harness records name the lean tasks' walks alone."
    basis: tests/blitz-tests/tests/common/mod.rs:93-98
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 101
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under the keyboard-harness procedure: - `text_range_no_change_event` (2 tests) pins, in both layout modes, typing in a text input and a range's key and pointerdown firing `input` and no `change` (tests/blitz-tests/tests/text_range_no_change_event.rs:59-76; tests/blitz-tests/tests/text_range_no_change_event.rs:78-97) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-harness: new record, text_range_no_change_event."
    rationale: "Report Changes New checks and Counts / qualifiers moved New targets add text_range_no_change_event 2, pinning the input-alone reading the range record states."
    basis: tests/blitz-tests/tests/text_range_no_change_event.rs:78-97
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 102
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨`select_interaction` (7 tests)⟩ with ⟨`select_interaction` (18 tests)⟩."
    sidecar: "keyboard-harness: select_interaction 7 to 18 tests."
    rationale: "Report Counts / qualifiers moved, n-select, select_interaction 7 to 18, changed at keyboard-navigation.md:102."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:102
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 102
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨`stand_act_select` (5 tests)⟩ with ⟨`stand_act_select` (10 tests)⟩."
    sidecar: "keyboard-harness: stand_act_select 5 to 10 tests."
    rationale: "Report Counts / qualifiers moved, n-act-select, stand_act_select 5 to 10, changed at keyboard-navigation.md:102."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:102
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 102
    section: "## 5. Keyboard Navigation"
    change: "New record after this line, under the keyboard-harness procedure: - Since 2026-10-31-engine-debts `select_interaction` also drives a `multiple` select's 17-row keys table and 10-row clicks table, typed text, a disabled select, `multiple` removed, a seeded set and a group's options, and Enter and Space opening nothing on a single select; through the driver `stand_act_select` reads a `multiple` select's name, role and rows each with its own bounds, a click on a row, a press with Shift spanning, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered`, each in both layout modes (tests/blitz-tests/tests/select_interaction.rs:355-380; tests/blitz-tests/tests/select_interaction.rs:478-521; tests/blitz-tests/tests/select_interaction.rs:523-556; tests/blitz-tests/tests/select_interaction.rs:601-640; tests/blitz-tests/tests/select_interaction.rs:781-855; tests/blitz-tests/tests/stand_act_select.rs:311-352; tests/blitz-tests/tests/stand_act_select.rs:354-379; tests/blitz-tests/tests/stand_act_select.rs:381-408; tests/blitz-tests/tests/stand_act_select.rs:410-446; tests/blitz-tests/tests/stand_act_select.rs:448-457) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-harness: new record, the multiple select's and groups' rows in select_interaction and stand_act_select."
    rationale: "Report Outcome F6 and F7 and Coverage of new surfaces name these checks as the new select surfaces' keyboard coverage; the harness record states the single-choice tables alone."
    basis: tests/blitz-tests/tests/select_interaction.rs:478-521
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 103
    section: "## 5. Keyboard Navigation"
    change: "In this record replace ⟨`stand_keyboard` holding 15 tests⟩ with ⟨`stand_keyboard` holding 21 tests⟩."
    sidecar: "keyboard-harness: stand_keyboard 15 to 21 tests, a form the n-keyboard pattern does not reach."
    rationale: "Report Counts / qualifiers moved, stand_keyboard 15 to 21; n-keyboard's listed sites omit this holding-N-tests form, and the report leaves detectors' own sweeps as the control."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:103
note: "D-a11y-obs-schema reads no violation (report: no log site or schema shape moved); the report's a11y-plan.md:55, :107, :123, :142 are body sites outside this file, and F13's unmet pointer-editor clause is the P2 escalation the report asks of the orchestrator, this file stating the behaviour only."
