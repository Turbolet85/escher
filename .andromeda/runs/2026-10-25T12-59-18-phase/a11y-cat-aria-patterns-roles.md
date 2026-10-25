# a11y · ARIA Patterns & Roles — category read

## Carried
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) The flight booker's one-way / return choice made a real `select` must be a single-choice select (no `multiple`), so its role reads ComboBox in the tree and the snapshot — the role the stand's tables, ids and the cold agent's goals must state.
  > - **Select** — `<select multiple>` is ListBox, else ComboBox (packages/blitz-dom/src/accessibility.rs:475-492; tests/blitz-tests/tests/accessibility_roles.rs:128-155)
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:16) Replacing the two `aria-pressed` mode buttons moves the stand's `pressed` carrier: the select's options must state their choice by the select's chosen state alone, with no `aria-selected` written on an option, since the builder reads none there.
  > `aria-selected` → `selected`, `aria-pressed` → `toggled` (a toggle button; `mixed` kept as Mixed), `aria-invalid` → `invalid` (True / Grammar / Spelling; a value outside the list reads True), read by value on any element — a select's option excepted, its `selected` its chosen state (packages/blitz-dom/src/accessibility.rs:171-181) — since 2026-10-10-snapshot-states
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:23) The flight booker select's options must stay in the tree though every unchosen one computes `display: none`, so the stand's snapshot, ids and diff tables must list both options as the select's children; an option the stand hides by `hidden`, `inert` or `visibility: hidden` leaves the tree.
  > an `option` whose parent is a `select` excepted from `display: none`, which every option but a single-choice select's chosen one computes: it stays in the tree, `hidden`, `inert` and `visibility: hidden` still excluding it
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:22) Cells served to the driver must be read with its grid cells' roles as the engine maps them — its role-less `div` cells reading GenericContainer — so the paged snapshot and any Cells goal the cold-agent pipe states name those roles and no second mapping.
  > - **Generic and text** — div→GenericContainer, h2→Heading, p→Paragraph (tests/blitz-tests/tests/accessibility_roles.rs:157-180)
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:21) The Cells editor input a selected cell opens must read the role its `type` maps to, TextInput by default, where the stand's Cells tables state it.
  > `<input>` roles are mapped by `type`, defaulting to TextInput
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:15) Any `role` attribute the extended stand writes (on Cells or the load-wait screen) must be a name `role_from_name` maps, or the node reads `Role::Unknown`.
  > `role_from_name` maps ARIA role names (alert, button, checkbox, dialog, link, tab, textbox, landmark roles, etc.) to AccessKit roles
- (.andromeda/a11y-plan.md:89) The broad line the stand's state carriers must keep matching after the flight booker change.
  > - **ARIA state attributes** — `aria-selected`, `aria-pressed`, `aria-checked` and `aria-invalid` map to `selected`, `toggled` and `invalid` as §2 Role derivation states, `role="switch"` mapping to `Switch`.
- (.andromeda/a11y-plan.md:91) The broad hidden-content rule the select's options are held to.
  > - **Hidden content** — `aria-hidden="true"` keeps a node, marked hidden; `hidden`, `display: none` and `visibility: hidden` exclude it and its children, an `option` under a `select` excepted from `display: none`.
- (.andromeda/a11y-plan.md:35) A select's option must read its chosen state as its `selected`, the reading the stand's select tables and the cold agent's `select` goal assert.
  > a select's option reads its chosen state
- (.andromeda/a11y-plan.md:196) The stand's stated-by-attribute inventory names the flight booker's mode buttons' `aria-pressed`; turning them into a `select` moves this claim, re-pinned in the tests that hold it and amended by the wrap, never a phase edit.
  > On the stand three kinds of control state a role or a state by attribute alone — the CRUD list `listbox` with `option` rows carrying `aria-selected`, the flight booker's mode buttons `aria-pressed`, its date inputs `aria-invalid`

## Bindings
- **Select role and option state ↔ §5 Keyboard Navigation (body)**: the ComboBox the flight booker's select reads (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) is the single-choice select whose ArrowDown / ArrowUp / Home / End and typeahead keys move its choice — the stand's Tab order and Enter/Space tables (`stand_keyboard`, `tab_order`, `ENABLED_TAB_ORDER`) move with it (.andromeda/a11y-plan.md:123).
- **Option `selected` ↔ §7 Screen Reader Support (body)**: the option's chosen state (.andromeda/a11y-plan-detail/aria-patterns-roles.md:16) is what the snapshot takes as `selected` from the tree, and the mode buttons' `pressed` leaves the stand's inventory (.andromeda/a11y-plan.md:196).
- **Option kept in the tree ↔ §6 Visibility and hit-testing (body)**: the `display: none` exception for a select's options (.andromeda/a11y-plan-detail/aria-patterns-roles.md:23) is the same exception the snapshot reads (.andromeda/a11y-plan.md:172).
- **Select on the stand ↔ §9 CI Integration (body)**: `select_interaction` and `stand_act_select` already run in the `a11y` leg; a stand select's pins join the stand checks that leg and `stand-repeat` run (.andromeda/a11y-plan.md:235).
