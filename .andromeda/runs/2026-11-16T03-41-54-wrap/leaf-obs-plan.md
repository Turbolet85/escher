@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=59
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=41
last: - 2026-10-06: With the workspace's tracing 0.1.44, `tracing::
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=12 class=omission
leaf-text: and `stand_mcp_tools`, which installs a hand-written in-process recorder (not escher's sink) that prints nothing.
source: dec-64bjtz2h «except: The MCP tools and layout stand checks install their own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.»
replace-with: and `stand_mcp_tools` and the layout stand check `stand_layout`, each of which installs its own hand-written in-process recorder (not escher's sink) that prints nothing.
@leaf_edit n=2 leaf=.claude/rules/observability.md line=18 class=omission
leaf-text: and `stand_mcp_tools`, which installs a hand-written in-process recorder that prints nothing)
source: dec-4n2hhddr «except: Telemetry checks install escher's sink, the MCP tools and layout stand checks their own recorder, the paint-tree bench reads one opt-in env var, a re-run child prints ids or digests for its parent.»
replace-with: and `stand_mcp_tools` and the layout stand check `stand_layout`, each of which installs its own hand-written in-process recorder that prints nothing)
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=49 class=omission
leaf-text: nor does auto-exploration, which opens no span and emits no event of its own,
source: dec-zhnzewyk «decision/layout-read-opens-no-span · The layout read opens no span — The driver's in-process layout read opens no span and emits no event, runs outside the command span, and writes no escher_ line at TRACE.»
replace-with: nor does the driver's in-process layout read, which opens no span, emits no event, runs outside the command span and writes no `escher_` line at TRACE (guarded by `stand_layout`), nor does auto-exploration, which opens no span and emits no event of its own,
@leaf_edit n=4 leaf=.claude/rules/observability.md line=24 class=omission
leaf-text: nor auto-exploration, which opens no span and emits no event of its own,
source: dec-zhnzewyk «decision/layout-read-opens-no-span · The layout read opens no span — The driver's in-process layout read opens no span and emits no event, runs outside the command span, and writes no escher_ line at TRACE.»
replace-with: nor the driver's in-process layout read, which opens no span, emits no event, runs outside the command span and writes no `escher_` line at TRACE, nor auto-exploration, which opens no span and emits no event of its own,
