group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:42   class: omission
  leaf-text: «no shot's image bytes, one `tool_call` line per call.»
  source: .andromeda/obs-plan.md:76 «and `host_log` and `mcp_log` each read a recording host and a recording `mcp` role at `trace`: 0 of the recording needles (the kept check's directory name and contents), the span counts unchanged»
  replace-with: «no shot's image bytes, one `tool_call` line per call; since 2026-10-19-session-recording `host_log` also reads a recording host (`--record <dir>`) and `mcp_log` a recording `mcp --record <dir>` role at `trace`: 0 of the recording needles (the kept check's directory name and contents), the span counts unchanged.»
