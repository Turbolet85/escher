group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «its two callers are blitz-paint's password mask and blitz-dom's password click caret (`click_password_input`)).»
  source: .andromeda/architecture.md:166 «- The text extent: `BaseDocument::text_extent(&self, NodeId) -> Option<TextExtent>`, a read-only reader on every document, and `TextExtent { content, text }`, two `BoundingRect`s re-exported from the crate root»
  replace-with: «its two callers are blitz-paint's password mask and blitz-dom's password click caret (`click_password_input`)), and a third read-only reader, `BaseDocument::text_extent(&self, NodeId) -> Option<TextExtent>` with `TextExtent { content, text }` (two `BoundingRect`s, re-exported from the crate root): `content` the client rect less border, padding and any scrollbar, `text` from the content box's top-left moved down by the inline layout's `block_offset` and sized by Parley `Layout::width()` / `height()` over its scale, `None` for an id that does not resolve, a node that is no inline root or one with no inline layout data — it mutates nothing and has no log site, print, env read or panic path; RTL and vertical text read as LTR horizontal lines and a transformed box at its client rect; its one non-test caller escher-driver's `Session::lint`.»

- leaf: .claude/docs/services/blitz-dom.md:30   class: tree
  leaf-text: «`src/document.rs`, `src/config.rs`, `src/clock.rs`, `src/resolve.rs`»
  source: .andromeda/architecture.md:326 «stylo_to_cursor_icon, stylo_to_kurbo, stylo_to_parley, text_extent, traversal, tree, url, net, util, accessibility (packages/blitz-dom/src/lib.rs:37-87) |»
  replace-with: «`src/document.rs`, `src/config.rs`, `src/clock.rs`, `src/resolve.rs` · **Text extent:** `src/text_extent.rs`»
