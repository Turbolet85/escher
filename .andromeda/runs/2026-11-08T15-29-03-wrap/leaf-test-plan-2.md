@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=65
last: - 2026-11-07: A regression test ported from upstream across 
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=15 class=stale
leaf-text: (33 readers, `stand_cells_served` and `stand_load_page` the thirtieth and thirty-first, `stand_act_popup` the thirty-second, `stand_graph` the thirty-third;
source: fct-7m7lxsmy «and `tests/session_common/mod.rs`, read through `mod session_common;` by 34 checks, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md (`stand_actions.rs` the latest, then `stand_graph.rs`»
replace-with: (34 readers, `stand_cells_served` and `stand_load_page` the thirtieth and thirty-first, `stand_act_popup` the thirty-second, `stand_graph` the thirty-third, `stand_actions` the thirty-fourth;
@leaf_edit n=2 leaf=.claude/rules/testing.md line=15 class=stale
leaf-text: what 31 checks — 23 `stand_*` files (`stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select`, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint, `stand_act_popup`)
source: fct-osorhgow «32 checks declare `mod fixtures;` — 24 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`; `stand_act_popup`; `stand_actions`)»
replace-with: what 32 checks — 24 `stand_*` files (`stand_snapshot_parts` since 2026-10-20-large-screen-in-parts, `stand_act_select`, `stand_load_wait` since 2026-10-20-load-wait, `stand_lint` since 2026-10-19-visual-lint, `stand_act_popup`, `stand_actions`)
@leaf_edit n=3 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: and `graph_needles()` (the component and state names of the UI graph as needles), called by the same two;
source: fct-k5y3m6cu «and `actions_needles()`, the CRUD screen's listed author-key ids holding a `-` as needles, called by the same two, each making one `actions` call and reading 0 id, name and actions needles at `trace`»
replace-with: and `graph_needles()` (the component and state names of the UI graph as needles), called by the same two, and `actions_needles()` (the CRUD screen's listed author-key ids holding a `-` as needles), called by the same two;
@leaf_edit n=4 leaf=.claude/rules/testing.md line=48 class=omission
leaf-text: as read at CI#42371051720's `Coverage report` job), the 97 % floor holding,
source: fct-fx4qekph «with escher-driver's `actions.rs` and `check_target` and dioxus-native-dom's `actionable_nodes` and `Readings` in the tree, CI#42493052318's `Coverage report` check passes under the same floors»
replace-with: as read at CI#42371051720's `Coverage report` job; with `actions.rs`, `check_target`, `actionable_nodes` and `Readings` in the tree CI#42493052318's `Coverage report` check passes under the same floors, the per-crate figures not read), the 97 % floor holding,
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=21 class=stale
leaf-text: runs the 23 `stand_*` readers of the shared module
source: fct-osorhgow «32 checks declare `mod fixtures;` — 24 `stand_*` files (`stand_snapshot_parts`; `stand_act_select`; `stand_load_wait`, reading `HeldNet`; `stand_lint`; `stand_act_popup`; `stand_actions`)»
replace-with: runs the 24 `stand_*` readers of the shared module
@leaf_edit n=6 leaf=.claude/rules/verification-harness.md line=25 class=stale
leaf-text: (the thirteen verbs;
source: fct-63656bwh «the agent talks to the driver's own MCP surface, `escher-session mcp` (the fourteen verbs)»
replace-with: (the fourteen verbs;
@leaf_edit n=7 leaf=.claude/rules/verification-harness.md line=26 class=stale
leaf-text: exactly the thirteen `mcp__escher__*` tools
source: fct-szqqqnrt «`isolated` = the init tools exactly the fourteen `mcp__escher__{verb}` tools, one MCP server `escher` connected»
replace-with: exactly the fourteen `mcp__escher__*` tools
