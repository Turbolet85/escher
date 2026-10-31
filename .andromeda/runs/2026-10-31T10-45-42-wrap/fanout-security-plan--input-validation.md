proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 186
    section: "## Input Validation"
    change: "In the `open` row, replace `no test asserts `:open`, while `details_element` and the a11y leg read green` with `since 2026-10-31-engine-debts `open_state`'s 4 tests assert `:open` in both layout modes — at construction from the attribute, on a write and a removal through the mutator, on a `summary`'s click and Enter — and `details_element` and the a11y leg read green (tests/blitz-tests/tests/open_state.rs:54-143)`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the `open` row: the `no test asserts` gap closed by `open_state` 4 (CARRY 2)."
    rationale: "Report, Counts / qualifiers moved, Gaps CARRY 2 closes, `no-assert-gap` names input-validation.md:186 changed; Expected amendments retires the row's `no test asserts`; Outcome (CARRY 2) reads `open_state` 4."
    basis: tests/blitz-tests/tests/open_state.rs:54-143
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 222
    section: "## Input Validation"
    change: "In the control-states row, replace `and its native checked state reaches no tree node either — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next) — the toggle read per role ratified by the founder (2026-10-18)` with `and since 2026-10-31-engine-debts its native checked state writes `toggled` `True` or `False` under a checkable role and nothing under any other, so `pressed` never reads on it — the toggle read per role ratified by the founder (2026-10-18), the native write on 2026-10-31, «Писать на checkable-ролях (Рекомендую)»`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the control-states row: a native checkbox's or radio's checked state now writes `toggled` under a checkable role; the measured absence and its residual owner retired (F5)."
    rationale: "Report, Symbols / APIs F5: `is_native_checkable && is_checkable(role)` writes `Toggled::True` or `False` from `checkbox_input_checked()`, under any other role nothing, neither author attribute read; Expected amendments names `native-toggled` at input-validation.md:222; P5 card row 5 absorbs the residual lines into `v030-04`."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 223
    section: "## Input Validation"
    change: "In the control-states row, replace `each attribute is read once and nothing else writes `toggled`` with `each attribute is read once, and only it and the native state write `toggled``."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the control-states row: `toggled`'s writers now include a native control's checked state (F5)."
    rationale: "Report, Symbols / APIs F5: the native write is a second writer of `toggled`, so `nothing else writes` no longer holds; another occurrence of the claim the F5 change retires."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 200
    section: "## Input Validation"
    change: "In the `id` row's third content class, replace `(`toggled` from `aria-checked` on a checkable role and from `aria-pressed` on every other;` with `(`toggled` from `aria-checked` on a checkable role and from `aria-pressed` on every other, and since 2026-10-31-engine-debts a native checkbox's or radio's from its checked state;`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the `id` row: `toggled`'s sources on the adapter crossing gain a native control's checked state, a closed enum (F5)."
    rationale: "Report, Symbols / APIs F5 and Counts / qualifiers moved `native-toggled`: this parenthetical lists every source of `toggled` on the adapter crossing and now omits one; the values stay closed, `Toggled::True` or `False`, inside the ratified states class."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 227
    section: "## Input Validation"
    change: "In the `inert` row, replace `a known limit: HTML's focus fixup is not built, so a node that holds focus when it becomes inert keeps it until focus moves, its fix owned by a `.andromeda/residuals.md` line (target: next)` with `since 2026-10-31-engine-debts the focus fixup stands: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, a subtree moved under an inert box included, with no DOM `blur` or `focusout` fired and no `inert` arm in the attribute writes, `disabled` and `hidden` reading as before (packages/blitz-dom/src/mutator.rs:1051-1063), and the tree's `focus` is written only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window (packages/blitz-dom/src/accessibility.rs:104-110) — an engine change for every document, ratified by the founder (2026-10-31, «Снимать фокус без blur (Рекомендую)», «Откат на окно (Рекомендую)»), pinned by `inert_subtree` 6, what an assistive technology reads unmeasured`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the `inert` row: the focus-fixup limit ended, focus cleared at the flush with no `blur`, the tree's focus falling back to the window (F3, F4)."
    rationale: "Report, Symbols / APIs F3 and F4; Counts / qualifiers moved, `focus-fixup` names input-validation.md:227 changed; Expected amendments; Outcome (F3, a11y) and (F4, a11y) read `inert_subtree` 6 and state what an assistive technology reads stays unmeasured."
    basis: packages/blitz-dom/src/mutator.rs:1051-1063
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 240
    section: "## Input Validation"
    change: "In the `select` row, replace `stated limits: a `multiple` select and an `optgroup`'s options are not modelled, no popup is painted and an option is not clicked (refused `covered`);` with `stated limits: no popup is painted and a closed select's option is not clicked (refused `covered`, under a group too);`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the `select` row: `multiple` and `optgroup` leave the stated limits; no popup and the closed option's `covered` refusal stay (F6, F7, F8)."
    rationale: "Report, Symbols / APIs F6, F7 and F8 (an option of a closed single select has no click arm and stays refused `covered`, under a group too); Counts / qualifiers moved, `multiple-select`, `optgroup` and `option-click` each name input-validation.md:240 changed; Expected amendments."
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 239
    section: "## Input Validation"
    change: "In the `select` row, replace `read the `option` children in tree order (an `optgroup`'s not listed)` with `read the `option` children and its `optgroup` children's, in tree order`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the `select` row: the rules' option list runs through a group (F7)."
    rationale: "Report, Symbols / APIs F7: `options` walks a select's `option` children and its `optgroup` children's, in tree order; Counts / qualifiers moved, `optgroup` and `type-on-select` name input-validation.md:239 changed; another occurrence of the limit the F7 change retires."
    basis: packages/blitz-dom/src/node/select.rs:125-129
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 240
    section: "## Input Validation"
    change: "In the `select` row, replace `the driver's `type` on a select is the same typeahead` with `the driver's `type` on a single-choice select is the same typeahead`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the `select` row: the driver's `type` sentence narrowed to a single-choice select, a `multiple` one having its own row (F6)."
    rationale: "Report, Symbols / APIs (driver and stand) F6: `type` on a `multiple` select focuses with no click and chooses its match alone; `type-on-select` keeps a single select's reading as no change."
    basis: packages/escher-driver/src/execute.rs:557-563
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 240
    section: "## Input Validation"
    change: "In the `select` row, replace `pinned by the rules module's 4 unit tests, `select_interaction` 7 and `stand_act_select` 5, both layout modes` with `pinned by the rules module's unit tests, `select_interaction` 18 and `stand_act_select` 10, both layout modes`."
    sidecar: "2026-10-31-engine-debts — §Input Validation, the `select` row: `select_interaction` 7 → 18, `stand_act_select` 5 → 10; the rules module's own count dropped, the report stating none."
    rationale: "Report, Counts / qualifiers moved, `n-select` and `n-act-select` name input-validation.md:240 changed; New text, by line adds test functions to node/select.rs and states no new count for that module, so its figure 4 is dropped rather than guessed."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "After the `select` row add one record — **Boundary:** Markup attributes | **What to validate:** `select` with `multiple` — its keys and its one write path | **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: on a focused `multiple` select `press_multiple_select_key` is a closed key match (packages/blitz-dom/src/events/keyboard.rs:213-244): ArrowDown · ArrowUp · Home · End alone choose that row alone, with Shift they span from the anchor (with no cursor yet from the last chosen row), with Ctrl or Meta they move the current row and change nothing; a Space with no typed text standing flips the current row and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone; disabled rows are never chosen, a disabled select takes no key, Ctrl+A is not modelled and `size` is not read; `apply_select_choice` takes the chosen set and stays the one write path (packages/blitz-dom/src/events/keyboard.rs:351-380); a `multiple` write or removal records the select, so removal settles it to one, and `set_option_selectedness` leaves its other options alone (packages/blitz-dom/src/mutator.rs:1262-1271); no print, log site, env read or panic path on the added lines, and no carrier, verb, argument, key name, result field, cause, session error, exit, log field or span added; pinned by `select_interaction` 18 and `stand_act_select` 10, both layout modes — an engine change for every document, ratified by the founder (2026-10-31, «Построить сейчас», inputs#I8, inputs#I9 of that chunk) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new row, a `multiple` select's keys and its one write path (F6)."
    rationale: "Report, Symbols / APIs F6 (`events/keyboard.rs`, `mutator.rs`); Coverage of new surfaces: validation closed key match and one write path; Deviations 6b; Negative API facts: no `size` read."
    basis: packages/blitz-dom/src/events/keyboard.rs:213-244
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "After the `select` row add one record — **Boundary:** Markup attributes | **What to validate:** a click on a `multiple` select's row | **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: `click_select_row`, behind a new `option` arm of `handle_click` (packages/blitz-dom/src/events/pointer.rs:1013-1054): a disabled select stops it; it focuses the select and clears the typeahead and the text selection; plain chooses the row alone, Ctrl or Meta flips it, Shift spans; a disabled row changes nothing; an option of a closed single select has no click arm and stays refused `covered`, under a group too; pinned by `select_interaction` and `stand_act_select`, both layout modes — an engine change for every document, ratified by the founder (2026-10-31, «Построить сейчас», «Только строки multiple (Рекомендую)») — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new row, a click on a `multiple` select's row (F6, F8)."
    rationale: "Report, Symbols / APIs F6 (`events/pointer.rs`) and F8; Coverage of new surfaces names keys and clicks in every document, remote pages included."
    basis: packages/blitz-dom/src/events/pointer.rs:1013-1054
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "After the `select` row add one record — **Boundary:** Markup attributes | **What to validate:** where a `multiple` select's set leaves | **How (stack-specific):** Since 2026-10-31-engine-debts the set leaves only as form submission's entries, less its disabled rows (packages/blitz-dom/src/form.rs:281; packages/blitz-dom/src/form.rs:558-592), the `input` and `change` events — `multiple_select_values` fills `values`, one `(name, FormValue::Text(value))` per chosen option, so `evt.values()` reads the set, and `value` stays `select_value`, a `multiple` select's first chosen option in tree order (packages/dioxus-native-dom/src/dioxus_document.rs:31-45) —, the snapshot's `value`, which reads `select_value`, its rows and groups reading their own client rects, and the options' tree nodes, the select itself `set_multiselectable` — inside the ratified name and control-value crossings, with no carrier added; ratified by the founder (2026-10-31, «Построить сейчас») — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new row, the exits of a `multiple` select's set, all inside the ratified crossings (F6)."
    rationale: "Report, Symbols / APIs F6 (`dioxus_document.rs`, `snapshot.rs`, `accessibility.rs`, `document.rs`); Coverage of new surfaces: the set leaves only as form entries, `input` and `change`, the snapshot's `value`, the options' tree nodes."
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:31-45
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "After the `select` row add one record — **Boundary:** Markup attributes | **What to validate:** `optgroup` — its options, its `disabled`, its `label` | **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included, by a closed element-name match: a select's `options` run through its `optgroup` children in tree order, `is_select_option` holding under a group, with `owner_select` (packages/blitz-dom/src/node/select.rs:125-129) and `option_disabled` (packages/blitz-dom/src/node/select.rs:175-190) the one rule the readings and form submission share (packages/blitz-dom/src/form.rs:281), a disabled group's rows never chosen; `maybe_record_node`'s `optgroup` arm records its select (packages/blitz-dom/src/mutator.rs:1253-1257); in the tree an `optgroup` is `Role::Group`, its non-empty `label` attribute its name, inside the ratified name crossing, the label named in the tree and unpainted; no print, log site, env read or panic path on the added lines; pinned by `select_interaction`, `stand_act_select` and `form.rs`' two tests, both layout modes for the checks — an engine change for every document, ratified by the founder (2026-10-31, «Построить сейчас») — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new row, `optgroup`'s options, `disabled` and `label` (F7)."
    rationale: "Report, Symbols / APIs F7; Coverage of new surfaces: validation closed element-name match, a group's `label` a name inside the ratified name crossing; Expected amendments names `optgroup` at this file."
    basis: packages/blitz-dom/src/node/select.rs:175-190
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 241
    section: "## Input Validation"
    change: "After the `select` row add one record — **Boundary:** Markup attributes | **What to validate:** a `NodeId` handed to the public select readers — `BaseDocument::select_values` · `select_of` · `clear_select_typeahead`, `dioxus_native_dom::closed_select_of` · `is_multiple_select` | **How (stack-specific):** Since 2026-10-31-engine-debts, each in process and through `get_node`, a stale id reading none (packages/blitz-dom/src/document.rs:736-744; packages/blitz-dom/src/document.rs:746-750; packages/blitz-dom/src/document.rs:752-755; packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38); `closed_select_of` has two callers, the snapshot's builder and `lint.rs`, `is_multiple_select` has `closed_select_of` and `execute.rs` `locate`, and `clear_select_typeahead`'s one non-test caller is `execute.rs` `focus_multiple_select`; no type, verb or field added and no `NodeId` crosses — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new row, the public select readers, a stale id reading none (F6, F7, Deviations 4)."
    rationale: "Report, Coverage of new surfaces: validation `get_node`, a stale id reads none; Symbols / APIs, Sole-caller facts; Deviations 4: the three public items add no type, verb or field."
    basis: packages/dioxus-native-dom/src/snapshot.rs:242-252
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 60
    section: "## Input Validation"
    change: "After the Driver command schema row add one record — **Boundary:** Driver command schema (escher-driver) | **What to validate:** `type` and `press` on a `multiple` select | **How (stack-specific):** Since 2026-10-31-engine-debts the call passes `validate` as before; `Target` gains `multiple`, and `focus_multiple_select` focuses with no click, which would land on the row at the centre, and clears the typeahead (packages/escher-driver/src/execute.rs:557-563); a text opening on a space types nothing, no label being able to match and the Space flipping a row (packages/escher-driver/src/execute.rs:235-239); `press` holds no Ctrl, so a set of rows that are not adjacent cannot be built through the driver, its help saying so in fixed `'static` text (packages/escher-driver/src/schema.rs:261-263), a limit owned by a `.andromeda/residuals.md` line; no verb, argument, key name, result field, cause, session error or exit added; pinned by `stand_act_select` 10, both layout modes — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new row, `type` and `press` on a `multiple` select, the closed sets unmoved (F6)."
    rationale: "Report, Symbols / APIs (driver and stand) F6 and Fixed `'static` text moved; Deviations 4 and 6c; the P5 card's row 4 and the founder's «Ограничение и residual (Рекомендую)» own the `press` limit as a residual line."
    basis: packages/escher-driver/src/execute.rs:557-563
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 257
    section: "## Input Validation"
    change: "After the text-extent row add one record — **Boundary:** Layout values | **What to validate:** A clipping box through its transforms (`BaseDocument::visible_region`) | **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: `visible_region` keeps its signature and reads each clipping box through its transforms, by the private `padding_box_in_view` (packages/blitz-dom/src/scrolling.rs:790-812) over `transformed_bounds`, now `pub(crate)` (packages/blitz-dom/src/document.rs:2574-2578); with no transform on the chain it reads as before, a mapped box with a non-finite edge answers `None`, a translate and a scale read exactly, and under a rotation it reads the rotated padding box's bounding box, so it holds corners the box does not show, `off-screen`'s fixed text saying so; no print, log site, env read or panic path on the added lines; pinned by `visible_region_transformed` 6, `a_box_mapped_to_a_point_leaves_nothing_visible_inside_it` among them, and `stand_act_transformed` 3, both layout modes — an engine change for every document, ratified by the founder (2026-10-31, «Починить (Рекомендую)») — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new Layout values row, `visible_region` through transforms, a non-finite edge reading `None` (F1)."
    rationale: "Report, Symbols / APIs F1; Coverage of new surfaces: validation a non-finite edge reads `None`; Expected amendments: the Layout and Paint row is new text here, this file carrying no `visible_region` row."
    basis: packages/blitz-dom/src/scrolling.rs:790-812
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 257
    section: "## Input Validation"
    change: "After the text-extent row add one record — **Boundary:** Layout values | **What to validate:** A hit under a non-invertible transform (`Node::hit_inner`) | **How (stack-specific):** Since 2026-10-31-engine-debts, in every document, remote pages included: a transform whose determinant is 0 or not finite answers `None` for the whole subtree (packages/blitz-dom/src/node/node.rs:1472-1477), an invertible twin hit where it shows; no print, log site, env read or panic path on the added lines; pinned by `hit_non_invertible_transform` 2, both layout modes (tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71) — an engine change for every document, ratified by the founder (2026-10-31, «Починить (Рекомендую)») — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new Layout values row, a zero or non-finite determinant takes no hit (F2)."
    rationale: "Report, Symbols / APIs F2; Coverage of new surfaces: a zero or non-finite determinant no hit; Counts / qualifiers moved: `non-invertible`'s control unfired, so no master states F2 and the fact is new text; Expected amendments."
    basis: packages/blitz-dom/src/node/node.rs:1472-1477
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 380
    section: "## Input Validation"
    change: "After the 7GUIs inputs row add one record — **Boundary:** Example apps | **What to validate:** Cells' grid keys | **How (stack-specific):** Since 2026-10-31-engine-debts `cells-grid` takes one Tab stop and an `onkeydown`, with no role, a closed key match: arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`; Enter opens the editor on `current`, focused, and its Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early, so typed text stays (examples/seven_guis/src/tasks/cells.rs:425-433); pinned by `stand_keyboard` 21 and `stand_cells_served` 5, both layout modes — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts — §Input Validation: new Example apps row, Cells' grid keys, a closed key match (F13)."
    rationale: "Report, Symbols / APIs F13; Coverage of new surfaces: `cells-grid` validation closed key match; Deviations 6a; Counts / qualifiers moved: `stand_keyboard` 15 → 21, `stand_cells_served` 4 → 5."
    basis: examples/seven_guis/src/tasks/cells.rs:425-433
note: "D-security-input finds no unvalidated boundary, each new surface's mechanism standing in the report's Coverage of new surfaces, D-security-auth and D-security-deps hit nothing in this file, and no Retired item stands here; every proposal is a doc correction or new detail, and line 240's rules-module figure 4 is dropped because the report states no new count for it."
