group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:35   class: omission
  leaf-text: «- **Tests:** in-file `#[cfg(test)]` (document.rs, mutator.rs, net.rs, …) + `tests/blitz-tests/tests/`»
  source: .andromeda/architecture.md:115 «blitz-dom's `document::changed_set_tests` and `document::font_reading_tests` in `document_changed_set_tests.rs` and `document_font_reading_tests.rs`»
  replace-with: «- **Tests:** in-file `#[cfg(test)]` (document.rs, mutator.rs, net.rs, …) — and, since 2026-10-16-file-splits, `document`'s two `#[cfg(test)]` children declared through `#[path]`, `changed_set_tests` in `src/document_changed_set_tests.rs` and `font_reading_tests` in `src/document_font_reading_tests.rs` — + `tests/blitz-tests/tests/`»
