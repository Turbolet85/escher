group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/dioxus-native-dom.md:18   class: omission
  leaf-text: «No `Display` impl, no parser, no JSON form, no truncation.»
  source: .andromeda/architecture.md:209 «`to_text` itself never truncates or pages; beside it `Snapshot::text_parts(&self, budget: usize) -> Vec<TextPart>` (`TextPart { text: String, ids: Vec<String> }`»
  replace-with: «No `Display` impl, no parser, no JSON form, and `to_text` itself never truncates or pages; beside it `Snapshot::text_parts(&self, budget: usize) -> Vec<TextPart>` (`TextPart { text: String, ids: Vec<String> }`, re-exported beside the budget) cuts that text into parts — consecutive pre-order lines while a part stays within `budget` bytes, never splitting a line, a line longer than `budget` a part alone, an empty snapshot one empty part — the parts concatenated being `to_text()`, with no document access, failure path, log or print; its one caller outside the tests is escher-driver's executor, which for a screen over the budget or a `part` asked returns one part of `text_parts(SNAPSHOT_TEXT_BUDGET)` as `Outcome::Part` (written with the counts `part` and `parts`, over MCP its JSON line), a whole screen still `Outcome::Screen`.»
