@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=43 class=stale
leaf-text: A graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name
source: ctr-5upxmhpy «A graph's line, a part read's included, is `span="command" verb="graph"` (with `cause` when refused, an argument or a `part` outside 1 to 1000 refused `malformed`) and holds no node, state, id, name, part number or count.»
replace-with: A graph's line, a part read's included, is `span="command" verb="graph"` (with `cause` when refused, an argument or a `part` outside 1 to 1000 refused `malformed`) and holds no node, state, id, name, part number or count
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=43 class=omission
leaf-text: `unconfirmed`, and booleans only, never a state's value
source: ctr-5pwrxdkn «Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a part its part number and part count only»
replace-with: `unconfirmed`, booleans, and for a part its part number and part count only, never a state's value
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=49 class=omission
leaf-text: written by `to_json` alone as `{"nodes":[…],"states":[…]}`
source: ctr-5r6uwi5s «`{"nodes":[…],"states":[…]}` for a whole graph; a part read is `{"nodes":[…],"states":[…],"part":n,"parts":m}`, its states held in part 1 alone.»
replace-with: written by `to_json` alone as `{"nodes":[…],"states":[…]}` for a whole graph and, for a part read, `{"nodes":[…],"states":[…],"part":n,"parts":m}`, its states held in part 1 alone
@leaf_edit n=4 leaf=.claude/rules/observability.md line=32 class=stale
leaf-text: a graph's `command` line, `verb="graph"`, holds no node, state, id or name
source: ctr-5upxmhpy «A graph's line, a part read's included, is `span="command" verb="graph"` (with `cause` when refused, an argument or a `part` outside 1 to 1000 refused `malformed`) and holds no node, state, id, name, part number or count.»
replace-with: a graph's `command` line, a part read's included, `verb="graph"`, holds no node, state, id, name, part number or count
@leaf_edit n=5 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: `unconfirmed` and booleans, never a state's value
source: ctr-5pwrxdkn «Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a part its part number and part count only»
replace-with: `unconfirmed`, booleans and, for a part, its part number and part count, never a state's value
