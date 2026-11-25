@task_prose part=session
The session brings a canvas's virtual nodes into every read: the UI graph, the actions answer, handler coverage, the layout verb and exploration now hold each virtual node, read by its owner///key id.
The verbs that take a virtual id move from click alone to click and layout; type, scroll, screenshot with an id and canvas still read a virtual id not-found (5 such verbs to 4), and the lint and the canvas answer still hold no virtual node.
A virtual node in the graph is owned by its canvas, carries its canvas's component and no handlers or reads; in actions it is listed with owner its canvas, keyed false, each verb's cause equal to its call's; a click on one counts its canvas's listener under the canvas's id.
Layout on a virtual id answers placed by app in its canvas at the region's offset from the canvas, the snapshot bounds, the canvas's shifts, size cause layout; the engine is never asked for a region's reason; the reserved status of the app placer is retired.
A new integration test target covers the reads in both layout modes, controlled red by eight edits; a stand span check that pinned the refused virtual layout was restated; no verb, cause, argument kind, session error, exit, log site or span field added; verbs 16, causes 8 unchanged.
Measured: a fixture region's page bounds sit 8 px off the plan's forecast, the body's default margin; CI green 17/17, the new target's two unix-only cases leaving an unused-import warning on windows, carried not fixed.
@task_file n=1 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: edited (new) — the integration test target for virtual nodes in every read: graph, actions, layout, the answers' id and no declared name, the lint holding none, exploration, handler coverage by replay and a kept check replaying, in both layout modes
@task_file n=2 path=packages/dioxus-native-dom/src/graph.rs
what: edited — the UI graph built from the snapshot: one graph node per snapshot node, its owner, component, handlers and reads; now holds each virtual node owned by its canvas
@task_file n=3 path=packages/dioxus-native-dom/src/actionable.rs
what: edited — the actionable nodes reader and its keyed rule (docs only): a virtual node listed by the owner arm, keyed false
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: edited — the driver's executor and id resolution: the table of verbs that take a virtual id (click and layout) and the resolver they share, with its unit test
@task_file n=5 path=packages/escher-driver/src/actions.rs
what: edited — the actions verb's answer: each listed node with its owner, keyed and the cause each target verb would return; a virtual node's target found through its canvas
@task_file n=6 path=packages/escher-driver/src/layout.rs
what: edited — the layout verb: what placed a node, its bounds, laid-out size, shifts and size causes; a virtual node answered as placed by app in its canvas
@task_file n=7 path=packages/escher-driver/src/refusal.rs
what: edited — the refusal causes and their fixed meanings: the not-found meaning now names click and layout as the verbs taking a virtual id
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: edited — the driver schema's verb table and help text: the help of graph's nodes, actions' nodes and layout's placed restated
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: edited — handler coverage by replay (docs only): a virtual node carries no handler; a click on one counts its canvas's listener under the canvas's id
@task_file n=10 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited — the shared fixture pages module: a new virtual canvas page with a listened canvas, and the declares-five check moved in
@task_file n=11 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: edited — the integration test target for virtual nodes as elements: snapshot, accessibility tree, diff, click and refusals; now reads its shared check from the fixtures module
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: edited — the integration test target for a canvas's declared virtual nodes, read and validated in process
@task_file n=13 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the stand check that each driver command leaves one span line holding nothing it handled; its virtual-fixture layout call now expected laid out, not refused
@task_file n=14 path=docs/capabilities.md
what: edited — the capability statement pinned to the driver schema: the stated limits on virtual nodes and what each answer holds
@task_item n=1 part=touches
decision/reserved-graph-id-form — the owner///key id now appears in the UI graph, the actions answer and the layout answer, not only in the snapshot and click.
@task_item n=2 part=touches
contract/element-id/virtual-form — a virtual id is now taken by click and layout (was click alone); the four other id verbs read it not-found.
@task_item n=3 part=touches
The three not-found contracts (contract/refusal-cause/not-found, contract/driver-refusal-causes/not-found, contract/driver-session/stale-or-not-found) — the not-found meaning reads a virtual id as taken by click and layout only; the count of verbs reading it not-found moves 5 to 4.
@task_item n=4 part=touches
contract/handler-coverage-limits/canvas-handler and decision/handler-coverage-canvas-uncounted — a virtual node carries no handler; a replayed virtual click records its canvas's listener under the canvas's id, without it the listener is listed unexercised.
@task_item n=5 part=touches
contract/coverage-line/owner — the owner a coverage line names for a virtual click is the canvas's id; no line holds a virtual id.
@task_item n=6 part=touches
The graph rows (contract/driver-verbs/graph, contract/session-run/graph, contract/driver-session/graph, contract/session-wire/graph-answer, contract/graph-command/argument) — the graph now holds every snapshot node, each virtual node owned by its canvas with no handlers or reads; the verb equals the in-process graph.
@task_item n=7 part=touches
decision/caused-owner-pinned-as-graph-reads — a virtual node's owner as the graph reads it is its canvas.
@task_item n=8 part=touches
The actions rows (contract/driver-verbs/actions, contract/actions-command/argument, contract/cli-actions/output, contract/mcp-actions/argument, contract/command-span/actions) — actions now lists each virtual node, owner its canvas, keyed false, six verbs each with the cause its call returns.
@task_item n=9 part=touches
decision/one-actionable-rule — a virtual node is listed by the existing owner arm; keyed reads false for it by the no-slash rule.
@task_item n=10 part=touches
The layout rows (contract/session-layout/changes-nothing, contract/session-run/layout-answer, decision/layout-reason-words-each-read) — layout on a virtual id answers by app with its canvas, changes nothing, reads stale or not-found as before; the app placer word is now read on a fixture.
@task_item n=11 part=touches
decision/graph-answer-holds-no-value and decision/actions-answer-holds-no-value — the new answers carry the virtual id and never the declared name.
@task_item n=12 part=touches
contract/snapshot-node/clickable — a virtual node's clickable stays false, now a decided limit.
@task_item n=13 part=touches
The lint rows (contract/lint-defects/overlap, contract/lint-defects/clipped, contract/lint-defects/off-screen) — the lint still reads no virtual node, now pinned by a check; its limit stated and carried.
@task_item n=14 part=touches
contract/session-recording/exploration-checks and decision/exhaustion-asserted-on-finite-page — exploration acts on each virtual node as on any listed node, its explored path's check replaying green.
@task_item n=15 part=touches
decision/table-tests-count-their-rows — the other id verbs' not-found table's row count moves 5 to 4; the new actions table counts 5 nodes by 6 verbs.
@task_item n=16 part=touches
decision/layout-tests-in-both-layout-modes and decision/stand-checks-in-both-layout-modes — every case of the new target and the restated stand span check run in both layout modes.
@task_item n=17 part=touches
decision/shared-fixture-pages and decision/shared-fixtures-in-one-module — a new shared page and a shared check moved into the fixtures module.
@task_item n=18 part=touches
decision/one-test-target-per-behaviour and decision/one-test-target-per-file — a new blitz-tests target for the behaviour.
@task_item n=19 part=touches
decision/capability-statement-pinned and decision/capability-claims-name-their-checks — the capability statement's virtual-node limits restated; verbs 16 and causes 8 unchanged.
@task_item n=20 part=touches
decision/mutation-controlled-tests — eight controls (W1 to W7, L1) read red with their edits.
@task_item n=21 part=touches
decision/count-and-rename-sweeps — a sweep keyed on moved texts missed a refusal pinned through a call builder; a pinned refusal is found by the builder and the expected cause together.
@task_item n=22 part=touches
decision/socket-session-checks-unix-only and decision/socket-checks-unix-gated — two unix-gated cases of the new target left an import unused on windows, a warning carried, not fixed.
@task_item n=23 part=touches
decision/expected-value-pinned-as-literal — a region's page bounds pinned as measured literals, the forecast's off by the body's default margin.
@task_item n=24 part=touches
contract/driver-session/refusal-order — the virtual-id resolution keeps its stale then not-found order; a virtual node whose canvas no longer resolves reads stale on every verb in actions.
@task_fact n=1 slug=integration-test-strategy
fact: Layout on a virtual id answers placed by app in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and size cause layout; the engine is never asked for a region's reason.
@task_fact n=2 slug=integration-test-strategy
fact: Actions finds a virtual node's target through its canvas; a verb that takes no virtual id reads not-found for it, every other verb is decided as for any node.
@task_fact n=3 slug=unit-test-strategy
fact: The lint reads no virtual node: no overlap, clip or off-screen finding on a canvas's region, a stated limit carried on the circle-drawer virtual-nodes entry.
@task_fact n=4 slug=integration-test-strategy
fact: Only click and layout take a virtual id; type, scroll, screenshot with an id and canvas read it not-found, and a virtual node's keyed and clickable read false — decided limits, not owed.
@task_fact n=5 slug=test-data-fixtures
fact: A fixture page's absolute coordinates sit inside the body's default 8 px margin; a check pinning a fixture's page coordinates pins them as measured.
@task_fact n=6 slug=session-lifecycle
fact: Exploration acts on a canvas's virtual nodes as on every listed node, and its explored path's check replays them.
@task_item n=1 part=untouched
WPT conformance, its runner, statuses and expectations: no web-platform reading moved.
@task_item n=2 part=untouched
CI workflows, legs, jobs, caches, package installs, coverage floors and mutation runs: no harness or gate surface changed; no count of the a11y leg's or stand-repeat's targets moved.
@task_item n=3 part=untouched
The cold-agent pipe and agent-run contracts, events, exit codes and verdicts.
@task_item n=4 part=untouched
The session wire, protocol versions, session errors, exit statuses, state directory and recording permissions: no verb, cause, argument kind, session error or exit added.
@task_item n=5 part=untouched
The harness's settle, virtual clock, advance and timer rows, and the harness construction, input and screenshot helpers.
@task_item n=6 part=untouched
Mocking and stand-in rows, network stand-ins and font discipline.
