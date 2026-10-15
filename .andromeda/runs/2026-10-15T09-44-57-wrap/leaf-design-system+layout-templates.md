group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «`scripts/cold-agent.sh` (the cold-agent run pipe, same CLI shape)»
  source: .andromeda/layout-templates.md:73 «the one task `stand` — the four lean apps in one client session under the fixed prompt `STAND_PROMPT`, which names no element id or app slug»
  replace-with: «`scripts/cold-agent.sh` (the cold-agent run pipe, same CLI shape; its one task `stand`, the four lean apps in one client session given the driver's own MCP server `escher`, `escher-session mcp`)»
