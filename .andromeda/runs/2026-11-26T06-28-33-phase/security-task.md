@task_prose part=chunk
Circle Drawer's canvas becomes a Tab stop and keys move a current circle among its virtual nodes; the current circle is selected and its diameter dialog reached by keyboard.
Engine focus, the accessibility tree's focus and the snapshot's focused agree on the current circle; Circle Drawer's Tab order joins the stand's shared table.
The founder's B1: a virtual node declaration gains selected, a click inside a circle selects it and adds none, its diff naming that node; a click outside every circle still adds one.
The stand reads a circle refused covered, off-screen and disabled where it can make one so; two windows build warnings are fixed and witnessed by the CI windows job log.
It re-reads the handler-coverage drives and sweeps the literal pins (canvas pins, stated sizes and counts) that the filled selected circle moves.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the 7GUIs Circle Drawer task, its circle canvas custom widget and the virtual nodes it declares — edits: the canvas's focusability, its key handling, the selection, CircleCanvas::virtual_nodes declaring selected
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget boundary behind the custom-widget feature and its virtual node declaration — edits: VirtualNodeDeclaration gains selected
@task_file n=3 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: the virtual node's accessibility tree node and its state readings — edits: the virtual node's tree node (selected, and focus)
@task_file n=4 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's verb execution and target refusals — edits: press on a focused canvas, the covered reading
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree build, where the tree's focus is written only for a built node, else the window — edits: the tree's focus
@task_file n=6 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared readers and focus sequences — edits: tab_order, the Tab order table (ENABLED_TAB_ORDER, LOAD_TAB_ORDER, ORDER_TAB_ORDER, CELLS_TAB_ORDER, Circle Drawer's), the focused reader
@task_file n=7 path=tests/blitz-tests/tests/stand_circle_nodes.rs mark=edits
what: the stand check of Circle Drawer's circles as virtual nodes — its click, refusal and literal pins
@task_file n=8 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard walk check, reading Circle Drawer's Tab order beside the others
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand check of accessibility ids — edits: tab_order_is_unchanged
@task_file n=10 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=edits
what: the stand check of Circle Drawer's canvas and its committed canvas pins
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=edits
what: the stand check of the canvas draw commands and their literal pins
@task_file n=12 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand check of canvas readings across operating systems and their literal pins
@task_file n=13 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand check of the UI graph and its literal pins
@task_file n=14 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=re-reads
what: the stand's handler coverage check, its GRAPH_DRIVES whose first click on a circle now selects
@task_file n=15 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of act error texts — edits: the windows build warning at its line 27
@task_file n=16 path=examples/seven_guis/tests/host_binary.rs mark=edits
what: the seven_guis host binary test — edits: the windows build warning at its line 10
@task_file n=17 path=docs/capabilities.md mark=edits
what: the committed capability statement — edits: its stated sizes and counts
@task_item n=1 part=needs
The canvas takes focus in the Tab walk and its key handler moves a current circle among the declared virtual nodes; Enter or Space on the focused canvas selects the current circle and reaches its diameter dialog as the double click does.
@task_item n=2 part=needs
Engine focus, the tree's focus and the snapshot's focused name one consistent target; a virtual node has no NodeId, so the engine keeps focus on the canvas element, and the accessibility focus is written only for a built node, else the window — a virtual node as the tree's focus is an engine change for every document behind the feature, or none (P3/P4 settles which).
@task_item n=3 part=needs
The engine reads no aria-activedescendant: that gap stays its stated limit unless P4 forks on it; focus events fired by a programmatic focus change and by a click on the canvas stay as the engine's rules have them.
@task_item n=4 part=needs
The virtual node declaration gains a selected field, an escher addition behind the custom-widget feature; the declared virtual nodes stay validated by their closed refusal set, and every existing declaration literal takes the new field.
@task_item n=5 part=needs
selected is written on the circle's accessibility node and read as the snapshot's existing selected state, crossing as a closed value through the snapshot text's selected and unselected words and the diff node's selected key; a driver click on a circle's node settles with a diff naming that node, differs: [selected].
@task_item n=6 part=needs
A driver click on a circle's virtual node, synthesized on the custom-widget target, selects it and adds no circle; a click on the canvas outside every circle still adds one; the canvas answer keeps carrying a virtual node by its index and its stable id in the virtual-node form.
@task_item n=7 part=needs
Driver press on a focused canvas: the key reaches the canvas's key handler through the closed key argument kind and the single validation gate; no call input is echoed or panics.
@task_item n=8 part=needs
A circle refused covered, off-screen and disabled where the stand can make one so, under the closed refusal cause set and the fixed order of target refusals; a later-declared sibling virtual node read as covering a circle (the founder's F4 B) is an open question for the forks file.
@task_item n=9 part=needs
The virtual node crossing: a circle's accessible name, stable id and its new selected state leave only by the ratified exits (snapshot, diff, actions, graph, canvas answers) and are kept out of logs.
@task_item n=10 part=needs
Circle Drawer's Tab order enters the stand's shared focus-sequence table and the keyboard walk reads it beside the others; tab_order_is_unchanged is re-read for the other apps.
@task_item n=11 part=needs
The handler coverage drives are re-read: their double click lands on the circle the first drive added, and under B1 that first click selects.
@task_item n=12 part=needs
Literal pins swept: the selected circle repaints filled, moving the committed canvas pins, draw-command and graph pins, and the committed capability statement's stated sizes and counts.
@task_item n=13 part=needs
Two windows build warnings fixed where they stand, witnessed by the CI windows job's own log, read only after its Compiling line is seen; the CI verdict is read again before the P5 card.
@task_item n=14 part=needs
Circle Drawer stays served by no escher-session role; its binary reading of the acceptance waits on the cold-agent live run carried elsewhere.
@task_item n=15 part=needs
The security summary's two unwritten gaps — OS drag-and-drop as an untrusted input source and the dropped-file reader with no path restriction — ride to this entry's wrap leaf pass; no plan step writes them by hand.
@task_item n=16 part=needs
Evidence written by this chunk, including the founder forks file, holds no raw log with a temp or home path and no key twice on a line; each control enters with its red reading.
@task_standing n=1
convention: every new log line or span field the chunk adds keeps screen content, accessible names, stable ids and caller input out of it, under the allowlisted telemetry sink
home: logging-monitoring
@task_standing n=2
convention: every new refusal or error path the chunk adds comes from the closed cause set, holds nothing a call supplied and never panics on call input or a stale node
home: error-handling
@task_standing n=3
convention: every new test or test file joins the test harness with no tracing, no style and no new dependency outside the locked, workspace-inherited set
home: dependency-security
@task_standing n=4
convention: every evidence file or CI log the chunk reads into evidence holds no credentials and reads no environment beyond what is ratified
home: secret-management
@task_item n=1 part=not_needed
Authentication and authorization rows (CI tokens, publish guards, cookie store): the chunk touches no credential, token or access rule.
@task_item n=2 part=not_needed
Data protection rows on browser history, the HTTP cache, TLS, the clipboard, the native file dialog and kept check file storage: nothing stored or encrypted changes.
@task_item n=3 part=not_needed
Outbound request, fetch, navigation, CDP devtools and load-wait rows of API security: the chunk makes no request and adds no listener.
@task_item n=4 part=not_needed
MCP server transport, protocol errors and request or result bounds: Circle Drawer is served by no role and no driver verb is added to the MCP listing.
@task_item n=5 part=not_needed
Session socket wire grammar, recording, kept check and cold-agent run rows: no wire request, recorded check or live agent run is built here.
@task_item n=6 part=not_needed
Select, multiple select, range, password and cells grid keyboard and pointer rows: those controls' key models are not reached.
@task_item n=7 part=not_needed
HTML, CSS, SVG, font and stylesheet parsing or fetching rows, script fetchers and the WPT runner: no page parsing or loading changes.
@task_item n=8 part=not_needed
The lint reading virtual nodes (the founder's F6 B) and the canvas label crossing in a lint finding: that lint belongs to the node-graph stand app, not built here.
@task_item n=9 part=not_needed
Shell scripts' argument validation (apt-install, agent-run, coverage floor, build-directory sweep) and the coverage role's answer line: none is edited.
