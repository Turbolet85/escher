@task_prose part=chunk
Circle Drawer's canvas widget declares one virtual node per circle it paints (an author key, a role, a non-empty name, a region in its content box), so each circle reads as a node in the accessibility tree, snapshot, diff, UI graph, actions, layout and exploration.
A click on a circle's node acts on its circle and its diff names it; a covered, off-screen, disabled or removed circle node is refused with that cause.
The canvas reading groups a canvas's draw commands under the virtual node that covers them, and a changed command reads with an identity in the reading's diff.
It adds stand checks in both layout modes and the binary-level reading of a virtual node's id and name, sweeps every committed stand pin and literal pin that reads Circle Drawer's screen, and re-verifies the seven folded CARRYs (label region, covered by element only, handler coverage of a virtual node, the lint reading no virtual node, a windows unused-import warning).
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: Circle Drawer's canvas widget, the circles it paints and their colours — edits: CircleCanvas's virtual node declaration (each circle's key, role, name and region)
@task_file n=2 path=packages/escher-driver/src/canvas.rs mark=edits
what: the canvas reading and its line diff, the canvas verb's answer — edits: the grouping of draw commands under the virtual node that covers them, a draw command's identity in the diff
@task_file n=3 path=packages/escher-driver/src/lint.rs
what: the lint, which reads no virtual node (no overlap, clip or off-screen finding on a declared region)
@task_file n=4 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint's label region — a canvas label read against the content box cut by each clip, layer or mask path
@task_file n=5 path=packages/escher-driver/src/coverage.rs
what: handler coverage — the coverage line, a click on a virtual node counted as its canvas's own listener under the canvas's id
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: the driver verbs' execution — click on a node and its refusals with a cause (covered, off-screen, disabled, removed)
@task_file n=7 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the virtual node — a canvas's declared region read and validated, and the covered reading over its region's centre
@task_file n=8 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs mark=edits
what: the check that virtual nodes appear in every read — edits: the snapshot import, put behind a unix-only gate
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: seven_guis' process check of the command line's stdout — a virtual node's id and name read on the binary
@task_file n=10 path=examples/seven_guis/tests/mcp_server.rs mark=edits
what: seven_guis' process check of the socket host and mcp role answers — a virtual node's id and name read on the binary
@task_file n=11 path=docs/capabilities.md mark=edits
what: the capabilities document — its stated sizes and counts, and its statement of a virtual node's covered reading
@task_item n=1 part=needs
A circle's virtual node: its author key (a circle has no id of its own), its role, its non-empty name and its region inside the canvas content box; a circle near the canvas edge spills its content box, which the region-outside refusal turns away.
@task_item n=2 part=needs
Each circle reads as a node by the reserved owner-and-key id form in the accessibility tree, the snapshot and its diff, and in the UI graph line, the actions line and the layout answer, as the three earlier chunks built them.
@task_item n=3 part=needs
The graph, actions and layout answers of escher-session commands now carry Circle Drawer's circle nodes, and the actions line's nodes and the UI graph line's content name them.
@task_item n=4 part=needs
The click verb on a circle's node acts on its circle and its diff names the node; a covered, off-screen, disabled or removed circle node is refused with that cause.
@task_item n=5 part=needs
A virtual node reads covered only by an element over its region's centre, never by another virtual node of its canvas; Circle Drawer's overlapping circles put that rule on the stand.
@task_item n=6 part=needs
The canvas verb's reading groups a canvas's draw commands under the virtual node that covers them, and a changed command reads with an identity in the reading's line diff instead of one line removed and one added; the grouping must keep the canvas line form conventions.
@task_item n=7 part=needs
The canvas answer holds no virtual node today; a grouping that names a node's id in it changes that answer's content, a crossing change.
@task_item n=8 part=needs
Handler coverage: a virtual node carries no handler of its own, so the coverage line lists none for it and counts a click on one as its canvas's own listener under the canvas's id; Circle Drawer's canvas carries an onclick.
@task_item n=9 part=needs
The lint reads no virtual node: no overlap, clip or off-screen finding on a declared region, even with Circle Drawer's overlapping circles.
@task_item n=10 part=needs
A canvas label's region in the canvas lint: read against the content box cut by each clip, layer or mask path and the element's clip region, or as the shape or the virtual node under it.
@task_item n=11 part=needs
escher-session stdout on the command line and the mcp role's tool results carry a virtual node's id and name, now read on the binary against a served app.
@task_item n=12 part=needs
Circle Drawer's circle colours stay widget constants in code while the canvas gains virtual nodes.
@task_item n=13 part=needs
Every committed stand pin that reads Circle Drawer's screen moves: the cross-OS digest tables, the canvas commands table, the canvas lint and non-lean task checks, the Circle Drawer canvas check, and the capabilities document's stated sizes and counts — literal pins swept too.
@task_item n=14 part=needs
Stand checks run in both layout modes, and the logical viewport the regions are read in is physical size over total scale.
@task_item n=15 part=needs
The windows build's unused snapshot import warning in the every-read check is fixed by a unix-only gate, checked by a windows-target test check.
@task_item n=16 part=needs
Hit-testing that decides a covered circle node stops where paint clips, and the stacking-context root groups paint and hit-testing.
@task_standing n=1
convention: escher-session output is read by an agent, not a person — no colour, exit statuses read from the schema's table, every new answer line plain text
home: surface
@task_standing n=2
convention: driver command line output is written for agents, on stdout as one command line line or one mcp role line
home: surface-cli
@task_standing n=3
convention: a canvas reading line follows the canvas line form conventions
home: surface
@task_item n=1 part=not_needed
The select element's list, its rows, events, colours and accessibility: Circle Drawer's canvas has no select.
@task_item n=2 part=not_needed
The WPT runner's output and result colours: the chunk adds no web-platform test and reads no runner line.
@task_item n=3 part=not_needed
Typography, font contexts and the screenshot's fonts-unavailable refusal: a circle node's name is not painted text, and no font choice moves.
@task_item n=4 part=not_needed
The shell provider's window controls and cursor icons, and the JS window object's sizes: no window chrome or script value changes.
@task_item n=5 part=not_needed
Color scheme, base colour, alpha mode and engine colour constants: no colour outside Circle Drawer's own widget changes.
@task_item n=6 part=not_needed
Keyboard reach of the circles and focus order on a canvas: that is the next entry, the canvas keyboard path.
@task_item n=7 part=not_needed
A rotated or scaled canvas's virtual nodes: those belong to pan and zoom.
@task_item n=8 part=not_needed
Overlay scrollbars, smooth scroll and into-view scrolling: the Circle Drawer canvas does not scroll.
@task_item n=9 part=not_needed
Spacing hints of tables and the body margin, and the presentational font hints: the chunk lays out no table and adds no markup hint.
@task_item n=10 part=not_needed
Mobile targets and the WASM page surface: the stand runs on desktop builds only.
