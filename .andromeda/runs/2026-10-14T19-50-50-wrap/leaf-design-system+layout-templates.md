group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:13   class: stale
  leaf-text: «`#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`)»
  source: .andromeda/design-system.md:24 «| seven_guis accent | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) |»
  replace-with: «`#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`; one step darker since 2026-10-14-stand-a11y-assertions, the operator's choice, until then `#4a6cf7`)»

- leaf: .claude/docs/design-summary.md:13   class: stale
  leaf-text: «measures 4.39 against SC 1.4.3's 4.5, a fail (since 2026-10-13-stand-contrast-harness)»
  source: .andromeda/design-system.md:24 «5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27»
  replace-with: «measures 5.47 against SC 1.4.3's 4.5, a pass (since 2026-10-14-stand-a11y-assertions; the old `#4a6cf7` read 4.39, a fail), and 6.83 on the hover shade, 8.27 on the active shade»

- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «the focus on the list, never the row, adding no element, class, style, wrapper or row.»
  source: .andromeda/layout-templates.md:10 «since 2026-10-14-stand-a11y-assertions the list is named by an attribute alone — `aria-label="People"`, its accessible name `People` in the tree and the snapshot»
  replace-with: «the focus on the list, never the row, adding no element, class, style, wrapper or row; since 2026-10-14-stand-a11y-assertions `crud-list` is named by `aria-label="People"` alone, its accessible name `People` in the tree and the snapshot, adding no element, class, style, wrapper, id or order.»
