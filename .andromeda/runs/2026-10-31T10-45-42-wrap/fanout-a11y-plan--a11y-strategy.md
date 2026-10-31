proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 20
    section: "## 2. A11y Strategy"
    change: "Replace the passage from `each attribute is read once and nothing else writes `toggled`` through `(target: next);` with: each attribute is read once; since 2026-10-31-engine-debts a native control of those two types on a checkable role also writes `toggled` True or False from its own checked state, under any other role nothing, so `pressed` never reads on it — «Писать на checkable-ролях (Рекомендую)», the founder, 2026-10-31 — and nothing else writes `toggled`;"
    sidecar: "2026-10-31-engine-debts: the builder writes a native checkbox's or radio's `toggled` on a checkable role; the measured absence and its residual owner end (F5)."
    rationale: "Report Changes, Symbols / APIs, F5: `build_accessibility_node` writes `Toggled::True` or `False` from `checkbox_input_checked()` where `is_native_checkable && is_checkable(role)`, nothing under any other role, and neither author attribute is read on a native control; so `nothing else writes toggled` and the owed fix no longer stand. Counts / qualifiers moved: `native-toggled`. Expected amendments: a11y-plan §2, native `toggled`."
    basis: packages/blitz-dom/src/accessibility.rs:199-224
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-strategy.md
    near: 23
    section: "## 2. A11y Strategy"
    change: "New bullet after this line: - Since 2026-10-31-engine-debts the builder maps an `optgroup` to `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted, and calls `set_multiselectable` on a `multiple` select, read as `ListBox`; what an assistive technology reads of them is not measured (`unrunnable-here`); both on the founder's «Построить сейчас» (2026-10-31), checked by `select_interaction` and `stand_act_select` — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "2026-10-31-engine-debts: the builder's `optgroup` role and name and a `multiple` select's `set_multiselectable` recorded (F6, F7)."
    rationale: "Report Changes, Symbols / APIs: F6 `accessibility.rs`: `set_multiselectable` on a `multiple` select; F7 `accessibility.rs`: `optgroup` → `Role::Group`, its non-empty `label` attribute its name, unpainted. Coverage of new surfaces rows `multiple` select and `optgroup` name the role, the checks and AT `unrunnable-here`. This file's Role derivation group details what the builder writes and no line of it states either."
note: "Outside this file, for the body's own pass: .andromeda/a11y-plan.md:35 states the same native-toggled limit, and .andromeda/a11y-plan.md:53 holds `accessibility_states` as `5 tests`, a form absent from the report's listed n-a11y-states hits (report moves it to 6)."
