@task_prose part=chunk
Circle Drawer's canvas widget declares one virtual node per painted circle (author key, AccessKit role, non-empty name, region inside its content box), so each circle reads as a node in the accessibility tree, snapshot, diff, UI graph, actions, layout and exploration of a served app.
A click on a circle's node acts on that circle and its diff names it; a covered, off-screen, disabled or removed circle node is refused with that cause, proved on the stand in both layout modes.
The canvas reading groups a canvas's draw commands under the virtual node that covers them, and a changed command reads with an identity in the canvas reading's diff.
It folds seven carried items: the canvas label region read against a shape or virtual node, covered between overlapping virtual nodes, the binary-level reading of a virtual node's id and name, the windows unused-import warning, handler coverage of a click on a virtual node, and the lint over declared regions.
It sweeps every committed stand pin and capability-statement count that reads Circle Drawer's screen, re-measuring them once the circles become nodes.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the seven_guis Circle Drawer task and its custom canvas widget, the first served app to declare virtual nodes — edits: CircleCanvas::virtual_nodes
@task_file n=2 path=packages/escher-driver/src/canvas.rs mark=edits
what: the driver's canvas reading behind Session::canvas and the canvas verb, with its line diff — edits: the grouping of draw commands under a virtual node, a draw command's identity in the diff
@task_file n=3 path=packages/escher-driver/src/lint.rs mark=edits
what: the driver's lint behind Session::lint and the lint verb, which reads no virtual node today
@task_file n=4 path=packages/escher-driver/src/canvas_lint.rs mark=edits
what: the canvas lint, which reads a canvas label against its label region
@task_file n=5 path=packages/escher-driver/src/coverage.rs mark=edits
what: the handler coverage reader, which counts a click on a virtual node as its canvas's own listener
@task_file n=6 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's acting step and per-verb target check, which reads a virtual node's covered, off-screen, disabled and removed refusals
@task_file n=7 path=packages/dioxus-native-dom/src/virtual_node.rs mark=edits
what: the Dioxus DOM bridge's virtual nodes — the virtual node id, its refusal and its covered reading
@task_file n=8 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs mark=edits
what: the blitz-tests check that virtual nodes read in every read — edits: the snapshot import placed behind a unix cfg
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: seven_guis' process check of the escher-session command line's answers on stdout
@task_file n=10 path=examples/seven_guis/tests/mcp_server.rs mark=edits
what: seven_guis' process check of the MCP role's tool results
@task_file n=11 path=tests/blitz-tests/tests/stand_cross_os_readings.rs mark=edits
what: the stand's cross-OS digest table of screen readings — edits: Circle Drawer's pinned digests
@task_file n=12 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS digest table of canvas readings — edits: Circle Drawer's pinned digests
@task_file n=13 path=tests/blitz-tests/tests/stand_canvas_commands.rs mark=edits
what: the stand's canvas commands table — edits: Circle Drawer's pinned command readings
@task_file n=14 path=tests/blitz-tests/tests/stand_canvas_lint.rs mark=edits
what: the stand's canvas lint check — edits: Circle Drawer's pinned lint readings
@task_file n=15 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand's check of the non-lean tasks — edits: Circle Drawer's pinned readings
@task_file n=16 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs mark=edits
what: the stand's Circle Drawer canvas check — edits: its pinned readings
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement — edits: its stated sizes and counts, and the statement of a virtual node's covered
@task_item n=1 part=needs
The custom widget's virtual node declaration: Widget::virtual_nodes returning one declaration per circle, each with an author key, a role that is AccessKit's Role, a non-empty name and a region inside the widget's content box; a region spilling the content box is refused, so a circle near the canvas edge must be handled.
@task_item n=2 part=needs
The virtual node id: a circle's node reads as its owner's id joined to its author key, through the stable element id's virtual node tier and the Dioxus bridge's virtual_node_id; a circle has no id of its own, so its key must stay stable across the circles' list.
@task_item n=3 part=needs
Every reading a served app's virtual node now reaches: the accessibility tree's virtual nodes, the snapshot's virtual node and its diff (added, removed, changed, differs), the UI graph, the actions answer's virtual node, the layout answer's app placer of a virtual node, and exploration.
@task_item n=4 part=needs
Click on a virtual node id: the click verb acts on the circle under it and the acting result's changed set names it; the per-verb target check refuses a covered, off-screen, disabled or removed circle node with that refusal cause, inside the closed set of refusal causes.
@task_item n=5 part=needs
Covered between virtual nodes: a virtual node reads covered only by an element over its region's centre, never by another virtual node of its canvas; overlapping circles put this provisional rule to the test, and visibility is read from geometry, never from a hit.
@task_item n=6 part=needs
The canvas reading's grouping: Session::canvas and the canvas verb group a canvas's draw commands under the virtual node covering them; the canvas answer holds no virtual node today, so naming a node's id in it changes what crosses the carriers (the canvas crossing as draw commands and pixels, the canvas verb's command kinds and brush forms, a dump over the snapshot budget crossing in parts).
@task_item n=7 part=needs
A draw command's identity in the canvas reading's diff: the reading stays deterministic and diffable by line, and a changed command reads as one changed command rather than one line removed and one added.
@task_item n=8 part=needs
The canvas label region: a canvas label's region read as the shape or the virtual node under it, against today's provisional reading by the content box cut by each clip, layer or mask path and the element's clip region.
@task_item n=9 part=needs
The lint over declared regions: the lint reads no virtual node today — no overlap, clip or off-screen finding on a declared region; overlapping circles test that provisional rule, its finding, reason keys and written form staying as the lint verb states them unless it changes.
@task_item n=10 part=needs
Handler coverage of a canvas: a virtual node carries no handler of its own, so the coverage reader lists none for it and counts a click on one as its canvas's own onclick listener under the canvas's id; the coverage role's answer and written form carry it.
@task_item n=11 part=needs
The binary-level reading of a virtual node's id and name: the socket host's answer, the escher-session command line's stdout and the MCP role's tool result each read a circle node's id and name on the built binary, plus the live cold-agent run's carrying of the stand's screen.
@task_item n=12 part=needs
The windows build's unused snapshot import in the virtual-nodes check moves behind a unix cfg, proved by a locked windows-gnu cargo check of the blitz-tests tests; cross-process session stays unsupported on Windows.
@task_item n=13 part=needs
A virtual node declaration change marks nothing in the changed set: adding, moving or removing a circle must reach the readings through the canvas's own repaint and the accessibility rebuild, not through a new mark.
@task_item n=14 part=needs
Every actionable element reads an author key and the one actionable rule: a circle node that takes click must read as actionable and keyed, with clickable read as the snapshot and bridge define it.
@task_item n=15 part=needs
Widget click synthesis and the widget click point read against the content box: a click on a circle node is synthesised at its region's centre on the canvas widget, and a canvas placement happens only at the click centre, so a click on a circle node must not place a new circle.
@task_item n=16 part=needs
The one table of verbs taking a virtual id and the validated verb set: no verb is added; the verbs a circle node takes are the ones that table already lists.
@task_item n=17 part=needs
Stand readings identical on every CI OS and the stand's pinned environment and DejaVu Sans typography: every moved digest and canvas reading is re-pinned once, identically for every OS.
@task_item n=18 part=needs
The literal pin sweep: every committed stand pin reading Circle Drawer's screen — cross-OS digests, canvas commands, canvas lint, non-lean tasks, the Circle Drawer canvas check — and the capability statement's stated sizes and counts move together, the capability statement held to the driver schema and its sections.
@task_item n=19 part=needs
Snapshot and diff content never logged, no snapshot content in logs, and the driver logs no content: a circle's name or region reaches no span or log line.
@task_item n=20 part=needs
No NodeId and no process-local value crosses a session: a circle node's id, name and region are author-derived and stable across runs.
@task_item n=21 part=needs
Measured limits stated where an agent reads the tool: a provisional rule the chunk leaves standing (covered between virtual nodes, the lint over regions, the canvas label region, handler coverage of a canvas) stays stated as measured in the capability statement.
@task_item n=22 part=needs
Scope boundaries the chunk's code must stay inside: no keyboard reach of the circles, no virtual node on a rotated or scaled canvas, and the canvas lint's label-overlap, label-contrast, command-outside-region and no-area-shape checks stay absent unless a carried item takes one.
@task_standing n=1
convention: every new stand scenario runs in both layout modes and its readings agree between them
home: conventions
@task_standing n=2
convention: one behaviour per integration test file and one blitz-tests target per file, declared in the one test table
home: conventions
@task_standing n=3
convention: a new test asserts its fixture's condition first and bears a descriptive name
home: conventions
@task_standing n=4
convention: stand-check helpers live in non-target shared modules, stated once
home: conventions
@task_standing n=5
convention: expect and unwrap carry a justification; new modules and public items carry docs
home: conventions
@task_standing n=6
convention: a stand reading is identical on every CI OS and the stand boots offline
home: standard-contracts
@task_item n=1 part=not_needed
WPT runner, its result kinds, statuses, outputs, script fetcher and PR-body diff tooling: the chunk runs no web platform test.
@task_item n=2 part=not_needed
The browser app, its chrome, about pages, history store and persistence: Circle Drawer is a seven_guis task, never a browser page.
@task_item n=3 part=not_needed
The open select list, select model, select keyboard and form submission rows: no circle is a select or form control.
@task_item n=4 part=not_needed
Page JavaScript under Boa, script globals, script fetching and microtask draining: the stand's apps run no page script.
@task_item n=5 part=not_needed
Networking — the net provider, per-host request caps, the HTTP cache and URL scheme routing: the chunk fetches nothing.
@task_item n=6 part=not_needed
CSS engine internals — Stylo cascade, Taffy layout, tables as grid, damage propagation, inline layout and text decoration: the circles are painted shapes inside a canvas, not laid-out boxes.
@task_item n=7 part=not_needed
Keyboard activation, key names and the press verb's keys: the circles' keyboard reach is the next entry's.
@task_item n=8 part=not_needed
Upstream sync by merge, cherry-picks and upstream-only workflows: the chunk takes nothing from another tree.
@task_item n=9 part=not_needed
The session socket's wire versions, the recording directory and the kept-check file format: the chunk widens no wire and adds no kept-verb.
@task_item n=10 part=not_needed
Paint-scene internals — paint layers, gradient tile caps, the select list painted overlay and scrollbar painting: the circles' paint stays as Circle Drawer issues it.
