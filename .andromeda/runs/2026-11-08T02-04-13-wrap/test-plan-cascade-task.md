@task_prose part=session
Chunk 2026-11-07-ui-graph added one read of the screen's UI graph: a new instance verb graph (VERBS 12 to 13, instance verbs 8 to 9) answering every snapshot node with its owner, component, handlers, declared and confirmed writes, and each registered state's writers and reading components.
dioxus-native-dom gained the graph module (DioxusDocument::graph, UiGraph, WriteFlag Undeclared and Unconfirmed) and the signal_registry module (use_graph_state, DeclaredWrite, SignalRegistry), a reserved fifth id form owner///key with virtual_node_id, and the bridge dispatch now marks a dispatch where a registry exists.
The snapshot gained NodeState.clickable after focused (NODE_FIELDS 13 to 14, Reading::ALL 11 to 12, FieldKind 10 to 12, describe row [13, 14, 12, 3, 8, 4, 15]); the capability statement's not-built click line was retired.
New tests: stand_graph (6 tests, both layout modes), graph::tests and signal_registry::tests units, CLICKABLE table, graph checks on the command line, MCP and the two log checks; dioxus-native-dom units 83 to 100, escher-driver units unchanged at 100.
Measured: run stand 244 passed, 0 failed, 7 ignored over 52 stand files; Cells screen 53,859 bytes, 740 lines, 6 parts; Cells graph answer 293,869 bytes, an unpaged-graph stated limit owned by residual res-7cc5bi4k; CI on HEAD green 17 of 17.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=tests/blitz-tests/tests/stand_graph.rs
@task_file n=4 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=5 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot_text.rs
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_diff.rs
@task_file n=8 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=9 path=packages/dioxus-native-dom/src/dioxus_document_tests.rs
@task_file n=10 path=packages/escher-driver/src/schema.rs
@task_file n=11 path=packages/escher-driver/src/schema_tests.rs
@task_file n=12 path=packages/escher-driver/src/command.rs
@task_file n=13 path=packages/escher-driver/src/json.rs
@task_file n=14 path=packages/escher-driver/src/describe.rs
@task_file n=15 path=packages/escher-driver/src/mcp.rs
@task_file n=16 path=packages/escher-driver/src/mcp_tests.rs
@task_file n=17 path=packages/escher-driver/src/recording_tests.rs
@task_file n=18 path=packages/escher-driver/src/cli_tests.rs
@task_file n=19 path=examples/seven_guis/src/tasks/mod.rs
@task_file n=20 path=examples/seven_guis/tests/common/mod.rs
@task_file n=21 path=examples/seven_guis/tests/host_log.rs
@task_file n=22 path=examples/seven_guis/tests/mcp_log.rs
@task_file n=23 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=24 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=25 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=26 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=27 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
@task_file n=28 path=tests/blitz-tests/tests/stand_diff.rs
@task_file n=29 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=30 path=tests/blitz-tests/tests/stand_act_spans.rs
@task_file n=31 path=tests/blitz-tests/tests/stand_act_scroll.rs
@task_file n=32 path=tests/blitz-tests/tests/stand_cells_served.rs
@task_file n=33 path=tests/blitz-tests/tests/stand_snapshot_parts.rs
@task_file n=34 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
@task_file n=35 path=tests/blitz-tests/checks/counter-cli.json
@task_file n=36 path=tests/blitz-tests/checks/counter-mcp.json
@task_file n=37 path=docs/capabilities.md
@task_file n=38 path=README.md
@task_file n=39 path=scripts/cold-agent.sh
@task_file n=40 path=.github/scripts/test_cold_agent.py
@task_item n=1 part=touches
The dioxus-native-dom unit-test count moved from 83 to 100 #[test] functions: every row stating 83 unit tests is stale; the 83 % coverage floor rows share the token and stay true.
@task_item n=2 part=touches
The dioxus-native-dom unit coverage list (snapshot builder, text form, parts, diff, stable id, bridge, mutation writer, events) now also covers the UI graph reader, the signal registry, the reserved virtual id form and the clickable reading.
@task_item n=3 part=touches
The driver verb count moved from twelve to thirteen and the instance-verb count from eight to nine: graph sits after lint at the instance level; rows reading the twelve verbs or the eight instance verbs as today's count are stale, while a row naming lint the eighth instance verb stays true.
@task_item n=4 part=touches
The node-key count moved from thirteen to fourteen: clickable joins after focused in NODE_FIELDS, the JSON node, the text token and the help of snapshot text and changed.
@task_item n=5 part=touches
The describe count row moved from [12, 13, 12, 3, 8, 4, 15] to [13, 14, 12, 3, 8, 4, 15], stated in the scope summary and in the unit strategy.
@task_item n=6 part=touches
The newest run stand reading moved from 237 passed, 0 failed, 7 ignored over 51 files to 244 passed, 0 failed, 7 ignored over 52 files, stand_graph picked up by its stand_ prefix with no script change; the CI-integration current stand-file count is stale too, while dated readings at earlier chunks stay historical.
@task_item n=7 part=touches
The Cells screen text moved from 47,089 to 53,859 bytes and from 5 to 6 parts, its lines unmoved at 740; the stand Cells served check's PARTS constant reads 6.
@task_item n=8 part=touches
The cold-agent pipe's verb set gained graph (thirteen): the allowed tool set the judge's valid_step admits and the isolated check's init tool set now count thirteen mcp__escher tools; the fake server answers graph with an empty nodes and states object; the capability-unused set is unchanged, graph not required of a cold agent.
@task_item n=9 part=touches
The MCP listing now holds thirteen tools with graph after lint; graph joins the closed sets of the tool_call span's tool and the command span's verb, a graph command line recording verb alone, no span field added.
@task_item n=10 part=touches
The capability-statement pin now reads thirteen verbs and fourteen node keys in the capability doc and the thirteen driver commands as tools in the readme, all green, with no change to the pin's test code; the doc's not-built reacts-to-a-click line is retired.
@task_item n=11 part=touches
The two committed Counter checks (command-line and MCP) were re-recorded through record_checks.sh, the clickable key moving their bytes, and stand_replay replays them green.
@task_item n=12 part=touches
The stand shared table module gained the Drive enum and GRAPH_DRIVES, seven rows; the session shared module gained the graph call builder and the graphed reader, so its reader count and the shared table module's reader count can move with stand_graph.
@task_item n=13 part=touches
The stand-repeat leg's stand_ glob now takes stand_graph; the seventeen named non-stand targets are unchanged.
@task_item n=14 part=touches
The stand non-lean tasks check gained the CLICKABLE table and a clickable-without-role count; the listener mark count stays 677.
@task_item n=15 part=touches
The session-over-held-instance boundary gains a read-only verb: graph runs no step, changes nothing, holds no typed text, feeds the record of ids from the graph's node ids, and an argument to it is refused malformed with nothing run.
@task_item n=16 part=touches
The host and mcp-role log checks now make one graph call each and read 0 id, component and state-name needles at trace under both builds, through a new graph_needles helper in seven_guis' shared test module.
@task_item n=17 part=touches
The bridge test child gained a dispatch test, and DioxusDocument gained a cfg(test) dispatch_to_vdom: any standing statement that no function under test gained a cfg(test) shim is no longer true of the bridge tests.
@task_item n=18 part=touches
The bridge dispatch, DioxusEventHandler::handle_event, now sets and clears the registry's dispatch where a SignalRegistry is provided at the root, a change on every Dioxus document's event path, inert without a registry; the buffered render, initial_build, poll and flush_queued_mounted_events are untouched.
@task_item n=19 part=touches
The stable element id grammar gains a reserved fifth form owner///key; element_id's tests gain a no-id-holds-the-separator check and a virtual_node_id check, every earlier element_id test reading as before.
@task_item n=20 part=touches
The escher-driver unit count stays at 100: the JSON forms, schema, command, describe, MCP, recording and command-line tests extend existing cases with graph and clickable rows; its unit coverage list now includes the graph verb's schema and JSON form.
@task_item n=21 part=touches
The driver command-span check gains one graph read per layout mode, a command line recording verb alone.
@task_item n=22 part=touches
The command-line and MCP child checks each gain a graph case answering one JSON line of the UI graph.
@task_item n=23 part=touches
The per-crate coverage floor of dioxus-native-dom (83 %) now covers the two new modules graph and signal_registry; the floor value is unchanged.
@task_item n=24 part=touches
The seven_guis tasks register their signals through a use_graph_signal wrapper with declarations read off the code; the stand shell's active signal and stand.rs's linked stay plain use_signal.
@task_fact n=1 slug=integration-test-strategy
fact: stand_graph holds six tests over both layout modes: Counter's count declared at boot and confirmed after a click, every stand task's declarations agreeing with its code once its handlers have run, the served screens' handlers, owners and clicks, every graph node a snapshot node with no served id holding the reserved owner///key form, a graph read changing nothing and holding no typed text, and the whole Cells answer within the answer bound; the graph verb equals DioxusDocument::graph on every read.
@task_fact n=2 slug=unit-test-strategy
fact: graph::tests pins the UI graph reader: each node's component, current listeners with mounted included and a removed one gone, a declared write reading declared before a dispatch and confirmed after, two reader components reading a state the writer does not, an option owned by its select, a prefix declaration naming its elements alone, a drifted declaration reading both flags, a write by an element since gone not handed to its ancestor, and a repeated name reading its ordinal with a page holding no state reading none.
@task_fact n=3 slug=unit-test-strategy
fact: signal_registry::tests pins the registry, its mechanism control first: a handler's write recorded against its dispatch and one outside any dispatch against none, a state's entry leaving with its scope and its Debug printing counts only, a declaration naming its element or its prefix, and a page with no registered state holding no registry.
@task_fact n=4 slug=unit-test-strategy
fact: the bridge test a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is drives the dispatch through a cfg(test) DioxusDocument::dispatch_to_vdom and reads the registry's dispatch record.
@task_fact n=5 slug=unit-test-strategy
fact: the snapshot unit test an_element_with_an_app_click_listener_reads_clickable pins NodeState.clickable: true where the element's current listeners hold click and it is no disabled form control.
@task_fact n=6 slug=unit-test-strategy
fact: element_id's tests no_id_the_walk_writes_holds_the_virtual_separator and a_virtual_node_id_spells_the_reserved_form_and_names_no_element pin the reserved owner///key form over an adversarial page; virtual_node_id returns None for an empty or slash-bearing key and no caller produces one.
@task_fact n=7 slug=test-data-fixtures
fact: GRAPH_DRIVES in the stand shared table module (seven rows of Drive steps) drives each stand task's conditional writes once; Cells clicks A0, C0, A0 and leaves its keyed editor once by Enter and once by Tab, so no stand write reads unconfirmed or undeclared.
@task_fact n=8 slug=test-scope-summary
fact: the CLICKABLE table of stand_non_lean_tasks reads clickable-without-role counts per task in both layout modes: Counter, Flight Booker, Timer, CRUD and Temperature Converter 0 each, Circle Drawer 1, Cells 676; counter-increment reads clickable, counter-value and cells-grid do not.
@task_fact n=9 slug=test-data-fixtures
fact: the Cells graph answer measured 293,869 bytes on a fresh start cells (547,685 at first build, 522,692 after the readers narrowing); graph takes no argument and answers the whole screen at once, the stated limit An unpaged graph owned by residual res-7cc5bi4k, target next.
@task_fact n=10 slug=e2e-test-strategy
fact: the graph verb is exercised as a process by cli_commands graph_answers_one_line_of_the_ui_graph and over the mcp child by mcp_server a_graph_tool_call_answers_its_line_of_the_ui_graph.
@task_fact n=11 slug=integration-test-strategy
fact: stand_mcp_tools a_graph_tool_call_answers_the_readers_graph_and_parents_a_command_recording_its_verb_alone holds the graph tool call in process in both layout modes, its tool_call span parenting one command span recording verb alone.
@task_fact n=12 slug=integration-test-strategy
fact: a graph answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value: a sentinel typed into crud-name occurs 0 times in it.
@task_fact n=13 slug=test-scope-summary
fact: a stated limit of the graph's checks: unconfirmed fires only once a dispatch of the handler's event passed through its element and no run wrote the state; a conditional write a run skipped stays unflagged, a stated limit on the operator's word.
@task_item n=1 part=untouched
blitz-dom's tests, covered behaviours and pipeline tests: blitz-dom, every manifest, the lock file and deny.toml are byte-unchanged.
@task_item n=2 part=untouched
The blitz-test-harness Harness surface (construction, pump, settle, screenshot, text_contrast, input and inspection helpers, HarnessOptions) is unchanged.
@task_item n=3 part=untouched
The agent-run.sh 5-command contract, its exit grammar, events and state are unchanged; the stand_ prefix picks the new file with no script change.
@task_item n=4 part=untouched
The mutation writer and mutation buffer, byte-equal to upstream, and the dioxus_runtime_probe check's 10 tests, still green, are unchanged.
@task_item n=5 part=untouched
The WPT runner, its workflows, scoring and fixtures, the apps/browser tests, blitz-vibey-script, blitz-net, blitz-paint and escher-telemetry tests are unreached.
@task_item n=6 part=untouched
The coverage floor values (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %), the mutation tier, the fast and slow split, cache, package installs and failure-log uploads are unchanged.
@task_item n=7 part=untouched
The shared fixtures module and its pages, HeldNet, LoadNet and LateNet, and the stand's pinned options are unchanged.
@task_item n=8 part=untouched
Dated historical readings stay as they are: the 2026-10-25 cold-agent live run's twelve tools and 66 calls, and earlier run stand readings over 51 files at their own chunks.
@task_item n=9 part=untouched
No third-party test framework or dev-dependency was added; test-that stays at 0.6.0.
