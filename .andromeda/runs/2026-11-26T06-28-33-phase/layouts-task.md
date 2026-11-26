@task_prose part=chunk
Circle Drawer's canvas becomes a Tab stop and keys move a current circle among its virtual nodes; the current circle is selected and its diameter dialog reached by keyboard as the double click reaches it.
Engine focus, the accessibility tree's focus and the snapshot's focused agree on one target; Circle Drawer's Tab order joins the stand's shared focus-sequence table and keyboard walk.
A click inside a circle selects it and adds none (the founder's B1): the virtual node declaration gains selected, read as the snapshot's selected; a click outside every circle still adds one.
The stand reads a circle refused covered, off-screen and disabled where it can make one so; two windows build warnings are fixed where they stand.
It re-reads the double-click drive of the graph coverage check and sweeps every literal pin the selection repaint moves (canvas pins, stated sizes and counts).
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer task, its canvas circle-canvas painting the circles and declaring them as virtual nodes — edits: the canvas's focusability, its key handling, the selection, CircleCanvas::virtual_nodes declaring selected
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget boundary behind the custom-widget feature, the virtual node declaration a canvas makes — edits: VirtualNodeDeclaration gains selected
@task_file n=3 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: the virtual node's accessibility tree node built from a declaration — edits: its selected state and its focus
@task_file n=4 path=packages/blitz-dom/src/accessibility.rs
what: the engine's accessibility tree, its focus written only where the focused node is among the built nodes, else the window
@task_file n=5 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's verb execution — edits: press on a focused canvas, the covered refusal reading
@task_file n=6 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared test readers and focus-sequence table — edits: tab_order, the Tab order lists (ENABLED_TAB_ORDER, LOAD_TAB_ORDER, ORDER_TAB_ORDER, CELLS_TAB_ORDER) gaining Circle Drawer's, the focused reader
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs mark=edits
what: the virtual node declaration check — edits: its declaration literals taking the new selected field
@task_file n=8 path=tests/blitz-tests/tests/fixtures/mod.rs mark=edits
what: the shared test fixtures — edits: their virtual node declaration literals taking the new selected field
@task_file n=9 path=tests/blitz-tests/tests/stand_circle_nodes.rs mark=edits
what: the stand check of Circle Drawer's circles as virtual nodes — edits: the click-selects reading, the covered, off-screen and disabled readings
@task_file n=10 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard walk check — edits: Circle Drawer's walk and keyboard reach of a circle
@task_file n=11 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=edits
what: the stand check of Circle Drawer's painted canvas — edits: its literal pins moved by the filled selection
@task_file n=12 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=edits
what: the stand check of a canvas's draw commands — edits: its literal pins
@task_file n=13 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS canvas pin check — edits: its literal pins
@task_file n=14 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=re-reads
what: the stand's accessibility ids check, tab_order_is_unchanged
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs mark=re-reads
what: the stand's UI graph check
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=re-reads
what: the stand's handler coverage check, GRAPH_DRIVES, whose double click lands on the circle its first drive added
@task_file n=17 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand's act error text check — edits: the windows build warning at its line 27
@task_file n=18 path=examples/seven_guis/tests/host_binary.rs mark=edits
what: the 7GUIs host binary test — edits: the windows build warning at its line 10
@task_file n=19 path=docs/capabilities.md mark=edits
what: the capabilities document — edits: its stated sizes and counts
@task_item n=1 part=needs
Circle Drawer's circles stay painted on one canvas and are reached as its declared virtual nodes; the canvas keeps its stable id circle-canvas and becomes a Tab stop with key handling that moves a current circle.
@task_item n=2 part=needs
The diameter dialog (circle-diameter) is reached by keyboard from the current circle as the double click reaches it; Circle Drawer's undo, redo and close controls keep their stable ids.
@task_item n=3 part=needs
Selection is carried by attributes alone: the circle's selected state is written on its accessibility node from the declaration and read as the snapshot's existing selected, so a driver click on a circle settles with a diff naming that node (differs: selected).
@task_item n=4 part=needs
Behaviour change: a click inside a circle selects it and adds none, a click outside every circle still adds one; the selected circle repaints filled, moving the committed canvas pins, which are swept, never left stale.
@task_item n=5 part=needs
Focus agreement: the engine's focus on the canvas element, the tree's focus and the snapshot's focused read one consistent target; a virtual node has no engine node id and the engine reads no aria-activedescendant, which stays a stated limit unless P4 forks on it.
@task_item n=6 part=needs
The stand's Tab order lives in one shared focus-sequence table; Circle Drawer's sequence joins it and the stand's keyboard walk reads it beside the others, every other task's order unchanged.
@task_item n=7 part=needs
Refusals on a circle: covered, off-screen and disabled read on the stand where it can make one so; off-screen is judged through the visible region the clipping boxes leave and into-view scrolling; covered may read a later-declared sibling virtual node covering a circle (F4 B, an open question).
@task_item n=8 part=needs
The driver's press acts on a focused canvas, reaching the current circle through the canvas's key handling.
@task_item n=9 part=needs
Stand readings run in the headless stand's pinned viewport, a held instance reading as a fresh boot, so the swept canvas pins, sizes and counts are measured there.
@task_item n=10 part=needs
The stand still lints clean at boot with the canvas now focusable and keyed; the canvas's key handling leaves every other stand control's keyboard behaviour as it is.
@task_item n=11 part=needs
Circle Drawer stays served by no escher-session role: the served tasks list gains no circle-drawer entry, and v040-11's binary reading waits on the cold-agent live run.
@task_item n=12 part=needs
The graph coverage drive whose double click lands on the circle its first drive added is re-read: under the click-selects rule that first click selects, so its expectations follow.
@task_item n=13 part=needs
Every caller-visible reading added (selected, focus on a virtual node) writes back no caller input; a virtual node's name and state come from its declaration alone.
@task_standing n=1
convention: every stand control and element a new test drives is keyed by a stable id carried as an attribute alone, listed among the stand's stable element ids
home: surface-desktop-native
@task_standing n=2
convention: every name, role and state a new stand reading asserts is carried by attributes alone
home: surface-desktop-native
@task_standing n=3
convention: every new stand test runs in the pinned headless viewport and reads a held instance as a fresh boot
home: surface-desktop-native
@task_standing n=4
convention: the stand lints clean at boot after every change to a stand task
home: surface-desktop-native
@task_item n=1 part=not_needed
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): the chunk touches no window chrome.
@task_item n=2 part=not_needed
The cold-agent and agent-run pipes and their CLI contracts (usage, exits, record dir, windows pass-through): the live run is not built here.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: no benchmark or multi-document harness is touched.
@task_item n=4 part=not_needed
The escher-session MCP server's protocol revisions, stdio, result bound, screenshot result and end; its exit codes, help and host log service name: no session role or MCP tool changes.
@task_item n=5 part=not_needed
escher-session recording (check files, record dir, recording failure) and session recording as a replayable check: the chunk adds no recording.
@task_item n=6 part=not_needed
The load page's network provider, the flight type select's boot reading and the order page built to cause: those tasks' behaviour is untouched.
@task_item n=7 part=not_needed
Arbitrary index.html templates and the fixed document skeleton with its main mount: the document shell is unchanged.
@task_item n=8 part=not_needed
The mobile-native and web-spa surfaces: no rows, and the chunk reaches neither.
