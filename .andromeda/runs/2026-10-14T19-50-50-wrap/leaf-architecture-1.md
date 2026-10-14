group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-shell.md:18   class: omission
  leaf-text: «a windowed witness is still owed — no windowed run witnesses the refresh on the dev host (AT-SPI inactive; the stand binary builds without `accessibility`)»
  source: .andromeda/architecture.md:122 «whose headless halves — the tree the shell hands the adapter, read through `&dyn Document`, and the refresh on a Tab and an Enter — are proven on the stand since 2026-10-14-stand-a11y-assertions»
  replace-with: «a windowed witness is still owed, held by a `.andromeda/residuals.md` line (target: next) — no windowed run witnesses the refresh on the dev host (AT-SPI inactive; the stand binary builds without `accessibility`); its headless halves — the tree the shell hands the adapter, read through `&dyn Document`, and the refresh on a Tab and an Enter — are proven on the stand since 2026-10-14-stand-a11y-assertions (`stand_accessibility_ids`)»
