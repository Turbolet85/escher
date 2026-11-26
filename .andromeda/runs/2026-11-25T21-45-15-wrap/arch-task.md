@task_prose part=session
Circle Drawer's circles became virtual nodes on the seven_guis stand: its canvas widget declares one node per circle, id circle-canvas///c{id}, role Image, name Circle {id}, {d} px, region the circle's box cut to the canvas; the circle id is app-given from 1, stable across undo and redo, carried as a fourth field of the canvas attribute data-circles, with a registered graph state next_id.
The canvas verb now groups draw commands by those nodes: every line of the full answer and of every part closes with node={k} or node=-, the 1-based index of the virtual node whose region holds the command's centre, nearest centre first; the summary's shape is unchanged, its digest now of the tagged text; the reading struct gains a per-command node list.
A glyph run's grouping bounds are its ink box (the canvas lint's ink reader, now crate-visible), its glyph-origins box the fallback; the origins box grouped only 95 of 100 labels.
Handler coverage now lists a virtual node's click under its id, owned by the canvas; blitz-dom re-exports AccessKit's Role as VirtualNodeRole behind custom-widget, flagged upstreamable.
Measured: Circle Drawer's content box 696 x 396 (plan 700 x 400), 7 spills at 100 circles, full text 59 / 562 / 6,190 / 51,656 bytes at 0 / 1 / 12 / 100 circles, every canvas-commands table row's size and digest re-measured; the tag spelling n= was retired for node= because the glyph line already holds n.
No dependency, IPC method, port, env var, crate, CI leg or gate changed; CI green on linux, windows and macOS.
@task_file n=1 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: new stand integration target reading Circle Drawer's circle virtual nodes through the tree, snapshot, diff, graph, actions, layout, exploration, lint, canvas grouping and handler coverage, in both layout modes
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: blitz-dom crate root, its public re-exports, now VirtualNodeRole behind the custom-widget feature
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the seven_guis stand's Circle Drawer task, its canvas custom widget, its circle list and its registered graph signals
@task_file n=4 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas reader, Session::canvas and CanvasReading, now grouping draw commands under the canvas's virtual nodes
@task_file n=5 path=packages/escher-driver/src/json/canvas.rs
what: the canvas verb's answer writer, its line form for the full answer, parts and summary
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the driver JSON writer's unit tests, the canvas line table
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the driver's command schema and self-description, the canvas verb's text field help
@task_file n=8 path=packages/escher-driver/src/canvas_tests.rs
what: the driver canvas reader's unit tests
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: the handler coverage reader behind the coverage role
@task_file n=10 path=packages/escher-driver/src/execute.rs
what: the driver's command execution, the canvas verb's part cut
@task_file n=11 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas label lint and its ink box reader
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the integration target reading a canvas's virtual node by id in the UI graph, actions, layout, exploration and replayed kept checks
@task_file n=13 path=tests/blitz-tests/tests/session_common/mod.rs
what: shared stand session helpers for blitz-tests, the Circle Drawer drawing helper
@task_file n=14 path=tests/blitz-tests/tests/common/mod.rs
what: shared blitz-tests fixtures, the unexercised-handlers-after-drives list and the Circle Drawer spill and cause tables
@task_file n=15 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand target reading Circle Drawer's canvas through the canvas verb
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the stand target pinning the canvas answer's sizes and digests, the cross-OS byte-equal table
@task_file n=17 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: the stand target reading the canvas answer across the carriers, its glyph-line key pin
@task_file n=18 path=docs/capabilities.md
what: the capability statement, its canvas-read-as-data section and Circle Drawer's measured sizes
@task_item n=1 part=touches
decision/canvas-reading-diffable: every canvas line now ends with a node tag, so a changed command's removed and added lines carry one node=; the index is positional, by declaration order.
@task_item n=2 part=touches
The canvas verb's line form and its contracts (contract/driver-verbs/canvas, contract/escher-session/canvas, contract/canvas-verb/command-kinds, contract/canvas-verb/brush-forms): every command kind's line closes with node={k} or node=-, a key no other pair of a line holds.
@task_item n=3 part=touches
contract/driver-session/canvas: Session::canvas keeps its signature but now groups commands through the document's virtual nodes and the viewport scale; CanvasReading gains a public per-command nodes list; NoCanvas and NotLaidOut group nothing.
@task_item n=4 part=touches
decision/canvas-crossing and decision/canvas-dump-in-parts: the tag rides every part and the full answer, a number only, no id or name crossing; the summary keeps its shape with its digest now of the tagged full text.
@task_item n=5 part=touches
Handler coverage (contract/handler-coverage/canvas, contract/driver-recording/handler-coverage, contract/coverage-role/answer, contract/cli-coverage-role/shape, contract/escher-session-cli/coverage-role): the coverage line now lists a virtual node's click under its owner///key id, owned by the canvas; the line's form is unchanged.
@task_item n=6 part=touches
contract/nodes-widgets/virtual-nodes and decision/virtual-node-role-is-accesskit-role: the role type VirtualNodeDeclaration.role takes is now reachable as blitz-dom's VirtualNodeRole, a re-export of AccessKit's Role behind custom-widget; the field's type is unchanged.
@task_item n=7 part=touches
decision/direct-engine-changes-flagged-upstreamable and decision/fork-additions-in-own-functions: the blitz-dom re-export is an escher addition to an upstream crate, flagged upstreamable.
@task_item n=8 part=touches
The virtual node rows now have their first stand app declaring nodes (contract/accessibility-tree/virtual-nodes, contract/snapshot/virtual-node, contract/actions-verb/virtual-node, contract/driver-verbs/click-virtual-node, contract/stable-element-id/virtual-node-id, contract/driver-session/layout-virtual-node, decision/virtual-id-verb-table, decision/declaration-change-marks-nothing): circles read by id, refused stale after undo, never by name.
@task_item n=9 part=touches
decision/canvas-label-region and absence/canvas-lint-checks-beyond-truncation: the lint's ink box reader is now shared with the canvas grouping; the lint's findings and the absent checks are unchanged (witnessed, a later entry owns the three virtual-node lint checks).
@task_item n=10 part=touches
decision/seven-guis-graph-signals, decision/graph-state-by-registration and contract/ui-graph/writers: Circle Drawer registers the graph state next_id, declared written by the canvas's click.
@task_item n=11 part=touches
decision/canvas-placement-at-click-centre and decision/widget-click-point-content-box: driver clicks on Circle Drawer all land at the canvas's centre, so circles drawn through the stand's click path share one region and read node=1; the content box measured 696 x 396.
@task_item n=12 part=touches
decision/stand-readings-same-on-every-ci-os: the canvas commands table was re-measured, every full and part row's size and digest moved, and read byte-equal on linux, windows and macOS.
@task_item n=13 part=touches
decision/capability-statement and contract/capability-statement/sections: the capability statement's canvas section adds the node tag and its key rule, the click-adds-a-circle and label-region limits, and Circle Drawer's full-text sizes moved to 59, 562, 6,190 and 51,656 bytes.
@task_item n=14 part=touches
decision/measured-limits-stated-in-the-tool, decision/fixed-program-text-on-carriers and decision/one-self-description-for-cli-and-mcp: the canvas verb's text field help adds one clause stating the node tag; its opening clause is unchanged.
@task_item n=15 part=touches
contract/driver-explore/step-budget and contract/driver-explore/explore: exploration reaches a circle's click id only with a budget of 80 (path 37), a test parameter; any row holding a budget of 40 for this reach is stale.
@task_item n=16 part=touches
decision/no-node-id-across-session and contract/snapshot/no-process-local-values: the node index crossing in canvas lines is a positional number, no NodeId and no process-local value.
@task_item n=17 part=touches
contract/headless-stand/boot and decision/session-apps-one-table: Circle Drawer's stand app now declares virtual nodes and holds circle ids; it is still served by no escher-session role.
@task_fact n=1 slug=established-decisions
fact: The canvas reading groups each draw command under the canvas's virtual node whose region holds the command's centre, the region whose centre lies nearest first, by its 1-based positional index, or none; a glyph run's bounds are its ink box with its glyph-origins box the fallback.
@task_fact n=2 slug=standard-contracts
fact: Every canvas answer line, full or part, closes with node={k} or node=-, under a key no other pair of a line holds, so no canvas line holds a key twice; the summary's shape is unchanged and its digest is of the tagged text.
@task_fact n=3 slug=standard-contracts
fact: Handler coverage lists a virtual node's click under its owner///key id, owned by the canvas, exercised where a replayed click named it; a canvas click landing inside a node is counted for the canvas's own listener under the canvas's id alone.
@task_fact n=4 slug=established-decisions
fact: Circle Drawer declares each circle as a virtual node circle-canvas///c{id}, role Image, name Circle {id}, {d} px, region its box cut to the canvas; the id is app-given from 1, stable across undo and redo, carried as data-circles x,y,d,id, a three-field or zero or non-whole id entry read as no circle.
@task_fact n=5 slug=standard-contracts
fact: blitz-dom's crate root re-exports AccessKit's Role as VirtualNodeRole behind the custom-widget feature, the type a virtual node declaration's role takes, so an app reaches it without naming accesskit.
@task_fact n=6 slug=established-decisions
fact: A sibling virtual node over another is not refused covered, and every driver click on Circle Drawer lands at the canvas's centre, so circles added through stand clicks share one region and read index 1; the positional index is proven only on circles written apart.
@task_fact n=7 slug=standard-contracts
fact: A canvas's virtual nodes take no keyboard path and hold no selected state; a click inside a circle's node adds a circle rather than selecting it.
@task_item n=1 part=untouched
The CSS, style, layout and text engine rows (Stylo, Taffy, Parley, damage, incremental layout, tables, scrolling, select lists): no engine behaviour changed.
@task_item n=2 part=untouched
WPT runner, browser app, history, networking, script engine and their contracts: none reached.
@task_item n=3 part=untouched
Stack and dependency rows: no dependency added or bumped, no toolchain or tool version moved.
@task_item n=4 part=untouched
CI legs, gates, coverage floors and the stand-repeat budget: the new stand target joins stand-repeat by its prefix, the leg's named-target count and every gate unmoved.
@task_item n=5 part=untouched
The session socket wire, MCP server protocol, session errors, exit codes and served-port absences: no method, endpoint, port, socket or env var added.
@task_item n=6 part=untouched
Occupied names (crates, binaries, env vars, servers): no name taken or freed.
@task_item n=7 part=untouched
Telemetry, spans and logging rows: the canvas read opens no new span, the command span is unchanged.
@task_item n=8 part=untouched
Refusal causes and faults: the closed set is unchanged; stale and not-found are reused for circle ids.
@task_item n=9 part=untouched
Snapshot text line fields, node keys, the kept check file format and recording directory: their forms are unchanged.
