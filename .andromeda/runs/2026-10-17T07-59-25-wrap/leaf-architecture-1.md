group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/dioxus-native-dom.md:41   class: stale
  leaf-text: «six `bridge_tests` — a Dioxus key that is empty or holds `/` reading a positional segment, `create_head_element`, `mounted` delivery after the build and after a poll, `Document::id`, a handled event's cancel and stop — in `dioxus_document_tests.rs`»
  source: .andromeda/architecture.md:255 «seven unit tests of the bridge (six until 2026-10-16-fast-failing-mutants, which added a created text node's mapping and stack read)»
  replace-with: «seven `bridge_tests` — a Dioxus key that is empty or holds `/` reading a positional segment, `create_head_element`, `mounted` delivery after the build and after a poll, `Document::id`, a handled event's cancel and stop and, since 2026-10-16-fast-failing-mutants, a created text node's mapping and stack read — in `dioxus_document_tests.rs`»
