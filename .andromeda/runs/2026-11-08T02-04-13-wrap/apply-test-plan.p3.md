@apply_row n=1 ref_file=fanout-test-plan--cascade-g4.md ref_n=2 v=applied edit=E1
@apply_row n=2 ref_file=fanout-test-plan--cascade-g4.md ref_n=3 v=applied edit=E2
@apply_row n=3 ref_file=fanout-test-plan--cascade-g4.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-test-plan--cascade-g4.md ref_n=5 v=applied edit=E4
@apply_row n=5 ref_file=fanout-test-plan--cascade-new-unit-test-strategy.md ref_n=1 v=applied edit=E5
@apply_row n=6 ref_file=fanout-test-plan--cascade-new-unit-test-strategy.md ref_n=2 v=applied edit=E6
@apply_row n=7 ref_file=fanout-test-plan--cascade-new-unit-test-strategy.md ref_n=3 v=applied edit=E7
@apply_row n=8 ref_file=fanout-test-plan--cascade-new-unit-test-strategy.md ref_n=4 v=applied edit=E8
@apply_row n=9 ref_file=fanout-test-plan--cascade-new-unit-test-strategy.md ref_n=5 v=applied edit=E9
@apply_row n=10 ref_file=fanout-test-plan--cascade-g4.md ref_n=6 v=applied edit=E10
@apply_row n=11 ref_file=fanout-test-plan--cascade-g4.md ref_n=7 v=applied edit=E11
@apply_row n=12 ref_file=fanout-test-plan--cascade-g4.md ref_n=8 v=applied edit=E12
@apply_row n=13 ref_file=fanout-test-plan--cascade-g4.md ref_n=9 v=applied edit=E13
@apply_row n=14 ref_file=fanout-test-plan--cascade-g4.md ref_n=10 v=applied edit=E14
@apply_row n=15 ref_file=fanout-test-plan--cascade-g5.md ref_n=1 v=applied edit=E15
@apply_row n=16 ref_file=fanout-test-plan--cascade-g5.md ref_n=2 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=1
@apply_row n=17 ref_file=fanout-test-plan--cascade-g5.md ref_n=3 v=applied edit=E16
@apply_row n=18 ref_file=fanout-test-plan--cascade-g5.md ref_n=4 v=folded_into to_file=fanout-test-plan--cascade-g5.md to_n=3

@fact id=fct-klxgwgcy expect_rev=1
old: twelve verbs in a stated order
new: thirteen verbs in a stated order
old: the eight instance-level verbs, `screenshot` after `scroll` since 2026-10-12-headless-screenshot, `lint` after `screenshot`, then
new: the nine instance-level verbs, `screenshot` after `scroll` since 2026-10-12-headless-screenshot, `lint` after `screenshot`, `graph` after `lint` (no argument; fields `nodes` and `states`), then
old: the field kinds, ten since 2026-10-12-headless-screenshot
new: the field kinds, twelve
old: and `Png`)
new: and `Png`; then `GraphNodes` (`graph-nodes`) and `GraphStates` (`graph-states`))
old: the thirteen node keys named in order, each with its help (`differs` after `bounds`)
new: the fourteen node keys named in order, each with its help (`differs` after `bounds`, `clickable` after `focused`)
old: each outcome with its verb's fields in order
new: each outcome with its verb's fields in order, the graph's `nodes` and `states` among them
old: a changed node with `differs` after `bounds`
new: a changed node with `differs` after `bounds`, and a node's `clickable` written beside `focused`
old: the span's two tests and its controls at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md
new: the span's two tests and its controls at escher-0.1.0/chunks/2026-10-07-driver-command-spans/report.md, the verb table's thirteen verbs, twelve field kinds and fourteen node keys at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-fghufsse expect_rev=1
old: with `VERBS` at 12, causes
new: with `VERBS` at 13, causes
old: `[12, 13, 12, 3, 8, 4, 15]`
new: `[13, 14, 12, 3, 8, 4, 15]`
old: as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
new: as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, `VERBS` and the count row at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)

