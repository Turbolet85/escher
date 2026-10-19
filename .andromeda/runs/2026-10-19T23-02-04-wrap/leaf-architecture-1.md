group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/conventions.md:6   class: omission
  leaf-text: «`tests/blitz-tests/tests/session_common/mod.rs` (neither a test target)»
  source: .andromeda/architecture.md:142 «A third such module, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`»
  replace-with: «`tests/blitz-tests/tests/session_common/mod.rs`, and the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet` in a third, `tests/blitz-tests/tests/fixtures/mod.rs`, read by `mod fixtures;` — a page with one reader staying in its check's file (none of the three a test target)»
