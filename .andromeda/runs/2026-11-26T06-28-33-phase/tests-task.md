@task_prose part=chunk
Makes Circle Drawer's canvas a Tab stop whose keys move a current circle among its virtual nodes, the current circle selected by keyboard and its diameter dialog reachable as the double click reaches it.
Makes engine, accessibility-tree and snapshot focus read one consistent target for the current circle, and enters Circle Drawer's Tab order in the stand's shared table of focus sequences read by the keyboard walk.
Adds `selected` to the virtual node declaration (founder's B1): a click inside a circle selects it and adds none, its diff naming that node; a click outside every circle still adds one.
Reads v040-11's remaining stand readings on a circle (covered, off-screen, disabled refusals), sweeps the literal pins the selection repaint moves, and re-reads the graph drives and handler coverage under B1.
Fixes two windows build warnings, witnessed by the CI windows job's log.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer task of the seven_guis example app, its canvas and declared virtual nodes — edits: the canvas's focusability, its key handling, the selection, CircleCanvas::virtual_nodes declaring selected
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget's virtual node declaration, an escher addition behind the custom-widget feature — edits: VirtualNodeDeclaration gains selected
@task_file n=3 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: the virtual node's accessibility tree node built from a declaration — edits: its selected state and its focus
@task_file n=4 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver executor of the acting verbs and refusal causes — edits: press on a focused canvas, the covered reading on a virtual node
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree build, where the tree's focus is written from the engine's focused node, else the window
@task_file n=6 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared test module — the shared tab-order tables and the shared focused reader — edits: tab_order, a Circle Drawer tab order beside ENABLED_TAB_ORDER, LOAD_TAB_ORDER, ORDER_TAB_ORDER, CELLS_TAB_ORDER
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs mark=edits
what: the integration check of declared virtual nodes — edits: its declaration literals take selected
@task_file n=8 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared fixtures module — edits: its virtual node declaration literals take selected
@task_file n=9 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard walk check over every task's Tab order
@task_file n=10 path=tests/blitz-tests/tests/stand_circle_nodes.rs mark=edits
what: the stand check of Circle Drawer's circles as virtual nodes — click, refusals, diff
@task_file n=11 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand check of accessibility ids — edits: tab_order_is_unchanged
@task_file n=12 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=edits
what: the stand check of Circle Drawer's canvas paint, its committed literal pins
@task_file n=13 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=edits
what: the canvas commands table check, its committed literal pins
@task_file n=14 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the cross-OS canvas digest check, its committed literal pins
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs mark=re-reads
what: the stand's UI graph check over the shared graph drives
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=re-reads
what: the handler coverage check by replay — GRAPH_DRIVES, whose double click lands on the circle its first drive added
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement — edits: its stated sizes and counts
@task_file n=18 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of acting refusal text — edits: the windows build warning at its line 27
@task_file n=19 path=examples/seven_guis/tests/host_binary.rs mark=edits
what: the seven_guis host binary check — edits: the windows build warning at its line 10
@task_item n=1 part=needs
Keyboard reach of Circle Drawer: the canvas takes focus in the Tab walk, keys move a current circle among its virtual nodes, the current circle is selected by keyboard and its diameter dialog is reachable as the double click reaches it.
@task_item n=2 part=needs
Focus agreement: the engine's focus on the canvas, the accessibility tree's focus and the snapshot's focused read one consistent target; the shared focused reader asserts the snapshot's and the tree's focus agree; how a virtual node is named as focus is open at P3/P4.
@task_item n=3 part=needs
The tree's focus is written only where the engine's focused node is among the built nodes, else the window (the inert reading); a virtual node as the tree's focus would be an engine change for every document behind the feature.
@task_item n=4 part=needs
The engine reads no aria-activedescendant: a stated limit that stays unless P4 forks on it; the declaration has no focus field today.
@task_item n=5 part=needs
Circle Drawer's Tab order joins the stand's shared tab-order table; the stand keyboard walk reads it beside the other tasks; the unchanged-tab-order check stays held.
@task_item n=6 part=needs
The virtual node declaration gains selected, behind the custom-widget feature, written on the circle's accessibility node and read as the snapshot's existing selected; every existing declaration literal takes the new field.
@task_item n=7 part=needs
A driver click on a circle's node selects it and adds no circle; the acting call settles with a diff naming that node, differs selected; a click on the canvas outside every circle still adds one; the node is named by the virtual form of element ids.
@task_item n=8 part=needs
The selection repaints the chosen circle filled, moving committed canvas pins: the canvas commands table across hosts, the cross-OS canvas digests and the Circle Drawer canvas pins are swept and re-pinned as literals.
@task_item n=9 part=needs
The driver's press on a focused canvas reaches the current circle; keyboard input arrives through the harness key event and focus helpers.
@task_item n=10 part=needs
Refusal causes covered, off-screen and disabled read on a circle where the stand can make one, in the cause order; F4 B (a later-declared sibling read as covering a circle) rides as an open question.
@task_item n=11 part=needs
The shared graph drives and handler coverage by replay are re-read under B1: the first click of the drive that the double click lands on now selects; two clicks with no advance are a double click.
@task_item n=12 part=needs
The capability statement's stated sizes and counts are swept with the literal pins and stay held to the schema.
@task_item n=13 part=needs
Two windows build warnings in test targets are fixed where they stand and witnessed by the CI windows job's own log, read only after its Compiling line.
@task_item n=14 part=needs
Circle Drawer stays served by no escher-session role: its readings are stand readings only, its binary reading waiting on the cold-agent live run.
@task_item n=15 part=needs
Each new control enters with its red reading; no raw log with a temp or home path enters evidence; no key stands twice on a line.
@task_item n=16 part=needs
Focus tests pin where focus lands after each key, on the canvas and on the current circle.
@task_standing n=1
convention: each new stand check is its own test target, one target per file, sharing tables and fixtures through the shared module that is no target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new stand check runs under the bundled pinned font, with no skip
home: test-strategy · 2-test-strategy
@task_standing n=3
convention: each new requirement is witnessed inside stand-repeat, under the zero-failure budget
home: test-strategy · quality-gates-coverage-targets
@task_standing n=4
convention: each new test is controlled red-before-green, its expected values pinned as literals, table-driven tests asserting their row count, failures printing no argument value
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=5
convention: each new stand check runs in both layout modes
home: test-scope-summary
@task_standing n=6
convention: shared stand tables live in one module; stand logs carry no ids or names; test data stated in code
home: 3-test-harness-contract
@task_item n=1 part=not_needed
Session wire, socket, session state directory and session host lifecycle rows: Circle Drawer is served by no session role and the chunk adds no wire message.
@task_item n=2 part=not_needed
Cold-agent pipe rows (events, verdict, exit codes, prompt, isolation): the live run is another entry's.
@task_item n=3 part=not_needed
Agent-run verbs, events, selections and exit codes: the chunk adds no agent-run verb or selection.
@task_item n=4 part=not_needed
WPT runner, conformance and reftest rows: no web-platform suite is touched.
@task_item n=5 part=not_needed
MCP role protocol, tools list, help parity and MCP tool result rows: no MCP surface changes.
@task_item n=6 part=not_needed
Lint defects and lint findings rows: the lint reading virtual nodes belongs to the node-graph stand app entry.
@task_item n=7 part=not_needed
Timer ticks, load-page stand-in, network stand-in and failed-load settle rows: Circle Drawer has no timer or load.
@task_item n=8 part=not_needed
Typed-password masking and password caret rows: no password field is involved.
@task_item n=9 part=not_needed
Select list painting, scrollbar fade, box-shadow, font embolden and Thai-font paint rows: other widgets and paints.
@task_item n=10 part=not_needed
Rotated or scaled canvas readings: virtual nodes under pan and zoom belong to the Pan and zoom entry.
@task_item n=11 part=not_needed
CI action pinning, secrets, permissions, package install retry and cache rows: the chunk adds and edits no CI job.
