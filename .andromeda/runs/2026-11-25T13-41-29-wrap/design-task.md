@task_prose part=session
The session brought a canvas's declared virtual nodes into every read: the UI graph, the actions answer, the layout answer, handler coverage and exploration now hold each virtual node, by its owner///key id and never its declared name.
In the UI graph a virtual node is one node owned by its canvas, with the canvas's component and no handlers or reads; the actions answer lists it with owner its canvas, keyed false, and for each verb the cause its call returns.
layout now takes a virtual id beside click: it answers the region placed by app in its canvas, at the region's offset from the canvas, the canvas's shifts, size cause layout, and never asks the engine for a reason.
type, scroll, screenshot with an id and canvas still read a virtual id not-found (5 verbs to 4); the lint and the canvas answer remain the only reads holding no virtual node; a virtual click counts its canvas's listener under the canvas's id in coverage.
Retired: the limit that only click takes a virtual id, the limit that the graph, actions and exploration hold no virtual node, and the reserved status of the app placer; no verb, cause, argument, exit status, answer shape or session error added.
Measured: a fixture node's page bounds read 8 px past the forecast, the body's default margin; verbs 16, causes 8 unchanged.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph read in process (the graph the graph answer and the UI graph line print), its nodes, owners, handlers and reads — edited
@task_file n=2 path=packages/dioxus-native-dom/src/actionable.rs
what: the actionable node list the actions answer reads, each node's owner and keyed flag — edited (docs only)
@task_file n=3 path=packages/escher-driver/src/execute.rs
what: the driver's command runner, id resolution of the verbs (click, layout, type, scroll, screenshot, canvas) and which verbs take a virtual id — edited
@task_file n=4 path=packages/escher-driver/src/actions.rs
what: the actions answer, each listed node's verbs and the cause each verb's call returns — edited
@task_file n=5 path=packages/escher-driver/src/layout.rs
what: the layout answer, how a node is placed (by its placer, container, at, relative) and why it is sized — edited
@task_file n=6 path=packages/escher-driver/src/refusal.rs
what: the refusal causes and their fixed meanings, not-found's text — edited
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the verb schema, the help text of each command's answer fields (graph nodes, actions nodes, layout placed) — edited
@task_file n=8 path=packages/escher-driver/src/coverage.rs
what: handler coverage, the coverage answer and line, which listener a click counts — edited
@task_file n=9 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the check that virtual nodes appear in the graph, actions, layout, coverage and exploration reads, and that the other id verbs refuse them — added
@task_file n=10 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the test fixture pages, the virtual canvas page with a listened canvas — edited
@task_file n=11 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check of virtual nodes in the accessibility tree, snapshot and diff and taking click — edited
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check that a canvas declares keyed, named regions — edited
@task_file n=13 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the check that each driver command leaves one log line holding nothing it handled — edited (a virtual layout now laid, not refused)
@task_file n=14 path=docs/capabilities.md
what: the capability statement, its stated limits of virtual nodes and what each answer carries — edited
@task_item n=1 part=touches
contract/actions-line/nodes says a virtual node is never listed; it is now listed, owner its canvas, keyed false, with six verbs and each cause its call returns — stale.
@task_item n=2 part=touches
contract/escher-session-commands/actions-answer and contract/actions-line/content: the actions answer now carries virtual node ids (owner///key) and their canvas owner, never a declared name.
@task_item n=3 part=touches
contract/escher-session-commands/graph-answer and contract/ui-graph-line/content: the graph now holds each virtual node, owned by its canvas, the canvas's component, no handlers or reads.
@task_item n=4 part=touches
contract/escher-session-commands/layout-answer: layout on a virtual id answers placed by app with canvas its canvas's id, at the region's offset, the canvas's shifts, size cause layout; the app placer is no longer reserved.
@task_item n=5 part=touches
contract/escher-session-commands/layout-arguments: the layout id argument now takes a virtual id; stale where it says only element ids.
@task_item n=6 part=touches
contract/escher-session-commands/coverage-answer and contract/coverage-line/content: a virtual node carries no handler; a click on one counts its canvas's listener under the canvas's id; no line holds ///.
@task_item n=7 part=touches
decision/actions-share-the-verbs-checks: a virtual node's verbs are decided through its canvas's target; a verb taking a virtual id is decided by its call's resolution, the rest by the same checks as before.
@task_item n=8 part=touches
contract/escher-session-commands/help: the help text of graph's nodes, actions' nodes and layout's placed fields was restated.
@task_item n=9 part=touches
contract/driver-verbs-and-virtual-clock/click and contract/driver-verbs-and-virtual-clock/canvas: where either says click alone takes a virtual id, it is now click and layout; canvas still reads a virtual id not-found.
@task_item n=10 part=touches
contract/escher-session-commands/screenshot: screenshot with an id still reads a virtual id not-found, a decided limit, now one of four such verbs (was five).
@task_item n=11 part=touches
contract/escher-session-commands/graph-arguments and contract/escher-session-commands/actions-arguments: no argument changed, but where a row says the answers they select hold no virtual node it is stale.
@task_item n=12 part=touches
contract/escher-session-stdout/command-line-line and decision/driver-output-for-agents: the command line output of graph, actions and layout now carries virtual ids, closed words and numbers, never page text or a declared name.
@task_fact n=1 slug=surface
fact: Only click and layout take a virtual node's id; type, scroll, screenshot with an id and canvas read it not-found, a decided limit.
@task_fact n=2 slug=surface
fact: The layout answer of a virtual node is placed by app in its canvas at the region's offset from the canvas's border box, its sizes caused by layout, never an engine reason.
@task_fact n=3 slug=surface
fact: A virtual node carries no handler of its own; coverage counts a click on it as its canvas's listener under the canvas's id (a CARRY on Circle Drawer's circles as virtual nodes).
@task_fact n=4 slug=surface
fact: The lint and the canvas answer hold no virtual node; the lint makes no overlap, clip or off-screen finding on a region (a CARRY on Circle Drawer's circles as virtual nodes).
@task_fact n=5 slug=surface
fact: The graph, actions and layout answers carry a virtual node by its owner///key id and never its declared name.
@task_item n=1 part=untouched
Colour, typography, spacing, depth, radius and iconography rows: no colour, font, spacing, paint order, hit-test or cursor rule changed.
@task_item n=2 part=untouched
Motion rows and the virtual clock decisions: no verb moves the clock differently; settle, scroll and animation unchanged.
@task_item n=3 part=untouched
Exit status and exit code rows, the session error remedy and the stderr usage and session error rows: no exit, error or remedy added or moved.
@task_item n=4 part=untouched
The mcp role rows (stdout, tool results, err ending): the mcp results carry the same answers, no shape changed.
@task_item n=5 part=untouched
The select list, shell provider, JS window, dioxus-native configuration, MountedData and WPT runner rows: no select, window, config or runner behaviour touched.
@task_item n=6 part=untouched
The escher-session tasks rows (counter, crud, timer, cells and the rest) and the task order: no task added or reordered.
@task_item n=7 part=untouched
The snapshot and accessibility-tree rows: the snapshot's reading of virtual nodes, clickable false, is the prior session's and unchanged.