@fact id=fct-zot44d7e expect_rev=1
old: keeps its outer assertions and its row walk
new: asserts the thirteen verbs of the table in `VERBS`' order, `graph` after `lint`, keeps its outer assertions and its row walk
old: — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md
new: — as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md, the thirteen verbs at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-3ferhyj7 expect_rev=1
old: which speaks the twelve tools, answering `lint` with `{"findings":[]}` (.github/scripts/test_cold_agent.py:140-141)
new: which speaks the thirteen tools, answering `lint` with `{"findings":[]}` and `graph` with `{"nodes":[],"states":[]}`, no verdict requiring `graph` of a cold agent (.github/scripts/test_cold_agent.py:140-141)

@fact after=fct-ritaop23 file=unit-test-strategy gid=new:dnd-graph-reader kind=procedure subject=unit as=n1
- **dioxus-native-dom (the UI graph):** `graph::tests` pins the UI graph reader — each node's component; its current listeners, `mounted` included and a removed one gone; a declared write reading declared before a dispatch and confirmed after; two reader components reading a state the writer does not; an option owned by its select; a prefix declaration naming its elements alone; a drifting declaration reading both flags; a write by an element since gone not handed to its ancestor (`a_write_by_an_element_gone_since_is_not_handed_to_its_ancestor`); and a repeated name reading its ordinal, a page holding no state reading none (packages/dioxus-native-dom/src/graph.rs:401-670) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n1 file=unit-test-strategy gid=new:dnd-signal-registry kind=procedure subject=unit as=n2
- **dioxus-native-dom (the signal registry):** `signal_registry::tests` pins the registry, its mechanism control first — a handler's write recorded against its dispatch and one outside any dispatch against none; a state's entry leaving with its scope and its `Debug` printing counts alone; a declaration naming its element or its prefix; and a page with no registered state holding no registry (packages/dioxus-native-dom/src/signal_registry.rs:272-373) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n2 file=unit-test-strategy gid=new:dnd-bridge-dispatch kind=procedure subject=unit as=n3
- **dioxus-native-dom (the bridge's dispatch record):** `bridge_tests::a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is` drives a dispatch through the test-only `#[cfg(test)]` `DioxusDocument::dispatch_to_vdom` and reads the registry's dispatch record where a state is registered, a page with none acting as before (packages/dioxus-native-dom/src/dioxus_document_tests.rs:269-322; packages/dioxus-native-dom/src/dioxus_document.rs:240-250) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n3 file=unit-test-strategy gid=new:dnd-snapshot-clickable kind=procedure subject=unit as=n4
- **dioxus-native-dom (the snapshot's `clickable`):** `an_element_with_an_app_click_listener_reads_clickable` pins `NodeState.clickable` true where the element's current listeners hold `click` and it is no disabled form control (packages/dioxus-native-dom/src/snapshot.rs:1038-1059) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n4 file=unit-test-strategy gid=new:dnd-reserved-id-form kind=procedure subject=unit as=n5
- **dioxus-native-dom (the reserved id form):** `element_id`'s tests `no_id_the_walk_writes_holds_the_virtual_separator` and `a_virtual_node_id_spells_the_reserved_form_and_names_no_element` pin the reserved `{owner}///{key}` form over an adversarial page; `virtual_node_id` returns `None` for an empty or `/`-bearing key, and no caller produces one, the walk not calling it (packages/dioxus-native-dom/src/element_id.rs:722-775; packages/dioxus-native-dom/src/element_id.rs:42-47) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=fct-t7rg232i file=integration-test-strategy gid=new:session-ui-graph kind=procedure subject=integration as=n6
- **Session ↔ held instance, the UI graph:** `stand_graph` (6) holds, over both layout modes, Counter's `count` declared at boot and confirmed after a click; every stand task's declarations agreeing with its code once its handlers have run (`GRAPH_DRIVES`); the served screens' handlers, owners and clicks; every graph node a snapshot node, no served id holding the reserved `{owner}///{key}` form; a graph read changing nothing and holding no typed text; and the whole Cells answer within the answer bound — the `graph` verb equal to `DioxusDocument::graph` on every read (tests/blitz-tests/tests/stand_graph.rs:106-394) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n6 file=integration-test-strategy gid=new:session-graph-mcp-in-process kind=procedure subject=integration as=n7
- **Session ↔ held instance, the graph over MCP in process:** `stand_mcp_tools`' `a_graph_tool_call_answers_the_readers_graph_and_parents_a_command_recording_its_verb_alone` holds a `graph` tool call in process in both layout modes, answering the reader's graph, its `tool_call` span parenting one `command` span that records `verb` alone (tests/blitz-tests/tests/stand_mcp_tools.rs:771-843) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact after=@n7 file=integration-test-strategy gid=new:graph-answer-no-value kind=procedure subject=integration as=n8
- **The graph answer holds no value:** a `graph` answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value — a sentinel typed into `crud-name` occurs 0 times in it — and an argument to `graph` is refused `malformed` with nothing run (`stand_graph`, `cli_commands`, `mcp_server`, `stand_mcp_tools`) — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-adkwyvky expect_rev=1
old: and each also drives one `lint` and one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool calls and 8 `command` lines — still reading 0 needles on stderr at `trace`;
new: and each also drives one `lint`, one `snapshot` part read and one `graph` call — `host_log` eight calls and 8 command-span lines, `mcp_log` eleven tool calls and 9 `command` lines — still reading 0 needles on stderr at `trace`, the graph's needles (each served component and registered state name, `common::graph_needles`; examples/seven_guis/tests/common/mod.rs:425-452) among them, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md;

@fact id=fct-3vcgyjbv expect_rev=1
old: hook-local `use_signal` state, where the stand keeps all its state, exposes neither link
new: hook-local `use_signal` state exposes neither link
old: read from the code, not run (tests/
new: read from the code, not run, and the stand registers each task's signals through `tasks::use_graph_signal` — `use_signal` with dioxus-native-dom's `use_graph_state`, whose entry holds a `ReactiveContext` subscribed to the signal — so their writers and reading components reach the UI graph through the fork's `SignalRegistry`, with no dioxus-core change and no manifest moved, read by `graph::tests` and `stand_graph` (tests/
old: — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
new: — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md, the stand's registration at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md

@fact id=fct-csryo2zo expect_rev=1
old: each of the eight instance-level verbs is accepted
new: each of the nine instance-level verbs is accepted
old: a hosted `lint` answers status `0` and one line `{"findings":[]}` (examples/seven_guis/tests/cli_commands.rs:494-515);
new: a hosted `lint` answers status `0` and one line `{"findings":[]}` (examples/seven_guis/tests/cli_commands.rs:494-515); `graph_answers_one_line_of_the_ui_graph` runs `graph` as a process and reads one line of the UI graph (examples/seven_guis/tests/cli_commands.rs:467-492);

@fact id=fct-gtzxj2kr expect_rev=1
old: `mcp_server` (6)
new: `mcp_server` (7)
old: offering the verbs in order — twelve —
new: offering the verbs in order — thirteen, `graph` after `lint` —
old: a `lint` tool call answering its line of findings (examples/seven_guis/tests/mcp_server.rs:364-382),
new: a `lint` tool call answering its line of findings (examples/seven_guis/tests/mcp_server.rs:364-382), a `graph` tool call answering its line of the UI graph (`a_graph_tool_call_answers_its_line_of_the_ui_graph`; examples/seven_guis/tests/mcp_server.rs:339-360),

@pattern id=tp3-01
fixed: twelve verbs in a stated order
@pattern id=tp3-02
fixed: the thirteen node keys named in order
@pattern id=tp3-03
fixed: the field kinds, ten
@pattern id=tp3-04
fixed: `VERBS` at 12
@pattern id=tp3-05
fixed: speaks the twelve tools
@pattern id=tp3-06
fixed: `host_log` seven calls and 7 command-span lines
@pattern id=tp3-07
fixed: where the stand keeps all its state
@pattern id=tp3-08
fixed: each of the eight instance-level verbs
@pattern id=tp3-09
fixed: offering the verbs in order — twelve
@pattern id=tp3-10
fixed: `mcp_server` (6)
