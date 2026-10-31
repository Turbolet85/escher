proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 39
    section: "## Input Validation"
    change: "New entity after this one, in its group's shape — - **Boundary:** A `multiple` select's chosen set | - **What to validate:** Where the set a `multiple` select holds leaves the engine | - **How (stack-specific):** On every document the set leaves only as form entries, as `input` and `change` values (`evt.values()`, one `(name, FormValue::Text(value))` per chosen option, packages/dioxus-native-dom/src/dioxus_document.rs:31-45), as the snapshot's `value`, which reads `select_value`, the first chosen option in tree order, and as the options' tree nodes; no verb, argument, key name, result field, cause, session error or exit is added — built on the founder's word, 2026-10-31, «Построить сейчас» — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: Input Validation gains where a `multiple` select's chosen set leaves the engine, beside the Driver session socket row."
    rationale: "The report's Symbols / APIs F6 names `multiple_select_values` filling `values` on `Input` and `Change` and `value` staying `select_value`; its Coverage of new surfaces names the four exits. The row's carriers and control values hold, and no standing text states the set's exits."
    basis: packages/dioxus-native-dom/src/dioxus_document.rs:31-45
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 60
    section: "## Input Validation"
    change: "New entity after this one, in its group's shape — - **Boundary:** `type` on a `multiple` select (escher-driver) | - **What to validate:** A `type` call whose target is a `multiple` select | - **How (stack-specific):** The call passes `validate` like every `type`; since 2026-10-31-engine-debts the executor's `focus_multiple_select` focuses the select with no click, which would land on the row at the centre, and clears the typeahead through `BaseDocument::clear_select_typeahead`, and a text opening on a space types nothing, since no label can match and the Space would flip a row (packages/escher-driver/src/execute.rs:557-563; packages/escher-driver/src/execute.rs:235-239); no verb, argument, key name, result field, cause, session error or exit is added; witnessed by `type_on_a_multiple_select_chooses_its_match_alone_with_no_stray_click` (tests/blitz-tests/tests/stand_act_select.rs:410-446) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: Input Validation gains `type` on a `multiple` select, beside the Driver command schema row."
    rationale: "The report's Symbols / APIs (driver and stand), F6, and Deviations 6c state the focus step, the cleared typeahead and the text opening on a space. The schema row's verb table and refusal order hold, and it states none of this."
    basis: packages/escher-driver/src/execute.rs:557-563
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 186
    section: "## Input Validation"
    change: "Replace `no test asserts `:open`, while `details_element` and the a11y leg read green` with: since 2026-10-31-engine-debts blitz-tests' `open_state` asserts it in its 4 tests, in both layout modes: read from the attribute at construction, moved by a write and a removal through the mutator, and flipped by a summary's click and its Enter (tests/blitz-tests/tests/open_state.rs:54-73; tests/blitz-tests/tests/open_state.rs:75-104; tests/blitz-tests/tests/open_state.rs:106-123; tests/blitz-tests/tests/open_state.rs:125-143), as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: the `open` row's no-test statement ends; `open_state` (4 tests) asserts the presence read and `:open` matching."
    rationale: "The report's Counts / qualifiers moved, Gaps CARRY 2 closes, lists `no-assert-gap` as changed at `input-validation.md:186`; its Expected amendments carry the `open` row's `no test asserts` as retired; its Outcome reads CARRY 2 met, `open_state` 4."
    basis: .andromeda/security-plan-detail/input-validation.md:186
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 200
    section: "## Input Validation"
    change: "In the third content class, read `on each element node whose markup or native checked state states them`, and after `from `aria-pressed` on every other` add: since 2026-10-31-engine-debts a native checkable writes `toggled` from `checkbox_input_checked()` on a checkable role and nothing under any other role, neither author attribute read on it, so `pressed` never begins to read on one, on the founder's «Писать на checkable-ролях (Рекомендую)», 2026-10-31"
    sidecar: "2026-10-31-engine-debts: the `id` row's control-states class names a native checkable's `toggled`, read from its checked state."
    rationale: "The report's Symbols / APIs F5 has `build_accessibility_node` write `Toggled` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)` and read neither author attribute on a native control, so the row's markup-only account of `toggled` no longer covers every node. The values stay closed, so the crossing is unchanged."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 213
    section: "## Input Validation"
    change: "New entity after this one, in its group's shape — - **Boundary:** Markup attributes | - **What to validate:** `label` (`optgroup`) | - **How (stack-specific):** `optgroup` is read by a closed element-name match, on every document: `options` walks a select's `option` children and its groups' in tree order, a group takes `Role::Group`, and its non-empty `label` is its name in the tree, unpainted, inside the ratified name crossing (the `aria-label` · `<label for>` row) — built on the founder's word, 2026-10-31, «Построить сейчас» — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: Input Validation gains an `optgroup`'s `label` as a name in the tree, inside the name crossing."
    rationale: "The report's Symbols / APIs F7 names the walk, `Role::Group` and the non-empty `label` as the group's name, unpainted; its Coverage of new surfaces reads the label as a name inside the ratified name crossing. The accessible-names row holds that crossing and no standing text names this attribute."
    fact: F4
note: "No proposal of this group is a drift of D-security-input, D-security-auth or D-security-deps: each new input surface the report validates shows its closed match or `get_node` check, it marks `change` and the load page's rows validation n/a, no auth or secret moved and no crate was added. input-validation#1.59's edit shares its fact with the control-states row, whose `Toggled` statement (F8) is not in this group, and F1, F2, F3, F7, F8, F9, F10 and F11 fit no entity picked here."
