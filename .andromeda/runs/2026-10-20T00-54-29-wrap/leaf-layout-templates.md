group: layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:27   class: omission
  leaf-text: «its accessible name `People` in the tree and the snapshot, adding no element, class, style, wrapper, id or order.»
  source: .andromeda/layout-templates.md:46 «The stand's layout reads clean to the in-process visual lint (`Session::lint`, escher-driver; PROVISIONAL: in process only) since 2026-10-19-visual-lint: at boot each of the four lean tasks reads 0 findings»
  replace-with: «its accessible name `People` in the tree and the snapshot, adding no element, class, style, wrapper, id or order; since 2026-10-19-visual-lint the stand's layout reads clean to the in-process visual lint (`Session::lint`, escher-driver; PROVISIONAL: in process only) — at boot each of the four lean tasks reads 0 findings in both layout modes (no overlap, clipping, truncated text, off-screen element, zero-size element holding content or contrast failure), and after twelve Creates the last CRUD row, `crud-person-14`, reads `clipped` inside `crud-list`, agreeing with the driver's `off-screen` refusal of its `click`; no stand markup, id, class or style moved.»
