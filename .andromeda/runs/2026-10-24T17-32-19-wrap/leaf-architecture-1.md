group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «RTL and vertical text read as LTR horizontal lines and a transformed box at its client rect;»
  source: .andromeda/architecture-detail/standard-contracts.md:54 «Its limits: RTL and vertical text are read as LTR horizontal lines, and a transformed box is read at its client rect — each pinned by blitz-tests' `text_extent` in both layout modes (test-plan §1)»
  replace-with: «RTL and vertical text read as LTR horizontal lines and a transformed box at its client rect, each pinned by blitz-tests' `text_extent` in both layout modes;»
