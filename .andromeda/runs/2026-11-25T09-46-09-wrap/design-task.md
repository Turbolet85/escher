@task_prose part=session
The session made a canvas's declared virtual nodes behave as elements: each taken virtual node now stands in the accessibility tree under its canvas, and from it in the snapshot, its diff, the MCP results and kept checks, carrying its {owner}///{key} id and its author-declared name.
The click verb now takes a virtual id and acts on its region in its canvas, refused in the one order (disabled, off-screen, covered, not-found, stale); type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
The UI graph, the actions answer, the lint and exploration hold no virtual node; the lint holds none because a virtual entry has no element, so the planned lint skip was dropped as inert and the research premise that the lint would read overlap on a virtual id was measured false.
A virtual node declaration gains a disabled state; the click help, the clickable key's help and the meanings of the covered and not-found causes were rewritten; no verb, argument, key, result field, cause, exit, log field or span was added.
The a11y CI leg's file count moved nineteen to twenty (measured 20 · 199 · 0 · 0), the act-spans guard's child call count moved 43 to 46, and the capability statement drops the unbuilt virtual-node id producer and gains five stated limits.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited — the custom widget boundary where a canvas declares its virtual nodes (the virtual node declaration: key, name, role, region), now with a disabled state
@task_file n=2 path=packages/dioxus-native-dom/src/virtual_node.rs
what: edited — the in-process read of a canvas's declared virtual nodes, now also giving a region's page bounds and the virtual node's accessibility tree id
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited — the dioxus document, whose accessibility tree now adds each canvas's virtual nodes as children of the canvas with declared role, name, author id and disabled state
@task_file n=4 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot builder behind the snapshot verb, its diff and the MCP snapshot result, now reading virtual nodes with id, role, name, enabled, clickable false and bounds
@task_file n=5 path=packages/dioxus-native-dom/src/graph.rs
what: edited — the UI graph builder, which now skips every snapshot node holding the virtual separator, so the graph line and the actions line hold no virtual node
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
what: edited — the stable element id form and its reserved virtual-node form, its docs now saying the form is served in the accessibility tree, snapshot and diff
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited — the driver's verb execution and id resolution: click now locates a virtual id at its canvas, every other id verb reads it not-found
@task_file n=8 path=packages/escher-driver/src/lint.rs
what: edited — the canvas lint, its stated limits gaining that a canvas's virtual nodes are not linted (doc only)
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited — the driver schema of verbs, arguments, keys and help texts; the click help and the clickable key's help rewritten
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: edited — the schema check pinning each verb's help, its arguments and a result, moved with the click help
@task_file n=11 path=packages/escher-driver/src/refusal.rs
what: edited — the refusal causes and their meanings and remedies; the covered and not-found meanings rewritten, remedies unchanged
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: added — the integration check of virtual nodes in the tree, snapshot, click and refusals, other id verbs, graph, actions, lint, MCP and kept checks
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited — the integration check of declared virtual nodes read and validated in process, moved for the disabled field
@task_file n=14 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the guard that act spans carry no id or name at info, its child call count 43 to 46
@task_file n=15 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited — the test fixture pages, gaining a canvas page declaring five virtual nodes with a hide toggle
@task_file n=16 path=.github/scripts/ci-leg.sh
what: edited — the CI legs, the a11y leg now running twenty files
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: edited — the check of the CI workflows and each leg's file list
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: edited — the check that the capability statement states the leg counts in words, pinned at twenty
@task_file n=19 path=docs/capabilities.md
what: edited — the capability statement: what is built, what is not built and the stated limits
@task_item n=1 part=touches
The snapshot verb and the MCP snapshot tool result now carry a canvas's virtual nodes as nodes with id, role, name, enabled, clickable false and bounds; a row on the snapshot answer's content or nodes may be stale.
@task_item n=2 part=touches
The click verb's contract (under the virtual clock and in the commands) gains a virtual id as target, acting on its region in its canvas, settled, with the canvas focused.
@task_item n=3 part=touches
The canvas verb, layout, screenshot with an id, type and scroll answer not-found for a virtual id; rows on those verbs' answers or refusals may need it beside them.
@task_item n=4 part=touches
The UI graph line and the actions line hold no virtual node by an explicit exception; the rule that actions are decided by the verbs' own checks stands, but the graph now excepts virtual nodes.
@task_item n=5 part=touches
The escher-session help (the click help and the clickable key's help) was rewritten; a row quoting either help text is stale.
@task_item n=6 part=touches
The covered and not-found refusal meanings were rewritten; a virtual node reads covered only by an element over its region's centre — a hit-test rule beside the hit-testing and paint-clip decisions.
@task_item n=7 part=touches
The command line output and the MCP role output read by an agent now carry author-declared names and {owner}///{key} ids; the agent-read surface rows gain a new kind of content.
@task_item n=8 part=touches
A virtual node's snapshot bounds are its declared region moved by its canvas's content-box origin; rows on snapshot bounds (the select list row bounds kind) stand beside it.
@task_item n=9 part=touches
The snapshot and accessibility tree of a select rows rest on the snapshot builder and the tree, both changed; they stay true but share the component.
@task_item n=10 part=touches
The canvas line form conventions and the canvas lint: the canvas answer and the lint hold no virtual node, the lint's stated limits gain that virtual nodes are not linted.
@task_item n=11 part=touches
The reserved virtual-node id form is now served (tree, snapshot, diff, MCP, kept checks); any row saying no served id holds the separator, or that nothing produces the form, is stale.
@task_item n=12 part=touches
The a11y CI leg's file count moved nineteen to twenty and the act-spans guard's child call count 43 to 46; a row holding either count must move.
@task_fact n=1 slug=surface
fact: A canvas's declared virtual nodes stand in the accessibility tree as children of the canvas's tree node after its own, with the declared role, the declared name as label, author id {owner}///{key} and disabled where declared; a hidden, collapsed or inert canvas, or one whose read errs, gives none, and the tree writes no bounds or focus for them.
@task_fact n=2 slug=surface
fact: The snapshot and its diff read a virtual node with its id, role, declared name, enabled from its declared state, clickable false, highlighted false, no checked, value, selected or pressed, no children, and bounds its region moved by its canvas's content-box origin.
@task_fact n=3 slug=surface
fact: Only click takes a virtual id: it acts on the node's region in its canvas, refused in the one order disabled, off-screen, covered, not-found, stale; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
@task_fact n=4 slug=surface
fact: A virtual node reads covered only by an element over its region's centre.
@task_fact n=5 slug=surface
fact: The UI graph, the actions answer, the lint and exploration hold no virtual node; the lint holds none because a virtual entry has no element, not by a skip.
@task_fact n=6 slug=surface
fact: A virtual node's id and author-declared name cross to the snapshot text, its diff, the MCP results and kept checks, and a kept check holding a virtual click replays.
@task_fact n=7 slug=surface
fact: A virtual node declaration carries a disabled state, read by the tree as disabled and by the snapshot as enabled false.
@task_item n=1 part=untouched
Colour palette, engine colours and the colour scheme: no colour, token or recascade changed.
@task_item n=2 part=untouched
Typography and fonts: no font, family, locale or type-scale rule moved.
@task_item n=3 part=untouched
Spacing, border radius and presentational hints: no spacing, margin or radius rule moved.
@task_item n=4 part=untouched
Motion: scroll animation, scrollbar fade, select list motion and the advance verb are unchanged.
@task_item n=5 part=untouched
Iconography: cursors and favicon are unchanged.
@task_item n=6 part=untouched
The open select list's events, painting and placement, and the screenshot of its rows: not reached.
@task_item n=7 part=untouched
WPT runner output and its result colours, the shell provider and window controls, the JS window object, and dioxus-native configuration: not reached.
@task_item n=8 part=untouched
The escher-session tasks and the coverage line: no task, coverage argument or answer moved.
