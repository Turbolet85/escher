@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=13 class=omission
leaf-text: construct (`from_html`, `from_component`, `from_vdom` — which
source: ctr-xrs2rifi «Harness::from_component_alone(component, props maker, HarnessOptions) builds a harness over one component mounted as the one child of an anonymous root, the props maker run once in that root's scope»
replace-with: construct (`from_html`, `from_component`, `from_component_alone(component, props maker, options)` — one component mounted as the one child of an anonymous root, its props a maker run once in that root's scope, riding `from_vdom` and adding no carrier, `vdom_alone` its public mount, on which a caller provides a root context before the harness builds —, `from_vdom` — which
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=14 class=omission
leaf-text: and beside them `stand::boot_cells(options)` boots Cells
source: ctr-emytbabz «stand::boot_alone(component, props maker, HarnessOptions) boots a component alone at the stand's viewport and font, with TimerTicks provided on the root before the build.»
replace-with: and beside them `stand::boot_alone(component, props maker, options)` boots a component alone at the stand's viewport and font, `TimerTicks` provided on the root before the build, `stand::boot_cells(options)` boots Cells
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=16 class=stale
leaf-text: with its fourteen verbs, the driver's verbs other than `layout`,
source: ctr-epkgn7ep «A step holds `tool` among the fifteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.»
replace-with: with its fifteen verbs, the driver's verbs,
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=16 class=stale
leaf-text: exactly the fourteen `mcp__escher__*` tools
source: ctr-enkhxjrd «True when the init tools are exactly the fifteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty»
replace-with: exactly the fifteen `mcp__escher__*` tools
