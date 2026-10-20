group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: omission
  leaf-text: «0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host's and the `mcp` role's stderr at `trace`, under both builds (`host_log`, `mcp_log`).»
  source: .andromeda/obs-plan.md:195 «a lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles»
  replace-with: «0 occurrences of the first 64 base64 characters of a shot in the `escher-session` host's and the `mcp` role's stderr at `trace`, under both builds (`host_log`, `mcp_log`); a lint's command-span line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles.»

- leaf: .claude/docs/services/escher-telemetry.md:42   class: omission
  leaf-text: «0 typed-sentinel occurrences, 0 id needles, 0 name needles, no shot's image bytes, one `tool_call` line per call;»
  source: .andromeda/obs-plan.md:195 «`host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles»
  replace-with: «0 typed-sentinel occurrences, 0 id needles, 0 name needles, no shot's image bytes, one `tool_call` line per call; `host_log` and `mcp_log` each also drive one `lint` at `trace` with 0 needles;»
