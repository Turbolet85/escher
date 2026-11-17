@leaf_group group=architecture+security-plan+design-system+layout-templates+test-plan+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=165
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=36 class=stale
leaf-text: the layout read (`Session::layout`, in process alone)
source: ctr-5uc2ynp6 «A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span;»
replace-with: the layout read (`Session::layout`, in process and as the `layout` verb)
@leaf_edit n=2 leaf=CLAUDE.md line=112 class=omission
leaf-text: `graph` · `actions` · `status`
source: ctr-aj725x7q «Usage reads `snapshot [--part <part>]`, `start <app> [--record <dir>]`, `host role: serve <app> --session <dir> [--record <dir>]`, `mcp role: mcp [--record <dir>]` and `coverage role: coverage <app>»
replace-with: `graph` · `actions` · `layout --id` · `status`
@leaf_edit n=3 leaf=CLAUDE.md line=113 class=stale
leaf-text: its tools the fourteen verbs, `graph` after `lint`, `actions` after `graph`, `help` last
source: ctr-stdygm64 «The fifteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.»
replace-with: its tools the fifteen verbs, `graph` after `lint`, `actions` after `graph`, `layout` after `actions`, `help` last
@leaf_edit n=4 leaf=CLAUDE.md line=122 class=stale
leaf-text: one verb table of fourteen verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` · `actions` asked of the held instance
source: ctr-s4mfww2m «The verb table holds fifteen verbs, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.»
replace-with: one verb table of fifteen verbs in three levels, `snapshot` · `click` · `type` · `press` · `advance` · `scroll` · `screenshot` · `lint` · `graph` · `actions` · `layout` asked of the held instance
@leaf_edit n=5 leaf=CLAUDE.md line=122 class=omission
leaf-text: `snapshot`, `screenshot`, `lint`, `graph` and `actions` waiting on none
source: ctr-bsesr4vq «`snapshot`, `screenshot`, `lint`, `graph` and `layout` run no step and wait on nothing.»
replace-with: `snapshot`, `screenshot`, `lint`, `graph`, `actions` and `layout` waiting on none
@leaf_edit n=6 leaf=CLAUDE.md line=122 class=stale
leaf-text: the driver's fourteen verbs as MCP tools
source: ctr-stdygm64 «The fifteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.»
replace-with: the driver's fifteen verbs as MCP tools
@leaf_edit n=7 leaf=CLAUDE.md line=122 class=stale
leaf-text: the fourteenth and last verb
source: ctr-s4mfww2m «The verb table holds fifteen verbs, `graph` listed after `lint`, `actions` after `graph` and `layout` after `actions`.»
replace-with: the fifteenth and last verb
@leaf_edit n=8 leaf=CLAUDE.md line=122 class=retired
leaf-text: opens no span; no verb, MCP tool, session socket call or kept check reaches it
source: dec-xgwbuenq «Layout answer crosses the carriers — The layout answer crosses the session socket's call, the command line and MCP and rests in a kept check as the layout verb's answer, holding stable ids, closed words»
replace-with: opens no span of its own; the `layout` verb carries its answer — ids, closed words and numbers, never a value, typed text or `NodeId` — on the session socket's call, the command line and MCP and keeps it in a kept check, inside its call's command span, a ratified boundary widening
@leaf_edit n=9 leaf=CLAUDE.md line=122 class=omission
leaf-text: crossing in the snapshot and the diff and an option
source: dec-rt2rzqbq «The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, the placing words select and select-list in a layout answer, and as pixels»
replace-with: crossing in the snapshot and the diff, the placing words `select` and `select-list` in a layout answer, and an option
