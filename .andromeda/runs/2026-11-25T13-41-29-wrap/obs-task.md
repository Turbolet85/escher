@task_prose part=session
A canvas's virtual nodes now stand in every read: the UI graph holds each one as a graph node owned by its canvas, with the canvas's component and no handlers or reads; the actions answer lists each one with owner the canvas and keyed false; exploration acts on each as on any listed node.
The layout verb now takes a virtual id and explains it as placed by the app inside its canvas, at the region's offset from the canvas; the engine is never asked for a region's reason, and the app placer is no longer reserved.
The verbs that take a virtual id went from 1 (click) to 2 (click, layout); type, scroll, screenshot with an id and canvas still refuse a virtual id not-found (5 to 4 such verbs); the lint and the canvas answer alone still hold no virtual node.
A virtual node carries no handler of its own: a replayed click on one counts its canvas's listener under the canvas's id, and no coverage line holds a virtual id.
The graph, actions and layout answers carry a virtual node's id and never its declared name; no log site, print, span, span field, verb, cause, exit or env var was added, and the not-found cause's meaning text and three help texts were restated.
Retired: the stated limits that only click takes a virtual id and that the graph, actions and exploration hold none; measured: a fixture node's page bounds sit 8 px further than forecast (the body's default margin).
@task_file n=1 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the new integration check of virtual nodes in the graph, actions, layout, handler coverage, exploration and a kept check's replay, and that the new answers carry the id and no declared name and the lint holds none
@task_file n=2 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph built from the snapshot, one graph node per snapshot node with its owner, component, handlers and reads; edited to hold virtual nodes
@task_file n=3 path=packages/dioxus-native-dom/src/actionable.rs
what: the list of actionable nodes the actions read is built on, with its owner and keyed flags; doc comments edited
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's command run path and id resolution, now with the table of verbs that take a virtual id (click and layout); edited
@task_file n=5 path=packages/escher-driver/src/actions.rs
what: the actions read of the driver, each listed node with its owner and each verb's cause; edited to list virtual nodes
@task_file n=6 path=packages/escher-driver/src/layout.rs
what: the layout read that explains a node's placement, size causes and shifts; edited to answer a virtual id as placed by the app
@task_file n=7 path=packages/escher-driver/src/refusal.rs
what: the closed refusal causes and their fixed meaning texts; the not-found meaning restated
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the verb table with each verb's arguments and fixed help texts; graph, actions and layout help restated
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: the handler coverage of the coverage role, each listener with its owner; docs edited for a canvas's virtual nodes
@task_file n=10 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared test fixture pages and helpers; a listened virtual-canvas page added and a declared-nodes helper moved in
@task_file n=11 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the integration check of virtual nodes in the accessibility tree, snapshot and diff; edited to use the moved helper
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration check of a canvas's declared virtual nodes read and validated in process; edited
@task_file n=13 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check that each driver command leaves one span line holding nothing it handled; its virtual layout call restated from refused to laid out
@task_file n=14 path=docs/capabilities.md
what: the capability statement, updated by hand; its stated limits on virtual nodes and the actions and layout answers' content restated
@task_item n=1 part=touches
decision/declared-node-read-logs-nothing: the declared-node read now reaches the graph, actions, layout, coverage and exploration, still with no log site, print, span or span field added; its guard check may have moved with the declared-nodes helper moving into the shared fixtures.
@task_item n=2 part=touches
contract/graph-answer/content and contract/graph-answer/shape: the graph answer now holds every virtual node, its id in the id field, owner its canvas, no handlers or reads, and no declared name.
@task_item n=3 part=touches
contract/actions-answer/content, contract/actions-answer/owner and contract/actions-answer/shape: the actions answer now lists each virtual node with owner the canvas, keyed false, and each verb's cause equal to its call's.
@task_item n=4 part=touches
contract/actions-answer/cause and contract/actions-answer/logging: a virtual node's verbs read not-found for the four verbs that refuse it; the answer still logs nothing.
@task_item n=5 part=touches
contract/layout-answer/content and contract/layout-answer/logging: the layout answer now answers a virtual id, placed by app with the canvas's id, ids, closed words and numbers only, never the declared name.
@task_item n=6 part=touches
decision/layout-read-opens-no-span: layout on a virtual id opens no span of its own; only the existing command span.
@task_item n=7 part=touches
contract/coverage-line/owner and contract/coverage-line/content: a click on a virtual node counts its canvas's click listener under the canvas's id; no coverage line holds a virtual id.
@task_item n=8 part=touches
contract/refusal-cause/not-found: its meaning text now says only click and layout take a virtual id; the cause name, count of 8 causes and remedies unchanged.
@task_item n=9 part=touches
decision/exploration-adds-no-span: exploration now acts on virtual nodes as listed nodes, adding no span of its own.
@task_item n=10 part=touches
contract/command-span/graph, contract/command-span/actions, contract/command-span/layout and contract/command-span/field-count: these commands now handle virtual ids with no field added to the command span.
@task_item n=11 part=touches
decision/command-span-no-call-content and decision/driver-spans-carry-no-content: a virtual id now flows through graph, actions and layout calls; the span still carries no call content.
@task_item n=12 part=touches
decision/actions-read-changes-nothing: the actions read over virtual nodes and the layout read of a virtual id change nothing in the document.
@task_item n=13 part=touches
decision/graph-answer-never-logged and decision/session-logs-carry-no-page-content: the graph answer now holds virtual nodes, still never logged and never carrying the declared name.
@task_item n=14 part=touches
contract/kept-check/answer and contract/kept-check/arguments: a kept check of the new reads (graph, actions, layout on virtual ids) keeps their answers whole and replays green.
@task_item n=15 part=touches
decision/capability-statement-updated-by-hand: the capability statement's virtual-node limits were restated by hand (click and layout take a virtual id; the lint and canvas answer hold none).
@task_item n=16 part=touches
contract/snapshot-node/clickable: a virtual node's clickable stays false, now a decided limit.
@task_item n=17 part=touches
contract/cold-agent-counted-verbs/layout, contract/cold-agent-counted-verbs/graph and contract/cold-agent-counted-verbs/actions: these counted verbs now also read virtual nodes; verb count 16 unchanged.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: The graph, actions and layout answers carry a virtual node only by its id (canvas id, separator, key) and never its declared name; the lint and the canvas answer hold no virtual node.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: A virtual node carries no handler of its own: a click on one is counted on its canvas's listener under the canvas's id, and no coverage line holds a virtual id.
@task_fact n=3 slug=span-trace-coverage
fact: Reading virtual nodes in the graph, actions, layout, coverage and exploration opens no span and adds no span field or log site; they ride the existing command span alone.
@task_item n=1 part=untouched
Telemetry strategy rows (tracing crate, engine tracing feature, escher crates tracing ungated) and the absence of a telemetry backend or OTel export: no dependency, feature or subscriber changed.
@task_item n=2 part=untouched
Metric coverage and metric emission absences, and WPT score publishing: nothing measured or emitted moved.
@task_item n=3 part=untouched
The telemetry sink and its log line contracts (format, scrub, escaping, allowlist, RUST_LOG, stream, resource keys, span line layout): no sink or log-line change.
@task_item n=4 part=untouched
Error capture rows: panic hooks, script error surfaces, window error events, WPT report messages and the scrub set's error and url.
@task_item n=5 part=untouched
Agent-run and cold-agent harness rows: events, verdict fields, reasons, transcripts, record directories and cleanup; no harness or gate surface changed.
@task_item n=6 part=untouched
escher-session's exit codes, host wire, mcp role, error object, stderr diagnostics and session lifecycle: no exit, session error or wire change.
@task_item n=7 part=untouched
CI integration rows: workflows, log and coverage artifacts, recording bounds and privacy, the build-directory sweep line and the package-install diagnostic line.
@task_item n=8 part=untouched
Select, optgroup, multiple-select, page-event and open-list PII rows: form controls untouched.
@task_item n=9 part=untouched
The tool_call span and its ending and the refusal cause names other than not-found: no cause name, count or order moved.
