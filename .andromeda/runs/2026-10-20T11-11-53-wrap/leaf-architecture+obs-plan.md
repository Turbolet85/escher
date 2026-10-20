group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summary»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: omission
  leaf-text: «a lint's command-span line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles.»
  source: .andromeda/obs-plan.md:196 «a part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text»
  replace-with: «a lint's command-span line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles; a part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text — `host_log` and `mcp_log` each drive one part read at `trace` with 0 needles.»

- leaf: .claude/docs/services/escher-telemetry.md:42   class: omission
  leaf-text: «`host_log` and `mcp_log` each also drive one `lint` at `trace` with 0 needles;»
  source: .andromeda/obs-plan.md:196 «`host_log` and `mcp_log` each drive one part read at `trace` with 0 needles»
  replace-with: «`host_log` and `mcp_log` each also drive one `lint` and one part read (a `snapshot` of part 1) at `trace` with 0 needles;»
