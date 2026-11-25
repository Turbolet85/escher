@task_prose part=session
A canvas's declared virtual nodes now appear as elements: in the accessibility tree under their canvas, in the snapshot and its diff, and over MCP and in kept checks, by their owner///key id and declared name.
click takes a virtual id (located at its canvas region, refused disabled, off-screen, covered, not-found or stale in the one order); type, scroll, screenshot with an id, layout and canvas read a virtual id not-found; the graph, actions, lint and exploration hold none.
The virtual node declaration gains a disabled flag; click's help and the covered and not-found cause meanings were reworded; no verb, argument, key, result field, cause, session error, exit, log field or span was added.
One new integration test target (virtual_nodes_as_elements, 9 cases, both layout modes, the kept-check case unix-only) joined the a11y CI leg, its file count nineteen to twenty (measured 20 · 199 · 0 · 0); the stand act-spans check grew 43 to 46 child calls.
The planned lint skip for virtual ids was written, read inert by its control (V2 does not hold) and dropped on the operator's word; the lint answer holding no virtual id is pinned by a behaviour reading instead, so the every-control-reads-red acceptance stays unmet for V2.
CI on the pushed commit read green 17/17 on linux, macos and windows.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget's virtual node declaration (key, name, role, region), now with a disabled flag
@task_file n=2 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the in-process read of a canvas's declared virtual nodes, now with their page bounds and their accessibility tree ids
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the dioxus document's accessibility tree, which now adds each canvas's virtual nodes under the canvas's tree node
@task_file n=4 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot of the screen (node id, role, name, enabled, clickable, bounds), now holding virtual nodes
@task_file n=5 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph built from the snapshot, which now skips every virtual node
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id and its reserved owner///key virtual form, its docs now saying the form is served
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver executor's id resolution and click, now locating a virtual id at its canvas and reading it not-found for every other id verb
@task_file n=8 path=packages/escher-driver/src/lint.rs
what: the lint and its defects (overlap, clipped, truncated and the rest); only its stated limits doc gained that virtual nodes are not linted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the driver schema's verb table, click's help and the clickable snapshot key's help
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the schema unit tests pinning each verb's help, arguments and result
@task_file n=11 path=packages/escher-driver/src/refusal.rs
what: the refusal causes and their meanings and remedies, covered and not-found reworded, with its unit test pinning the meanings
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the new integration test target for virtual nodes as elements: tree, snapshot, click, refusal order, not-found verbs, graph actions and lint, MCP and kept check
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration test target for declared virtual nodes and the declaring canvas putting nothing on the graph
@task_file n=14 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand act-spans check that no log line holds an id, a name or typed text, its child call count now 46
@task_file n=15 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the shared fixtures module, gaining the virtual canvas fixture page and its five ids and six names
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the CI leg script, whose a11y leg arm now runs the new target
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the CI workflow unit tests holding each leg's membership tuple
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the capability statement unit tests pinning the leg counts in words and as an integer
@task_file n=19 path=docs/capabilities.md
what: the capability statement: claims, not built list and stated limits, now with five virtual-node limits and the a11y leg's twenty files
@task_item n=1 part=touches
decision/reserved-graph-id-form: no served id holding the owner///key form is no longer true; the tree, snapshot, diff, MCP and kept checks now serve it, so the rule narrows to the graph alone.
@task_item n=2 part=touches
contract/element-id/virtual-form: the virtual id form is now served in the accessibility tree and from it the snapshot and its diff; the element walk still writes none.
@task_item n=3 part=touches
contract/snapshot-node/clickable: a virtual node reads clickable false (and enabled from its declared disabled), and the clickable key's help text changed.
@task_item n=4 part=touches
The refusal cause meanings: contract/refusal-cause/covered and contract/refusal-cause/not-found (and the other not-found rows, contract/driver-refusal-causes/not-found, contract/driver-session/stale-or-not-found) carry reworded meanings covering a virtual id; remedies and the cause set are unchanged.
@task_item n=5 part=touches
Refusal order rows (decision/eight-refusal-causes-in-order, contract/driver-session/refusal-order, contract/refusal-causes/cause-set): a click on a virtual id is refused in the same one order, now read by a test; no cause added.
@task_item n=6 part=touches
Graph rows (contract/driver-verbs/graph, contract/driver-session/graph, contract/session-run/graph, contract/session-wire/graph-answer): the graph skips every virtual node though the snapshot it reads holds them.
@task_item n=7 part=touches
Actions and actionable rows (contract/driver-verbs/actions, decision/one-actionable-rule, decision/actions-answer-holds-no-value): actions and exploration read the graph and so hold no virtual node.
@task_item n=8 part=touches
Lint rows (contract/lint-defects/overlap and the other lint defects, contract/lint-findings/order): the lint holds no virtual node by construction, its entries having no element; no skip in code; a stated limit added.
@task_item n=9 part=touches
Click and the other id verbs (contract/driver-session/acting-verbs, contract/session-run/type, contract/session-run/scroll, contract/session-run/screenshot, contract/session-layout/changes-nothing, contract/escher-session-command-line/canvas): click accepts a virtual id; type, scroll, screenshot with an id, layout and canvas read it not-found.
@task_item n=10 part=touches
Help text rows (contract/mcp-server/help, contract/cli-help/output, contract/mcp-help/parity, contract/escher-session-command-line/help): click's help text changed, the same fixed text on every surface.
@task_item n=11 part=touches
Snapshot and diff rows (contract/driver-session/snapshot, contract/session-run/diff, contract/mcp-snapshot/parity, contract/cli-snapshot/budget, decision/snapshot-text-budget): the snapshot text and diff now carry each virtual node's id and declared name.
@task_item n=12 part=touches
Kept check and MCP rows (contract/session-recording/check-file, decision/recorded-checks-replayed-as-tests, decision/replay-through-the-call-path, contract/mcp-role-stdout/protocol-lines): a kept check holds a virtual click and replays it; the MCP role carries the virtual id and name.
@task_item n=13 part=touches
decision/a11y-leg-membership and decision/joining-file-keeps-its-legs: the a11y leg gains the new target, its file count nineteen to twenty.
@task_item n=14 part=touches
Capability statement rows (decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-check-names-held-to-schema, decision/capability-statement-from-schema, decision/capability-claims-name-their-checks): the statement's leg count words and integer pin moved to twenty, the reserved form's not-built line dropped, five stated limits added.
@task_item n=15 part=touches
Log silence rows (decision/no-ids-or-names-in-stand-logs, contract/escher-session-stderr/no-ids-names-or-typed-text, decision/session-logs-no-screen-content, decision/driver-log-fixed-words-only): the act-spans check now also reads that no virtual id or declared name reaches any log line; its child call count 43 to 46.
@task_item n=16 part=touches
decision/mutation-controlled-tests (tests controlled red-before-green): control V2 did not hold; its inert guard was dropped and the fact pinned by a behaviour reading, so the every-control-reads-red acceptance is unmet for V2.
@task_item n=17 part=touches
Target and both-mode rows (decision/one-test-target-per-behaviour, decision/blitz-tests-one-target-per-file, decision/one-test-target-per-file, decision/stand-checks-in-both-layout-modes, decision/layout-tests-in-both-layout-modes): a new target, each case run in both layout modes.
@task_item n=18 part=touches
Shared fixture rows (decision/shared-fixture-pages, decision/shared-fixtures-in-one-module, decision/test-data-in-code): a virtual canvas fixture page with its ids and names joined the shared module.
@task_item n=19 part=touches
Unix gating rows (decision/recording-checks-unix-gated, decision/socket-session-checks-unix-only, decision/socket-checks-unix-gated): the kept-check virtual click case is unix-only, compiled out on windows (8 of 9 cases there).
@task_item n=20 part=touches
decision/table-tests-count-their-rows and decision/expected-value-pinned-as-literal: the schema help pin and the refusal meaning pin are literal pins that moved with their texts.
@task_item n=21 part=touches
decision/count-and-rename-sweeps: a leg-count change also has a literal integer pin beside the number-word map, and a help or refusal text pin may hold the whole text or a prefix.
@task_fact n=1 slug=integration-test-strategy
fact: The lint holds no virtual node by construction (its entry has no element, so no visible part to pair), with no skip in code; the fact is pinned by a behaviour reading that the graph, actions and lint answers on the declaring fixture hold no owner///key id in both layout modes.
@task_fact n=2 slug=integration-test-strategy
fact: click is the one id verb that takes a virtual node, acting at its canvas region and focusing the canvas, refused disabled, off-screen, covered, not-found or stale in the one order; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
@task_fact n=3 slug=unit-test-strategy
fact: A snapshot's virtual node reads its id, declared role and name, enabled from its declared disabled, clickable false, highlighted false, no value, checked, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
@task_fact n=4 slug=test-strategy
fact: The accessibility tree holds each virtual node as a child of its canvas's tree node, after its own children, with author id owner///key, declared role and name and disabled where declared, under an even-versioned tree id equal to no DOM id; a hidden, collapsed or inert canvas, or one whose read errs, holds none.
@task_fact n=5 slug=session-lifecycle
fact: A virtual node's id and declared name cross the MCP role and a kept check, and the kept check replays its virtual click; the socket and command line carry the same run line by construction but are not read on the binary.
@task_fact n=6 slug=unit-test-strategy
fact: A guard no input can turn red is not kept; where its control reads it inert, the guard is dropped and the fact it guarded is pinned by a behaviour reading instead.
@task_item n=1 part=untouched
WPT conformance: the runner, its CLI, statuses, expectations and fork reading.
@task_item n=2 part=untouched
The agent-run and cold-agent contracts: verbs, selections, events, exit codes, verdicts and prompts.
@task_item n=3 part=untouched
Mocking and stand-ins, the virtual clock and advance, timer ticks and load stand-ins.
@task_item n=4 part=untouched
The session wire, protocol versions, state directory, idle expiry, session errors and exit codes: none added or moved.
@task_item n=5 part=untouched
Harness construction, settle, contrast and screenshot helpers, and pixel or shot comparison rules.
@task_item n=6 part=untouched
CI workflow security and caching: action pinning, secrets, permissions, caches, package installs, MSRV, clippy and audit.
@task_item n=7 part=untouched
Coverage floors, mutation tooling and complexity ceilings: no threshold moved.
@task_item n=8 part=untouched
Handler coverage, password masking and typed-password rows.
@task_item n=9 part=untouched
Dependencies and crates: none added or removed.
