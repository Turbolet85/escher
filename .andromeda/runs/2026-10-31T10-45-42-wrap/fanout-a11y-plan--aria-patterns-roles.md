proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 16
    section: "## 4. ARIA Patterns & Roles"
    change: "In the ARIA state attributes row, replace the clause `a native checkbox or radio reading neither` with `a native checkbox or radio reading neither, and writing `toggled` True or False from its own checked state (`checkbox_input_checked()`) under a checkable role and nothing under any other role, so `pressed` never reads on it (ratified by the founder, 2026-10-31), `accessibility_states::a_click_and_a_space_move_a_native_control_in_the_tree` its pin`; the rest of the row stands."
    sidecar: "ARIA state attributes: a native checkbox or radio on a checkable role now writes `toggled` from its checked state; neither author attribute is read on it (2026-10-31-engine-debts, F5)."
    rationale: "The report's Symbols / APIs F5 has `build_accessibility_node` write `Toggled` for `is_native_checkable && is_checkable(role)`, under any other role nothing, neither author attribute read on a native control; the row's silence leaves a reader taking a native control's `toggled` as unwritten, and Deviations 9 names the new test."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 19
    section: "## 4. ARIA Patterns & Roles"
    change: "New row directly after the Select row: `- **Select, `multiple`** — the builder calls `set_multiselectable` on a `multiple` select, its `ListBox` stating that more than one row may be chosen — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`."
    sidecar: "New row: a `multiple` select carries `set_multiselectable` in the tree (2026-10-31-engine-debts, F6)."
    rationale: "The report's Symbols / APIs F6, `accessibility.rs`: `set_multiselectable` on a `multiple` select; Coverage of new surfaces repeats `ListBox`, `set_multiselectable`. The Select row names the role alone."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 19
    section: "## 4. ARIA Patterns & Roles"
    change: "New row after the new `multiple` row: `- **Option group** — `optgroup` is `Role::Group`, its non-empty `label` attribute its name; the label is named in the tree and unpainted — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`."
    sidecar: "New row: `optgroup` maps to `Role::Group`, named by its non-empty `label`, unpainted (2026-10-31-engine-debts, F7)."
    rationale: "The report's Symbols / APIs F7, `accessibility.rs`: `optgroup` → `Role::Group`, its non-empty `label` attribute its name, a group's label named in the tree and unpainted; the catalog holds no group row."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 23
    section: "## 4. ARIA Patterns & Roles"
    change: "In the Hidden content row, replace `an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes` with `an `option` of a `select`, its parent the select or an `optgroup` under it, excepted from `display: none`, which every option of a single-choice select but its chosen one computes, a `multiple` select's rows computing `display: block``; the rest of the row stands."
    sidecar: "Hidden content: the option exception holds under an `optgroup`, and a `multiple` select's rows no longer compute `display: none` (2026-10-31-engine-debts, F6, F7)."
    rationale: "The report's Schema / config adds `select[multiple] > option, select[multiple] > optgroup > option { display: block; ... }`, so `every option but a single-choice select's chosen one` is no longer true; F7 has `is_select_option` hold under a group and F8 keeps an option under a closed select's group readable and refused `covered`, so `whose parent is a select` is too narrow."
    basis: packages/blitz-dom/assets/default.css:161-188
note: "Line 16 binds with the body at a11y-plan.md:35 (`nothing else writes toggled`, which F5 ends) and line 23 with a11y-plan.md:91; D-a11y-surface and D-a11y-obs-schema find no drift in this file, `cells-grid`'s `aria_label` and no role being left to screen-reader-support.md:38 where the report lists it."
