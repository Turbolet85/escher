# Apply — a11y-plan · part 2 of 5

## Verdicts
- fanout-a11y-plan--cascade-g1.md 6 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:16#1
- fanout-a11y-plan--cascade-g1.md 7 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:19+#1
- fanout-a11y-plan--cascade-g1.md 8 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:19+#2
- fanout-a11y-plan--cascade-g1.md 9 · applied · .andromeda/a11y-plan-detail/aria-patterns-roles.md:23#1

## Edits
- line: .andromeda/a11y-plan-detail/aria-patterns-roles.md:16
  old: «a native checkbox or radio reading neither,»
  new: «a native checkbox or radio reading neither and, on a checkable role alone, writing `toggled` true or false from its own checked state, nothing under any other role (packages/blitz-dom/src/accessibility.rs:199-224),»
- after: .andromeda/a11y-plan-detail/aria-patterns-roles.md:19
  anchor: «- **Select** — `<select multiple>` is ListBox, else ComboBox (packages/blitz-dom/src/accessibility.rs:498-515; tests/blitz-tests/tests/accessibility_roles.rs:128-155)»
  add: «- **Select, `multiple`** — a `multiple` select's `ListBox` writes `set_multiselectable` (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- after: .andromeda/a11y-plan-detail/aria-patterns-roles.md:19
  anchor: «- **Select** — `<select multiple>` is ListBox, else ComboBox (packages/blitz-dom/src/accessibility.rs:498-515; tests/blitz-tests/tests/accessibility_roles.rs:128-155)»
  add: «- **Option group** — `optgroup` → `Role::Group`, its non-empty `label` attribute its name; a group's label is named in the tree and unpainted (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»
- line: .andromeda/a11y-plan-detail/aria-patterns-roles.md:23
  old: «an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes:»
  new: «an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), excepted from `display: none`, which every option of a single-choice select but its chosen one computes and a `multiple` select's rows do not:»

## Patterns
[[pattern]]
id = "a11y2-01"
fixed = 'whose parent is a `select` excepted'

[[pattern]]
id = "a11y2-02"
fixed = "every option but a single-choice select's chosen one computes"
