proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 222
    section: "## Input Validation"
    change: "Replace «and its native checked state reaches no tree node either — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next) —» with «and since 2026-10-31-engine-debts its native checked state reaches the tree node under a checkable role and under no other (the native-checkable row below) —», the rest of the sentence standing."
    sidecar: "input-validation control-states row: a native checkable's checked state now reaches the tree under a checkable role; the measured absence and its residual owner end."
    rationale: "Report Changes, Symbols / APIs, fork F5 «Писать на checkable-ролях (Рекомендую)»: `build_accessibility_node` writes `Toggled` from `checkbox_input_checked()` on a checkable role; the `native-toggled` sweep and Expected amendments name this line."
    basis: .andromeda/security-plan-detail/input-validation.md:222
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 223
    section: "## Input Validation"
    change: "Replace «each attribute is read once and nothing else writes `toggled`» with «each attribute is read once and nothing else but a native checkable's own state, under a checkable role, writes `toggled`»."
    sidecar: "input-validation control-states row: a native checkable's own state is the one other writer of `toggled`."
    rationale: "Report Changes, Symbols / APIs, fork F5: `is_native_checkable && is_checkable(role)` writes `Toggled::True` or `False`, a second writer of the state this sentence says has one."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 224
    section: "## Input Validation"
    change: "New row after the control-states row: - **Boundary:** Markup attributes · **What to validate:** a native checkbox's or radio's checked state in the tree · **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`; under any other role a native control writes nothing, so `pressed` never begins to read on it, and neither author attribute is read on a native control; a closed value carrying no attribute text, with no log site, print, env read or panic path on the added lines; pinned by `accessibility_states` 6, what an assistive technology reads unmeasured — an engine change for every document, ratified by the founder (2026-10-31, «Писать на checkable-ролях (Рекомендую)», relayed verbatim by the operator) (packages/blitz-dom/src/accessibility.rs:199-224)"
    sidecar: "input-validation: new row for a native checkable's `Toggled`, a closed value on a checkable role only."
    rationale: "Report Changes, Symbols / APIs, fork F5 and Coverage of new surfaces, «The focus fixup · tree focus · native `toggled`»; the task's F8."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 227
    section: "## Input Validation"
    change: "Replace «a known limit: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next)» with «since 2026-10-31-engine-debts a node that holds focus when it comes under `inert` gives it up: `DocumentMutator::flush`, where `focus_node_id` resolves to a node reading `is_inert()`, calls `BaseDocument::clear_focus`, no `inert` arm joining the attribute writes, so a subtree moved under an inert box is covered too, and `disabled` and `hidden` read as before — an engine change for every document, ratified by the founder (2026-10-31, «Снимать фокус без blur (Рекомендую)», relayed verbatim by the operator) (packages/blitz-dom/src/mutator.rs:1051-1063)»."
    sidecar: "input-validation `inert` row: the focus-fixup limit ends; `flush` clears focus on an inert node."
    rationale: "Report Changes, Symbols / APIs, fork F3; the `focus-fixup` sweep and Expected amendments name this line."
    basis: .andromeda/security-plan-detail/input-validation.md:227
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 227
    section: "## Input Validation"
    change: "New row after the `inert` row: - **Boundary:** Markup attributes · **What to validate:** the focus an inert subtree loses, and the tree's `focus` · **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: `build_accessibility_tree` writes the tree's `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build; the inert focus clear fires no DOM `blur` or `focusout`; closed values with no log site added, pinned by `inert_subtree` 6, what an assistive technology reads unmeasured — an engine change for every document, ratified by the founder (2026-10-31, «Откат на окно (Рекомендую)» and «Снимать фокус без blur (Рекомендую)», relayed verbatim by the operator) (packages/blitz-dom/src/accessibility.rs:104-110; packages/blitz-dom/src/accessibility.rs:122; packages/blitz-dom/src/mutator.rs:1051-1063)"
    sidecar: "input-validation: new row for the tree's `focus` falling back to the window and the inert clear firing no event."
    rationale: "Report Changes, Symbols / APIs, forks F3 and F4 and Coverage of new surfaces; the task's F7."
    basis: packages/blitz-dom/src/accessibility.rs:104-110
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 236
    section: "## Input Validation"
    change: "Replace «a click on a `select` focusing it (the `select` row) and a click on any other element unchanged» with «a click on a `select` focusing it (the `select` row), and a click on any other element unchanged at that chunk; since 2026-10-31-engine-debts `handle_click` also holds an `option` arm for a `multiple` select's row (the `multiple` keys-and-clicks row) and its checkbox and radio arms send `change` (the checkbox-and-radio row)»."
    sidecar: "input-validation keyboard-activation row: `handle_click`'s `option` arm and its `change` on a checkbox and a radio named."
    rationale: "Report Changes, Symbols / APIs: fork F6 adds `click_select_row` behind a new `option` arm of `handle_click`, fork F10 moves the checkbox and radio arms, so «any other element unchanged» now reads true only at its own chunk; a released Space reaches them through this row's Click, `checkable_change_event`."
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 236
    section: "## Input Validation"
    change: "New row after the keyboard-activation row: - **Boundary:** Markup attributes · **What to validate:** the engine's `change` on a checkbox and a radio · **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: `handle_click`'s checkbox and radio arms send, after `input`, a `DomEventData::Change` with the same value, the value class `input` already carries — a checkbox always, a radio only where it was not checked before the click; `blitz-vibey-script`'s runtime loses its `change` synthesis and `target_is_checkbox_or_radio`, so a page's script hears one `change`, from the engine; pinned by `checkable_change_event` 3 (a click and a Space on a checkbox each fire `input` then `change`), `text_range_no_change_event` 2 and `blitz-vibey-script`'s `radio_click_fires_change_only_where_it_was_not_checked` — an engine change for every document, ratified by the founder (2026-10-31, «Шлёт движок (Рекомендую)», relayed verbatim by the operator) (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836)"
    sidecar: "input-validation: new row for the engine-sent `change` on a checkbox and a radio; the runtime's synthesis retired."
    rationale: "Report Changes, Symbols / APIs, fork F10, Retired and Coverage of new surfaces; the task's F9."
    basis: packages/blitz-dom/src/events/pointer.rs:827-836
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 239
    section: "## Input Validation"
    change: "Replace «read the `option` children in tree order (an `optgroup`'s not listed)» with «read the `option` children and those of its `optgroup` children, in tree order (the `optgroup` row)»."
    sidecar: "input-validation `select` row: the option list now runs through a group."
    rationale: "Report Changes, Symbols / APIs, fork F7: `options` walks a select's `option` children and its `optgroup` children's, in tree order; the `optgroup` sweep names this line."
    basis: .andromeda/security-plan-detail/input-validation.md:239
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 239
    section: "## Input Validation"
    change: "After «the last chosen, else the first option not disabled» add «; a `multiple` write or removal records the select, so removal settles it to one, and `set_option_selectedness` leaves a `multiple` select's other options alone (packages/blitz-dom/src/mutator.rs:1262-1271)»."
    sidecar: "input-validation `select` row: what a `multiple` write or removal does at the flush."
    rationale: "Report Changes, Symbols / APIs, fork F6 `mutator.rs`: this row states the settling rule the attribute's removal now reaches; the falsy-boolean row's own claim stands."
    basis: packages/blitz-dom/src/mutator.rs:1262-1271
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 240
    section: "## Input Validation"
    change: "Replace «stated limits: a `multiple` select and an `optgroup`'s options are not modelled, no popup is painted and an option is not clicked (refused `covered`)» with «stated limits: no popup is painted, Enter and Space open nothing on a single select, an option of a closed single select has no click arm and stays refused `covered`, under a group too, and `size` is not read»."
    sidecar: "input-validation `select` row: `multiple` and `optgroup` leave the limits; the option-click refusal narrows to a closed single select."
    rationale: "Report Changes, Symbols / APIs, forks F6, F7, F8 and Reverted / negative API facts; the `multiple-select`, `optgroup`, `option-click` and `popup` sweeps name this line."
    basis: .andromeda/security-plan-detail/input-validation.md:240
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 240
    section: "## Input Validation"
    change: "Replace «the rules module's 4 unit tests, `select_interaction` 7 and `stand_act_select` 5» with «the rules module's unit tests, `select_interaction` 18 and `stand_act_select` 10»."
    sidecar: "input-validation `select` row: pins read `select_interaction` 18 and `stand_act_select` 10; the rules module's stale test figure leaves."
    rationale: "Report Counts / qualifiers moved, `n-select` and `n-act-select`, both naming this line; New text lists new tests in `node/select.rs` at 438-457, 459-477 and 560-589 and states no total, so the figure 4 is dropped, not restated."
    basis: .andromeda/security-plan-detail/input-validation.md:240
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 240
    section: "## Input Validation"
    change: "Replace «the driver's `type` on a select is the same typeahead» with «the driver's `type` on a single select is the same typeahead (a `multiple` select's, the row below)»."
    sidecar: "input-validation `select` row: `type`'s typeahead reading narrows to a single select."
    rationale: "Report Changes, Symbols / APIs (driver and stand), fork F6: `type` on a `multiple` select focuses with no click and clears the typeahead; the `type-on-select` sweep names this row."
    basis: packages/escher-driver/src/execute.rs:557-563
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "New row after the `select` row: - **Boundary:** Markup attributes · **What to validate:** `select` with `multiple` — its keys and clicks · **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: a closed key match, `press_multiple_select_key`: ArrowDown · ArrowUp · Home · End alone choose that row alone, with Shift span from the anchor (with no cursor yet, from the last chosen row), with Ctrl or Meta move the current row and change nothing; a Space with no typed text standing flips the current row and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; Ctrl+A is not modelled; a click on a row, `click_select_row` behind `handle_click`'s `option` arm: a disabled select stops it, it focuses the select and clears the typeahead and text selection, plain chooses the row alone, Ctrl or Meta flips it, Shift spans, a disabled row changes nothing; disabled rows are never chosen and every choice passes `apply_select_choice`, the one write path, now taking a set; no log site, print, env read or panic path on the added lines; pinned by `select_interaction` 18 and `stand_act_select` 10 — an engine change for every document, ratified by the founder (2026-10-31, «Построить сейчас», relayed verbatim by the operator) (packages/blitz-dom/src/events/keyboard.rs:213-244; packages/blitz-dom/src/events/keyboard.rs:351-380; packages/blitz-dom/src/events/pointer.rs:1013-1054)"
    sidecar: "input-validation: new row for a `multiple` select's closed key match, row click and one write path."
    rationale: "Report Changes, Symbols / APIs, fork F6, Deviations 6b and Coverage of new surfaces; the task's F3."
    basis: packages/blitz-dom/src/events/keyboard.rs:213-244
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "New row after the `select` row: - **Boundary:** Markup attributes · **What to validate:** `optgroup`, its `label` · **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: read by a closed element-name match: `options` walks a select's `option` children and its groups', in tree order, `is_select_option` holding under a group; `owner_select` and `option_disabled`, the one rule `readings` and form submission share (packages/blitz-dom/src/form.rs:281); `maybe_record_node`'s `optgroup` arm records its select; a group takes `Role::Group` and its non-empty `label` is its name in the tree, unpainted, inside the ratified name crossing; pinned by `select_interaction`, `stand_act_select` and `form`'s tests — an engine change for every document, ratified by the founder (2026-10-31, «Построить сейчас», relayed verbatim by the operator) (packages/blitz-dom/src/node/select.rs:125-129; packages/blitz-dom/src/node/select.rs:175-190; packages/blitz-dom/src/mutator.rs:1253-1257)"
    sidecar: "input-validation: new row for `optgroup`, read by a closed element-name match, its label a name in the tree."
    rationale: "Report Changes, Symbols / APIs, fork F7 and Coverage of new surfaces; the task's F4."
    basis: packages/blitz-dom/src/node/select.rs:125-129
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "New row after the `select` row: - **Boundary:** Markup attributes · **What to validate:** a `multiple` select's chosen set, where it leaves · **How (stack-specific):** Since 2026-10-31-engine-debts the set leaves only as form entries, as `input` and `change` values — `multiple_select_values` fills `values` with one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set, `value` staying `select_value` — as the snapshot's `value`, `select_value`'s first chosen option in tree order, and as the options' tree nodes, the select taking `set_multiselectable`; no carrier, verb, field, log field or span added — an engine change for every document, ratified by the founder (2026-10-31, «Построить сейчас», relayed verbatim by the operator) (packages/dioxus-native-dom/src/dioxus_document.rs:31-45)"
    sidecar: "input-validation: new row naming the exits of a `multiple` select's chosen set."
    rationale: "Report Changes, Symbols / APIs, fork F6 `dioxus_document.rs`, `document.rs`, `snapshot.rs`, `accessibility.rs` and Coverage of new surfaces; the task's F5."
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:31-45
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "New row after the `select` row: - **Boundary:** Markup attributes · **What to validate:** the driver's `type` on a `multiple` select · **How (stack-specific):** Since 2026-10-31-engine-debts `focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead; a text opening on a space types nothing, since no label can match and the Space would flip a row; `Target` gains `multiple`, and no verb, argument, key name, result field, cause, session error or exit is added; `type`'s help is fixed text (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239; packages/escher-driver/src/schema.rs:239-240)"
    sidecar: "input-validation: new row for `type` on a `multiple` select, no click and no text opening on a space."
    rationale: "Report Changes, Symbols / APIs (driver and stand), fork F6 and Deviations 6c; the task's F6."
    basis: packages/escher-driver/src/execute.rs:557-563
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "New row after the `select` row: - **Boundary:** Markup attributes · **What to validate:** the public select readers `BaseDocument::select_of` · `select_values` · `clear_select_typeahead`, `dioxus_native_dom::closed_select_of` · `is_multiple_select` · **How (stack-specific):** Since 2026-10-31-engine-debts each reader taking a node goes through `get_node`, a stale id reading none, and `clear_select_typeahead` takes no node; `closed_select_of`'s callers are the snapshot's builder and `lint.rs`, `is_multiple_select`'s `closed_select_of` and `execute.rs` `locate`, and `clear_select_typeahead`'s one non-test caller `execute.rs` `focus_multiple_select`; no type, verb or field is added (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/document.rs:752-755; packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38)"
    sidecar: "input-validation: new row for the five public select readers, a stale id reading none."
    rationale: "Report Coverage of new surfaces, Sole-caller facts and Deviations 4; the task's F10, narrowed on one word: New text lists `clear_select_typeahead(&mut self)`, which takes no id."
    basis: packages/blitz-dom/src/document.rs:736-744
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 257
    section: "## Input Validation"
    change: "New row after the text-extent row: - **Boundary:** Layout values · **What to validate:** a clipping box's visible region (`BaseDocument::visible_region`) · **How (stack-specific):** Since 2026-10-31-engine-debts, on every document, remote pages included: `visible_region` keeps its signature and reads each clipping box through its transforms, by the private `padding_box_in_view` over `transformed_bounds`; with no transform on the chain it reads as before, and a mapped box with a non-finite edge answers `None`; exact under a translate and a scale, under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show; its callers are unchanged, `execute.rs` `in_view` and `crop` and `lint.rs` `clip_region`; no log site, print, env read or panic path on the added lines; pinned by `visible_region_transformed` 6, `stand_act_transformed` 3 and `transformed_client_rect` — an engine change for every document, ratified by the founder (2026-10-31, «Починить (Рекомендую)», relayed verbatim by the operator) (packages/blitz-dom/src/scrolling.rs:790-812; packages/blitz-dom/src/document.rs:2574-2578)"
    sidecar: "input-validation: new Layout values row for `visible_region` through transforms, a non-finite edge answering `None`."
    rationale: "Report Changes, Symbols / APIs, fork F1 and Coverage of new surfaces; Expected amendments: `input-validation.md` carries no `visible_region` row; the task's F2."
    basis: packages/blitz-dom/src/scrolling.rs:790-812
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 260
    section: "## Input Validation"
    change: "New row after the Geometry row: - **Boundary:** Paint values · **What to validate:** a non-invertible transform under a hit test · **How (stack-specific):** Since 2026-10-31-engine-debts, on every document, remote pages included: `Node::hit_inner` answers `None` for the whole subtree of a transform whose determinant is 0 or not finite, so it takes no hit; no log site, print, env read or panic path on the added lines; pinned by `hit_non_invertible_transform` 2 — an engine change for every document, ratified by the founder (2026-10-31, «Починить (Рекомендую)», relayed verbatim by the operator) (packages/blitz-dom/src/node/node.rs:1472-1477)"
    sidecar: "input-validation: new Paint values row, a non-invertible transform takes no hit."
    rationale: "Report Changes, Symbols / APIs, fork F2; Expected amendments, «The Layout and Paint row: `non-invertible` unfired, the fact is new text there»; the task's F1."
    basis: packages/blitz-dom/src/node/node.rs:1472-1477
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 380
    section: "## Input Validation"
    change: "New row after the 7GUIs inputs row: - **Boundary:** Example apps · **What to validate:** 7GUIs Cells, the `cells-grid` keys · **How (stack-specific):** Since 2026-10-31-engine-debts `cells-grid` takes `tabindex` 0 and an `onkeydown` with a closed key match and no role: arrows move `current`, clamped, Enter opens the editor on `current`, focused, and the editor's Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early (examples/seven_guis/src/tasks/cells.rs:425-433), ratified by the founder as to the grid (2026-10-31, «Одна Tab-остановка (Рекомендую)», relayed verbatim by the operator)"
    sidecar: "input-validation: new Example apps row for Cells' `cells-grid` closed key match."
    rationale: "Report Changes, Symbols / APIs (driver and stand), fork F13, Deviations 6a and Coverage of new surfaces; the task's F11."
    basis: examples/seven_guis/src/tasks/cells.rs:425-433
    fact: F11
note: "`open` row at `input-validation.md:186`, a changed site in the report's `no-assert-gap` and Expected amendments, is no picked entity of this group, so check another pass carries it; D-security-input, -auth and -deps fire on nothing here (Coverage reads every new surface validated, no auth, no crate added); the engine-change boundary reading stays P2's by the report's ask."
