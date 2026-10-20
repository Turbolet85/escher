group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «`snapshot` · `screenshot` · `screenshot_of`, the outcome reader `act`, the shot reader `shot`,»
  source: .andromeda/test-plan.md:103 «`screenshot()` and `screenshot_of(id)`, `lint()`, `act(session, call)` returning `Acted { settled, busy, diff, advanced_ms, in_view }` (`None` for a refused call, for a `snapshot`, for a shot and for a lint;»
  replace-with: «`snapshot` · `screenshot` · `screenshot_of` · `lint`, the outcome reader `act`, the shot reader `shot`, the lint reader `linted` (a lint's findings),»

- leaf: .claude/rules/verification-harness.md:25   class: stale
  leaf-text: «the driver's own `escher-session mcp` (the eleven verbs;»
  source: .andromeda/test-plan.md:126 «the agent talks to the driver's own MCP surface, `escher-session mcp` (the twelve verbs)»
  replace-with: «the driver's own `escher-session mcp` (the twelve verbs;»

- leaf: .claude/rules/verification-harness.md:26   class: stale
  leaf-text: «`isolated` (exactly the eleven `mcp__escher__*` tools, one server `escher`)»
  source: .andromeda/test-plan.md:127 «`isolated` = the init tools exactly the twelve `mcp__escher__{verb}` tools, one MCP server `escher` connected»
  replace-with: «`isolated` (exactly the twelve `mcp__escher__*` tools, one server `escher`)»
