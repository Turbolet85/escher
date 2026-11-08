@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summa
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=33 class=omission
leaf-text: through the `mcp` role (`mcp_log`, recording nothing) reads 0 option needles.
source: fct-gwvelgmp «a graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name»
replace-with: through the `mcp` role (`mcp_log`, recording nothing) reads 0 option needles; a graph's command-span line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name — `host_log` and `mcp_log` each drive one `graph` call at `trace` with 0 id, name and graph needles under both builds.
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=42 class=omission
leaf-text: `host_log` and `mcp_log` each also drive one `lint` and one part read (a `snapshot` of part 1) at `trace` with 0 needles;
source: fct-hlttx6z4 «`host_log` and `mcp_log` each make one `graph` call and read 0 id, name and graph needles (`graph_needles`, the stand's component and registered state names) at `trace` under both builds»
replace-with: `host_log` and `mcp_log` each also drive one `lint`, one part read (a `snapshot` of part 1) and one `graph` call at `trace` with 0 needles, the graph call's needles `graph_needles` (the stand's component and registered state names);
