@task_prose part=chunk
Makes Circle Drawer's canvas a Tab stop with key handling that moves a current circle among its virtual nodes and acts on it (selects it, reaches its diameter dialog as the double click does).
Makes the engine's focus, the accessibility tree's focus and the snapshot's focused agree on the current circle, and enters Circle Drawer's Tab order in the stand's shared focus-sequence table read by the keyboard walk.
Carries the founder's B1: a click inside a circle selects it and adds none, the virtual node declaration gains selected, read as the snapshot's selected so the click's diff names that node; a click outside every circle still adds one.
Adds the stand's covered, off-screen and disabled refusals on a circle where the stand can make one, fixes two windows build warnings where they stand, and sweeps the literal pins the selection repaint moves.
Re-reads the handler-coverage graph drive whose first click now selects, and re-measures the capability statement's stated sizes and counts.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the seven_guis Circle Drawer task, its canvas custom widget and its virtual node declarations — edits: the canvas's focusability, its key handler, the click selection, CircleCanvas::virtual_nodes declaring selected
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: blitz-dom nodes and widgets, the Widget trait's virtual nodes and VirtualNodeDeclaration, an escher addition behind the custom-widget feature — edits: VirtualNodeDeclaration gains selected
@task_file n=3 path=packages/dioxus-native-dom/src mark=edits
what: the Dioxus DOM bridge's virtual node readers, building a virtual node's accessibility tree node and its snapshot state — edits: the virtual node's tree node, its selected and its focus
@task_file n=4 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's acting step and per-verb target check — edits: press on a focused canvas, the covered reading on a virtual node
@task_file n=5 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the blitz-dom accessibility tree build, where the tree's focus is written from the engine's focused node or else the window — edits: the tree's focus naming a virtual node, if P4 settles on an engine change
@task_file n=6 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand checks' shared helpers, the focus-sequence tables and the shared focused reader — edits: tab_order, a Circle Drawer Tab order beside ENABLED_TAB_ORDER, LOAD_TAB_ORDER, ORDER_TAB_ORDER and CELLS_TAB_ORDER
@task_file n=7 path=tests/blitz-tests/tests/stand_keyboard.rs mark=edits
what: the stand's keyboard walk check, reading each task's Tab order from the shared table — edits: Circle Drawer's keyboard reach
@task_file n=8 path=tests/blitz-tests/tests/stand_circle_nodes.rs mark=edits
what: the stand check of Circle Drawer's circles as virtual nodes, their click and refusals — edits: the click-selects reading, covered, off-screen and disabled refusals, keyboard reach
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand check of accessibility ids and that the Tab order is unchanged — edits: tab_order_is_unchanged
@task_file n=10 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=edits
what: the stand check of Circle Drawer's canvas reading and its literal pins
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=edits
what: the stand check of the canvas verb's draw commands and their literal pins
@task_file n=12 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand check that canvas readings are byte-identical on every CI OS, its literal pins
@task_file n=13 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand check of the UI graph answer and its literal pins
@task_file n=14 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=re-reads
what: the handler coverage check, GRAPH_DRIVES whose double click lands on the circle its first drive added, re-read under B1
@task_file n=15 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the stand check of acting refusal error text — edits: the windows build warning at its line 27
@task_file n=16 path=examples/seven_guis/tests/host_binary.rs mark=edits
what: the seven_guis host binary test — edits: the windows build warning at its line 10
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement held to the driver schema — edits: its stated sizes and counts
@task_item n=1 part=needs
A declared virtual node gains a selected state: the declaration's new field, written on the virtual node's accessibility node and read as the snapshot's existing selected key, so a diff on a circle reads differs selected inside the closed differs set.
@task_item n=2 part=needs
A driver click on a circle's virtual node id selects that circle and adds none, its acting result naming that node as changed; a click on the canvas outside every circle still adds one, the click point read against the content box.
@task_item n=3 part=needs
The canvas becomes focusable and a Tab stop: focusability rules, the focus change dispatch and its events, and the shared Tab walk now include the canvas object element.
@task_item n=4 part=needs
Keys on the focused canvas move a current circle among its virtual nodes and act on it: the driver's press verb on a focused canvas, the key names it takes (arrows, Enter, Space, Tab), and Enter and Space activation on a focused control.
@task_item n=5 part=needs
Engine, tree and snapshot focus agree on one target: the accessibility tree's focus, the snapshot's focused key and the node-keys focused reading, with a virtual node having no NodeId; how a virtual node is named as focus is open at P3/P4.
@task_item n=6 part=needs
The engine reads no aria-activedescendant and that gap stays a stated limit; the tree's focus is written only where the engine's focused node is among the built nodes, else the window, so naming a virtual node as the tree's focus is an engine change for every document behind the feature or none.
@task_item n=7 part=needs
The selection repaints the chosen circle filled, moving the canvas's draw commands under its virtual node grouping, the canvas reading's deterministic line form and the cross-OS identical stand readings; the committed literal pins are swept.
@task_item n=8 part=needs
A circle refused covered, off-screen and disabled on the stand where it can be made so: the refusal causes, the per-verb target check for click, visibility read from geometry, and the open question of a later-declared sibling virtual node read as covering a circle.
@task_item n=9 part=needs
The engine addition to the virtual node declaration is an escher addition behind the custom-widget feature, written in functions of escher's own and flagged upstreamable as a direct engine change.
@task_item n=10 part=needs
The virtual node's tree node and snapshot reading sit behind the accessibility feature gate of the Dioxus bridge and the snapshot readers.
@task_item n=11 part=needs
The stand's shared Tab order table gains Circle Drawer's sequence beside the other tasks' tables, read by the keyboard walk and by the check that the Tab order is unchanged.
@task_item n=12 part=needs
The handler coverage reader's canvas handlers and its graph drive: under B1 the drive's first click selects rather than adds, re-read; the canvas's new key handler is a handler the coverage reader can reach.
@task_item n=13 part=needs
The actions answer, the UI graph and exploration read a canvas's virtual node by its id; a selected and focusable circle must keep reading through them, the virtual id verb table and the actions verb's virtual node row.
@task_item n=14 part=needs
The capability statement's stated sizes and counts are re-measured against the driver schema after the change.
@task_item n=15 part=needs
Two windows build warnings are fixed where they stand and witnessed by the CI windows job's own log after its Compiling line, since the host cannot build the windows test check.
@task_item n=16 part=needs
Circle Drawer stays served by no escher-session role: the session apps table and the cold-agent run do not gain it, and v040-11's binary reading waits on that.
@task_item n=17 part=needs
Stand readings are pinned: the stand boots offline, on its pinned environment and DejaVu Sans typography, and on the harness's virtual clock, with a double click read as two clicks without an advance.
@task_item n=18 part=needs
Evidence written by the phase carries no raw log with a temp or home path, no key twice on a line, and each control enters with its red reading.
@task_standing n=1
convention: each new integration test lives in blitz-tests as one test target per file, one behavior per file, with descriptive names
home: conventions
@task_standing n=2
convention: stand-check helpers shared between targets live in non-target shared modules, stated once
home: conventions
@task_standing n=3
convention: a test asserts its fixture's precondition first and names its element; assertion helpers carry track-caller
home: conventions
@task_standing n=4
convention: stand readings are identical on every CI OS, and the stand-repeat leg runs them with a zero-failure budget
home: standard-contracts
@task_standing n=5
convention: new public items carry docs, rustdoc warnings are errors, expect and unwrap carry a justification, and fmt and clippy gate the change
home: conventions
@task_standing n=6
convention: a driver or bridge log line carries no snapshot content, no typed input and no diff content
home: standard-contracts
@task_item n=1 part=not_needed
WPT runner rows, its result kinds, statuses, outputs, script fetcher and CI artifacts: the chunk runs no WPT suite and changes no web-platform behaviour.
@task_item n=2 part=not_needed
The browser app: its history store, about pages, toolbar, persistence and publish workflow.
@task_item n=3 part=not_needed
CSS styling and layout machinery: Stylo, Taffy, inline and table layout, writing modes, scrolling primitives, damage and incremental layout.
@task_item n=4 part=not_needed
The select list and select model rows: list opening, rows, colours, typeahead and choice writers; the canvas's keys are its own.
@task_item n=5 part=not_needed
Script and JavaScript runtime rows: Boa, script globals, script fetching and microtask draining.
@task_item n=6 part=not_needed
Network fetching, HTTP cache, net providers and the per-host request cap.
@task_item n=7 part=not_needed
The session socket's wire framing, MCP server handshake and protocol revisions, and the command line's roles: no carrier is widened and Circle Drawer gains no served role.
@task_item n=8 part=not_needed
A lint reading of virtual nodes and canvas lint checks beyond label truncation: owed to the node-graph stand app, not built here.
@task_item n=9 part=not_needed
A rotated or scaled canvas's virtual nodes and visible regions through such transforms of a canvas: owed to pan and zoom.
@task_item n=10 part=not_needed
Upstream sync rows: the two-parent merge, merge base and per-take ratification of upstream behaviour changes; no take from another tree is made.
@task_item n=11 part=not_needed
The security summary's two gaps on OS drag-and-drop and the dropped-file reader: written by the wrap's leaf pass, never by this chunk's plan.
@task_item n=12 part=not_needed
Rendering backends and window shell rows: wgpu, Vello GPU, Skia archive checks, winit and platform adapters.
