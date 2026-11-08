@task_prose part=session
The session added a read-only UI graph of a screen: a new driver verb graph that answers each snapshot node's id, owner, component, handlers with their declared and confirmed writes, and each registered state's writers and readers.
It added a snapshot field clickable (NodeState.clickable, text token clickable after focused), true where an element has a click listener and is no disabled form control; the capability line that said this reading was not built was retired.
Counts moved: VERBS 12 to 13 (instance verbs 8 to 9, graph after lint), NODE_FIELDS 13 to 14, FieldKind 10 to 12, Reading::ALL 11 to 12, describe's row to 13, 14, 12, 3, 8, 4, 15, the MCP listing to 13 tools.
It added a state registry hook use_graph_state with DeclaredWrite declarations in dioxus-native-dom, a reserved virtual id form owner///key with virtual_node_id, and registered every seven_guis task's signals through use_graph_signal.
Measured: the Cells screen text went from 47,089 to 53,859 bytes in 6 parts (740 lines); the Cells graph answer reads 293,869 bytes, kept as the stated limit An unpaged graph; run stand reads 244 passed, 0 failed, 7 ignored.
No dependency, manifest, blitz-dom, colour, font, layout or styling changed; no served screen's markup moved.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=tests/blitz-tests/tests/stand_graph.rs
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=6 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=7 path=packages/dioxus-native-dom/src/snapshot_text.rs
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot_diff.rs
@task_file n=9 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=10 path=packages/escher-driver/src/schema.rs
@task_file n=11 path=packages/escher-driver/src/schema/spec.rs
@task_file n=12 path=packages/escher-driver/src/command.rs
@task_file n=13 path=packages/escher-driver/src/execute.rs
@task_file n=14 path=packages/escher-driver/src/json.rs
@task_file n=15 path=packages/escher-driver/src/describe.rs
@task_file n=16 path=packages/escher-driver/src/lib.rs
@task_file n=17 path=packages/escher-driver/src/mcp.rs
@task_file n=18 path=examples/seven_guis/src/tasks/mod.rs
@task_file n=19 path=examples/seven_guis/src/tasks/counter.rs
@task_file n=20 path=examples/seven_guis/src/tasks/temp_converter.rs
@task_file n=21 path=examples/seven_guis/src/tasks/flight_booker.rs
@task_file n=22 path=examples/seven_guis/src/tasks/timer.rs
@task_file n=23 path=examples/seven_guis/src/tasks/crud.rs
@task_file n=24 path=examples/seven_guis/src/tasks/circle_drawer.rs
@task_file n=25 path=examples/seven_guis/src/tasks/cells.rs
@task_file n=26 path=examples/seven_guis/tests/common/mod.rs
@task_file n=27 path=examples/seven_guis/tests/host_log.rs
@task_file n=28 path=examples/seven_guis/tests/mcp_log.rs
@task_file n=29 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=30 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=31 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=32 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=33 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
@task_file n=34 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=35 path=tests/blitz-tests/tests/stand_act_spans.rs
@task_file n=36 path=tests/blitz-tests/checks/counter-cli.json
@task_file n=37 path=tests/blitz-tests/checks/counter-mcp.json
@task_file n=38 path=docs/capabilities.md
@task_file n=39 path=README.md
@task_file n=40 path=scripts/cold-agent.sh
@task_item n=1 part=touches
The cli surface's component patterns for escher-session and its mcp role: a new verb graph answers one uncoloured JSON line with exit status 0 on the command line and one JSON-RPC line over MCP; any text listing the driver's verbs or MCP tools now needs graph after lint.
@task_item n=2 part=touches
Any count of the driver's verbs (12 to 13), instance verbs (8 to 9) or MCP tools (12 to 13) stated on the cli surface is stale.
@task_item n=3 part=touches
The usage line and help on the cli surface gain graph after lint; a statement quoting the usage line or the tools/list order is stale.
@task_item n=4 part=touches
Any count of a snapshot node's keys (13 to 14) is stale: clickable joins after focused, always written in a node's JSON and written as the text token clickable only when true.
@task_item n=5 part=touches
The cli output contract (stdout holds only JSON, no colour or ESC byte, exit class 0, 1, 2, 3) now also covers the graph answer; a graph call with an argument is refused malformed with nothing run (exit class of a refusal).
@task_item n=6 part=touches
A statement that no snapshot field says an element reacts to a click, or that the click reading is not built, is stale: NodeState.clickable now reads it from the element's own click listener, excluding a disabled form control.
@task_item n=7 part=touches
Any figure for the Cells screen's snapshot text (47,089 bytes, 5 parts) is stale: it now reads 53,859 bytes, 740 lines, 6 parts.
@task_item n=8 part=touches
Stable element id forms: a fifth reserved form owner///key (VIRTUAL_KEY_SEPARATOR, virtual_node_id) is stated, produced by no caller; any rule enumerating the id forms an agent reads on the cli surface now has it.
@task_item n=9 part=touches
The Dioxus event path on every Dioxus document (desktop-native, mobile-native and web-spa launches through dioxus-native) now sets a registry dispatch around handle_event only where a state is registered; inert and unchanged where none is.
@task_item n=10 part=touches
The seven_guis stand tasks' signals are now registered with declarations through use_graph_signal; any example-fixture text describing how those tasks hold their state with use_signal alone is stale (use_signal stays only in the stand shell and stand.rs linked).
@task_fact n=1 slug=surface
fact: escher-session's graph verb, on the socket, the command line, MCP and in a kept check, answers the whole UI graph as one uncoloured JSON line (nodes and states) with exit status 0, takes no argument, and carries ids, component names, state names, handler event names, closed flag words and booleans only, never a typed value.
@task_fact n=2 slug=surface
fact: The snapshot's clickable reading is true where an element holds a click listener and is no disabled form control; in the stand, Counter, Flight Booker, Timer, CRUD and Temperature Converter read 0 clickable without a role, Circle Drawer 1 and Cells 676, in both layout modes.
@task_fact n=3 slug=surface
fact: The graph answer is unpaged: Cells at boot answers 293,869 bytes in one line, a stated limit (An unpaged graph) owned by residual res-7cc5bi4k, target next.
@task_item n=1 part=untouched
Color palette, surface scale, text hierarchy, semantic colours, border progression and the open select list's engine colour constants: no colour value or token moved.
@task_item n=2 part=untouched
Typography, font loading, the fonts-unavailable screenshot refusal and the system-fonts build feature: unchanged.
@task_item n=3 part=untouched
Spacing, depth strategy, hit-testing at clipped padding boxes, and border radius: no layout, paint or radius rule moved.
@task_item n=4 part=untouched
Motion: the VirtualClock, settle, advance, load wait and into-view scroll rules are unchanged; a graph read runs no step and moves no clock.
@task_item n=5 part=untouched
Iconography, the browser toolbar icons, IconButton, and the engine cursor rule (Pointer inside links, Text, Default): unchanged; clickable is a snapshot reading, not a cursor or icon change.
@task_item n=6 part=untouched
Desktop-native, mobile-native and web-spa platform, toolkit, window, titlebar, urlbar, back button, devtools overlay and canvas patterns: no shell, window or app UI changed.
@task_item n=7 part=untouched
The cli surface's WPT runner and example binaries (owo-colors output, paint_bench, screenshot) and its unmeasured navigation and platform notes: not reached.
@task_item n=8 part=untouched
Brand identity, anti-patterns, self-validation protocol and design decisions log: no recorded intent was added.
