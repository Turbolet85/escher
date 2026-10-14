group: layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «with `aria-selected` by attributes alone, adding no element, class, style, wrapper or order.»
  source: .andromeda/layout-templates.md:10 «since 2026-10-13-stand-keyboard-harness the CRUD list is a keyboard region — `crud-list` takes `tabindex="0"` and an `onkeydown`»
  replace-with: «with `aria-selected` by attributes alone, adding no element, class, style, wrapper or order; since 2026-10-13-stand-keyboard-harness `crud-list` is a keyboard region (`tabindex="0"` and an `onkeydown`: ArrowDown selects the next visible row, the first when none is selected, ArrowUp the previous, Home the first and End the last, each filling `crud-name` and `crud-surname` as a row's click does), so CRUD's Tab sequence reads `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` (six stops, five until then), the focus on the list, never the row, adding no element, class, style, wrapper or row.»
