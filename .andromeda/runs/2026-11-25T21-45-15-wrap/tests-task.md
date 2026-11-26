@task_prose part=session
Circle Drawer's circles became virtual nodes on the seven_guis stand: each circle a declared node with the id circle-canvas///c{id}, role Image, name Circle {id}, {d} px, its bounds its cut region, its id app-given and stable across undo and redo.
The driver's canvas reading now groups draw commands by the canvas's virtual nodes; every line of the canvas verb's full answer and parts closes with node={k} or node=-, the 1-based index of the node whose region holds the command's centre, nearest centre first, under a key no other pair of a line holds; the canvas text field help states it.
Handler coverage now lists a virtual node's click under its id, owned by the canvas, exercised where a replayed click named it; blitz-dom re-exports the role type as VirtualNodeRole; Circle Drawer registers the graph state next_id and its data-circles attribute holds x,y,d,id per circle.
Measured: Circle Drawer's full canvas text 59, 562, 6,190 and 51,656 bytes at boot, one, 12 and 100 circles; the canvas commands table re-measured with every digest moved; the unexercised-after-drives row for Circle Drawer 1 to 4 handlers; content box 696 x 396; 7 spills at 100 circles.
Added the stand check target stand_circle_nodes, run in both layout modes; retired the earlier tag spelling n= that shared a key with the glyph line's glyph count.
@task_file n=1 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: new stand check target, one behaviour: Circle Drawer's circles as virtual nodes read in the tree, snapshot, diff, graph, actions, layout, exploration, lint and handler coverage, in both layout modes
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: blitz-dom's public surface; edited to re-export the virtual node role type
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the seven_guis Circle Drawer task the headless stand boots; edited to declare each circle a virtual node, register the graph state next_id and write data-circles with an id per circle
@task_file n=4 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas reading behind the canvas verb; edited to group each draw command under a virtual node's region
@task_file n=5 path=packages/escher-driver/src/json/canvas.rs
what: the driver JSON answer writer for the canvas verb's full answer and parts; edited to close every line with the node tag
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: inline unit test module of the driver JSON answers; edited to share the canvas line table and add the row that no canvas line holds a key twice
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the driver schema, the verb table and its help text the capability statement is pinned to; edited the canvas text field help
@task_file n=8 path=packages/escher-driver/src/canvas_tests.rs
what: inline unit test module of the driver's canvas reading; edited to add the grouping table test
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: handler coverage measured by replay, the coverage line; edited to list a virtual node's click
@task_file n=10 path=packages/escher-driver/src/execute.rs
what: the driver executor; edited the canvas verb's part cut to carry each command's node
@task_file n=11 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint over canvas labels; its ink box made crate-visible
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: stand check target for virtual nodes in the UI graph, actions, layout and exploration; edited to read a replayed virtual click reaching its canvas listener
@task_file n=13 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared module of the session checks; edited its Circle Drawer drawing helper
@task_file n=14 path=tests/blitz-tests/tests/common/mod.rs
what: the shared stand tables module; edited the unexercised-after-drives table and added the circle spills and circle node causes tables
@task_file n=15 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: stand check target for Circle Drawer's canvas reading; edited for the node tag and the circle id
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the canvas commands table held to one table across hosts; its sizes and digests re-measured
@task_file n=17 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: stand check of the canvas answer in every carrier; its glyph-line key pin restated
@task_file n=18 path=docs/capabilities.md
what: the capability statement pinned to the driver schema, its claims naming their checks; edited Circle Drawer's canvas sizes and new claims on circle nodes and the node tag
@task_item n=1 part=touches
The reserved owner-key id form: the stand now declares virtual nodes, circle-canvas///c{id} on Circle Drawer, so a row saying the stand declares none or reads no such id goes stale.
@task_item n=2 part=touches
Canvas handlers not counted by handler coverage and the handler coverage limit on a canvas handler: coverage now lists a virtual node's click under its id, owned by the canvas; a canvas click inside a node still counts for the canvas alone.
@task_item n=3 part=touches
The coverage line's owner and shape and the coverage command of escher-session: the line now carries virtual ids with owner and event click; its form is unchanged.
@task_item n=4 part=touches
Canvas commands held to one table across hosts: every full and part row's size and digest moved, the summaries keep their sizes; CI read the table equal on windows and macOS.
@task_item n=5 part=touches
Cross-OS readings of the served tasks by answer digest and the canvas verb of the escher-session command line: every canvas answer line now closes with node={k} or node=-.
@task_item n=6 part=touches
Capability statement pinned to the driver schema and claim names held to the schema: the canvas text field help gained a clause and the statement's Circle Drawer sizes and claims moved.
@task_item n=7 part=touches
Capability claims name their checks: new claims on a click on a circle's node and a canvas label's region name the new stand target.
@task_item n=8 part=touches
Task state registered for the UI graph: Circle Drawer registers next_id, declared written by the canvas click.
@task_item n=9 part=touches
Shared graph drives exercise every stand write: the unexercised-after-drives row for Circle Drawer moved from 1 to 4 handlers, the three circle clicks added.
@task_item n=10 part=touches
Shared stand tables in one module: the circle spills table (7 rows) and the circle node causes table joined it.
@task_item n=11 part=touches
One test target per file and stand checks in both layout modes: the new target stand_circle_nodes joins stand-repeat by its prefix and runs each case in both modes.
@task_item n=12 part=touches
Element ids' virtual form and the snapshot node's clickable: circles read as virtual elements in the tree and snapshot, taking click.
@task_item n=13 part=touches
Driver refusal causes stale, not-found and covered: an undone circle reads stale, its redo keeps its id; a circle under another is not refused covered.
@task_item n=14 part=touches
Lint defects off-screen, clipped, zero-size and overlap: the lint holds no circle node; those checks on virtual nodes stay unbuilt.
@task_item n=15 part=touches
Table-driven tests assert their row count: the canvas line table is shared by both line tests, each asserting its count.
@task_item n=16 part=touches
No ids or names in stand logs and session writes nothing of the screen: no circle id or name crosses into a canvas answer, the node tag is a number.
@task_item n=17 part=touches
seven_guis headless stand boot of Circle Drawer: the canvas's content box reads 696 x 396 and the boot canvas text grew to 59 bytes.
@task_item n=18 part=touches
Exploration checks and exhaustion: a circle click is reached at path 37 with a budget of 80 in the new target.
@task_item n=19 part=touches
Verb-count and test-rename sweeps: the canvas help's opening clause is unchanged, its pins swept by it.
@task_fact n=1 slug=test-data-fixtures
fact: Driver clicks on Circle Drawer all land at the canvas's centre, so a check needing circles apart writes data-circles through the mutator rather than clicking.
@task_fact n=2 slug=unit-test-strategy
fact: No canvas answer line holds a key twice: a unit row reads every command kind's line, grouped or not, and the node tag uses a key no command's own pairs write.
@task_fact n=3 slug=integration-test-strategy
fact: Handler coverage lists a virtual node's click under its owner///key id, owned by its canvas, exercised where a replayed click named it; a canvas click landing inside a node is counted for the canvas listener alone.
@task_fact n=4 slug=unit-test-strategy
fact: The lint reads no virtual node: off-screen, clipped and zero-size on virtual nodes are absent, overlap skipped on either side.
@task_fact n=5 slug=3-test-harness-contract
fact: The stand's Circle Drawer declares each circle a virtual node, circle-canvas///c{id}, role Image, its bounds its cut region, its id app-given and kept across undo and redo.
@task_fact n=6 slug=integration-test-strategy
fact: A canvas command's node index is positional: circles sharing one region tie and read the first index, so a separated circle's index is proven only on circles written apart.
@task_item n=1 part=untouched
WPT runner, its suites, statuses and expectations: no web conformance surface moved.
@task_item n=2 part=untouched
CI legs, workflows, caches, package installs and their exit codes: no leg, job or named-target count moved and no dependency was added.
@task_item n=3 part=untouched
Cold-agent pipe and agent-run verbs, events, verdicts and exit codes: neither was run nor changed.
@task_item n=4 part=untouched
Session wire, socket, state directory, session errors and escher-session exit codes: no protocol, endpoint or error was added.
@task_item n=5 part=untouched
Coverage floors, mutation testing and complexity gates: no floor or threshold moved.
@task_item n=6 part=untouched
Virtual clock, advance, timer ticks and network stand-ins: no time or load behaviour changed.
@task_item n=7 part=untouched
Screenshots, pixel tests on a CPU buffer and shot digests: no paint or colour changed.
@task_item n=8 part=untouched
Harness construction, input and settle contracts: no harness API changed.
