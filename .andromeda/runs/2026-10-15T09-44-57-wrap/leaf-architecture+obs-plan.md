group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:6   class: omission
  leaf-text: «and the agent-run / cold-agent harness logs (harness metadata, not this sink)»
  source: .andromeda/obs-plan.md:87 «so the child installs escher's own sink (escher-telemetry, §3 above) at `info` and its closed `tool_call` / `command` span lines land in `target/cold-agent/server.log`»
  replace-with: «and the agent-run / cold-agent harness logs (harness metadata, not this sink) — though since 2026-10-15-cold-agent-test the cold-agent pipe spawns `escher-session mcp` with `RUST_LOG=info` set on that child only, so this sink's closed `tool_call` / `command` span lines land in `target/cold-agent/server.log`, which the pipe reads only to count ` span="tool_call" ` lines and their ` ending="refused" ` / ` ending="error" `, never printing or copying it; its two replay children send stderr to `/dev/null`»
