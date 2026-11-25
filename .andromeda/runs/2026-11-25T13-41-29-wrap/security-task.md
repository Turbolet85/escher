@task_prose part=session
The session put a canvas's virtual nodes into every read: the UI graph, the actions answer, handler coverage, the layout read and exploration now hold each virtual node by its {owner}///{key} id, carrying no declared name.
The verbs that take a virtual id went from one (click) to two (click and layout); type, scroll, screenshot with an id and canvas still refuse a virtual id as not-found, now four verbs instead of five.
The layout answer for a virtual node says it is placed by the app in its canvas, at its offset from the canvas, with the snapshot's bounds; the engine is never asked why a region sits where it does. Placer::App is no longer reserved.
A virtual node carries no handler of its own: a click on it counts its canvas's listener under the canvas's id in the coverage line. Of the reads, only the lint and the canvas answer now hold no virtual node.
No new verb, cause, argument kind, node key, session error, exit, env var, log site, span or span field was added; the not-found meaning text and the help text of graph, actions and layout were restated. Measured: a virtual node's page bounds sit past the body's default 8 px margin.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph read (the graph answer's nodes, owners, components, handlers and reads); it no longer drops a virtual node, giving it its canvas as owner and no handlers or reads
@task_file n=2 path=packages/dioxus-native-dom/src/actionable.rs
what: the actionable node list behind the actions read (owner and keyed per node); docs only, a virtual node listed with keyed false
@task_file n=3 path=packages/escher-driver/src/execute.rs
what: the driver's command run path and id resolution (stale and not-found refusal of a caller-supplied id); now holds the closed table of verbs that take a virtual id, click and layout
@task_file n=4 path=packages/escher-driver/src/actions.rs
what: the actions read (the actions answer: each listed node's id, owner, keyed and the cause each verb would return), sharing the verbs' target check; now resolves a virtual node's target through its canvas
@task_file n=5 path=packages/escher-driver/src/layout.rs
what: the in-process layout read on a session (the layout answer: bounds, placement, size causes); now answers a virtual id with the app's placement in its canvas
@task_file n=6 path=packages/escher-driver/src/refusal.rs
what: the closed refusal cause set and its fixed meaning texts; the not-found meaning restated to name click and layout as the verbs taking a virtual id
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the closed driver verb schema and its fixed help text; the help of the graph nodes, actions nodes and layout placed fields restated
@task_file n=8 path=packages/escher-driver/src/coverage.rs
what: handler coverage (the coverage answer line, held and unexercised listeners); docs only, a virtual node carries no handler and its click counts its canvas's listener
@task_file n=9 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the new integration check of virtual nodes in the graph, actions, layout, coverage, exploration and kept-check reads, including that the new answers carry the id and no declared name and the lint holds no virtual node
@task_file n=10 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixture pages; adds a listened virtual-canvas page and a moved declared-nodes check
@task_file n=11 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the check of virtual nodes in the accessibility tree, snapshot and diff and of their click and refusals; restated to the lifted limits
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the check of declared virtual nodes validated in process; restated to the lifted limits
@task_file n=13 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check that each driver command leaves one content-free command span line; its virtual-fixture layout call now expected laid out, not refused
@task_file n=14 path=docs/capabilities.md
what: the committed capability statement with its stated limits and what each answer carries (ids, owner ids, verb names, closed cause words, never a value or typed text)
@task_item n=1 part=touches
decision/virtual-node-crossing is stale: the graph, actions and layout answers now carry a virtual node's id (never its declared name), and only the lint and the canvas answer hold no virtual node; the founder's own word ratifies it.
@task_item n=2 part=touches
contract/actions-answer/verbs, contract/actions-answer/nodes, contract/actions-answer/owner, contract/actions-answer/id and contract/actions-answer/keyed: the actions answer now lists each virtual node, owner its canvas, keyed false, and a verb that takes no virtual id reads not-found for it.
@task_item n=3 part=touches
decision/actions-answer-crossing and decision/actions-keyed-field: the actions answer now crosses with virtual ids; keyed reads false for a virtual id by the no-slash rule.
@task_item n=4 part=touches
decision/actions-read-shares-target-check: for a virtual node the actions read finds the target through its canvas, and a verb outside the virtual-id table reads not-found before the shared check.
@task_item n=5 part=touches
decision/graph-answer-crossing and contract/graph-answer/nodes-states: the graph answer now holds every snapshot node, each virtual node owned by its canvas with its component and empty handlers and reads.
@task_item n=6 part=touches
contract/driver-verbs/layout, decision/session-layout-read-in-process and decision/kept-check-layout-answer: layout now takes a virtual id and answers placed by app in the canvas at the region's offset, with the canvas's shifts and size cause layout; it changes nothing.
@task_item n=7 part=touches
contract/driver-verbs/graph, contract/driver-verbs/actions and contract/driver-verbs/canvas: graph and actions now hold virtual nodes; canvas still reads a virtual id not-found.
@task_item n=8 part=touches
contract/refusal-causes/not-found and contract/refusal-json/meaning: the not-found meaning text now names click and layout as the verbs that take a virtual id; no count, name, order or remedy moved (causes stay 8, verbs 16).
@task_item n=9 part=touches
contract/driver-id-refusal/not-found, decision/unknown-id-refused-stale-or-not-found and decision/target-refusal-order: the id verbs refusing a virtual id not-found went from 5 to 4; stale and not-found order unchanged.
@task_item n=10 part=touches
contract/stable-element-id/virtual-node-form, contract/stable-element-id/carriers and decision/stable-id-exits: the virtual {owner}///{key} id now leaves through the graph, actions and layout answers as well.
@task_item n=11 part=touches
decision/coverage-answer-crossing, contract/coverage-line/unexercised and contract/coverage-line/held: a replayed virtual click records its canvas's click under the canvas's id; no coverage line holds a virtual id.
@task_item n=12 part=touches
decision/exploration-report-closed-content: exploration now acts on each virtual node as on any listed node.
@task_item n=13 part=touches
decision/kept-check-graph-answer, decision/kept-check-actions-answer and decision/kept-check-answer-whole: kept checks now hold graph and actions answers carrying virtual ids, kept whole and replaying green.
@task_item n=14 part=touches
decision/lint-findings-crossing and decision/lint-findings-closed-content: unchanged in effect but now the one read besides the canvas answer that holds no virtual node; no overlap, clip or off-screen finding on a region.
@task_item n=15 part=touches
decision/closed-driver-verb-schema and decision/closed-refusal-cause-set: help and meaning text restated only; the closed sets keep every member.
@task_item n=16 part=touches
decision/command-span-content-free, decision/stable-id-not-logged and decision/accessible-names-not-logged: hold as stated; no log site, span or span field added for the new reads.
@task_item n=17 part=touches
decision/one-actionable-rule and contract/snapshot-text/clickable: a virtual node's clickable still reads false while it is listed in the actions answer.
@task_fact n=1 slug=input-validation
fact: Only click and layout take a virtual node's id; type, scroll and screenshot with an id and canvas refuse a virtual id as not-found, a decided limit.
@task_fact n=2 slug=threat-model-summary
fact: The layout answer for a virtual node says it is placed by the app in its canvas at its offset from the canvas, with the snapshot's bounds and the canvas's shifts; the engine is never asked why a region is placed, and the answer carries the id and no declared name.
@task_fact n=3 slug=threat-model-summary
fact: A virtual node carries no handler of its own; coverage counts a click on it as its canvas's listener under the canvas's id, so a region's own handling is not counted.
@task_fact n=4 slug=threat-model-summary
fact: The lint reads no virtual node: no overlap, clip or off-screen finding is made on a region, a stated limit carried on Circle Drawer's circles as virtual nodes.
@task_fact n=5 slug=input-validation
fact: A virtual node's keyed and clickable both read false wherever it is listed, decided limits.
@task_item n=1 part=untouched
Dependency security: no dependency, lockfile, manifest or pinned tool moved.
@task_item n=2 part=untouched
Secret management and environment variables: no env var or secret read or added.
@task_item n=3 part=untouched
Authentication, the session socket and the MCP transport: no listener, bound, grammar, idle expiry or connection rule moved.
@task_item n=4 part=untouched
Outbound network, fetchers, CORS, CSP and navigation: no request, URL or scheme handling reached.
@task_item n=5 part=untouched
Masked input values, password painting and select list behaviour: no value mask or select reader changed.
@task_item n=6 part=untouched
Session errors, exit codes and MCP protocol errors: no session error, exit or protocol error added or moved.
@task_item n=7 part=untouched
Log sinks, target allowlists and telemetry: no sink, target or field moved.
@task_item n=8 part=untouched
CI workflows, publishing, signing, WPT runner, cold-agent and agent-run scripts: none reached.
@task_item n=9 part=untouched
The recording directory, kept-check file medium, lifecycle and owner-only rules: unchanged.
@task_item n=10 part=untouched
Declared virtual node validation and its closed refusal set: unchanged by this session.
