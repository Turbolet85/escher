@task_prose part=session
Circle Drawer's circles became virtual nodes: each circle is declared by the canvas widget as `circle-canvas///c{id}`, role Image, named `Circle {id}, {d} px`, its region the circle's box, so the accessibility tree, snapshot, diff, graph, actions, layout and exploration read each circle by an app-given id that holds across undo and redo; an undone circle reads stale.
The canvas verb's full answer and every part now close each command line with ` node={k}` or ` node=-`: the 1-based index of the canvas's virtual node whose region holds the command's centre (nearest centre first), a closed number under a key no other pair of a line holds; no id or name of a node crosses in a canvas answer, and the summary's shape is unchanged, its digest moved.
The coverage line (the escher-session coverage role and the handler-coverage read) now lists a virtual node's `click` under its `{owner}///{key}` id, owned by the canvas; its form is unchanged; a kept check's replayed clicks are read again through the strict check reader to find the virtual ids clicked.
Circle Drawer's `data-circles` attribute is now `x,y,d,id` per circle; a three-field entry, an id of 0 or a non-whole id reads as no circle; the UI graph gains Circle Drawer's registered state `next_id`, declared written by the canvas's click.
blitz-dom re-exports the accessibility role type as `VirtualNodeRole` behind custom-widget; no dependency, manifest or lock line changed; no IPC method, endpoint, port, socket or env var added.
Measured: Circle Drawer's full canvas text 59 bytes at boot, 562 after one circle, 6,190 at 12 and 51,656 at 100 (still six parts); content box 696 by 396; the retired tag spelling ` n=` was never committed.
@task_file n=1 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: added — the stand test of Circle Drawer's circle virtual nodes: tree, snapshot, stale on undo, diff, graph, actions, layout, exploration, lint, the canvas node tag and a kept check's coverage
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: edited — blitz-dom's public surface, now re-exporting the virtual node role type behind the custom-widget feature
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: edited — the Circle Drawer app: its canvas widget declaring each circle as a virtual node, the data-circles attribute parse with its id field, and the registered graph state next_id
@task_file n=4 path=packages/escher-driver/src/canvas.rs
what: edited — the driver's canvas read: the canvas reading now groups each draw command under a virtual node of the canvas by region
@task_file n=5 path=packages/escher-driver/src/json/canvas.rs
what: edited — the canvas answer's writer: the full answer and parts, each command line now closed by the node tag
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: edited — the driver's answer-writer unit tests, now reading that no canvas line holds a key twice
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: edited — the closed driver verb schema and its help text: the canvas verb's text field help gains the node tag clause
@task_file n=8 path=packages/escher-driver/src/canvas_tests.rs
what: edited — the canvas read's unit tests, now with the grouping rule's table
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: edited — the handler coverage read over kept checks: now lists a virtual node's click by its id
@task_file n=10 path=packages/escher-driver/src/execute.rs
what: edited — the driver's command execution: the canvas verb's part cut, now carrying each command's node index
@task_file n=11 path=packages/escher-driver/src/canvas_lint.rs
what: edited — the canvas lint: its ink-box reader opened within the driver crate for the grouping
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: edited — the stand test of virtual nodes in the graph, actions, layout, exploration and coverage reads
@task_file n=13 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — shared session test helpers, writing data-circles with the id field
@task_file n=14 path=tests/blitz-tests/tests/common/mod.rs
what: edited — shared stand test tables: the unexercised-handlers list after drives, the 100-circle spills and the circle node causes
@task_file n=15 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: edited — the stand test of Circle Drawer's canvas painting
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: edited — the stand test pinning the canvas answer's sizes and digests across hosts
@task_file n=17 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: edited — the stand test of the canvas answer on every carrier, its glyph-line key pin now ending in node
@task_file n=18 path=docs/capabilities.md
what: edited — the committed capability statement: the canvas read as data, the node tag and Circle Drawer's circles as virtual nodes
@task_item n=1 part=touches
canvas-content-crossing: every canvas command line in the full answer and each part now carries a ` node={k|-}` tag, a 1-based number from the driver; still no node id or name crosses in a canvas answer — the expected amendment of this row.
@task_item n=2 part=touches
virtual-node-crossing: a virtual node's `{owner}///{key}` id now also crosses in the coverage line, with owner and event click — the expected amendment of this row.
@task_item n=3 part=touches
The coverage answer rows (coverage-answer-crossing, coverage-line-no-user-data, the coverage answer line contracts, the coverage role's answer-line and stdout-line content, the unexercised list): a virtual node's id and click now among the listed handlers; form, content classes and counts unchanged.
@task_item n=4 part=touches
The kept-check reads (coverage-check-files-untrusted, recorded-check-strict-reader): coverage now reads a kept check's replayed calls a second time to find clicked virtual ids, still through the strict check reader.
@task_item n=5 part=touches
kept-check-canvas-answer and kept-check-answer-whole: the canvas answer a kept check holds now carries the node tag; its digest moved.
@task_item n=6 part=touches
The canvas verb contract and name (driver-verbs canvas, command canvas) and the closed driver verb schema: the text field help adds one clause stating the node tag; its opening clause unchanged; no new verb, argument or field.
@task_item n=7 part=touches
The part and full flags (cli-flags part, cli-flags full): every part of the canvas answer now carries the tag; the 100-circle full text is 51,656 bytes and still six parts, against the answer and MCP result size bounds.
@task_item n=8 part=touches
virtual-node-declarations-validated: Circle Drawer now declares virtual nodes through the closed refusal set; none refused at 12 and 100 circles.
@task_item n=9 part=touches
stable-element-id virtual-node-form and stability: circles read `circle-canvas///c{id}` with an app-given id that holds across undo and redo.
@task_item n=10 part=touches
unknown-id-refused-stale-or-not-found and the stale and not-found refusals: an undone circle's id reads stale, met for virtual nodes.
@task_item n=11 part=touches
The graph answer rows (graph-answer-crossing, graph-answer nodes and states, signal-registry-holds-no-state-value) and the declared-write contract: the UI graph gains Circle Drawer's registered state next_id, declared written by the canvas's click.
@task_item n=12 part=touches
The accessible name and role rows (accessibility-adapter-content-classes, accessible-name-exits, author-id-on-accessibility-nodes): circle nodes reach the tree and snapshot with role Image and an app-text name of id and diameter.
@task_item n=13 part=touches
stable-id-exits and stable-id-not-logged: a circle's id now leaves through the tree, snapshot, diff, graph, actions, layout, exploration and the coverage line; no log line added.
@task_item n=14 part=touches
The attribute reading rows (attribute-readers-no-log-or-panic, attribute-text-leaves-by-ratified-carriers): Circle Drawer's data-circles parse gains a fourth id field; a malformed entry reads as no circle.
@task_item n=15 part=touches
The canvas lint rows (lint-findings-closed-content, canvas-label-lint-crossing, lint-findings-crossing): the lint's ink-box reader now also serves the grouping; the lint answer holds no circle node and its findings are unchanged.
@task_item n=16 part=touches
cold-agent-canvas-to-provider and mcp-result-crossing: canvas answers carried to the provider and in MCP results now hold the node tag numbers.
@task_item n=17 part=touches
custom-widget-content-box-frame and custom-widget-click-synthesis: Circle Drawer's content box reads 696 by 396; a click on a circle node is synthesized on the canvas, which adds a circle at its centre.
@task_item n=18 part=touches
exploration-report-closed-content: exploration now reaches a path whose call clicks a circle's virtual id.
@task_item n=19 part=touches
command-span-content-free and one-command-span-per-call: the canvas read and its grouping open no span; the command span unchanged.
@task_fact n=1 slug=input-validation
fact: Circle Drawer's data-circles attribute is read by a closed parse of x,y,d,id per circle; a three-field entry, an id of 0 or a non-whole id reads as no circle.
@task_item n=1 part=untouched
Dependency security: no dependency added or bumped, no manifest or lock line changed, no git dependency, no audit ignore.
@task_item n=2 part=untouched
Secret management and environment variables: no env var read or added, no secret or CI token touched.
@task_item n=3 part=untouched
API security's network and socket rows: no endpoint, port, socket, IPC method or listener added; session socket bounds, idle expiry and outbound request rules unchanged.
@task_item n=4 part=untouched
Authentication and authorization: no auth surface exists or was added.
@task_item n=5 part=untouched
The MCP protocol rows: protocol errors, request ids, initialize revision and stdio transport unchanged.
@task_item n=6 part=untouched
Error handling's refusal cause set and session error table: no cause or error added or renamed.
@task_item n=7 part=untouched
Engine fetch, navigation, HTML, CSS, iframe and script loading rows: no network, parse or script path changed.
@task_item n=8 part=untouched
Logging and monitoring: no log target, sink, field allowlist or span added; cold-agent and agent-run logging unchanged.
@task_item n=9 part=untouched
Data protection's storage rows: history database, HTTP cache, cookies, clipboard, file dialog and kept check owner-only medium unchanged.
@task_item n=10 part=untouched
Masked input values, select list rows and password painting: no masked control or select path changed.
