@task_prose part=chunk
Circle Drawer's canvas declares one virtual node per circle it paints (author key, role, non-empty name, region in its content box), making it the first served app with virtual nodes.
Each circle reads as a node `{owner}///{key}` in the accessibility tree, snapshot, diff, UI graph, actions, layout and exploration; click on a circle node acts on its circle, and a covered, off-screen, disabled or removed circle node is refused with that cause.
The canvas reading groups a canvas's draw commands under the virtual node covering them, and a changed command reads with an identity in the reading's diff.
It adds stand checks in both layout modes and the binary-level reading of a virtual node's id and name, re-verifies the seven folded CARRYs (label region, covered, handler coverage, lint reads no virtual node, windows unused import) and sweeps every committed stand pin and literal pin that reads Circle Drawer's screen.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer task of the stand app, whose circles are painted on a canvas (circle-canvas) — edits: CircleCanvas::virtual_nodes
@task_file n=2 path=packages/escher-driver/src/canvas.rs mark=edits
what: the driver's canvas reading and its line diff, the answer of the canvas verb and canvas tool — edits: the grouping of draw commands by virtual node, draw-command identity in the diff
@task_file n=3 path=packages/escher-driver/src/lint.rs mark=edits
what: the driver's lint, the answer of the lint verb, which reads no virtual node
@task_file n=4 path=packages/escher-driver/src/canvas_lint.rs mark=edits
what: the canvas lint, reading a canvas label's region against the content box, clips and the element's clip region
@task_file n=5 path=packages/escher-driver/src/coverage.rs mark=edits
what: handler coverage, the coverage role's answer, counting a click on a virtual node as its canvas's own listener
@task_file n=6 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's action execution, where click acts and refusals (covered, off-screen, disabled, removed) are read
@task_file n=7 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: the virtual node component of the native DOM — its declared regions and its covered reading
@task_file n=8 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs mark=edits
what: the check that virtual nodes appear in every read (graph, actions, layout, exploration) — edits: the snapshot import, put behind a unix-only cfg
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: seven_guis' process check of the escher-session command line's stdout and answers
@task_file n=10 path=examples/seven_guis/tests/mcp_server.rs mark=edits
what: seven_guis' process check of the escher-session MCP server over stdio and its tools
@task_file n=11 path=docs/capabilities.md mark=edits
what: the capabilities document — its stated sizes and counts and the statement of a virtual node's covered
@task_item n=1 part=needs
Circle Drawer circles painted on a canvas: each circle becomes a declared virtual node with key, role, non-empty name and a region inside the canvas content box; a circle near the edge must not spill the region outside it.
@task_item n=2 part=needs
The circle-canvas stable element id: it is the owner of every circle node id `{owner}///{key}`, and stays the id under which coverage counts a click on a circle.
@task_item n=3 part=needs
Stand names, roles and states by attributes alone and stand ids as attributes alone: a circle node's role and name come from its declaration, read in the accessibility tree and snapshot.
@task_item n=4 part=needs
Stand lints clean at boot: Circle Drawer with overlapping circle nodes must still lint clean, the lint reading no virtual node.
@task_item n=5 part=needs
Visible region through clipping boxes and the off-screen refusal: a circle node off-screen is refused with that cause; covered is read only by an element over its region's centre, never another circle of the canvas.
@task_item n=6 part=needs
The canvas tool of the MCP server and the canvas verb's answer: draw commands grouped under the virtual node covering them, and a changed command carrying an identity in the diff instead of a removed and added line.
@task_item n=7 part=needs
The snapshot, graph, layout and actions answers of escher-session: each must carry the circle nodes' ids and names, read at the binary level on stdout, through the MCP server and in a cold-agent live run.
@task_item n=8 part=needs
The coverage role's answer: lists no handler for a virtual node and counts a click on a circle as the canvas's own onclick listener.
@task_item n=9 part=needs
The lint answer and the canvas lint's label region: a label read against the shape or virtual node under it is decided here or stays its residual.
@task_item n=10 part=needs
Headless stand pinned viewport, in both layout modes: the stand checks of circle nodes run there, and every committed stand pin reading Circle Drawer's screen (cross-OS digest tables, canvas commands table, canvas lint, non-lean tasks, circle drawer canvas) and literal pin in the docs is re-measured and moved.
@task_item n=11 part=needs
Held instance reads as a fresh boot and session roles share the stand boot: circle nodes declared at boot read the same under every session role.
@task_item n=12 part=needs
The escher-session refusals and exit codes: click on a refused circle node answers with its cause through the existing refusal forms.
@task_item n=13 part=needs
Cold-agent pipe and cold agent driving through the driver's MCP server: the binary-level reading of a circle node's id and name rides that pipe; the prompt still names no id.
@task_item n=14 part=needs
Windows build of the blitz tests: the unused snapshot import warning is removed, checked by cargo check of the tests for the windows gnu target.
@task_standing n=1
convention: every new stand check runs on the headless stand pinned viewport against the stand's keyed controls and boots clean of lint
home: surface-desktop-native
@task_standing n=2
convention: every new process check of the binary reads escher-session answers through its command form, exit codes and MCP tools as contracted, echoing no caller input
home: surface-cli
@task_item n=1 part=not_needed
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): the chunk touches no window chrome.
@task_item n=2 part=not_needed
Stable element ids of the other tasks (counter, temperature, flight booker, timer, CRUD, cells, load, order) and their task cards: only Circle Drawer's canvas changes.
@task_item n=3 part=not_needed
CRUD list as one keyboard region and any keyboard reach: keyboard reach of the circles is the next entry's, not built here.
@task_item n=4 part=not_needed
Load page network provider, order page built to cause and flight type select boot reading: unrelated tasks' behaviour.
@task_item n=5 part=not_needed
multi_doc and paint_bench command lines: benchmarks the chunk does not reach.
@task_item n=6 part=not_needed
agent-run.sh test contract: the chunk adds no agent-run test.
@task_item n=7 part=not_needed
Fixed document skeleton with main mount, arbitrary index.html templates and stand accent colour: document shell and styling are untouched.
@task_item n=8 part=not_needed
Into-view scroll moving every scroller and open list rows' own bounds: no scroller or list changes.
@task_item n=9 part=not_needed
Surfaces mobile-native and web-spa: not measured and not reached.
@task_item n=10 part=not_needed
A rotated or scaled canvas's virtual nodes: pan and zoom's, not built here.
