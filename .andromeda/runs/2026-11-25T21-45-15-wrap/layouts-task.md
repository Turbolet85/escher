@task_prose part=session
The session made each Circle Drawer circle a declared virtual node of the canvas on the stand: id circle-canvas///c{id}, role Image, name Circle {id}, {d} px, its bounds the circle's region cut to the content box.
Each circle now carries an app-given whole-number id from 1, stable across undo and redo; the canvas attribute data-circles reads x,y,d,id per circle, and the task registers the graph state next_id, written by the canvas click.
The canvas verb's full answer and every part now close each command line with node={k} or node=-, the 1-based index of the virtual node whose region holds the command's centre, nearest centre first; the text field help states it and the summary's shape is unchanged.
The coverage line now lists a virtual node's click under its circle-canvas///c{id} id, owned by the canvas; blitz-dom re-exports the accessibility role type for virtual node declarations.
Measured: Circle Drawer's content box is 696 x 396 (the plan said 700 x 400), 7 circles spill at 100, the full text grew to 59, 562, 6,190 and 51,656 bytes; the never-committed tag spelling n= was retired for node=.
@task_file n=1 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: new stand check that Circle Drawer's circles read as virtual nodes in the tree, snapshot, diff, graph, actions, layout, exploration, lint, coverage and the canvas answer's node tag
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: blitz-dom's public surface, now re-exporting the role type a virtual node declaration takes
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the stand's Circle Drawer task — its canvas, its circles with their ids, the data-circles attribute, the next_id graph state, the canvas widget declaring each circle as a virtual node, the canvas size constants
@task_file n=4 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas read, now grouping each draw command under the canvas virtual node holding its centre
@task_file n=5 path=packages/escher-driver/src/json/canvas.rs
what: the canvas verb's answer writer — the full answer and its parts, each command line now closed by its node tag
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: unit checks of the answer writers, among them the canvas line table and the check that no canvas line holds a key twice
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the session verbs' schema and field help, among them the canvas verb's text field help
@task_file n=8 path=packages/escher-driver/src/canvas_tests.rs
what: unit checks of the canvas read, among them the command grouping by region
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: the handler coverage reading behind the coverage role, now counting a replayed virtual node click
@task_file n=10 path=packages/escher-driver/src/execute.rs
what: the session verb executor, its canvas part cut now carrying each command's node index
@task_file n=11 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint, its ink box now shared with the command grouping
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the check that the UI graph, actions, layout and exploration read a canvas's virtual node by its id
@task_file n=13 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session test helpers, among them the one that writes circles through the data-circles attribute
@task_file n=14 path=tests/blitz-tests/tests/common/mod.rs
what: the shared stand test tables — the unexercised handlers after the drives, the 100-circle fixture's spills and the circle node refusal causes
@task_file n=15 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand check of Circle Drawer's canvas painting
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the stand check pinning the canvas answer's sizes and digests per task and part across hosts
@task_file n=17 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: the stand check that every carrier holds the canvas answer, pinning the glyph line's keys
@task_file n=18 path=docs/capabilities.md
what: the capability list, its section on reading a canvas as data with Circle Drawer's full-text sizes and the node tag
@task_item n=1 part=touches
The decision that Circle Drawer's circles are painted on a canvas: they are still painted, but each circle is now also a declared virtual node of the canvas, read by id and name in the tree, snapshot, diff and UI graph — the decision's pixels-only reading is stale.
@task_item n=2 part=touches
The stand element id circle-canvas: the canvas now owns virtual node ids of the form circle-canvas///c{id}, one per circle, and is the owner of each circle's click in coverage.
@task_item n=3 part=touches
Every stand control keyed: Circle Drawer's circles now carry an app-given id from 1, stable across undo and redo, a key on an item that held none.
@task_item n=4 part=touches
Stand ids as attributes alone, and stand names, roles and states by attributes alone: the circle nodes' ids, role Image and names are declared by the canvas widget's virtual node list, not by attributes — an exception or an extension of both rules.
@task_item n=5 part=touches
Stand lints clean at boot: the lint now holds no circle node and a circle's label wider than its circle reads no truncated finding — still clean, but a rule a picker should check against the new nodes.
@task_item n=6 part=touches
The escher-session MCP canvas tool contract: every command line of the full answer and of every part now closes with node={k} or node=-, under a key no other pair of a line holds; the summary's shape is unchanged, its digest moved.
@task_item n=7 part=touches
The escher-session MCP tools list and help answer: the canvas verb's text field help adds one clause stating the node tag; its opening clause is unchanged.
@task_item n=8 part=touches
The escher-session coverage answer: its line now carries a virtual node's {owner}///{key} id with owner and event click; the line's form is unchanged; Circle Drawer's unexercised handlers after the drives went from 1 to 4.
@task_item n=9 part=touches
The escher-session graph answer: the UI graph gains Circle Drawer's registered state next_id, declared written by the canvas click.
@task_item n=10 part=touches
The escher-session snapshot, actions and layout answers: Circle Drawer's circles now appear in them as virtual nodes by id, never by name.
@task_item n=11 part=touches
The headless stand pinned viewport and any row stating Circle Drawer's canvas size: the canvas content box reads 696 x 396, the widget's constants, not 700 x 400.
@task_item n=12 part=touches
Session recording as a replayable check: a kept check's replayed virtual node click is now counted by the coverage role under the node's id.
@task_item n=13 part=touches
The escher-session served tasks: Circle Drawer is served by no escher-session role, so the binary reading of its circles waits on it being served — a row listing the served tasks stays true but meets this gap.
@task_fact n=1 slug=surface-desktop-native
fact: Circle Drawer's circles are declared virtual nodes of the canvas — id circle-canvas///c{id}, role Image, name Circle {id}, {d} px, bounds the circle's region cut to the content box — while still painted on the canvas.
@task_fact n=2 slug=surface-desktop-native
fact: Circle Drawer's circle ids are app-given whole numbers from 1, kept by the next_id graph state and stable across undo and redo; data-circles reads x,y,d,id per circle, and an entry of three fields, an id of 0 or a non-whole id reads as no circle.
@task_fact n=3 slug=surface-desktop-native
fact: Circle Drawer's canvas content box is 696 x 396, its CSS sizing the border box.
@task_fact n=4 slug=surface-desktop-native
fact: Stand clicks on Circle Drawer's canvas all land at the canvas's centre, so every circle added through them shares one region; a check needing circles apart writes data-circles directly.
@task_fact n=5 slug=surface-cli
fact: The canvas verb's full answer and every part close each command line with node={k} or node=-, the 1-based index of the canvas virtual node whose region holds the command's centre, nearest centre first, under a key no other pair of a line holds; the index is positional.
@task_fact n=6 slug=surface-cli
fact: The coverage answer lists a canvas virtual node's click under its {owner}///{key} id, owned by the canvas, exercised where a replayed click named it.
@task_fact n=7 slug=surface-cli
fact: The canvas node tag's name node is occupied; the spelling n= is refused because the glyph line already holds n.
@task_item n=1 part=untouched
The ShellProvider window chrome contract items: no window control or decoration moved.
@task_item n=2 part=untouched
The stand element ids of the other tasks (counter, temperature, flight, timer, CRUD, cells, load, order) and the task cards: none renamed, added or removed.
@task_item n=3 part=untouched
Circle Drawer's undo, redo, diameter and close control ids: unchanged; only the canvas gained virtual nodes.
@task_item n=4 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command lines and the cold-agent decisions: no argument, exit code, output path or prompt rule moved.
@task_item n=5 part=untouched
The escher-session exit codes, refusals, command form, recording failures, host log name, screenshot answer, MCP stdio, protocol revisions and result bound: unchanged; no IPC method, port, env var or crate added.
@task_item n=6 part=untouched
The fixed document skeleton, the load page's network provider, the order page, the open list rows, scrolling into view and visible regions through clipping boxes: no layout or skeleton rule moved.
@task_item n=7 part=untouched
The stand accent colour and every colour: unchanged.
@task_item n=8 part=untouched
The mobile-native and web-spa surfaces and the absence of arbitrary index.html templates: not reached.
