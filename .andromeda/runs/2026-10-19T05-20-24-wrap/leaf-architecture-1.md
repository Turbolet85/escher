group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»

- leaf: .claude/docs/stack.md:26   class: stale
  leaf-text: «escher-driver's reader of MCP requests, writing nothing»
  source: .andromeda/architecture.md:57 «taken by escher-driver alone as its reader of MCP requests and of kept checks (packages/escher-driver/Cargo.toml:17)»
  replace-with: «escher-driver's reader of MCP requests and of kept checks, writing nothing»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: omission
  leaf-text: «a password or file `input` reads the public constant `MASKED_VALUE` (eight U+2022)»
  source: .andromeda/architecture.md:196 «`is_masked_input(&ElementData) -> bool`, public and re-exported beside `MASKED_VALUE`: true for an `input` whose `type` reads `password` or `file`, ASCII case-insensitive»
  replace-with: «a password or file `input` — the public `is_masked_input(&ElementData) -> bool`, re-exported beside `MASKED_VALUE`, true for an `input` whose `type` reads `password` or `file` (ASCII case-insensitive), the test this reader calls and the one escher-driver's recorder reads to keep a masked `type` as its stand-in — reads the public constant `MASKED_VALUE` (eight U+2022)»

- leaf: .claude/docs/services/dioxus-native-dom.md:18   class: omission
  leaf-text: «on the `escher-session mcp` role's stdout (ratified by the founder, 2026-10-10) (`stand_snapshot_text`, `stand_mcp_tools`)»
  source: .andromeda/architecture.md:192 «no log or event carries the snapshot or its text, nor does a kept check, which holds a recorded session's calls and its answers' digests alone»
  replace-with: «on the `escher-session mcp` role's stdout (ratified by the founder, 2026-10-10); no kept check holds it either — a recorded session keeps its calls and its answers' digests alone (`stand_snapshot_text`, `stand_mcp_tools`)»
