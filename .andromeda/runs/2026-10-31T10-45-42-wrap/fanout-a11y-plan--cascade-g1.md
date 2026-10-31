proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 19
    section: "## 1. A11y Scope Summary"
    change: "Replace «neither is a `LeanTask`, so neither carries a Tab-order, keyboard-activation or contrast row — the load page's an open gap (§5) — and their a11y reach is their own checks: keyed, `unkeyed_actionable` empty, the load page's lint 0» with «neither is a `LeanTask`; since 2026-10-31-engine-debts each holds a stated Tab sequence (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) that `stand_keyboard` walks, with keyboard rows of its own (the load page's `load-fetch` taking Enter and Space, Cells' arrows and Enter on `cells-grid`) and contrast rows in `stand_contrast`; their own checks read them keyed, `unkeyed_actionable` empty, the load page's lint 0»."
    sidecar: "Served pages: Cells and the load page now hold Tab sequences, keyboard rows and contrast rows; the load page's open gap closed."
    rationale: "Report Coverage of new surfaces, rows `cells-grid` and Load page rows (tests `stand_keyboard`, `stand_contrast`, a11y SC 2.1.1 and 1.4.3), and F13's `common/mod.rs:93-98` contradict the entity; report New text lists `the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space`, `cells_walks_its_stated_sequence`, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 23
    section: "## 1. A11y Scope Summary"
    change: "Replace «the named limits (the focus fixup not built, a `multiple` select and an `optgroup`'s options not modelled, `aria-activedescendant` unread, the native checked state never stated in the tree)» with «the named limits (the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread, `aria-activedescendant` unread)»."
    sidecar: "Capability statement: its named limits restated after the focus fixup, the `multiple` select, `optgroup` and native `toggled` were built."
    rationale: "Report Counts names `a11y-scope-summary.md:23` changed under `focus-fixup`, `multiple-select` and `optgroup`, and F5 ends the native checked limit (`native-toggled`); report New text lists `docs/capabilities.md` 194-195 «The focus fixup fires no `blur`», 196-198 «No popup is painted, and Enter and Space open nothing on a single select» and 208-211 «`size` is unread»."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 20
    section: "## 2. A11y Strategy"
    change: "Replace «each attribute is read once and nothing else writes `toggled`; the builder writes no `toggled` from a native checkbox's or radio's own checked state, so the tree the shell hands the adapter never states it checked while the snapshot's `checked` reads the element — a measured absence, its fix owned by a `.andromeda/residuals.md` line (target: next);» with «each attribute is read once, and the one other write of `toggled` is a native checkbox's or radio's own checked state on a checkable role, the next bullet;»."
    sidecar: "Role derivation: the native `toggled` absence and its residual owner retired; the builder now writes it on a checkable role."
    rationale: "Report F5 «Писать на checkable-ролях (Рекомендую)»: `build_accessibility_node` (`accessibility.rs:199-224`) writes `Toggled::True` or `False` from `checkbox_input_checked()`; report Outcome (F5, a11y) meets it with `accessibility_states` 6; `native-toggled` moves 9 of its 10 sites."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 21
    section: "## 2. A11y Strategy"
    change: "Add after line 21: «- Since 2026-10-31-engine-debts a native checkbox or radio on a checkable role writes `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, so `pressed` never begins to read on it, and neither author attribute is read on a native control — ratified by the founder (2026-10-31, «Писать на checkable-ролях (Рекомендую)», relayed verbatim by the operator), `accessibility_states`' `a_click_and_a_space_move_a_native_control_in_the_tree` its pin; what an assistive technology reads of it is not measured (packages/blitz-dom/src/accessibility.rs:199-224; tests/blitz-tests/tests/accessibility_states.rs:386-427) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»."
    sidecar: "Role derivation: new entity for the native `toggled` write on a checkable role (F19)."
    rationale: "Report F5 states the write, the other roles and the unread author attributes; report Outcome (F3, a11y) and (F4, a11y) keep what an assistive technology reads unmeasured, and report Coverage marks the AT reading `unrunnable-here`."
    fact: F19
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 35
    section: "## 2. A11y Strategy"
    change: "Replace «nothing else writes `toggled`, so the tree never states a native checkbox or radio checked — that fix owned by a `.andromeda/residuals.md` line (target: next), the windowed AT witness by one that reads `re-carried:0.2.0`» with «a native checkbox or radio on a checkable role writes `toggled` from its own checked state, reading neither attribute, and under any other role writes nothing — the windowed AT witness owned by a `.andromeda/residuals.md` line that reads `re-carried:0.2.0`»."
    sidecar: "Body §2 Role derivation: the broad line states the native `toggled` write; its residual owner dropped."
    rationale: "Report F5 (`accessibility.rs:199-224`); the broad line details the same claim as a11y-strategy.md line 20, and report P5 card row 5 absorbs the owning residual lines into `v030-04`, with the windowed AT witness unmeasured."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 16
    section: "## 4. ARIA Patterns & Roles"
    change: "Replace «a native checkbox or radio reading neither,» with «a native checkbox or radio reading neither and, on a checkable role alone, writing `toggled` from its own checked state (§2 Role derivation),»."
    sidecar: "ARIA state attributes: the native write named beside the attribute reads."
    rationale: "Report F5: `is_native_checkable && is_checkable(role)` writes `Toggled` from `checkbox_input_checked()`, and neither author attribute is read on a native control, so the attribute mapping's native clause gains the write."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 19
    section: "## 4. ARIA Patterns & Roles"
    change: "Add after line 19: «- **Select, `multiple`** — a `multiple` select reads `set_multiselectable` on its `ListBox` (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»."
    sidecar: "Per-component catalog: `set_multiselectable` on a `multiple` select (F18)."
    rationale: "Report F6, `accessibility.rs`: `set_multiselectable` on a `multiple` select; report Coverage of new surfaces, the `multiple` select row, a11y `ListBox`, `set_multiselectable`."
    fact: F18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 19
    section: "## 4. ARIA Patterns & Roles"
    change: "Add after line 19: «- **Option group** — `optgroup` → `Role::Group`, its non-empty `label` attribute its name, a group's label named in the tree and unpainted (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»."
    sidecar: "Per-component catalog: `optgroup` maps to `Role::Group`, its label a name in the tree and unpainted (F17)."
    rationale: "Report F7: `accessibility.rs`: `optgroup` → `Role::Group`, its non-empty `label` attribute its name, a group's label named in the tree and unpainted; report Coverage of new surfaces, the `optgroup` row."
    fact: F17
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 23
    section: "## 4. ARIA Patterns & Roles"
    change: "Replace «an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes:» with «an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), excepted from `display: none`, which every option of a single-choice select but its chosen one computes and a `multiple` select's rows do not:»."
    sidecar: "Hidden content: the option exception reaches an option under a group; a `multiple` select's rows no longer compute `display: none`."
    rationale: "Report F7: `is_select_option` holds under a group; report Schema / config: `assets/default.css:161-188` adds `display: block` on `select[multiple] > option` and `select[multiple] > optgroup > option`, and `display: inline` on `select:not([multiple]) > optgroup > option:checked`."
note: "a11y-scope-summary.md:19 is outside the report's `load-rows` changed list and is proposed on its Coverage and New text rows; a11y-scope-summary.md:23's new limits come from `docs/capabilities.md`'s New text rows, so their place under `## Accessibility reach` is unread; `a11y-plan.md:89` and `:91` still hold and take no edit."
