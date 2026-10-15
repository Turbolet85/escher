group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «the two MCP targets run on windows too.»
  source: .andromeda/architecture.md:150 «`scripts/cold-agent.sh run` spawns one `claude` client (the Claude Code CLI) per run, which spawns `sh` → `escher-session mcp` (`target/debug/escher-session`»
  replace-with: «the two MCP targets run on windows too. Since 2026-10-15-cold-agent-test the cold-agent pipe (`scripts/cold-agent.sh run`) gives its `claude` client this binary's `mcp` role as its one MCP server, `escher` (`target/debug/escher-session`, spawned through the per-run `mcp.json`, `RUST_LOG=info`, stderr to `target/cold-agent/server.log`), and its judge spawns two more `escher-session mcp` children to replay the agent's check, full and ablated; the pipe builds nothing — a missing or non-executable binary is exit 3 naming `cargo build --locked -p seven_guis --bin escher-session`.»
