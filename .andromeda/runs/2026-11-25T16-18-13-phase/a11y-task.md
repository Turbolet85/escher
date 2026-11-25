@task_prose part=chunk
Circle Drawer's canvas declares one virtual node per circle it paints (an author key, an AccessKit role, a non-empty name, a region in its content box), the first served app to declare virtual nodes.
Each circle then reads as a node `{owner}///{key}` in the accessibility tree, the snapshot, the diff, the UI graph, `actions`, `layout` and exploration; `click` on a circle's node acts on its circle, and a covered, off-screen, disabled or removed circle node is refused with that cause.
The canvas reading groups a canvas's draw commands under the virtual node that covers them, and a changed command reads with an identity in the reading's diff.
It adds stand checks in both layout modes and the binary-level reading of a virtual node's id and name, sweeps every committed stand pin and stated literal that reads Circle Drawer's screen, and re-verifies seven carried items (label region, virtual-node `covered`, handler coverage of a virtual node, the lint reading no virtual node, a Windows unused-import warning).
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: Circle Drawer's drawing area, its canvas widget painting the circles — edits: the canvas widget's virtual node declaration (one node per circle: key, role, name, region)
@task_file n=2 path=packages/escher-driver/src/canvas.rs mark=edits
what: the canvas reading and its line diff of draw commands — edits: grouping of draw commands under the covering virtual node, a command's identity in the diff
@task_file n=3 path=packages/escher-driver/src/lint.rs
what: the lint, which reads no virtual node (no overlap, clip or off-screen finding on a declared region)
@task_file n=4 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint and its reading of a canvas label's region
@task_file n=5 path=packages/escher-driver/src/coverage.rs
what: handler coverage, which lists no handler for a virtual node and counts a click on one as its canvas's own listener under the canvas's id
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: driver action execution, the click on a node and its refusal causes (covered, off-screen, disabled, removed)
@task_file n=7 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the virtual node of a canvas and its `covered` reading (covered only by an element over its region's centre)
@task_file n=8 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs mark=edits
what: the virtual-nodes-in-every-read test file — edits: the `snapshot` import, put behind a unix-only cfg so the Windows build warns no unused import
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: seven_guis' process checks of the command line's stdout — edits: the binary-level reading of a virtual node's id and name
@task_file n=10 path=examples/seven_guis/tests/mcp_server.rs mark=edits
what: seven_guis' process checks of the socket host and MCP tool results — edits: the binary-level reading of a virtual node's id and name
@task_file n=11 path=docs/capabilities.md mark=edits
what: the capability statement, edited by hand — edits: its stated sizes and counts that read Circle Drawer's screen, and the statement of a virtual node's `covered`
@task_item n=1 part=needs
Circle Drawer's drawing area changes from one named canvas node read as pixels only to a canvas whose circles each read as a node of their own; the rule that names that drawing area as one canvas node is what this changes.
@task_item n=2 part=needs
Each circle node's id takes the reserved separator form `{owner}///{key}`; a circle has no id of its own, so its key, role and name are open choices, and every interactive node must carry a non-empty name.
@task_item n=3 part=needs
A circle's region must lie inside its canvas's content box (a region outside it is refused); a circle near the canvas edge spills the box, which the declaration must handle.
@task_item n=4 part=needs
Circle nodes appear in the accessibility tree, the snapshot and its text form, the snapshot diff, the UI graph (one node per snapshot node, carrying no values), the actions answer and its fields, layout and exploration, as served for this app.
@task_item n=5 part=needs
Click on a circle's node is a synthetic primary click at its centre that acts on its circle, and the diff names the node; the click refusal causes covered, off-screen, disabled and removed apply to a circle node and are read on the stand.
@task_item n=6 part=needs
A virtual node reads covered only by an element over its region's centre, never by another virtual node of its canvas; overlapping circles put this rule to the test and it is re-verified here.
@task_item n=7 part=needs
Handler coverage lists no handler for a virtual node and counts a click on one as its canvas's own listener under the canvas's id; Circle Drawer's canvas carries an onclick listener, so the coverage answer for it is re-verified.
@task_item n=8 part=needs
The lint reads no virtual node: overlapping circles must raise no overlap, clip or off-screen finding on a declared region, and the stand must still read no lint finding at boot.
@task_item n=9 part=needs
A canvas label's region (read against the content box cut by clip, layer or mask paths and the element's clip region, or as the shape or virtual node under it) is a carried question re-verified here.
@task_item n=10 part=needs
The canvas reading groups draw commands under the virtual node covering them and gives a changed command an identity in its diff, where today a changed command reads as one line removed and one added; naming a node's id in the canvas answer is a crossing change.
@task_item n=11 part=needs
The binary-level reading: the socket host's answer, the command line's stdout and a cold-agent live run each carry a virtual node's id and name, read on the binary.
@task_item n=12 part=needs
Every committed stand pin reading Circle Drawer's screen moves and is swept, literal pins included: the cross-OS digest tables, the canvas commands table, the canvas lint, non-lean task and Circle Drawer canvas checks, and the capability statement's sizes and counts.
@task_item n=13 part=needs
Stand checks of the circle nodes run in both layout modes.
@task_item n=14 part=needs
The Windows build must warn no unused import in the virtual-nodes test file, checked by a cargo check of the tests for the Windows GNU target.
@task_item n=15 part=needs
Evidence holds no raw log with a temp or home path; a control enters with its red reading; a probe worktree lives outside target/ and is not deleted by a plan step.
@task_standing n=1
convention: stand expectations (roles, names, focus order, readings) are stated once in shared tables the stand checks read
home: screen-reader-support
@task_standing n=2
convention: accessibility assertions are written as Rust tests over the AccessKit tree
home: 3-a11y-assertion-harness-contract
@task_standing n=3
convention: stand checks run in the slow-tier accessibility test leg, repeated three rounds, and gate CI
home: ci-integration · 9-ci-integration
@task_standing n=4
convention: the stand reads no lint finding at boot, and contrast on the stand is measured from computed styles
home: visual-design-verification
@task_item n=1 part=not_needed
Keyboard rows (Tab order, Enter and Space activation, focus moves and focus fixup, Cells grid and CRUD list keys): keyboard reach of the circles is the next entry's, not built here.
@task_item n=2 part=not_needed
Single and multiple select, open select list, list box and range input rows (their keys, rows, colours, events and snapshot readings): Circle Drawer's circles are no form control.
@task_item n=3 part=not_needed
ARIA attribute reads and ARIA state rows (aria-checked, aria-pressed, aria-selected, aria-invalid, aria-expanded, aria-level, aria-describedby): a virtual node's role and name are declared by the canvas widget, not read from ARIA attributes.
@task_item n=4 part=not_needed
Password masking, file input value and IME rows: no text entry is touched.
@task_item n=5 part=not_needed
Hidpi, zoom and colour scheme change rows: a rotated or scaled canvas's virtual nodes are another entry's, not built here.
@task_item n=6 part=not_needed
Platform adapter and assistive-technology reach rows, and the keyboard or assistive-technology drag-and-drop absence: the stand builds no platform adapter, and the binary reading here is over the socket host, command line and cold-agent pipe.
@task_item n=7 part=not_needed
Cognitive accessibility rows (invalid state not by colour or class alone): no form validity state is touched.
@task_item n=8 part=not_needed
UI graph state registration rows (use_graph_state, undeclared, unconfirmed): circles register no graph state.
