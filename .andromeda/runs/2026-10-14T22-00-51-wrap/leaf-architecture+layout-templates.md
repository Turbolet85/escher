group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «the temp converter, circle drawer and cells are not keyed yet (2 · 3 · 676 pinned)»
  source: .andromeda/architecture.md:136 «and, since 2026-10-14-stand-requirement-sweep, on Temperature Converter, Circle Drawer and Cells too (2 · 3 · 676 until then), Circle Drawer's dialog open included»
  replace-with: «since 2026-10-14-stand-requirement-sweep the temp converter, circle drawer and cells read it empty too (2 · 3 · 676 until then), Circle Drawer's dialog open included — keyed by author ids added as `id:` lines alone, no element, class, style, wrapper or order moving: `temp-celsius` · `temp-fahrenheit`; `circle-undo` · `circle-redo` · `circle-canvas` and, only while its dialog is open, `circle-diameter` (the dialog's range input) · `circle-close`; `cell-{A..Z}{0..25}` on the 676 data cells through the file's own `col_label(c)` and `cell-input` on the one edit input (`stand_non_lean_tasks` reads it)»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «reads Enter and Space on the later-enabled `crud-update` and `crud-delete` each as its click.»
  source: .andromeda/architecture.md:263 «`stand_requirement_sweep` and `stand_non_lean_tasks` the twentieth and twenty-first, since 2026-10-14-stand-requirement-sweep»
  replace-with: «reads Enter and Space on the later-enabled `crud-update` and `crud-delete` each as its click. Since 2026-10-14-stand-requirement-sweep two more stand checks read `tests/common/mod.rs`, the twentieth and twenty-first: `stand_requirement_sweep` (2 tests, both layout modes) holds every requirement through `Session::run` alone on each lean task and names each requirement's witness tests in `WITNESSES` (16 rows, each found in its file through `include_str!`), and `stand_non_lean_tasks` (3 tests, both layout modes) pins the per-task count of elements that react to a click and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676), the Circle Drawer dialog rule applying, and its open dialog read keyed.»
