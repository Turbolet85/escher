group: design-system
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
read: CLAUDE.md · 146 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «exit `0` accepted · `1` refused · `2` usage · `3` session error; since 2026-10-10-mcp-surface its `mcp` role»
  source: .andromeda/design-system.md:396 «since that chunk `escher-session help` answers the tool's self-description the same way, one uncoloured line of JSON on stdout with no ESC byte, status `0`, reaching no session»
  replace-with: «exit `0` accepted · `1` refused · `2` usage · `3` session error, read from the schema's `EXIT_STATUSES` table; since 2026-10-12-self-description `escher-session help` answers the tool's self-description the same way, one uncoloured line of JSON on stdout with no ESC byte, status `0`, reaching no session, and a session error's object carries its fixed `remedy`; since 2026-10-10-mcp-surface its `mcp` role»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «newline-delimited JSON-RPC 2.0 protocol lines only on stdout, uncoloured»
  source: .andromeda/design-system.md:396 «a tool result's text the screen text for a `snapshot`, the self-description's line for `help` since 2026-10-12-self-description, and the JSON line otherwise»
  replace-with: «newline-delimited JSON-RPC 2.0 protocol lines only on stdout — a tool result's text the screen text for a `snapshot`, the self-description's line for `help`, the JSON line otherwise — uncoloured»

- leaf: CLAUDE.md:100   class: omission
  leaf-text: «`status` · `stop`; one line of JSON on stdout, exit `0` · `1` · `2` · `3`»
  source: .andromeda/design-system.md:396 «since that chunk `escher-session help` answers the tool's self-description the same way, one uncoloured line of JSON on stdout with no ESC byte, status `0`, reaching no session»
  replace-with: «`status` · `stop`, and `help` (the tool's self-description, reaching no session); one line of JSON on stdout, exit `0` · `1` · `2` · `3`»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «a `snapshot` result the screen text, any other the JSON line»
  source: .andromeda/design-system.md:396 «a tool result's text the screen text for a `snapshot`, the self-description's line for `help` since 2026-10-12-self-description, and the JSON line otherwise»
  replace-with: «a `snapshot` result the screen text, a `help` result the self-description's line, any other the JSON line»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and — still to come — the driver's self-description and the headless screenshot.»
  source: .andromeda/design-system.md:396 «since that chunk `escher-session help` answers the tool's self-description the same way, one uncoloured line of JSON on stdout with no ESC byte, status `0`, reaching no session»
  replace-with: «and the driver's self-description (built — `escher-session help`, and `help` over MCP, answer the tool's description of itself as one line of JSON, reaching no session; a session error's object carries its fixed `remedy`), and — still to come — the headless screenshot.»
