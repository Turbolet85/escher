@task_prose part=session
The session serves a canvas's declared virtual nodes as elements: each taken virtual node becomes a child of its canvas's accessibility tree node, after the canvas's own children, with its declared role, its declared name as label, author id {owner}///{key} and disabled where declared.
The snapshot reads each virtual node (id, role, name, enabled from the declared disabled state, clickable false, bounds its region moved by the canvas's content-box origin, no children), so the snapshot text, its diff, MCP results and kept checks carry the virtual id and declared name.
The driver's click takes a virtual id and acts at its region on its canvas, refused in the one order (disabled, off-screen, covered, not-found, stale); type, scroll, screenshot with an id, layout and canvas read a virtual id not-found; the UI graph, the actions answer, exploration and the lint hold no virtual node.
A virtual node declaration gains a disabled flag; a hidden, collapsed or inert canvas takes no virtual node; no log site, span or field was added.
The a11y CI leg runs twenty files (was nineteen), measured 20 · 199 · 0 · 0; a new integration target holds these readings in both layout modes; the research premise that the lint would report overlaps naming a virtual id was disproved, and the planned lint skip was dropped as inert.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited — the custom widget's virtual node declaration (key, name, role, region), now with a declared disabled flag
@task_file n=2 path=packages/dioxus-native-dom/src/virtual_node.rs
what: edited — the in-process declared virtual node read and validation, now with a virtual node's page bounds and its accessibility tree id
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited — the Dioxus document's accessibility tree build, which now adds each canvas's virtual nodes under the canvas node
@task_file n=4 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot model read from the accessibility tree (ids, roles, names, enabled, clickable, bounds), now reading virtual nodes
@task_file n=5 path=packages/dioxus-native-dom/src/graph.rs
what: edited — the UI graph built from the snapshot, now skipping every virtual node
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
what: edited — the stable element id and the reserved virtual node id form, its doc now saying the form is served
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited — the driver's verb execution and id resolution: click on a virtual id located at its canvas; other id verbs read a virtual id not-found
@task_file n=8 path=packages/escher-driver/src/lint.rs
what: edited — the visual lint, its stated limits doc gaining that a canvas's virtual nodes are not linted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited — the driver schema's verb help and node key help (click and clickable texts)
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: edited — the driver schema's unit check pinning each verb's help
@task_file n=11 path=packages/escher-driver/src/refusal.rs
what: edited — the driver refusal causes and their meanings (not-found and covered), with their unit check
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: added — the integration check of virtual nodes in the accessibility tree, snapshot, click, refusals, other id verbs, graph, actions, lint, MCP and kept checks
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited — the integration check of declared virtual nodes and that a declaring canvas puts nothing on the graph
@task_file n=14 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the check that driver command spans and log lines carry no node id or name, now over a virtual click, snapshot and refused layout
@task_file n=15 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited — the shared test fixtures, gaining a page with a canvas declaring five virtual nodes
@task_file n=16 path=.github/scripts/ci-leg.sh
what: edited — the CI leg runner, its a11y leg's list of test files
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: edited — the check of the CI workflows' legs and their test file lists
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: edited — the check that the capability statement states each leg's file count
@task_file n=19 path=docs/capabilities.md
what: edited — the capability statement: what is built, not built and its stated limits, and the a11y leg's file count
@task_item n=1 part=touches
contract/node-id/reserved-separator is stale: a served id now holds the separator /// — the accessibility tree's author id, the snapshot, its diff, MCP results and kept checks carry {owner}///{key}; only the graph, actions, lint, canvas answer and exploration hold none.
@task_item n=2 part=touches
contract/accessibility-tree/author-id: a virtual node's tree node carries author id {owner}///{key}, beside the DOM elements' stable ids.
@task_item n=3 part=touches
decision/snapshot-reads-accessibility-tree, decision/snapshot-reads-the-tree, decision/snapshot-model-single-source and decision/readers-of-one-snapshot-model: virtual nodes now enter the one snapshot model from the tree, read by every snapshot reader, with bounds taken from the declared region rather than the tree node.
@task_item n=4 part=touches
decision/ui-graph-one-node-per-snapshot-node: the graph no longer mirrors every snapshot node — it excepts virtual nodes.
@task_item n=5 part=touches
contract/actionable-nodes/listing, decision/actionable-key-rule and decision/one-actionable-rule: a virtual node reads clickable false and is in no actionable listing, yet click accepts its id.
@task_item n=6 part=touches
contract/snapshot/clickable, contract/snapshot-node/clickable and contract/snapshot-text/clickable: a virtual node reads clickable false, and the clickable node key's help text changed.
@task_item n=7 part=touches
contract/snapshot/enabled and decision/enabled-by-disabled-presence: a virtual node's enabled comes from its declared disabled flag, not from a disabled attribute.
@task_item n=8 part=touches
contract/driver-click-refusals/covered: the covered cause's meaning changed; a virtual node reads covered when an element lies over its region's centre.
@task_item n=9 part=touches
contract/driver-refusal/disabled and contract/driver-actions/disabled: a click on a declared-disabled virtual node is refused disabled, first in the one refusal order.
@task_item n=10 part=touches
decision/synthetic-click-at-node-centre: a click on a virtual id is delivered at its region on its canvas, the canvas focused.
@task_item n=11 part=touches
decision/hidden-content-excluded, decision/hidden-content-excluded-from-a11y-tree, decision/inert-subtree-excluded and contract/accessibility-tree/hidden-and-inert-nodes: a hidden, collapsed or inert canvas takes no virtual node into the tree.
@task_item n=12 part=touches
contract/snapshot-diff/differs, contract/snapshot-diff/entries and contract/mcp-result/snapshot: the diff and MCP results now carry a virtual node's id and declared name; a click on one reads differs name.
@task_item n=13 part=touches
decision/accessibility-tree-through-document-method, decision/a11y-tree-through-document-trait, decision/adapter-tree-is-document-tree and decision/adapter-tree-equals-harness-tree: the Dioxus document's tree override now returns the tree with virtual nodes, so the adapter and the harness read the same widened tree.
@task_item n=14 part=touches
contract/accessibility-tree/focus: the tree's focus is left untouched by virtual nodes; no virtual node takes focus in the tree.
@task_item n=15 part=touches
decision/role-attribute-then-element-then-unknown, decision/role-mapping-html-aam and decision/accessible-name-precedence: a virtual node is an exception — its role is the declared role and its name the declared name, with no attribute or element read.
@task_item n=16 part=touches
decision/interactive-nodes-named: each virtual node carries its declared name in the tree and snapshot.
@task_item n=17 part=touches
decision/circle-drawer-canvas-pixels-only: a canvas node may now carry virtual node children in the tree; the Circle Drawer itself declares none yet.
@task_item n=18 part=touches
decision/changed-set-marks and decision/a11y-tree-refresh-on-change: a change of what a canvas declares marks nothing; the snapshot builds the tree per read, so a windowed adapter sees a declaration change late.
@task_item n=19 part=touches
decision/a11y-ci-leg, decision/a11y-test-leg and decision/a11y-ci-gate: the a11y leg gains a test target and runs twenty files, measured 20 · 199 · 0 · 0.
@task_item n=20 part=touches
decision/a11y-assertions-as-rust-tests: a new Rust integration check asserts the virtual nodes over the tree in both layout modes.
@task_item n=21 part=touches
decision/driver-schema-has-no-role-set: the click verb's help text changed; no role or name set was added to the schema.
@task_item n=22 part=touches
decision/stand-lint-clean-at-boot and decision/lint-contrast-single-formula: the lint's stated limits now say a canvas's virtual nodes are not linted; its contrast and overlap readings take no virtual node because a virtual entry has no element.
@task_fact n=1 slug=screen-reader-support
fact: A canvas's taken virtual nodes are children of the canvas's accessibility tree node, after its own children, with the declared role, the declared name as label, author id {owner}///{key} and disabled where declared; no bounds are written on them, and a hidden, collapsed or inert canvas, or one whose read errs, takes none.
@task_fact n=2 slug=screen-reader-support
fact: A virtual node's tree id carries the canvas's slot index low and an even high half, equal to no DOM id; past 2^31 - 1 nodes a canvas takes no further tree node.
@task_fact n=3 slug=screen-reader-support
fact: The snapshot reads a virtual node with id, role, name, enabled from its declared disabled flag, clickable false, highlighted false, no checked, value, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
@task_fact n=4 slug=screen-reader-support
fact: The UI graph excepts virtual nodes, so the actions answer and exploration hold none of them.
@task_fact n=5 slug=screen-reader-support
fact: Only click takes a virtual id, located at its canvas and refused in the one order disabled, off-screen, covered, not-found, stale; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
@task_fact n=6 slug=visual-design-verification
fact: A virtual node reads covered only when an element lies over its region's centre.
@task_fact n=7 slug=visual-design-verification
fact: The lint excludes virtual nodes by construction: a virtual node's lint entry has no element, so no region and no visible part, and no overlap or contrast finding names a virtual id; no skip in code holds this.
@task_fact n=8 slug=screen-reader-support
fact: A virtual node's id and declared name cross the snapshot text, its diff, MCP results and kept checks, which replay a virtual click; no log line at info carries either.
@task_fact n=9 slug=screen-reader-support
fact: A virtual node declaration states disabled, read by the tree as disabled and by the snapshot as enabled false.
@task_fact n=10 slug=screen-reader-support
fact: A transformed canvas's virtual node bounds are its region moved by the content-box origin alone, a stated limit.
@task_fact n=11 slug=a11y-strategy
fact: A change of what a canvas declares marks nothing in the changed set; the snapshot rebuilds the tree per read, and a windowed adapter sees the change late, a stated limit.
@task_item n=1 part=untouched
Keyboard navigation contracts — range input keys and pointer, the Cells grid, the CRUD list, single and multiple select keys, typeahead and Tab order — are unchanged; virtual nodes take no key or focus route.
@task_item n=2 part=untouched
The ARIA state and property reads of DOM elements (checked, pressed, selected, invalid, expanded, describedby, labelledby, level) are unchanged; virtual nodes read no ARIA attribute.
@task_item n=3 part=untouched
Stand colour contrast, WCAG thresholds, the stand accent colour, scrollbar thumbs and open select list colours and rows are unchanged.
@task_item n=4 part=untouched
The absence of a platform accessibility adapter on the stand binaries and the platform adapter choice are unchanged; no binary-level reading of virtual nodes was made.
@task_item n=5 part=untouched
Password masking, file input values, select list snapshot rows and their bounds, and the open-list highlighted row are unchanged.
@task_item n=6 part=untouched
Cognitive accessibility rows (invalid state beyond colour or class, the shared tab order table) are unchanged.
@task_item n=7 part=untouched
The UI graph state registration and the Dioxus reached-handler record are unchanged.
