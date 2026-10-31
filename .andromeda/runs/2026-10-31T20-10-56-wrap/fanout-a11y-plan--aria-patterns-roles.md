proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 21
    section: "## 4. ARIA Patterns & Roles"
    change: "- **Option group** — `optgroup` → `Role::Group`, its non-empty `label` attribute its name; a group's label is named in the tree and, since 2026-10-31-select-popup, painted as a header row of an open list, heavier than an option's row, a header choosing nothing (packages/blitz-dom/src/accessibility.rs) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md"
    sidecar: "aria-patterns-roles Option group: a group's label is no longer unpainted; it is painted as a header row of an open list (2026-10-31-select-popup)."
    rationale: "The report's Retired bullet names «a group's label is unpainted»; its `group-label` row lists aria-patterns-roles:21 as changed, and Schema / config states a group's `label` painted in every shot of an open list."
    basis: .andromeda/a11y-plan-detail/aria-patterns-roles.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 19
    section: "## 4. ARIA Patterns & Roles"
    change: "- **Select, single** — a single select's `ComboBox` writes `set_expanded(open)` from its list's engine state and reads no `aria-expanded`, every other element keeping that read; `:open` and `flush_open_state` are unmoved, so an author's `open` or `aria-expanded` on a select steers nothing; focus stays on the select, its chosen option reads `selected`, the highlighted row is unread by the tree and the snapshot holds no `expanded` node key; what an assistive technology reads of it is not measured (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md"
    sidecar: "aria-patterns-roles: new record for a single select's native `expanded`, the author attributes it ignores and the highlight the tree leaves unread (2026-10-31-select-popup)."
    rationale: "The report's Symbols / APIs Tree row (`accessibility.rs:238-242`) and its Coverage row `tree expanded on a single select` bring role and state detail this category's Select records lack; inputs#I6 and `NODE_FIELDS` staying thirteen hold the no-node-key clause, and Outcome leaves the assistive technology's reading unmeasured."
    basis: packages/blitz-dom/src/accessibility.rs:238-242
note: "D-a11y-surface finds no other gap in this file and D-a11y-obs-schema does not fire (no log field or a11y schema moved); a11y-plan.md:202 holds the same retired «unpainted» clause outside this file, named by the report's `group-label` row."
