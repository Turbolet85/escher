proposals:
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "Replace the clause `a `multiple` select and an `optgroup`'s options are not modelled (a `multiple` select takes no key) and no popup is painted` with `an `optgroup`'s options are the select's own in tree order, a `multiple` select takes its own keys (the entries below), and no popup is painted`; the rest of the line stands."
    sidecar: "keyboard-handling: the `multiple` and `optgroup` not-modelled limit retires for the built select, no popup is painted staying true."
    rationale: "Report Symbols / APIs, «Построить сейчас»: a `multiple` select is built in the keys, pointer, form, tree, sheet, snapshot and bridge, and `optgroup` with it; Counts lists `keyboard-navigation.md:38` changed under `multiple-select` and `optgroup`; the plan's text denies the new element the keyboard coverage §5 requires."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:38
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 123
    section: "## 5. Keyboard Navigation"
    change: "Replace `a `multiple` select and an `optgroup`'s options are not modelled, and no popup is painted` with `an `optgroup`'s options are the select's own in tree order, a `multiple` select takes its own keys and row clicks, and no popup is painted`."
    sidecar: "§5 Keyboard handling: the broad line drops the `multiple` and `optgroup` limit, no popup is painted staying true."
    rationale: "Report Counts lists `a11y-plan.md:123` changed under `multiple-select` and `optgroup`, and Expected amendments carries a11y-plan §5's `multiple` and groups' keys."
    basis: .andromeda/a11y-plan.md:123
    dependent-of: D-a11y-surface
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- On a focused `multiple` select that is no disabled form control, ArrowDown, ArrowUp, Home and End alone choose that row alone, with Shift span from the anchor, and with Ctrl or Meta move the current row and change nothing — ratified by the founder (2026-10-31, «Построить сейчас») (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:351-380; packages/blitz-dom/src/node/select.rs:317-358; tests/blitz-tests/tests/select_interaction.rs:523-556) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: a `multiple` select's arrow, Home and End keys, with Shift and with Ctrl or Meta."
    rationale: "Report Symbols / APIs, `events/keyboard.rs`: the keys of the `multiple` select the founder's «Построить сейчас» built."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- On a focused `multiple` select, a Space with no typed text standing flips the current row and with text standing joins it, and a character with no Ctrl, Alt or Meta chooses its match alone (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:326-332; tests/blitz-tests/tests/select_interaction.rs:558-588) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: a `multiple` select's Space and typed text."
    rationale: "Report Symbols / APIs, `events/keyboard.rs`: the Space and character rules, with `select_text_stands` reading whether text stands."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- A `multiple` select's disabled rows are never chosen, and Ctrl+A is not modelled (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/node/select.rs:317-358) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: a `multiple` select's disabled rows are never chosen; Ctrl+A is not modelled."
    rationale: "Report Symbols / APIs, `events/keyboard.rs`: Disabled rows are never chosen. Ctrl+A is not modelled."
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- On a `multiple` select, Shift with no cursor yet spans from the last chosen row, so a set the markup seeds is stepped and spanned from its last chosen row (packages/blitz-dom/src/node/select.rs:269-272; tests/blitz-tests/tests/select_interaction.rs:725-743) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: Shift with no cursor spans from the last chosen row."
    rationale: "Report Deviations 6b, beyond the plan's letter, pinned by `a_set_the_markup_seeds_is_stepped_and_spanned_from_its_last_chosen_row`."
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- A click on a `multiple` select's row reaches `click_select_row`, behind `handle_click`'s `option` arm: a disabled select stops the click; otherwise it focuses the select and clears the typeahead and text selection; plain chooses the row alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing (packages/blitz-dom/src/events/pointer.rs:1013-1054; tests/blitz-tests/tests/select_interaction.rs:601-640; tests/blitz-tests/tests/select_interaction.rs:642-664) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: `click_select_row`, the pointer's rule for a `multiple` select's rows."
    rationale: "Report Symbols / APIs, `events/pointer.rs`: placed beside the select's keys, since the row's click and its keys share `apply_row_gesture`'s select model and the pointer rule's own entity is not among this group's picks."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- An `optgroup`'s options are among its select's options in tree order, reachable by keys and text, a disabled group's rows reading as disabled through `option_disabled`, the one rule `readings` and `form.rs:281` share — ratified by the founder (2026-10-31, «Построить сейчас») (packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; tests/blitz-tests/tests/select_interaction.rs:781-855; tests/blitz-tests/tests/select_interaction.rs:857-894) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: a group's options are a select's own, its disabled rows read through `option_disabled`."
    rationale: "Report Symbols / APIs: `options` walks a select's `option` children and its `optgroup` children's in tree order; `option_disabled` is the one rule."
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- Enter and Space open nothing on a single select, and an option of a closed single select has no click arm and stays refused `covered`, under a group too — ratified by the founder (2026-10-31, «Давай нарисовать в отдельном чанке» and «Только строки multiple (Рекомендую)») (tests/blitz-tests/tests/select_interaction.rs:355-380; tests/blitz-tests/tests/stand_act_select.rs:448-457) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: a single select opens on no key; a closed select's option stays refused `covered`, under a group too."
    rationale: "Report Symbols / APIs: No popup is built, and `packages/blitz-paint/` shows no diff; the fork's own words."
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- A `multiple` write or removal records the select, so removal settles it to one, and `set_option_selectedness` leaves a `multiple` select's other options alone (packages/blitz-dom/src/mutator.rs:1262-1271; tests/blitz-tests/tests/select_interaction.rs:666-689) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: a `multiple` write or removal records its select; other options stay under `set_option_selectedness`."
    rationale: "Report Symbols / APIs, `mutator.rs`: `record_select_of_multiple_write` and `set_option_selectedness`."
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 40
    section: "## 5. Keyboard Navigation"
    change: "- After `input`, `handle_click`'s checkbox and radio arms dispatch a `change` with the same value, a checkbox always and a radio only where it was not checked before the click, so a page's script hears one `change`, from the engine — ratified by the founder (2026-10-31, «Шлёт движок (Рекомендую)») (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836; tests/blitz-tests/tests/checkable_change_event.rs:68-98; tests/blitz-tests/tests/checkable_change_event.rs:100-119) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: `handle_click` dispatches `change` after `input` on a checkbox and a radio."
    rationale: "Report Symbols / APIs: the fork «Шлёт движок (Рекомендую)», and `blitz-vibey-script/src/runtime/events.rs` loses its `change` synthesis; placed beside the Space activation that runs through `handle_click`."
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- The driver's `type` on a `multiple` select focuses it with no click, which would land on the row at the centre, and clears the typeahead, and a text opening on a space types nothing; no verb, argument, key name, result field, cause, session error or exit is added (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239; tests/blitz-tests/tests/stand_act_select.rs:410-446) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: the driver's `type` on a `multiple` select."
    rationale: "Report Symbols / APIs (driver and stand), `type` on a `multiple` select, and Deviations 6c; Counts lists `keyboard-navigation.md:38` changed under `type-on-select`."
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: "## 5. Keyboard Navigation"
    change: "- The driver's `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver, a limit its help states and a `.andromeda/residuals.md` line owns — ratified by the founder (2026-10-31, «Ограничение и residual (Рекомендую)») (packages/escher-driver/src/schema.rs:261-263) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-handling: `press` holds no Ctrl, its limit and owner stated."
    rationale: "Report Symbols / APIs, the fixed text at `schema.rs:261-263`; Decisions & corrections, the founder's word on `press`; P5 card row 4 lands the residual line."
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 61
    section: "## 5. Keyboard Navigation"
    change: "- Hazard: a `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` (`events.rs:162`), the task first polled inside `render_immediate`, measured headlessly and the windowed path read and not run, so Cells calls its handles synchronously, each acting at the call — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-framework: the measured `MountedData` hazard in a spawned task, and Cells' synchronous handles."
    rationale: "Report Reverted / negative API facts and Decisions & corrections, a hazard outside this chunk's fixes; `dioxus_document.rs:743`'s async form did not ship."
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 76
    section: "## 5. Keyboard Navigation"
    change: "- Cells: `cells-grid` is one Tab stop, and its arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)` — ratified by the founder (2026-10-31, «Одна Tab-остановка (Рекомендую)») (tests/blitz-tests/tests/stand_keyboard.rs:193-200; tests/blitz-tests/tests/stand_keyboard.rs:293-335) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-shortcuts: `cells-grid`, one Tab stop, and its arrows."
    rationale: "Report Symbols / APIs (driver and stand), `cells.rs`, the fork «Одна Tab-остановка (Рекомендую)»."
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 76
    section: "## 5. Keyboard Navigation"
    change: "- Cells' editor: Enter on the grid opens the editor on `current`, focused; its Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early, so typed text stays (examples/seven_guis/src/tasks/cells.rs:425-433; tests/blitz-tests/tests/stand_keyboard.rs:202-249) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-shortcuts: Cells' editor by keyboard, and the early return on a click inside it."
    rationale: "Report Symbols / APIs, `cells.rs`, and Deviations 6a, the early return the keyboard-opened editor needs to keep its return to the grid."
    fact: F14
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 100
    section: "## 5. Keyboard Navigation"
    change: "Add a continuation line after this entity: ` — since 2026-10-31-engine-debts `stand_keyboard` also walks the load page's and Cells' stated sequences forward and back in both layout modes: `load-fetch` taking Enter and Space, Cells' arrows and Enter opening the moved-to cell's editor and its value committing by keyboard, Tab leaving an open editor and a click on another cell each committing it, and the arrows clamping at the grid's ends and bringing a cell into view (tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_keyboard.rs:193-200; tests/blitz-tests/tests/stand_keyboard.rs:202-249; tests/blitz-tests/tests/stand_keyboard.rs:251-268; tests/blitz-tests/tests/stand_keyboard.rs:270-291; tests/blitz-tests/tests/stand_keyboard.rs:293-335 — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)`."
    sidecar: "keyboard-harness: `stand_keyboard`'s reach grows to the load page and Cells, `cells-grid`'s keyboard coverage stated."
    rationale: "Report Coverage of new surfaces: `cells-grid` Tab stop and keys and Load page rows, each asserted through `stand_keyboard`; New text lists the six new tests, 15 to 21; the entity stated the lean tasks alone."
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 140
    section: "## 5. Keyboard Navigation"
    change: "After the `ENABLED_TAB_ORDER` parenthesis add `and, since 2026-10-31-engine-debts, the load page's and Cells' stated orders (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`)`, and read the closing clause `every lean task's control lying in one of the two lean orders`."
    sidecar: "§5 Test harness pattern: the broad line names the load page's and Cells' walks; its every-control clause reads the lean tasks it measures."
    rationale: "Report Coverage of new surfaces and Symbols / APIs: `common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, and `load-fetch`, a stand control, is in neither lean order."
    dependent-of: D-a11y-surface
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 142
    section: "## 5. Keyboard Navigation"
    change: "Replace the OPEN line with: `- Since 2026-10-31-engine-debts the load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`), so SC 2.1.1 and 1.4.3 are asserted on the load page's `load-fetch` Button and on Cells' `cells-grid`.`; the OPEN mark and its owner clause leave."
    sidecar: "§5 Test harness pattern: the served pages' OPEN gap closes, its owner clause leaving with it."
    rationale: "Report Outcome: «The OPEN gap closes» is `a11y-plan.md:142`, this wrap's P2 amendment; Counts lists `a11y-plan.md:142` changed under `load-rows` and `no-assert-gap`; Coverage of new surfaces asserts SC 2.1.1 and 1.4.3 on both pages."
    basis: .andromeda/a11y-plan.md:142
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 100
    section: "## 5. Keyboard Navigation"
    change: "- `tests/blitz-tests/tests/common/mod.rs` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, the stated Tab sequences of the load page and Cells, beside `tab_order` and `ENABLED_TAB_ORDER` (tests/blitz-tests/tests/common/mod.rs:93-98) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "keyboard-harness: the shared module's two new stated Tab sequences."
    rationale: "Report Symbols / APIs (driver and stand): `common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`."
    fact: F15
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 102
    section: "## 5. Keyboard Navigation"
    change: "Read `select_interaction` (18 tests) and `stand_act_select` (10 tests), and before ` — both in the `a11y` leg` add: `; since 2026-10-31-engine-debts `select_interaction` also drives Enter and Space opening nothing on a single select, a 17-row keys table and a 10-row click table on a `multiple` select, its typed text, a disabled one, removing `multiple`, a seeded set, the chosen-row pair's ratio and both kinds of group, and `stand_act_select` reads a `multiple` select's rows each with its own bounds, chooses one by a click, spans by `press` with Shift, chooses its match by `type` with no stray click, and refuses an option under a closed select's group as `covered` (tests/blitz-tests/tests/select_interaction.rs:355-380; tests/blitz-tests/tests/select_interaction.rs:523-556; tests/blitz-tests/tests/select_interaction.rs:601-640; tests/blitz-tests/tests/stand_act_select.rs:311-352; tests/blitz-tests/tests/stand_act_select.rs:448-457)`."
    sidecar: "keyboard-harness: `select_interaction` 7 to 18 and `stand_act_select` 5 to 10, with what the new rows drive."
    rationale: "Report Counts: `n-select` and `n-act-select` list `keyboard-navigation.md:102` changed; New text lists the eleven and the five new tests."
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:102
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 103
    section: "## 5. Keyboard Navigation"
    change: "Read `stand_keyboard` holding 21 tests, in place of 15; the rest of the line stands."
    sidecar: "keyboard-harness: `stand_keyboard`'s count 15 to 21."
    rationale: "Report Counts: `stand_keyboard` 15 to 21; the entity's form `stand_keyboard` holding 15 tests is not among `n-keyboard`'s listed hits, the kind of miss the report warns its count patterns have."
note: "Not proposed, for want of a pick: the report names `keyboard-navigation.md:8` (`focus-fixup`), `:25` (`option-click`), `:28` (`no-assert-gap`) and `a11y-plan.md:107` changed, the Touches name `a11y-plan.md:113`, so the orchestrator should reach them, and F9 has no picked carrier for the new-detail pass; `a11y-plan.md:53`, `:55`, `:62` are §3's; the report's P2 ask on the pointer-opened editor's unmet criterion is matrix text, carried here only as F14's fact."
