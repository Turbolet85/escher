group: architecture+layout-templates
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «`start <task>` spawns the same binary in its host role, `serve <task> --session <dir>`, which serves until»
  source: .andromeda/layout-templates.md:73 «`start` checks its recording directory itself before it spawns, then spawns the same binary in its host role, `serve <task> --session <dir> [--record <dir>]` (in that order, passing `--record` to the host)»
  replace-with: «`start <task> [--record <dir>]` checks its recording directory itself before it spawns, then spawns the same binary in its host role, `serve <task> --session <dir> [--record <dir>]` (passing `--record` to the host); since 2026-10-19-session-recording `--record <dir>`, taken at most once on `start`, the host role and the `mcp` role, keeps the session as one owner-only replayable check — `check-{n}.json` (`0600`, never replacing a file) in a `0700` directory whose parent must exist, never removed by escher, holding the calls and no screen reading (ratified by the founder, 2026-10-19) — a directory open to its group or others refused as the session error `record-dir-not-private` (exit 3, `start` spawning nothing), a recording that ends answering `recording-failed` (exit 3; over MCP `isError`); the host serves until»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «Since 2026-10-10-mcp-surface its `mcp` role, `escher-session mcp`, is an MCP server»
  source: .andromeda/architecture.md:201 «since 2026-10-10-mcp-surface the `mcp` role is `mcp [--record <dir>]`, an MCP server on the binary's standard streams whose `start` tool boots the same four through the same boot»
  replace-with: «Since 2026-10-10-mcp-surface its `mcp` role, `escher-session mcp [--record <dir>]` (the `--record` since 2026-10-19-session-recording), is an MCP server»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «Seven test targets spawn it»
  source: .andromeda/architecture.md:334 «and the crate's eight integration-test targets, all spawning the built `escher-session` binary — `tests/host_binary.rs`, `tests/host_log.rs`, since 2026-10-10-driver-cli `tests/cli_commands.rs`, `tests/cli_flow.rs` a»
  replace-with: «Eight test targets spawn it»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «per package and under the workspace build; the two MCP targets run on windows too.»
  source: .andromeda/architecture.md:220 «and since 2026-10-10-mcp-surface `mcp_server` and `mcp_log` spawn its `mcp` role as a child spoken to over piped stdin, and `recording` does both, with `--record` — the eight sharing the helpers»
  replace-with: «per package and under the workspace build; the two MCP targets run on windows too; since 2026-10-19-session-recording `tests/recording.rs` spawns both the host role and the `mcp` role, with `--record`.»

- leaf: .claude/docs/services/seven_guis.md:11   class: stale
  leaf-text: «the `stand-repeat` CI leg runs these seven by name (`cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log`)»
  source: .andromeda/architecture.md:204 «then `cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and `recording`; the first red command ends the leg»
  replace-with: «the `stand-repeat` CI leg runs these eight by name (`cargo test -p seven_guis --locked` over `cli_commands`, `cli_flow`, `host_binary`, `host_log`, `host_timer`, `mcp_server`, `mcp_log` and `recording`)»
