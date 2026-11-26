@task_prose part=session
The session made Circle Drawer's circles virtual nodes: each circle carries an app-given id from 1 and is declared on the canvas as a node with the id circle-canvas///c{id}, role Image and the name Circle {id}, {d} px. Its region is the circle's cut box.
The canvas verb's full answer and every part now close each command line with a node tag, node={k} or node=-. k is the 1-based index of the canvas's virtual node whose region holds the command's centre, nearest centre first, and - means none. No other pair of a line uses the key node, so no line holds a key twice. The summary's shape is unchanged, and its digest is now taken over the tagged full text.
The canvas verb's text field help adds one clause stating the tag; its opening clause is unchanged. The coverage line now lists a virtual node's click under the node's id, owned by the canvas; the line's form is unchanged.
Retired: the earlier tag spelling n={k}, which was never committed, because it reused the glyph line's key n. Measured: Circle Drawer's content box is 696 x 396 (not 700 x 400), 7 circles spill at 100, and a glyph run is grouped by its ink box.
Re-measured: Circle Drawer's full canvas text is 59, 562, 6,190 and 51,656 bytes at boot, after one circle, at 12 and at 100. The canvas commands table was re-measured, and Circle Drawer's unexercised handlers after the drives go from 1 to 4.
blitz-dom re-exports the accessibility role type for virtual node declarations. No dependency, IPC method, endpoint or crate was added.
@task_file n=1 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: new stand target, the stand check of Circle Drawer's circle virtual nodes; it covers tree, snapshot, diff, undo and redo staleness, graph, actions, layout, exploration, lint, node tags and kept coverage checks
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: edited; the engine DOM crate's public surface, now re-exporting the virtual node role type behind custom-widget
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: edited; the Circle Drawer task's widget and canvas: circles with ids, the data-circles attribute (x,y,d,id), the next_id graph signal, the canvas's virtual node declarations and its size constants
@task_file n=4 path=packages/escher-driver/src/canvas.rs
what: edited; the driver's canvas reading behind the canvas verb, which now groups each command under a virtual node region
@task_file n=5 path=packages/escher-driver/src/json/canvas.rs
what: edited; the canvas verb's answer writer, which writes the canvas line form for the full answer, the parts and the summary, now closing each line with the node tag
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: edited; unit checks of the driver's answer text, including the canvas line table and the check that no line holds a key twice
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: edited; the verbs' schema and field help that escher-session help and describe read, including the canvas verb's text field help
@task_file n=8 path=packages/escher-driver/src/canvas_tests.rs
what: edited; unit checks of the canvas reading, adding the command grouping table
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: edited; the handler coverage reading behind the coverage line and coverage answer, now counting a virtual node's click
@task_file n=10 path=packages/escher-driver/src/execute.rs
what: edited; the driver's verb execution, where the canvas verb's part cut now carries the node indices with the commands
@task_file n=11 path=packages/escher-driver/src/canvas_lint.rs
what: edited; the canvas label lint, whose ink box is now shared with the grouping
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: edited; the stand check that the UI graph, actions, layout, exploration and coverage read a canvas's virtual node by its id
@task_file n=13 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited; the stand's shared session helpers, including the Circle Drawer data-circles writer
@task_file n=14 path=tests/blitz-tests/tests/common/mod.rs
what: edited; the stand's shared lists: the unexercised handlers after the drives, the 100-circle spills and the circle node refusal causes
@task_file n=15 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: edited; the stand check of Circle Drawer's canvas and its data-circles attribute
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: edited; the stand check of the canvas verb's summary, full answer and parts, with the table of sizes and digests
@task_file n=17 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: edited; the stand check of every carrier of the canvas answer, which pins the glyph line's keys (now ending in node)
@task_file n=18 path=docs/capabilities.md
what: edited; the capabilities document: the canvas read as data, its line tag, its full-text sizes, and the circle node limits
@task_item n=1 part=touches
decision/canvas-line-form-conventions: the line form now closes with the node tag under the key node. That key appears in no other pair of a line, so no line holds a key twice. The text help states the tag.
@task_item n=2 part=touches
contract/driver-verbs-and-virtual-clock/canvas: the canvas verb's full answer and every part now carry the node tag on each line, and the summary's digest is taken over the tagged text. Its clock behaviour is unchanged.
@task_item n=3 part=touches
contract/coverage-line/content: the coverage line now carries a virtual node's id, of the form owner///key, with the canvas as owner and the event click. The line's form is unchanged.
@task_item n=4 part=touches
contract/escher-session-commands/coverage-answer: the coverage answer now lists a circle's click under the circle's own id, and the circle reads exercised when a replayed click named it.
@task_item n=5 part=touches
contract/escher-session-commands/help and contract/escher-session-mcp/help-result: the help text read from the schema adds the canvas text field's tag clause.
@task_item n=6 part=touches
contract/ui-graph-line/content: the UI graph now holds Circle Drawer's registered state next_id, which a click on circle-canvas writes, and the circles as virtual nodes by id.
@task_item n=7 part=touches
contract/actions-line/content and contract/actions-line/nodes: Circle Drawer's circles now appear as nodes by their circle-canvas///c{id} ids. They are read by id, never by name.
@task_item n=8 part=touches
contract/actions-line/stale: an undone circle now reads stale, and its redo keeps the same id.
@task_item n=9 part=touches
contract/escher-session-commands/snapshot, contract/escher-session-commands/layout-answer and contract/escher-session-commands/graph-answer: each circle now reaches these answers as a virtual node: role Image, name Circle {id}, {d} px, bounds its cut region.
@task_item n=10 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: a canvas answer now carries a number per line, in the tag only. No id or name crosses into it.
@task_item n=11 part=touches
decision/circle-drawer-canvas-colours-in-code (indirect): the Circle Drawer widget changed. It gained ids, virtual nodes and size constants of 696 x 396. Its colours are unchanged.
@task_item n=12 part=touches
contract/coverage-check-files/read-bound and contract/coverage-check-files/refusal (indirect): a kept check now lists an unclicked circle's click, and is still read by the same check reader.
@task_fact n=1 slug=surface
fact: The canvas line's node index is positional. It is the 1-based place of the canvas's virtual node in declared order, never its id. A command whose centre lies in no region reads node=-, ties go to the nearest centre, and driver clicks all land at the canvas centre, so they share one index.
@task_fact n=2 slug=surface
fact: The canvas line's node tag groups a glyph run by its ink box, with the box of its glyph origins as the fallback. A shape or fill is grouped by its own bounds.
@task_item n=1 part=untouched
color-palette tokens, engine colours, select list colours and color-scheme rows: no engine colour or token changed.
@task_item n=2 part=untouched
typography, spacing and depth-strategy rows (fonts, font fallback, margins, z-index, hit-testing at clips, stacking contexts): nothing in text layout, spacing or paint order moved.
@task_item n=3 part=untouched
motion rows other than the canvas verb (advance, click, press, type, scroll, animation, settle, select list motion): the virtual clock and scrolling are unchanged.
@task_item n=4 part=untouched
iconography (cursor and favicon) and shell provider rows (cursor, maximize, minimize, drag, decorations): untouched.
@task_item n=5 part=untouched
the select list's events, accessibility, screenshot and snapshot bounds rows: no select behaviour changed.
@task_item n=6 part=untouched
escher-session exit statuses, exit codes, stderr, the session error object and stdout lines: no new status or error form.
@task_item n=7 part=untouched
escher-session tasks rows: no role was added. Circle Drawer is still served by no escher-session task.
@task_item n=8 part=untouched
WPT runner output and result colour rows, js-window rows, the dioxus-native config rows and the web-spa surface: untouched.
