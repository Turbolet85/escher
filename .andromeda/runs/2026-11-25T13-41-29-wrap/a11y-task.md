@task_prose part=session
The session put a canvas's virtual nodes into every read but the lint and the canvas answer: the UI graph, the actions answer, handler coverage, layout explained and exploration now each hold a virtual node by its id.
In the UI graph a virtual node is one graph node of its own, owned by its canvas, carrying its canvas's component and no handlers or reads.
The actions answer lists each virtual node, owner the canvas, keyed false, six verbs, each cause equal to the cause its real call returns.
The verbs that take a virtual id moved from one (click) to two (click and layout); layout explains a virtual node as placed by the app in its canvas, at its offset from the canvas, and changes nothing.
The id verbs reading a virtual id not-found moved from five to four (type, scroll, screenshot with an id, canvas); the reads holding no virtual node moved from graph, actions, lint, canvas answer and exploration to the lint and the canvas answer alone.
Retired: the stated limit that only click takes a virtual id, the limit that the UI graph, actions, the lint and exploration hold no virtual node, and the reserved status of the app placer; verbs 16, causes 8 and the a11y leg's 20 targets are unchanged.
@task_file n=1 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the new integration check that a canvas's virtual nodes stand in the UI graph, the actions answer, layout, handler coverage, exploration and a kept check, carry the id and never the declared name, and stay out of the lint
@task_file n=2 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph built from the snapshot, one graph node per snapshot node with its owner, component, handlers and reads
@task_file n=3 path=packages/dioxus-native-dom/src/actionable.rs
what: the actionable nodes listing and its keyed reading, under the one actionable rule
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's run path and id resolution, which decides which verbs take a virtual id and refuses the rest not-found
@task_file n=5 path=packages/escher-driver/src/actions.rs
what: the actions verb and its answer, each listed node's owner, keyed and per-verb cause
@task_file n=6 path=packages/escher-driver/src/layout.rs
what: the layout verb that explains a node's placement, its placer, bounds, shifts and size causes
@task_file n=7 path=packages/escher-driver/src/refusal.rs
what: the driver refusal causes and their fixed meanings, not-found among them
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the driver schema's verb table and the help text of its arguments and answers
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: handler coverage, which listeners a run exercised and under which owner id
@task_file n=10 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixture pages, among them the virtual canvas pages and the five declared regions check
@task_file n=11 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check that a canvas's virtual nodes stand in the accessibility tree, snapshot and diff and take click like elements
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check that a canvas declares keyed, named, role-bearing virtual regions, read and validated in process
@task_file n=13 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check that each driver command leaves one log line holding nothing it handled, its virtual fixture's layout call now laid out rather than refused
@task_file n=14 path=docs/capabilities.md
what: the capability statement and its stated limits, here which verbs take a virtual node and what the actions and layout answers carry
@task_item n=1 part=touches
decision/ui-graph-one-node-per-snapshot-node: its stated exception that the graph and exploration hold no virtual node is now stale; every snapshot node, a virtual node included, is one graph node, owned by its canvas.
@task_item n=2 part=touches
contract/node-id/reserved-separator: a node whose id holds the separator is now read by the id alone in the UI graph, the actions answer and layout, its owner the text before the separator.
@task_item n=3 part=touches
decision/keyed-wire-form: a virtual node's keyed reads false on the actionable listing and the actions answer, a decided limit that stands and must be stated.
@task_item n=4 part=touches
contract/actionable-nodes/listing: virtual nodes are now listed, through the existing owner arm, with keyed false.
@task_item n=5 part=touches
contract/actions-answer/causes and contract/actions-answer/fields: the actions answer now lists each virtual node with owner the canvas and keyed false; click and layout are decided by the virtual-id resolution, the other verbs read the cause their real call returns (not-found).
@task_item n=6 part=touches
decision/actions-verb-changes-nothing: indirect, through the actions verb now resolving a virtual node's target through its canvas; it still changes nothing.
@task_item n=7 part=touches
contract/handler-coverage/answer: a virtual node carries no handler; a click on one counts its canvas's listener under the canvas's id, and no coverage line holds the separator.
@task_item n=8 part=touches
contract/snapshot-node/clickable, contract/snapshot/clickable and contract/snapshot-text/clickable: a virtual node's clickable still reads false, now a decided limit beside click taking its id.
@task_item n=9 part=touches
decision/circle-drawer-canvas-pixels-only: indirect, the drawing-area canvas's regions now read in the graph, actions, layout, coverage and exploration; the lint and the canvas answer still hold none.
@task_item n=10 part=touches
decision/readers-of-one-snapshot-model, decision/snapshot-model-single-source and decision/one-actionable-rule: indirect, the graph, actions and layout readers now read virtual nodes from the same snapshot model.
@task_item n=11 part=touches
decision/ui-graph-carries-no-values: a virtual graph node carries its id, owner and component and no declared name, value, handlers or reads.
@task_item n=12 part=touches
contract/visual-lint-finding/content: the lint still reads no virtual node, so no overlap, clip or off-screen finding on a region; now the only read but the canvas answer that holds none.
@task_item n=13 part=touches
contract/driver-refusal/disabled and the refusal causes: the not-found meaning now says only click and layout take a virtual id; the count of id verbs reading a virtual id not-found moved 5 to 4.
@task_item n=14 part=touches
decision/listeners-from-vdom-walk: indirect, through handler coverage counting a virtual click under its canvas's listener read from the vdom walk.
@task_fact n=1 slug=screen-reader-support
fact: Only click and layout take a virtual node's id; type, scroll, screenshot with an id and canvas read a virtual id not-found, a decided limit not owed.
@task_fact n=2 slug=screen-reader-support
fact: The UI graph, the actions answer, layout, handler coverage and exploration each hold a canvas's virtual nodes by id; the lint and the canvas answer alone hold none.
@task_fact n=3 slug=screen-reader-support
fact: Layout explains a virtual node as placed by the app in its canvas, at its bounds' offset from the canvas, with the canvas's shifts and size cause layout, the engine never asked for a region's reason.
@task_fact n=4 slug=screen-reader-support
fact: The graph, actions and layout answers carry a virtual node's owner///key id and never its declared name.
@task_fact n=5 slug=screen-reader-support
fact: A virtual node carries no handler of its own; a click on it is counted against its canvas's listener under the canvas's id, a stated limit carried on Circle Drawer's circles as virtual nodes.
@task_fact n=6 slug=visual-design-verification
fact: The lint reads no virtual node, so no overlap, clip or off-screen finding is raised on a canvas region; a stated limit carried on Circle Drawer's circles as virtual nodes.
@task_item n=1 part=untouched
The ARIA property reads, accessibility tree states and role mapping rows: the session changed no ARIA read, role or state on the tree.
@task_item n=2 part=untouched
Keyboard navigation rows, the select, list box, range input and Cells grid key contracts, and focus rules: no key handling or focus rule moved.
@task_item n=3 part=untouched
Contrast rows, the lint contrast finding fields and the stand's contrast and accent decisions: no colour or contrast reading moved.
@task_item n=4 part=untouched
The accessibility feature, AccessKit adapter and platform adapter decisions: no feature flag, adapter or dependency changed.
@task_item n=5 part=untouched
The a11y CI leg and the CI gate rows: the leg's 20 targets and the stand-repeat rounds are unchanged, no harness or gate surface moved.
@task_item n=6 part=untouched
The accessible name, password masking and snapshot value and select rows: no name, value or state reading of an element changed.
@task_item n=7 part=untouched
The UI graph state registration contracts: no change to states, writers or readers in the graph.
