group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «as do each CRUD row (`crud-person-{person.id}`) and each of Home's seven task cards (`task-card-{slug}`),»
  source: .andromeda/layout-templates.md:10 «since 2026-10-14-stand-requirement-sweep the three tasks outside the lean four carry author `id`s on their controls too, their stable element ids»
  replace-with: «as do each CRUD row (`crud-person-{person.id}`) and each of Home's seven task cards (`task-card-{slug}`), and, since 2026-10-14-stand-requirement-sweep, the controls of the three tasks outside the lean four (`temp-celsius` · `temp-fahrenheit`; `circle-undo` · `circle-redo` · `circle-canvas`, with `circle-diameter` · `circle-close` on the open dialog; `cell-{A..Z}{0..25}` · `cell-input`), added as `id:` lines alone, so `unkeyed_actionable()` reads 0 on all seven tasks, the dialog open included,»
