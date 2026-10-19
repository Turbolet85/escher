group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «exit `0` accepted · `1` refused · `2` usage · `3` session error, read from the schema's `EXIT_STATUSES` table»
  source: .andromeda/design-system.md:396 «`3` session error (the recording's `record-dir-not-private` and `recording-failed` among the session errors, with no new exit)»
  replace-with: «exit `0` accepted · `1` refused · `2` usage · `3` session error (the recording's `record-dir-not-private` and `recording-failed` among the session errors, with no new exit), read from the schema's `EXIT_STATUSES` table»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «its `mcp` role, `escher-session mcp`, read by an agent's MCP client»
  source: .andromeda/layout-templates.md:73 «Since 2026-10-19-session-recording `--record <dir>`, taken at most once on `start`, the host role and the `mcp` role, keeps the session as one owner-only replayable check»
  replace-with: «its `mcp` role, `escher-session mcp` (since 2026-10-19-session-recording `escher-session mcp [--record <dir>]`, `--record <dir>` taken at most once on `start`, the host role and the `mcp` role, keeping the session as one owner-only replayable check `check-{n}.json` that holds the calls and no screen reading), read by an agent's MCP client»
