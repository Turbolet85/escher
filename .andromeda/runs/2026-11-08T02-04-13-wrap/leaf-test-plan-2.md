@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=65
last: - 2026-11-07: A regression test ported from upstream across 
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: (32 readers, `stand_cells_served` and `stand_load_page` the thirtieth and thirty-first, `stand_act_popup` the thirty-second;
source: fct-7m7lxsmy «`tests/session_common/mod.rs`, read through `mod session_common;` by 33 checks, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md (`stand_graph.rs` the latest»
replace-with: (33 readers, `stand_cells_served` and `stand_load_page` the thirtieth and thirty-first, `stand_act_popup` the thirty-second, `stand_graph` the thirty-third;
@leaf_edit n=2 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: `screenshot_of` · `lint` · `snapshot_part`
source: fct-prxcxiqp «`screenshot()` and `screenshot_of(id)`, `lint()`, `graph()`, `snapshot_part(n)` and the part reader `part(outcome)`»
replace-with: `screenshot_of` · `lint` · `graph` · `snapshot_part`
@leaf_edit n=3 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: the lint reader `linted` (a lint's findings),
source: fct-prxcxiqp «`graphed(session, call)` (a graph's `UiGraph`, `None` for a refused call or any other outcome; tests/blitz-tests/tests/session_common/mod.rs:134-147)»
replace-with: the lint reader `linted` (a lint's findings), the graph reader `graphed` (a graph's `UiGraph`),
@leaf_edit n=4 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: and `option_needles()` (the needle list of a select's options read from a snapshot), called by the same two;
source: fct-k5y3m6cu «and `graph_needles()`, the component and state names of the UI graph as needles, called by the same two»
replace-with: `option_needles()` (the needle list of a select's options read from a snapshot), called by the same two, and `graph_needles()` (the component and state names of the UI graph as needles), called by the same two;
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=25 class=stale
leaf-text: (the twelve verbs; its stderr
source: fct-63656bwh «the agent talks to the driver's own MCP surface, `escher-session mcp` (the thirteen verbs)»
replace-with: (the thirteen verbs; its stderr
@leaf_edit n=6 leaf=.claude/rules/verification-harness.md line=26 class=stale
leaf-text: `isolated` (exactly the twelve `mcp__escher__*` tools, one server `escher`)
source: fct-szqqqnrt «`isolated` = the init tools exactly the thirteen `mcp__escher__{verb}` tools, one MCP server `escher` connected»
replace-with: `isolated` (exactly the thirteen `mcp__escher__*` tools, one server `escher`)
