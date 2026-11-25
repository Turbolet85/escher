@task_prose part=session
A canvas's declared virtual nodes now act as elements: each taken virtual node joins the accessibility tree as a child of its canvas's node, with its declared role, name, an author id of the form owner///key, and disabled where declared; a hidden, collapsed or inert canvas contributes none.
The snapshot, and so the diff, the MCP results and the kept checks, now carry each virtual node's owner///key id and declared name, with enabled from the declared state, clickable false, no children, and bounds equal to its region moved by its canvas's content-box origin.
The click verb now takes a virtual id and acts on its region at its canvas, refused in the one order (stale or not-found, disabled, off-screen, covered); type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
The graph, the actions answer, the lint answer, the canvas answer and exploration hold no virtual node; the declaration gains a disabled flag; no verb, argument, key, result field, cause, exit or log field was added, and no log line carries a virtual id or name.
Measured: the a11y CI leg runs twenty files (was nineteen), reading 20 · 199 · 0 · 0; the act-span test's child call count moved 43 to 46; CI green 17/17 on linux, macos and windows.
No served stand app declares a virtual node yet, so the socket host, command line and cold-agent run carry the id and name by construction but are not read on the binary.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget boundary a canvas implements, holding the virtual-node declaration (key, name, role, region, now disabled)
@task_file n=2 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the in-process virtual-node read of a canvas, now also its page bounds and its accessibility tree id
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document bridge, whose accessibility tree now adds each canvas's virtual nodes under the canvas
@task_file n=4 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot reader behind the snapshot answer and its diff, now reading virtual nodes as nodes
@task_file n=5 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph builder behind the graph and actions answers, which now skips every virtual node
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id scheme and its reserved owner///key virtual-node form, documented as now served
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the session verb executor, where click resolves a virtual id at its canvas and every other id verb reads it not-found
@task_file n=8 path=packages/escher-driver/src/lint.rs
what: the lint check behind the lint answer, its stated limits now saying a canvas's virtual nodes are not linted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the verb schema and help text, with click's help and the clickable node key's help reworded
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the test pinning every verb's help, arguments and result, holding click's help
@task_file n=11 path=packages/escher-driver/src/refusal.rs
what: the refusal causes and their meanings, with not-found and covered reworded for virtual nodes
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the new integration test of virtual nodes in the tree, snapshot, click, refusals, MCP and kept checks
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration test of a canvas's declared virtual nodes and that a declaring canvas puts nothing on the graph
@task_file n=14 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the act-span log test that every act's span holds no caller or declared text, now 46 child calls
@task_file n=15 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixtures, gaining a declaring virtual-canvas page with five virtual ids and a hide button
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the CI leg runner, whose a11y leg now runs twenty files
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the CI workflow test holding the a11y leg's file list
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the capability statement test pinning the leg counts in words and as an integer
@task_file n=19 path=docs/capabilities.md
what: the capability statement, its Not built list and Stated limits, now saying virtual nodes are produced
@task_item n=1 part=touches
The escher-session snapshot answer now carries virtual nodes: an owner///key id, the declared role and name, enabled from the declared state, clickable false, highlighted false, no children, bounds as the region moved by the canvas's content-box origin.
@task_item n=2 part=touches
The escher-session graph answer and the actions answer now hold an exception: the graph skips every node whose id holds the virtual separator, so neither ever lists a virtual node.
@task_item n=3 part=touches
The escher-session lint answer holds no virtual node, by construction (a virtual entry has no element and so no visible part); its stated limits name that.
@task_item n=4 part=touches
The escher-session layout answer and the MCP canvas tool now read a virtual id not-found, as do type, scroll and screenshot with an id.
@task_item n=5 part=touches
The escher-session help answer and the MCP tools list change text: click's help and the clickable key's help are reworded; no verb, argument or key added.
@task_item n=6 part=touches
The refusal set's meanings move: not-found now covers a virtual id on a verb other than click, and covered on a virtual node means an element over its region's centre; the off-screen refusal now applies to a virtual node too.
@task_item n=7 part=touches
The session recording as a replayable check now records and replays a click on a virtual id, carrying the owner///key id and the declared name.
@task_item n=8 part=touches
Caller input never written back: declared virtual-node names, author text not caller input, now cross the snapshot, diff, MCP results and kept checks.
@task_item n=9 part=touches
Stand names, roles and states by attributes alone: the accessibility tree now also takes role, name and disabled from a canvas's virtual-node declarations, not only from element attributes.
@task_item n=10 part=touches
Stand ids as attributes alone and the stable element id scheme: the reserved owner///key virtual-node form is now served in the tree, snapshot and diff.
@task_item n=11 part=touches
Circle Drawer circles painted on a canvas: the circle canvas still declares no virtual node; the first served app to declare one owes the binary-level reading of the virtual id and name.
@task_item n=12 part=touches
Stand lints clean at boot: unchanged in result, as a declaring canvas adds no lint finding and no virtual id to the lint answer.
@task_item n=13 part=touches
Visible region through clipping boxes: a virtual node's click is checked through the same off-screen and covered targeting at its region, but it has no visible region of its own in the lint.
@task_item n=14 part=touches
The MCP server result and the command line's stdout now carry a virtual node's id and name through the same session answer line; the MCP role is read, the socket and command line only by construction.
@task_fact n=1 slug=surface-cli
fact: Only click takes a virtual node id; it acts on the node's region at its canvas and is refused in the one order stale or not-found, disabled, off-screen, covered; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
@task_fact n=2 slug=surface-cli
fact: The snapshot, its diff, MCP results and kept checks carry each virtual node with its owner///key id and declared name, clickable false and bounds as its region moved by its canvas's content-box origin; the graph, actions, lint, canvas answer and exploration hold none.
@task_fact n=3 slug=surface-desktop-native
fact: A canvas's taken virtual nodes sit in the accessibility tree as children of the canvas's node after its own, with declared role, name, author id owner///key and disabled where declared; a hidden, collapsed or inert canvas contributes none, and no bounds or focus are written on them.
@task_fact n=4 slug=surface-desktop-native
fact: No served stand app declares a virtual node yet; the virtual id and name on the socket host, command line and cold-agent run are carried by construction and not yet read on the binary.
@task_item n=1 part=untouched
The ShellProvider window chrome contracts: no window decoration, maximize, minimize, drag or close behaviour moved.
@task_item n=2 part=untouched
The agent-run and cold-agent run pipe contracts (usage, exits, stdout, record dir, Windows pass-through) and the cold-agent prompt and build decisions.
@task_item n=3 part=untouched
The multi_doc and paint_bench command lines and their arguments, environment variables and stdout.
@task_item n=4 part=untouched
The escher-session exit codes, host log service name, coverage role, MCP protocol revisions, stdio transport, result bound and end.
@task_item n=5 part=untouched
The stand stable element ids of every stand control and task card, and the served task list: no stand id was added, renamed or freed.
@task_item n=6 part=untouched
Stand boot and layout decisions: the fixed document skeleton, pinned viewport, held instance as fresh boot, shared session boot, accent colour, load page network, order page, flight type select, CRUD keyboard region, open list rows and into-view scroll.
@task_item n=7 part=untouched
The mobile-native and web-spa surfaces and the arbitrary index.html templates absence.
