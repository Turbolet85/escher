@task_prose part=session
Circle Drawer's circles became virtual nodes on its canvas: each circle declares a node with id circle-canvas///c{id}, role Image, name Circle {id}, {d} px, its bounds its cut region, read in the accessibility tree, the snapshot, the diff, the UI graph, actions, layout and exploration by id.
Each circle now carries an app-given whole-number id from 1, stable across undo and redo; an undone circle reads stale, and Circle Drawer registers the graph state next_id, written by the canvas click.
The canvas read groups its draw commands by node: every line of the canvas answer, full and in parts, closes with node={k} (the 1-based index of the virtual node whose region holds the command's centre, nearest centre first) or node=- for none; no id or name crosses in a canvas answer; the canvas verb's text help states the tag.
Handler coverage now lists a virtual node's click under its virtual id, owned by its canvas; Circle Drawer's unexercised handlers after the drives went 1 to 4.
blitz-dom re-exports the AccessKit role type as VirtualNodeRole for a declared virtual node's role.
Measured: Circle Drawer's content box 696 x 396 (not 700 x 400), 7 circles spill at 100, the full canvas text 59, 562, 6,190 and 51,656 bytes at 0, 1, 12 and 100 circles; a keyboard path to the circles is not built.
@task_file n=1 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: the stand check of Circle Drawer's circle nodes - tree, snapshot, diff, stale on undo, graph, actions, layout, exploration, lint, covered refusal, coverage, node tags, both layout modes
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: blitz-dom's public surface - the VirtualNodeRole re-export of the AccessKit role behind the custom-widget feature
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: seven_guis Circle Drawer - the circle canvas widget declaring a virtual node per circle, the circle ids, data-circles, the registered next_id graph state
@task_file n=4 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas reading - grouping each draw command under a virtual node's region by index
@task_file n=5 path=packages/escher-driver/src/json/canvas.rs
what: the canvas answer's line writer - every line closed by the node tag, full answer and parts
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: unit tests of the canvas answer's line form - one line per command kind, no key twice
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the driver schema's verb help - the canvas verb's text field help
@task_file n=8 path=packages/escher-driver/src/canvas_tests.rs
what: unit tests of the canvas reading - the grouping under the region holding a command's centre
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: handler coverage - the coverage answer listing a virtual node's click under its id and its canvas owner
@task_file n=10 path=packages/escher-driver/src/execute.rs
what: the driver's verb execution - the canvas verb's part cut carrying the node indexes
@task_file n=11 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint - its ink box reader, now shared with the grouping
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the virtual node read checks - a replayed virtual click reaching its canvas listener, in coverage
@task_file n=13 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session test helpers - drawing circles through data-circles
@task_file n=14 path=tests/blitz-tests/tests/common/mod.rs
what: the stand's shared expectation tables - unexercised handlers after the drives, the circle spills, the circle node refusal causes
@task_file n=15 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand check of Circle Drawer's canvas reading
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the stand's canvas command table - sizes and digests of the canvas answers across hosts
@task_file n=17 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: the stand check of the canvas answer on every carrier - the glyph line's keys
@task_file n=18 path=docs/capabilities.md
what: the capability statement - canvas read as data, Circle Drawer's circle nodes, the canvas label region, sizes
@task_item n=1 part=touches
decision/circle-drawer-canvas-pixels-only is broken: Circle Drawer's drawing area is no longer one named canvas node with pixels alone - each circle reaches the tree, the snapshot and the graph as a virtual node by id and name.
@task_item n=2 part=touches
decision/ui-graph-one-node-per-snapshot-node and decision/readers-of-one-snapshot-model: the circles' virtual nodes are snapshot nodes, so the UI graph, actions, layout and exploration each carry one node per circle.
@task_item n=3 part=touches
decision/snapshot-reads-the-tree and decision/snapshot-reads-accessibility-tree: the snapshot now reads Circle Drawer's circles from the declared virtual nodes in the tree, role Image, named Circle {id}, {d} px.
@task_item n=4 part=touches
contract/node-id/reserved-separator: Circle Drawer's circles take the ids circle-canvas///c{id}, the canvas's id, the separator, then the app-given key.
@task_item n=5 part=touches
contract/handler-coverage/answer: the coverage line now carries a virtual node's id with its owner and the event click; its form is unchanged.
@task_item n=6 part=touches
decision/listeners-from-vdom-walk: a canvas click inside a virtual node is counted as the virtual node's click in coverage when a replayed click named it, the canvas's own listener under the canvas's id.
@task_item n=7 part=touches
contract/ui-graph-state/use-graph-state: Circle Drawer registers the graph state next_id, declared written by the canvas click.
@task_item n=8 part=touches
decision/ui-graph-carries-no-values: the UI graph gains Circle Drawer's registered state next_id; the graph reads circles by id, never by name.
@task_item n=9 part=touches
decision/interactive-nodes-named: each circle node carries a name, Circle {id}, {d} px, from app text.
@task_item n=10 part=touches
contract/driver-click-refusals/covered: a circle under another is not refused covered - a sibling virtual node is not read as covering.
@task_item n=11 part=touches
contract/actionable-nodes/listing and contract/actions-answer/fields: the actions answer lists a click on each circle node; a click on a circle's node adds a circle at its centre.
@task_item n=12 part=touches
contract/snapshot-diff/entries: a click on Circle Drawer's canvas adds the new circle's node in the diff; an undone circle reads stale, its redo keeps its id.
@task_item n=13 part=touches
decision/stand-lint-clean-at-boot: the lint holds no circle node, and a circle label wider than its circle reads no truncated finding.
@task_item n=14 part=touches
decision/stand-expectation-tables and decision/role-name-tables-stated-once: the shared stand tables gain Circle Drawer's circle node refusal causes, its spill table and three circle click handlers among the unexercised.
@task_item n=15 part=touches
decision/driver-schema-has-no-role-set: the canvas verb's text help gains the node tag clause; a virtual node's role type now comes from blitz-dom's re-export of the AccessKit role.
@task_item n=16 part=touches
decision/accesskit-accessibility-tree: blitz-dom re-exports AccessKit's role as VirtualNodeRole, the type a declared virtual node's role takes.
@task_item n=17 part=touches
decision/stand-checks-repeated-three-rounds: the new circle node stand check joins the repeated stand rounds by its name prefix.
@task_item n=18 part=touches
decision/capability-statement-reach-by-hand: the capability statement was edited by hand with the circle nodes, the node tag and the measured canvas sizes.
@task_item n=19 part=touches
absence/keyboard-drag-and-drop and the keyboard rows: the circle nodes are reached by tree and snapshot only, keyboard reach not built.
@task_fact n=1 slug=screen-reader-support
fact: Circle Drawer's canvas declares one virtual node per circle - id circle-canvas///c{id}, role Image, name Circle {id}, {d} px, bounds its cut region - the id app-given from 1 and stable across undo and redo, an undone circle reading stale.
@task_fact n=2 slug=screen-reader-support
fact: The canvas answer ties a draw command to a virtual node by its 1-based index alone - a closing node={k} or node=- on every line - and carries no virtual node's id or name.
@task_fact n=3 slug=screen-reader-support
fact: Handler coverage lists a virtual node's click under the virtual id, owned by its canvas, exercised where a replayed click named it; a canvas click inside a node counts for the canvas alone otherwise.
@task_fact n=4 slug=visual-design-verification
fact: A sibling virtual node does not cover another: a circle under another circle is not refused covered.
@task_fact n=5 slug=keyboard-navigation
fact: No keyboard route reaches a canvas's virtual nodes; Circle Drawer's circles are reached by click, tree and snapshot alone until the canvas keyboard path.
@task_fact n=6 slug=visual-design-verification
fact: The lint reads no virtual node: off-screen, clipped and zero-size checks on virtual nodes are not built, overlap skipped on either side.
@task_item n=1 part=untouched
Tab order, focus movement, focus fixup and the focusable rules - no focus behaviour changed.
@task_item n=2 part=untouched
Select lists, single and multiple selects, optgroups and their keys, rows and colours.
@task_item n=3 part=untouched
Range inputs, the Cells grid and the CRUD list keyboard contracts.
@task_item n=4 part=untouched
ARIA property reads, ARIA states, accessible name precedence and the HTML-AAM role mapping of elements.
@task_item n=5 part=untouched
Hidden and inert content exclusion, and hit-testing of inert elements.
@task_item n=6 part=untouched
Text contrast thresholds, the contrast finding's fields and the stand accent colour - no colour changed.
@task_item n=7 part=untouched
Password masking, the accessibility feature defaults, the platform adapter and assistive technology reach.
@task_item n=8 part=untouched
Cognitive accessibility rows - invalid state not by colour or class alone.
