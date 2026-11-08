@proposal n=1 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-z2ynupl3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 1. Test Scope Summary
change: in the stand_non_lean_tasks clause replace «measured and pinned, the snapshot field for it deferred, `NODE_FIELDS` still thirteen» with «measured and pinned, the count unchanged since 2026-11-07-ui-graph shipped the snapshot field for it, `NodeState.clickable`, and `NODE_FIELDS` fourteen»
sidecar: test-scope-summary: stand_non_lean_tasks no longer reads the click field deferred; clickable shipped, NODE_FIELDS fourteen (2026-11-07-ui-graph)
rationale: the report's Symbols / APIs ship NodeState.clickable and Schema / config moves NODE_FIELDS 13 to 14; its Counts section keeps the mark's count at 677
@proposal n=2 detector=D-new-detail severity=warning file=test-scope-summary anchor=fct-z2ynupl3 fact=F8 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 1. Test Scope Summary
change: after the stand_non_lean_tasks clause add «; since 2026-11-07-ui-graph its `CLICKABLE` table reads the clickable-without-role count per task in both layout modes — Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676 — `counter-increment` reading clickable and `counter-value` and `cells-grid` not»
sidecar: test-scope-summary: stand_non_lean_tasks' CLICKABLE table added (2026-11-07-ui-graph)
rationale: the report's Counts section states the new CLICKABLE table with no master site, and its Outcome reads counter-increment true, counter-value and cells-grid false
@proposal n=3 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-z2ynupl3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-dxcpaddh
section: ## 1. Test Scope Summary
change: read «`stand_mcp_tools` 7» for «`stand_mcp_tools` 6», adding «— since 2026-11-07-ui-graph a `graph` tool call answering the reader's graph and parenting one `command` span recording `verb` alone, in both layout modes —» beside its lint tool call clause
sidecar: test-scope-summary: stand_mcp_tools 6 -> 7, the graph tool call (2026-11-07-ui-graph)
rationale: the report's New text lists one new test in stand_mcp_tools.rs at 771-843, a_graph_tool_call_answers_the_readers_graph_and_parents_a_command_recording_its_verb_alone, and Outcome names it
@proposal n=4 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-z2ynupl3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: ## 1. Test Scope Summary
change: in the stand_act_spans clause after «eight verbs read back» add «; since 2026-11-07-ui-graph it reads the ninth instance verb `graph` too, one call per mode, its `command` line recording `verb` alone»
sidecar: test-scope-summary: stand_act_spans reads the graph verb's command line (2026-11-07-ui-graph)
rationale: the report's Coverage of new surfaces names stand_act_spans one call per mode for graph's instrumentation, and New text lists its read_graph at stand_act_spans.rs 230-247
@proposal n=5 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-tz35iqmq basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 1. Test Scope Summary
change: read «a screen of 53,859 bytes in 740 lines, 6 parts of `SNAPSHOT_TEXT_BUDGET`» for «a screen of 47,089 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`»
sidecar: test-scope-summary: the served Cells screen 47,089 B / 5 parts -> 53,859 B / 6 parts (2026-11-07-ui-graph)
rationale: the report's Counts section, pattern cells-parts, names test-scope-summary.md:325 stale: 53,859 B, 740 lines, parts 6, stand_cells_served PARTS 6
@proposal n=6 detector=D-cascade severity=warning file=body anchor=fct-jydhmppz basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: ## 1. Test Scope Summary
change: read «unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id and its reserved virtual form, the actionable-key check, the bridge and its dispatch record, the UI graph reader, the signal registry, the mutation writer and the document's events» for the list it holds, the rest of the line kept
sidecar: test-plan body §1: dioxus-native-dom's unit scope names the graph reader, the signal registry and the reserved id form (2026-11-07-ui-graph)
rationale: the report adds the graph and signal_registry modules with graph::tests and signal_registry::tests, a bridge dispatch test and the virtual_node_id tests, all unit-tier per its Coverage of new surfaces
@remark
F13 (the graph checks' unconfirmed limit) and stand_graph itself have no carrier among this group's picks, and the dioxus-native-dom 83->100 rows (test-scope-summary.md:21, :173) were not picked here: left to the new-detail pass and the other groups
