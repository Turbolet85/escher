group: layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «WPT runner (owo-colors status words + colour), `paint_bench`, `screenshot`, and»
  source: .andromeda/layout-templates.md:72 «- `multi_doc` — a root example taking a mode and an iteration count as positional arguments, an unknown mode ending in a panic; its document count from `MULTI_DOC_N` (a default when absent or unparsed)»
  replace-with: «WPT runner (owo-colors status words + colour), `paint_bench`, `screenshot`, `multi_doc` (a root example: a mode and an iteration count as positional arguments, an unknown mode a panic; its document count from `MULTI_DOC_N`, its layout and style threading from the flags `MULTI_DOC_FLAT` and `MULTI_DOC_SEQ`; its timing one line on stdout), and»
