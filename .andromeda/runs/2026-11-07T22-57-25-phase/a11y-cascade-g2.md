@carried n=1 label=a11y-plan/aria-patterns-roles fact=fct-h5c66ocw
why: An element that reacts to a click and states no role must keep reading GenericContainer while the new closed «reacts to a click» boolean says it reacts, and the re-pinned stand_non_lean_tasks reading counts against this mapping.
quote: **Generic and text** — div→GenericContainer, h2→Heading, p→Paragraph (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
@carried n=1 run=2 label=a11y-plan fact=fct-4yuvm6ln
quote: Native element mapping follows HTML-AAM: landmarks, headings, lists, tables, interactive and inline semantics (packages/blitz-dom/src/accessibility.rs:467-536)
@carried n=2 label=a11y-plan/aria-patterns-roles fact=fct-4bekc2yb
why: An `<a>` with no href that carries a click handler must read GenericContainer with the new field true, its role untouched, as an input of the re-pinned no-role reading.
quote: **Link** — `<a>` is a Link only with `href`, else GenericContainer (packages/blitz-dom/src/accessibility.rs:502-519; tests/blitz-tests/tests/accessibility_roles.rs:117-126)
@carried n=3 label=a11y-plan/aria-patterns-roles fact=fct-5m2yc5ak
why: Whether an element "states a role" is decided by the `role` attribute through `role_from_name`, so the «reacts to a click» field and its re-pinned count must take the role as the tree gives it, adding no second role mapping.
quote: **ARIA `role` attribute** — `role_from_name` maps ARIA role names (alert, button, checkbox, dialog, link, tab, textbox, landmark roles, etc.) to AccessKit roles (packages/blitz-dom/src/accessibility.rs:357-420)
@binding n=1
**aria-patterns GenericContainer reading ↔ sr-actionable-keys**: the new «reacts to a click» snapshot field must fill the gap the actionable-key rule leaves (a11y-plan fct-77ewqepn) for elements whose role reads GenericContainer (a11y-plan/aria-patterns-roles fct-h5c66ocw) (a11y-plan/aria-patterns-roles fct-4bekc2yb), saying they react while their role stays untouched.
@binding n=2
**aria-patterns role mapping ↔ sr-snapshot**: the field must come from the snapshot model that takes each node's role from `accessibility_tree` (a11y-plan fct-uz4kdss3), the role as `role_from_name` or the native mapping gives it (a11y-plan/aria-patterns-roles fct-5m2yc5ak), never a second role or name mapping.
