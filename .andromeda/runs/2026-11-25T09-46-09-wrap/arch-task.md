@task_prose part=session
The session made a canvas's declared virtual nodes act as elements: each taken virtual node now stands in the accessibility tree under its canvas, and from it in the snapshot, its text and its diff, with id {owner}///{key}, declared role, declared name and declared disabled state.
The click verb now takes a virtual node id, located at its canvas region and refused in the one order (disabled, off-screen, covered, not-found, stale); type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
The UI graph, the actions answer, exploration's candidates and the lint hold no virtual node; the lint holds none by construction (a virtual entry has no element, so no visible part), not by a skip, after a planned skip proved inert and was dropped.
The virtual node declaration gained a public disabled field; the click help text and the covered and not-found refusal meanings were reworded; the capability statement now says the reserved virtual id form is produced and gains five stated limits.
The a11y CI leg gained the new integration target, its file count nineteen to twenty, measured 20 files, 199 passed, 0 failed, 0 ignored; the act-spans stand check's child call count moved 43 to 46.
No crate, dependency, verb, argument, key name, result field, refusal cause, session error, exit, log field or span was added or retired; the snapshot, diff, MCP results and kept checks now carry a virtual node's id and declared name, a boundary widening.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: edited — the custom widget trait and VirtualNodeDeclaration, the declaration a canvas widget returns for its virtual nodes (key, name, role, region), now with a public disabled flag
@task_file n=2 path=packages/dioxus-native-dom/src/virtual_node.rs
what: edited — the Dioxus document's virtual-node reader (DioxusDocument::virtual_nodes and its validation), now also the page-bounds reader of a virtual region and the accessibility tree id of a virtual node
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited — the Dioxus document and its Document trait accessibility_tree override, which now adds each canvas's virtual nodes to the accessibility tree, and the place map of virtual nodes the snapshot reads
@task_file n=4 path=packages/dioxus-native-dom/src/snapshot.rs
what: edited — the snapshot reader that builds snapshot nodes (id, role, name, state, bounds) from the accessibility tree, now reading virtual nodes with their declared state and region bounds
@task_file n=5 path=packages/dioxus-native-dom/src/graph.rs
what: edited — the UI graph reader built from the snapshot, which now skips every node whose id holds the virtual key separator
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
what: edited — the stable element id module and virtual_node_id, the reserved {owner}///{key} virtual node id form, its docs now saying the form is served in the tree, snapshot and diff
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited — the driver's command execution and target resolution for the acting verbs (click, type, scroll, screenshot, layout, canvas), now resolving a virtual id for click alone and reading it not-found elsewhere
@task_file n=8 path=packages/escher-driver/src/lint.rs
what: edited — the driver's lint verb and its findings (overlap, clip, truncation and the rest); only its doc's stated limits changed, naming that a canvas's virtual nodes are not linted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited — the driver's command schema, the self-description of every verb and node key; the click help and the clickable node key's help reworded
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: edited — the schema's unit tests that pin each verb's help, arguments and result; the click help pin moved
@task_file n=11 path=packages/escher-driver/src/refusal.rs
what: edited — the driver's refusal causes and their fixed meanings; the covered and not-found meanings reworded, with their unit-test pins
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: added — a new blitz-tests integration target holding virtual nodes as elements: the tree, the hidden canvas, the snapshot, click and its refusals, the other id verbs, the graph, actions and lint exclusion, MCP and kept-check crossing, in both layout modes
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited — the integration target holding declared virtual nodes read and validated in process, its declaration helper gaining the disabled field
@task_file n=14 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the stand check that the driver's command spans log no content, now also over a snapshot, a click and a refused layout on the declaring fixture
@task_file n=15 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited — the shared blitz-tests fixtures module, gaining a declaring canvas page with five virtual nodes and a hide toggle
@task_file n=16 path=.github/scripts/ci-leg.sh
what: edited — the one CI leg runner, its a11y leg's list of accessibility test targets gaining the new target
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: edited — the tests that pin the CI workflow contract, their a11y leg target tuple gaining the new target
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: edited — the tests holding the capability statement to the driver schema and the CI legs, their a11y leg file count pin moved to twenty
@task_file n=19 path=docs/capabilities.md
what: edited — the capability statement, its virtual node id form now produced, its Not built list dropping the producer of that form, its stated limits gaining five lines and its a11y leg count twenty
@task_item n=1 part=touches
decision/virtual-node-declaration-marks-nothing: a declaration change still marks and signals nothing, but no longer awaits a first reader; the snapshot builds the tree per read and its «yet» and the wait on Virtual nodes as elements are now stale.
@task_item n=2 part=touches
contract/nodes-widgets/virtual-nodes: VirtualNodeDeclaration gains a public disabled field, a struct-literal break; the engine still never calls Widget::virtual_nodes itself.
@task_item n=3 part=touches
contract/dioxus-bridge/virtual-node-id and contract/stable-element-id/virtual-node-id: the reserved {owner}///{key} form is now served, in the accessibility tree as author id and from it in the snapshot, its text, its diff, MCP results and kept checks; any wording that no carrier serves it or no tier produces it is stale.
@task_item n=4 part=touches
contract/dioxus-bridge/virtual-nodes and contract/dioxus-bridge/virtual-node-refusal: the reader is now consumed by the accessibility tree; a canvas whose read errs takes no virtual node, the refusal set itself unchanged.
@task_item n=5 part=touches
contract/document-trait/accessibility-tree, contract/accessibility-tree/build-accessibility-tree and contract/accessibility-tree/author-id: the Dioxus document's tree now adds each canvas's virtual nodes as children after the canvas's own, with role, label, author id {owner}///{key} and disabled; a hidden, collapsed or inert canvas takes none; no bounds written, tree focus untouched.
@task_item n=6 part=touches
decision/virtual-node-role-is-accesskit-role: the declared role is now the tree node's role and the snapshot's role as declared.
@task_item n=7 part=touches
decision/node-id-layout, decision/versioned-node-ids and contract/stable-element-id/tier-distinctness: a virtual node's tree id takes the canvas slot index low and 2·(n+1) high, an even high half equal to no DOM node id; none past 2^31 − 1 nodes.
@task_item n=8 part=touches
contract/snapshot/membership, contract/snapshot/snapshot-node and contract/snapshot/bounds: a virtual node is now a snapshot node with no children, bounds its region moved by its canvas's content-box origin.
@task_item n=9 part=touches
contract/snapshot/enabled, contract/node-keys/enabled, decision/disabled-one-reading and decision/one-disabled-reading: a virtual node's enabled reads its declared disabled flag, not the form-control disabled reading.
@task_item n=10 part=touches
contract/snapshot/clickable, contract/node-keys/clickable, contract/snapshot-node/clickable, contract/driver-snapshot/clickable and contract/dioxus-bridge/clickable: a virtual node reads clickable false yet click takes it; the clickable key's help text was reworded.
@task_item n=11 part=touches
contract/snapshot-diff/membership, contract/driver-diff/differs and contract/snapshot-text/lines: the diff and the snapshot text now carry virtual nodes; a click on one reads differs name in the diff.
@task_item n=12 part=touches
contract/ui-graph/graph, contract/actionable-nodes/actionable-nodes, contract/actionable-readers/actionable-nodes and contract/driver-explore/explore: the graph skips every node whose id holds the virtual separator, so actions and exploration's candidates hold none; an exception to graph readers built from the snapshot.
@task_item n=13 part=touches
decision/actionable-elements-keyed and decision/one-actionable-rule: virtual nodes are outside the actions answer and the actionable rule, though click takes them.
@task_item n=14 part=touches
contract/driver-verbs/click, contract/escher-session/click and contract/target-check/click-type: click now takes a listed virtual id, located at its canvas (the first element whose stable id is the owner), and runs the per-verb target check on its region; the click help text was reworded.
@task_item n=15 part=touches
contract/refusal-causes/covered and contract/refusal-causes/not-found: both fixed meanings were reworded; a virtual id reads not-found on every id verb but click, and a virtual node reads covered by an element over its region; decision/refusal-causes-closed-at-eight holds, no cause added.
@task_item n=16 part=touches
contract/driver-verbs/type, scroll, screenshot, layout, canvas, contract/driver-session/layout and contract/driver-session/canvas: each reads a virtual id not-found, recording the screen first.
@task_item n=17 part=touches
contract/lint-verb/overlap, contract/driver-session/lint and contract/driver-lint/answer: the lint holds no virtual node because a virtual entry has no element and so no visible part; no skip exists in code, a stated limit names it.
@task_item n=18 part=touches
decision/canvas-placement-at-click-centre, decision/widget-click-point-content-box and decision/widget-click-synthesis: a click on a virtual node acts on its canvas at its region, read against the canvas's content box, and focuses the canvas.
@task_item n=19 part=touches
decision/new-carrier-widens-boundary, decision/fixed-program-text-on-carriers, contract/mcp-role/tool-call-crossings and contract/kept-check/answer: no carrier was added, but the snapshot text, diff, MCP results and kept checks now carry author-declared virtual ids and names, a boundary widening on the existing carriers; a kept check holding a virtual click replays.
@task_item n=20 part=touches
decision/accessibility-tree-refresh-on-change and decision/a11y-rebuild-on-nonempty-changed-set: a declaration change marks nothing, so a windowed adapter sees it late; the snapshot rebuilds per read.
@task_item n=21 part=touches
contract/ci-leg-legs/a11y and decision/a11y-leg-widens-without-new-job: the a11y leg runs the new virtual_nodes_as_elements target, its file count nineteen to twenty, measured 20 · 199 · 0 · 0; any row cell holding nineteen is stale.
@task_item n=22 part=touches
decision/capability-statement-matches-schema, decision/capability-statement and contract/capability-statement/sections: the statement's reserved-form line, its Not built list and five new stated limits moved; its test pins the a11y leg count at twenty.
@task_item n=23 part=touches
decision/driver-logs-no-content, decision/no-snapshot-in-logs and decision/snapshot-text-never-logged: hold unchanged, now over virtual ids and names; no log site, span or field was added and the act-spans stand check reads none of them.
@task_item n=24 part=touches
decision/blitz-tests-one-target-per-file and decision/integration-tests-in-blitz-tests: one new blitz-tests target was added, run in both layout modes.
@task_fact n=1 slug=standard-contracts
fact: The Dioxus document's accessibility tree holds each canvas's taken virtual nodes as children of the canvas's tree node, after its own: role the declared role, label the declared name, author id {owner}///{key}, disabled where declared; a hidden, collapsed or inert canvas, or one whose read errs, takes none; no bounds are written and tree focus is untouched.
@task_fact n=2 slug=established-decisions
fact: A virtual node's accessibility tree id is its canvas's slot index low and 2·(n+1) high, an even high half equal to no DOM node id; a canvas past 2^31 − 1 virtual nodes takes no tree node.
@task_fact n=3 slug=standard-contracts
fact: A snapshot virtual node reads id, declared role and name, enabled as not declared disabled, invalid and focused, clickable false, highlighted false, no checked, value, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
@task_fact n=4 slug=standard-contracts
fact: click is the one verb that takes a virtual node id: it acts on the canvas at the virtual region and is refused in the one order (disabled, off-screen, covered, not-found, stale) with nothing run; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
@task_fact n=5 slug=standard-contracts
fact: The UI graph, the actions answer, exploration's candidates, the lint answer and the canvas answer hold no virtual node; the graph skips ids holding the virtual separator, and the lint holds none by construction since a virtual entry has no element and no visible part.
@task_fact n=6 slug=established-decisions
fact: A virtual node's id and author-declared name cross the snapshot text, the diff, MCP results and kept checks, a ratified widening of what the existing carriers hold, with no carrier, verb, key, field or cause added.
@task_fact n=7 slug=standard-contracts
fact: A virtual node reads covered only by an element over its region's centre, and its bounds are its region moved by its canvas's content-box origin, so a transformed canvas's virtual nodes read untransformed bounds; both are stated limits.
@task_fact n=8 slug=cross-cutting-patterns
fact: A change of what a canvas declares marks nothing in the changed set, so a windowed accessibility adapter sees it only at its next rebuild, while each snapshot reads the declarations afresh.
@task_item n=1 part=untouched
Paint, layout, style, CSS cascade and hit-testing internals: no rendering, layout or style row changes; the click reuses the existing target check.
@task_item n=2 part=untouched
The session socket wire, its versions and bounds, the MCP server protocol, session errors and exit codes: none added or changed.
@task_item n=3 part=untouched
Telemetry, its scrub, line format and target filter, and every span contract: no log site, span or field was added.
@task_item n=4 part=untouched
The select list, form controls, range input and keyboard activation rows: virtual nodes do not reach them.
@task_item n=5 part=untouched
The browser app, its history and persistence, networking, script engine, WPT runner and examples: untouched.
@task_item n=6 part=untouched
The stack, dependencies, toolchain and stack decisions, and the names sheet: no crate, binary, dependency or name was added or freed.
@task_item n=7 part=untouched
The recording directory, kept-check file shape and coverage role contracts: their shapes are unchanged; only the content they carry widened.
