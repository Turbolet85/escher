@task_prose part=chunk
Circle Drawer's circles become reachable and actionable by keyboard: the circle canvas is a Tab stop, keys move a current circle among its virtual nodes, and the current circle is selected and its diameter dialog reached as the double click reaches it.
A click inside a circle selects it and adds none (the founder's B1): the virtual node declaration gains `selected`, written on the circle's accessibility node and read as the snapshot's existing `selected`, so a driver click's diff names that node; a click outside every circle still adds one.
Engine focus, the accessibility tree's `focus` and the snapshot's `focused` agree on the current circle, and Circle Drawer's Tab order joins the stand's shared focus-sequence table read by the keyboard walk.
It adds the stand's `covered`, `off-screen` and `disabled` readings on a circle where the stand can make one, re-reads the handler-coverage graph drives under B1, sweeps every literal pin the filled selected circle moves, and fixes two windows build warnings witnessed by the CI windows job's own log.

@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer task and its circle canvas — edits: the canvas's focusability, its key handling, the selection, `CircleCanvas::virtual_nodes` declaring `selected`
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom-widget virtual node declaration, an escher addition behind the `custom-widget` feature — edits: `VirtualNodeDeclaration` gains `selected`
@task_file n=3 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: the virtual node's accessibility tree node and its snapshot reading — edits: the node's `selected` and its focus
@task_file n=4 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command execution — edits: `press` on a focused canvas, the `covered` refusal reading
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs
what: the engine's accessibility tree build, where the tree's `focus` is written from the engine's focused node or else the window
@task_file n=6 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared test module — edits: the focus-sequence table (`tab_order`, the `*_TAB_ORDER` lists), the shared `focused` reader
@task_file n=7 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard walk check — edits: Circle Drawer's walk beside the others
@task_file n=8 path=tests/blitz-tests/tests/stand_circle_nodes.rs mark=edits
what: the stand check of Circle Drawer's circles as virtual nodes — edits: click-selects, keyboard reach, the `covered`, `off-screen` and `disabled` readings
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand's accessibility id check — edits: `tab_order_is_unchanged`
@task_file n=10 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=edits
what: the stand check of Circle Drawer's canvas and its literal pins
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=edits
what: the stand check of canvas draw commands and their literal pins
@task_file n=12 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS canvas check and its literal pins
@task_file n=13 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph check and its literal pins
@task_file n=14 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=re-reads
what: the stand's handler coverage check, its `GRAPH_DRIVES` whose double click lands on the circle its first drive added
@task_file n=15 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of act error text — edits: the windows build warning at its line 27
@task_file n=16 path=examples/seven_guis/tests/host_binary.rs mark=edits
what: the seven_guis host binary test — edits: the windows build warning at its line 10
@task_file n=17 path=docs/capabilities.md mark=edits
what: the hand-updated capability statement — edits: its stated sizes and counts

@task_item n=1 part=needs
A virtual node's new `selected` field is written on the circle's accessibility node and read as the snapshot node's existing `selected`; a driver click on a circle's node settles with a diff naming that node as `differs: [selected]`.
@task_item n=2 part=needs
A click inside a circle selects it and adds none, a click outside every circle still adds one, and the selected circle repaints filled, moving the committed canvas pins.
@task_item n=3 part=needs
The circle canvas takes focus in the Tab walk and handles keys: a key moves the current circle among the virtual nodes, a key selects it and reaches its diameter dialog; the driver's `press` acts on a focused canvas.
@task_item n=4 part=needs
Engine focus, the accessibility tree's `focus` and the snapshot's `focused` must read one consistent target for the current circle, the stand's shared `focused` reader asserting the snapshot and tree agree; a virtual node has no engine node id, the tree's focus is written only from a built node or else the window, and the engine's unread `aria-activedescendant` stays a stated limit unless P4 forks.
@task_item n=5 part=needs
Circle Drawer's Tab order joins the stand's shared focus-sequence table and the keyboard walk reads it beside the other tasks; the unchanged-Tab-order check is updated to match.
@task_item n=6 part=needs
The driver's refusal causes `covered`, `off-screen` and `disabled` are read on a circle where the stand can make one so; under B1 a click on a covered circle selects the top one unless a later-declared sibling reads as covering it.
@task_item n=7 part=needs
The driver command span for a click or press on a circle carries only its closed fields and counts; the new `selected` and focus readings travel in the command answer and the snapshot diff alone, never as span field values.
@task_item n=8 part=needs
The declared-node read and the canvas read stay as they are toward telemetry: reading a virtual node's `selected` or focus logs nothing and opens no span of its own.
@task_item n=9 part=needs
The graph drives the handler coverage check runs are re-read under B1, its first drive's click on a circle now selecting it rather than adding one; the coverage line the stand reads holds handler ids only.
@task_item n=10 part=needs
Every literal pin the change moves is swept: the stand's canvas and graph pins and the capability statement's stated sizes and counts, which are updated by hand.
@task_item n=11 part=needs
The two windows build warnings are fixed where they stand and witnessed by the CI windows job's own log, read only after its compiling line is seen; no raw log with a temp or home path enters evidence.
@task_item n=12 part=needs
Every control enters evidence with its red reading, and no evidence line carries the same key twice.
@task_item n=13 part=needs
Every fork other than the founder's answered click clause is written to the chunk's founder-forks evidence with what crosses, the rule it meets, each option's measured cost, its dependences, whose it is and the most conservative option, and the phase stops there; with none, the phase goes on to the P5 card.

@task_standing n=1
convention: each new or changed stand check is silent and runs headless with no telemetry sink, its shared helpers kept in the shared module
home: 3-observability-harness-contract
@task_standing n=2
convention: a check that asserts an empty stderr unsets RUST_LOG before it runs
home: 3-observability-harness-contract
@task_standing n=3
convention: a log or output check's failure names a kind and a count, never the content it read
home: 3-observability-harness-contract
@task_standing n=4
convention: every committed check, fixture and evidence file holds no private data, no secret and no home or temp path
home: ci-integration · 9-ci-integration

@task_item n=1 part=not_needed
The cold-agent pipe, its events, counted verbs, kept checks, transcript and verdict fields: Circle Drawer stays served by no escher-session role and its live run is another entry's.
@task_item n=2 part=not_needed
escher-session's roles, commands, exit codes, host wire, error object, recording files, recording and state directories and session lifecycle: no session role serves Circle Drawer and none changes.
@task_item n=3 part=not_needed
Metrics, metric exporters, telemetry backends, OTel export and WPT score publishing: the chunk emits no metric and exports nothing.
@task_item n=4 part=not_needed
The script error surface, window error events, script error source labels, panic hooks and WPT report messages: the chunk touches no script or error path.
@task_item n=5 part=not_needed
Select, optgroup and multiple-select readings, checkbox and radio change events, and the open list shot: no form control is changed.
@task_item n=6 part=not_needed
The agent-run harness's events, raw log and cleanup: the chunk runs no agent harness.
@task_item n=7 part=not_needed
The build-directory sweep line and the package-install diagnostic line: the chunk changes neither CI step.
@task_item n=8 part=not_needed
The lint reading virtual nodes and a rotated or scaled canvas's virtual nodes: those belong to the node-graph stand app and pan-and-zoom entries.
@task_item n=9 part=not_needed
The mutation buffer, the state and signal registry and the runtime probe: the selection is the task's own state, held by no registry.
@task_item n=10 part=not_needed
Which telemetry crate is used and how engine tracing is gated by cargo feature: the chunk adds no tracing.
