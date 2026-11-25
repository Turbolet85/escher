@task_prose part=session
The session puts a canvas's virtual nodes (ids of the form {owner}///{key}) into every read but the lint and the canvas answer: the UI graph, the actions answer, the layout answer, handler coverage and exploration now hold them.
In the graph a virtual node is owned by its canvas, carries the canvas's component and has no handlers and no reads; in actions it is listed with owner the canvas, keyed false, and each verb's cause equal to what its call returns.
The verbs that take a virtual id move from one (click) to two (click and layout); type, scroll, screenshot with an id and canvas still refuse a virtual id not-found (5 to 4 such verbs).
The layout answer explains a virtual node as placed by the app (by app) in its canvas, at its offset from the canvas, with the canvas's shifts and size cause layout; the app placer is no longer reserved.
Coverage counts a click on a virtual node as its canvas's click listener under the canvas's id; the lint still holds no virtual node, now a stated limit carried on Circle Drawer's circles as virtual nodes.
No verb, cause, argument kind, node key, exit code, env var, log site or span field added; verbs 16 and causes 8 unchanged; answers carry the id and never the declared name.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph read in process, behind the escher-session graph answer — one graph node per snapshot node, its owner, component, handlers and reads
@task_file n=2 path=packages/dioxus-native-dom/src/actionable.rs
what: the actionable node list read in process, behind the escher-session actions answer — owner and keyed per listed node
@task_file n=3 path=packages/escher-driver/src/execute.rs
what: the escher-session command runner — resolving an id for each verb, the table of verbs that take a virtual id
@task_file n=4 path=packages/escher-driver/src/actions.rs
what: the escher-session actions answer — each listed node with its verbs and the cause each call returns
@task_file n=5 path=packages/escher-driver/src/layout.rs
what: the escher-session layout answer — why a node sits where it does, its placer, container, offset, shifts and size causes
@task_file n=6 path=packages/escher-driver/src/refusal.rs
what: the escher-session refusal causes and their fixed meanings — the not-found cause's text
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the escher-session verb schema and help answer — the help text of graph's nodes, actions' nodes and layout's placed
@task_file n=8 path=packages/escher-driver/src/coverage.rs
what: the escher-session coverage role — handler coverage, which listener a click counts and under which id
@task_file n=9 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixture pages — a virtual canvas page with a click listener on its canvas, and the declares-five check
@task_file n=10 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the new check that virtual nodes appear in graph, actions, layout, coverage, exploration and kept checks, and not in the lint
@task_file n=11 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check of virtual nodes in the snapshot, accessibility tree and diff, taking click and refused like elements
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check of a canvas declaring keyed, named, role-bearing virtual nodes
@task_file n=13 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check that each driver command leaves one log line holding nothing it handled — its virtual fixture's layout call now laid out, not refused
@task_file n=14 path=docs/capabilities.md
what: the capability statement — stated limits of virtual nodes, and what each answer may carry (ids, owner ids, closed words, never a value or declared name)
@task_item n=1 part=touches
contract/escher-session-graph/answer: the graph answer now holds each virtual node, owned by its canvas, with the canvas's component and no handlers or reads; any statement that a virtual node is excepted from the graph is stale.
@task_item n=2 part=touches
contract/escher-session-actions/answer: the actions answer now lists each virtual node with owner its canvas, keyed false, the six verbs and each cause equal to its call's; a statement that a virtual node is never listed is stale.
@task_item n=3 part=touches
contract/escher-session-layout/answer: layout now takes a virtual id and answers placed by app in the canvas (container the canvas's id), at the region's offset from the canvas, the canvas's shifts, size cause layout; the app placer is no longer reserved.
@task_item n=4 part=touches
contract/escher-session-coverage/answer: a click on a virtual node counts its canvas's click listener under the canvas's id; no coverage line holds a virtual id; a virtual node carries no handler of its own.
@task_item n=5 part=touches
contract/escher-session-help/answer: the help text of graph's nodes, actions' nodes and layout's placed restated to include virtual nodes; no verb, argument or order moved.
@task_item n=6 part=touches
The not-found refusal cause's fixed meaning now reads that only click and layout take a virtual id (was click alone); any row stating click is the only verb taking a virtual id, or that five id verbs read it not-found (now four), is stale.
@task_item n=7 part=touches
contract/escher-session-lint/answer: the lint still holds no virtual node — no overlap, clip or off-screen finding on a region — now a stated limit carried on Circle Drawer's circles as virtual nodes; a row beside it may need that limit.
@task_item n=8 part=touches
contract/escher-session-mcp/canvas-tool: the canvas answer still holds no virtual node and canvas still reads a virtual id not-found — a decided limit beside it.
@task_item n=9 part=touches
decision/no-caller-input-echoed: the graph, actions and layout answers now carry a virtual node's id and never its declared name — the rule holds and extends to these answers.
@task_item n=10 part=touches
decision/session-recording-replayable-check: a kept check and an explored path's check of the new reads (graph, actions, layout on virtual nodes) replay green; exploration acts on a virtual node as on every listed node.
@task_item n=11 part=touches
decision/circle-drawer-circles-painted-on-canvas and contract/stand-element-ids/circle-canvas: indirectly, the CARRYs for coverage's and the lint's limits on virtual nodes (and a windows unused-import warning) sit on Circle Drawer's circles as virtual nodes, whose canvas carries onclick.
@task_item n=12 part=touches
contract/escher-session-snapshot/answer: the snapshot's virtual nodes keep clickable false and keyed false (keyed false since a virtual id holds a slash) — decided limits, unchanged, now read by graph and actions too.
@task_fact n=1 slug=surface-cli
fact: Only click and layout take a virtual node id; type, scroll, screenshot with an id and canvas refuse it not-found — a decided limit, not owed.
@task_fact n=2 slug=surface-cli
fact: The layout answer for a virtual node reads placed by app, container its canvas's id, at the node's bounds minus the canvas's, the canvas's shifts, both sizes caused by layout; the engine is never asked to explain a region.
@task_fact n=3 slug=surface-cli
fact: In the graph and actions answers a virtual node is owned by its canvas, carries no handlers or reads, reads keyed false, and its verbs' causes equal what each call returns.
@task_fact n=4 slug=surface-cli
fact: Handler coverage counts a click on a virtual node as its canvas's click listener under the canvas's id; a virtual node carries no handler of its own.
@task_fact n=5 slug=surface-cli
fact: The lint and the canvas answer hold no virtual node — no overlap, clip or off-screen finding on a declared region (a stated limit, carried on Circle Drawer's circles as virtual nodes).
@task_item n=1 part=untouched
contract/stand-element-ids rows other than circle-canvas: no stand id was added, renamed or freed; the new fixture page is a test page, not the stand.
@task_item n=2 part=untouched
contract/shell-provider-window-chrome rows: window chrome controls are not reached.
@task_item n=3 part=untouched
agent-run, cold-agent, multi_doc and paint_bench command-line contracts and the cold-agent decisions: no script, argument, exit or output moved.
@task_item n=4 part=untouched
contract/escher-session-exit-codes rows and the escher-session command form: no exit code, session error or command form added.
@task_item n=5 part=untouched
MCP server protocol revisions, stdio, tools list, result bound, screenshot result and end: no tool or protocol change; the verb count stays 16 and causes 8.
@task_item n=6 part=untouched
Served tasks, recording check-file and record-dir rows, and the host log service.name: no task served, recording form or log field changed.
@task_item n=7 part=untouched
Desktop-native stand decisions (boot, viewport, accent colour, document skeleton, scroll into view, clipping region, a11y by attributes, lints clean at boot): no stand layout or boot path reached.
@task_item n=8 part=untouched
absence/arbitrary-index-html-templates: no document template changed.
