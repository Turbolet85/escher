group: test-plan-2
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »
- leaf: .claude/rules/verification-harness.md:26   class: omission
  leaf-text: «`isolated` (exactly the eleven `mcp__escher__*` tools, one server `escher`)»
  source: .andromeda/test-plan.md:126 «`isolated` = the init tools exactly the eleven `mcp__escher__{verb}` tools, one MCP server `escher` connected, `skills` and `slash_commands` empty, `apiKeySource` none, and no `cwd` or `memory_paths` string naming»
  replace-with: «`isolated` (exactly the eleven `mcp__escher__*` tools, one server `escher` connected, `skills` and `slash_commands` empty, `apiKeySource` none, no `cwd` or `memory_paths` string naming the repo root's basename, case-folded)»
