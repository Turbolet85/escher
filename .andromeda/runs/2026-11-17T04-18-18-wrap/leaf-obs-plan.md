@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing:
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=49 class=stale
leaf-text: nor does the driver's in-process layout read, which opens no span, emits no event, runs outside the command span and writes no `escher_` line at TRACE (guarded by `stand_layout`)
source: dec-zhnzewyk «The driver's layout read opens no span and emits no event of its own: read in process it writes no escher_ line at TRACE, and reached by the layout verb it runs inside that call's command span.»
replace-with: nor does the driver's layout read, which opens no span and emits no event of its own — read in process it writes no `escher_` line at TRACE (guarded by `stand_layout`), and reached by the `layout` verb it runs inside that call's `command` span
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=49 class=omission
leaf-text: an accepted `actions` (`Outcome::Actions`) and a part read (`Outcome::Part`) record `verb` alone too
source: ctr-5cyzlcp6 «Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout and a part read record verb alone, and a refusal records verb and cause alone.»
replace-with: an accepted `actions` (`Outcome::Actions`), an accepted `layout` and a part read (`Outcome::Part`) record `verb` alone too
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=43 class=omission
leaf-text: A part read's line is
source: ctr-xldqcthd «A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer; the layout read the call reaches opens no span of its own.»
replace-with: A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer, the layout read the call reaches opening no span of its own. A part read's line is
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=43 class=omission
leaf-text: The actions answer (`actions`, `Outcome::Actions`) carries
source: ctr-s7cswrn7 «Carries ids, closed words and numbers only, never a value or typed text; the sink at trace carries none of its keys (laid_out, placed, shifts).»
replace-with: The layout answer (`layout`) carries ids, closed words and numbers only, never a value or typed text; it leaves on the session socket, the command line and MCP and is kept whole in a recorded check, never logged or fielded, and the sink-installing host and its `mcp` role read at `trace` under both builds show none of its keys (`laid_out`, `placed`, `shifts`). The actions answer (`actions`, `Outcome::Actions`) carries
@leaf_edit n=5 leaf=.claude/rules/observability.md line=24 class=stale
leaf-text: nor the driver's in-process layout read, which opens no span, emits no event, runs outside the command span and writes no `escher_` line at TRACE
source: dec-zhnzewyk «The driver's layout read opens no span and emits no event of its own: read in process it writes no escher_ line at TRACE, and reached by the layout verb it runs inside that call's command span.»
replace-with: nor the driver's layout read, which opens no span and emits no event of its own — read in process it writes no `escher_` line at TRACE, and reached by the `layout` verb it runs inside that call's `command` span
@leaf_edit n=6 leaf=.claude/rules/observability.md line=24 class=omission
leaf-text: a `screenshot`, `lint`, `graph` or `actions` call's or a part read's
source: ctr-5cyzlcp6 «Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout and a part read record verb alone, and a refusal records verb and cause alone.»
replace-with: a `screenshot`, `lint`, `graph`, `actions` or `layout` call's or a part read's
@leaf_edit n=7 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: an option's label, on a select list opened and chosen
source: dec-5zkib22n «A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel or layout-answer key needle in what they write.»
replace-with: a layout call's `command` line, `verb="layout"`, holds nothing of its answer — `host_log` and `mcp_log` read at `trace` under both builds show no layout-answer key needle (`laid_out`, `placed`, `shifts`); an option's label, on a select list opened and chosen
@leaf_edit n=8 leaf=.claude/rules/observability.md line=32 class=omission
leaf-text: and the `coverage` role's line — ids, owner ids
source: ctr-nepx5i6r «The answer leaves on the session socket, the command line and MCP and is kept whole in a recorded check; it is never logged or fielded.»
replace-with: and a `layout` answer's ids, closed words and numbers, never a value or typed text, and the `coverage` role's line — ids, owner ids
