@task_prose part=session
The session put a canvas's virtual nodes into every driver read: the UI graph, the actions answer, layout explained, handler coverage and exploration now hold each virtual node, by its {owner}///{key} id and never its declared name.
In the UI graph a virtual node is one node owned by its canvas, with the canvas's component and no handlers or reads; actionable_nodes lists it, keyed false, clickable false.
The verbs that take a virtual id moved from click alone to click and layout, read from one table keyed on the verb; type, scroll, screenshot with an id and canvas still refuse a virtual id not-found (5 such verbs to 4).
layout answers a virtual node as placed by its app in its canvas (Placer::App, no longer reserved), at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and size cause layout; the engine is never asked for a region's reason.
Retired: the stated limits «only click takes a virtual id» and «the UI graph, actions, the lint and exploration hold no virtual node» (now the lint and the canvas answer alone); no verb, cause, argument kind, node key, session error, exit, env var, dependency, log site or span field added; verbs 16 and causes 8 unchanged.
@task_file n=1 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: added — a new blitz-tests integration target, one behavior per file, both layout modes: virtual nodes in the UI graph, actions, layout, coverage replay, exploration and kept checks
@task_file n=2 path=packages/dioxus-native-dom/src/graph.rs
what: edited — the UI graph reader of the Dioxus DOM bridge (DioxusDocument::graph, GraphNode owner), which now keeps a virtual node owned by its canvas
@task_file n=3 path=packages/dioxus-native-dom/src/actionable.rs
what: edited — the Dioxus actionable readers (DioxusDocument::actionable_nodes, ActionableNode), docs only: a virtual node listed by its owner, keyed false
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: edited — the driver's run path and per-verb id resolution: the table of verbs that take a virtual id (click, layout) and the shared resolver for them
@task_file n=5 path=packages/escher-driver/src/actions.rs
what: edited — the actions verb (Session::actions): each listed node's target verbs and the cause each would return, now for a virtual node through its canvas
@task_file n=6 path=packages/escher-driver/src/layout.rs
what: edited — the layout verb (Session::layout) and its placing word: a virtual node explained as placed by its app in its canvas
@task_file n=7 path=packages/escher-driver/src/refusal.rs
what: edited — the closed refusal causes and their fixed meanings: not-found's meaning now names click and layout as the verbs taking a virtual id
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: edited — the driver's command schema and verb table help text (graph nodes, actions nodes, layout placed)
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: edited — the handler coverage reader, docs only: a virtual node carries no handler; its click counts its canvas's listener under the canvas's id
@task_file n=10 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited — the shared blitz-tests fixtures module: a new listened virtual-canvas page and a shared declared-nodes helper moved into it
@task_file n=11 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: edited — the blitz-tests target for virtual nodes as elements (snapshot, accessibility tree, diff, click), now reading the shared fixture
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited — the blitz-tests target for virtual nodes declared by a canvas, its graph case restated now the graph holds them
@task_file n=13 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the stand check that each driver command leaves one span line; its virtual-fixture layout call now expected laid out, not refused
@task_file n=14 path=docs/capabilities.md
what: edited — the capability statement: the stated limits on virtual nodes and what the actions and layout answers carry
@task_item n=1 part=touches
contract/ui-graph/graph: the UI graph's «a virtual node excepted» is stale; every snapshot node, virtual nodes included, is one graph node.
@task_item n=2 part=touches
contract/ui-graph/owner: a virtual node's owner is its canvas (the id's text before the separator), its component the canvas's, handlers and reads empty.
@task_item n=3 part=touches
contract/driver-verbs/click-virtual-node: the verbs that take a virtual id are now click and layout (1 to 2), read from one table.
@task_item n=4 part=touches
contract/refusal-causes/not-found: its meaning now reads that only click and layout take a virtual id; type, scroll, screenshot with an id and canvas refuse one not-found (4 verbs, was 5).
@task_item n=5 part=touches
contract/actions-verb/target-verbs and contract/actions-answer/node-fields: actions now lists each virtual node, owner its canvas, keyed false, its target found through the canvas; a verb not taking a virtual id reads not-found.
@task_item n=6 part=touches
contract/answer-json/layout-placing and contract/layout-reason/placed-by: the app placing word (Placer::App) is no longer reserved; a virtual node reads by app with its canvas as container.
@task_item n=7 part=touches
contract/driver-session/layout and contract/driver-verbs/layout: layout now takes a virtual id and explains it from the snapshot and its canvas, changing nothing; stale and not-found order unchanged.
@task_item n=8 part=touches
contract/handler-coverage/canvas and contract/driver-recording/handler-coverage: a replayed virtual click records the canvas's click listener under the canvas's id; no coverage line holds a virtual id.
@task_item n=9 part=touches
decision/keyed-wire-form and decision/actionable-elements-keyed: a virtual node reads keyed false by the no-slash rule; stated as a decided limit, not owed.
@task_item n=10 part=touches
contract/actionable-readers/actionable-nodes and contract/actionable-nodes/actionable-nodes: actionable_nodes now lists virtual nodes through the owner arm.
@task_item n=11 part=touches
contract/driver-explore/explore and contract/driver-explore/explored-verbs: exploration acts on each virtual node as on every listed node.
@task_item n=12 part=touches
decision/ui-graph-crossing, decision/actions-answer-crossing and decision/layout-answer-crossing: the graph, actions and layout answers now carry a virtual node's id and never its declared name.
@task_item n=13 part=touches
contract/graph-verb/node, contract/driver-graph/answer and contract/dioxus-bridge/screen-readers: the graph verb's answer equals the in-process graph and now holds virtual nodes.
@task_item n=14 part=touches
decision/measured-limits-stated-in-the-tool and decision/capability-statement: the capability statement's virtual-node limits restated (click and layout take one; the lint and canvas answer hold none).
@task_item n=15 part=touches
contract/target-check/click-type, contract/target-check/canvas, contract/target-check/screenshot and contract/target-check/scroll: per-verb target checks of a virtual id go through the one virtual-id table; only click and layout pass it.
@task_item n=16 part=touches
contract/stable-element-id/virtual-node-id and contract/dioxus-bridge/virtual-node-id: the {owner}///{key} id form now crosses in graph, actions and layout answers.
@task_item n=17 part=touches
decision/validated-commands-only and contract/validation-order/first-failure: actions decides a virtual node's non-virtual verbs by validate then the target check as before; validation order unchanged.
@task_fact n=1 slug=standard-contracts
fact: layout on a virtual node id answers it placed by its app in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and size cause layout; the engine is never asked for a region's reason.
@task_fact n=2 slug=standard-contracts
fact: the verbs that may take a virtual node id are one table keyed on the verb (click, layout), read alike by the run path, the in-process layout and actions; every other id verb refuses a virtual id not-found.
@task_fact n=3 slug=standard-contracts
fact: absence: the lint reads no virtual node, so no overlap, clip or off-screen finding falls on a canvas's declared region; a decided limit carried, not owed here.
@task_item n=1 part=untouched
Engine layout, style, paint, hit testing and WPT rows: no engine reader or layout pass changed; a region's reason is never asked of the engine.
@task_item n=2 part=untouched
Snapshot, accessibility tree and diff virtual-node rows (contract/snapshot/virtual-node, contract/accessibility-tree/virtual-nodes, contract/nodes-widgets/virtual-nodes): their shapes and membership are unchanged here.
@task_item n=3 part=untouched
Telemetry, spans and logging rows: no log site, span or span field added; the command span is unchanged.
@task_item n=4 part=untouched
Session wire, socket, MCP role, command line, exit codes and session errors: no verb, argument kind, node key, error, exit or env var added; answer shapes unchanged.
@task_item n=5 part=untouched
Closed counts: verbs 16, refusal causes 8 (decision/refusal-causes-closed-at-eight), the a11y leg's 20 targets and stand-repeat's 17 named targets are unchanged.
@task_item n=6 part=untouched
Stack, dependencies, CI legs and gates, coverage floors and the names sheet: no dependency, crate, binary, leg or name added or freed.
@task_item n=7 part=untouched
The canvas verb and its answer (contract/canvas-verb/*, decision/canvas-crossing): still holds no virtual node and refuses a virtual id as before.
