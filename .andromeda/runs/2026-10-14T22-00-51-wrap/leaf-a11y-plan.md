group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:21   class: omission
  leaf-text: «`aria-checked` is not read.»
  source: .andromeda/a11y-plan.md:102 «owned by a `.andromeda/residuals.md` line (target: next) on the founder's C7 «Отложить (Рекомендую)» (2026-10-14), `accessibility_states`' fourth test standing as its pin»
  replace-with: «`aria-checked` is not read — reading it as `checked` or `pressed` is owned by a `.andromeda/residuals.md` line (target: next) on the founder's C7 (2026-10-14), `accessibility_states`' fourth test standing as its pin.»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «its narrowing to `button`, `input`, `select` and `textarea` a CARRY on "Stand requirement sweep"»
  source: .andromeda/a11y-plan.md:144 «its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 «Оставить по наличию, отложить (Рекомендую)» (2026-10-14)»
  replace-with: «its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 (2026-10-14), `stand_keyboard`'s pin standing»

- leaf: .claude/docs/a11y-summary.md:24   class: stale
  leaf-text: «which bullet a click lands on can depend on the real characters' widths — not measured, owned by a `CARRY:` on "Stand requirement sweep"»
  source: .andromeda/a11y-plan.md:281 «a click 30 px into the content box (the bundled font, 20 px) places the caret at index 5 after eight narrow characters and at 2 after eight wide ones»
  replace-with: «the bullet a click lands on depends on the real characters' widths — measured since 2026-10-14-stand-requirement-sweep: a click 30 px into the content box (the bundled font, 20 px) places the caret at index 5 after eight narrow characters and at 2 after eight wide ones, in both layout modes (`password_caret_click.rs`); placing it by the bullet layout is owned by a `.andromeda/residuals.md` line (target: next) on the founder's C11 (2026-10-14), this reading its starting point»

- leaf: .claude/docs/a11y-summary.md:25   class: omission
  leaf-text: «`stand_actionable_keys.rs` asserts every actionable element of the lean tasks and Home reads an author key (`DioxusDocument::unkeyed_actionable()` empty, in every state), with no `tabindex`, role or Tab-order change;»
  source: .andromeda/a11y-plan.md:282 «since 2026-10-14-stand-requirement-sweep, returns nothing on the three non-lean tasks too — Temperature Converter · Circle Drawer · Cells 0 · 0 · 0, in both layout modes»
  replace-with: «`stand_actionable_keys.rs` asserts every actionable element of the lean tasks and Home reads an author key (`DioxusDocument::unkeyed_actionable()` empty, in every state), with no `tabindex`, role or Tab-order change, and since 2026-10-14-stand-requirement-sweep the three non-lean tasks too — Temperature Converter · Circle Drawer · Cells 0 · 0 · 0, in both layout modes and with Circle Drawer's dialog open, through nine author ids added by `id:` lines alone (until then 2 · 3 · 676); the elements that react to a click and state no interactive role are counted and pinned as measured (`stand_non_lean_tasks`: 0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676) — no snapshot field says that an element reacts to a click, that reading owned by a `.andromeda/residuals.md` line (target: next) on the founder's C3 (2026-10-14), this count its starting reading;»

- leaf: .claude/rules/a11y.md:25   class: omission
  leaf-text: «`aria-checked` is not read — do not fold it into `checked` or `pressed`.»
  source: .andromeda/a11y-plan.md:102 «owned by a `.andromeda/residuals.md` line (target: next) on the founder's C7 «Отложить (Рекомендую)» (2026-10-14), `accessibility_states`' fourth test standing as its pin»
  replace-with: «`aria-checked` is not read — do not fold it into `checked` or `pressed`: reading it so is owned by a `.andromeda/residuals.md` line (target: next) on the founder's C7 (2026-10-14), `accessibility_states`' fourth test standing as its pin.»

- leaf: .claude/rules/a11y.md:28   class: omission
  leaf-text: «`stand_actionable_keys` asserts it empty on the four lean tasks and Home, so a new control there needs an `id`»
  source: .andromeda/a11y-plan.md:282 «since 2026-10-14-stand-requirement-sweep, returns nothing on the three non-lean tasks too — Temperature Converter · Circle Drawer · Cells 0 · 0 · 0, in both layout modes»
  replace-with: «`stand_actionable_keys` asserts it empty on the four lean tasks, Home and, since 2026-10-14-stand-requirement-sweep, the three non-lean tasks (Temperature Converter, Circle Drawer with its dialog open included, Cells — keyed by `id:` lines alone), so a new control there needs an `id`»

- leaf: .claude/rules/a11y.md:32   class: stale
  leaf-text: «its narrowing to `button`, `input`, `select` and `textarea` a CARRY on "Stand requirement sweep"»
  source: .andromeda/a11y-plan.md:144 «its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 «Оставить по наличию, отложить (Рекомендую)» (2026-10-14)»
  replace-with: «its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next) on the founder's C15 (2026-10-14), `stand_keyboard`'s pin standing»
