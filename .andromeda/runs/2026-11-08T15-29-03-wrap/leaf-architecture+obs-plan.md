@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/services/escher-telemetry.md lines=46
last: - `.claude/rules/observability.md` · `.claude/docs/obs-summary
@leaf_edit n=1 leaf=.claude/docs/services/escher-telemetry.md line=33 class=omission
leaf-text: drive one `graph` call at `trace` with 0 id, name and graph needles under both builds.
source: fct-gwvelgmp «an actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer»
replace-with: drive one `graph` call at `trace` with 0 id, name and graph needles under both builds; an actions command-span line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer — `host_log` and `mcp_log` each drive one `actions` call at `trace` with 0 id, name and actions needles under both builds.
@leaf_edit n=2 leaf=.claude/docs/services/escher-telemetry.md line=42 class=omission
leaf-text: and one `graph` call at `trace` with 0 needles, the graph call's needles `graph_needles` (the stand's component and registered state names);
source: fct-hlttx6z4 «`host_log` and `mcp_log` each make one `actions` call and read 0 id, name and actions needles (`actions_needles`, the CRUD screen's listed author-key ids) at `trace` under both builds»
replace-with: one `graph` call and one `actions` call at `trace` with 0 needles, the graph call's needles `graph_needles` (the stand's component and registered state names) and the actions call's `actions_needles` (the CRUD screen's listed author-key ids);
