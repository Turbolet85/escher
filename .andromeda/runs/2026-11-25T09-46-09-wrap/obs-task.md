@task_prose part=session
The session served a canvas's declared virtual nodes as elements: each taken virtual node now stands in the accessibility tree under its canvas (declared role, declared name, author id owner///key, disabled where declared), and from the tree in the snapshot and its diff.
A snapshot virtual node reads id, role, name, enabled from the declared state, clickable false, highlighted false, bounds its region moved by the canvas's content-box origin; a hidden, collapsed or inert canvas takes none.
The click verb takes a listed virtual id and acts on its canvas, refused disabled, off-screen, covered, not-found or stale in the one order; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found; the graph, actions, lint and exploration hold no virtual node.
The snapshot text, diff, MCP results and kept checks now carry a virtual node's id and author-declared name (a boundary widening); no log site, span or span field was added, and no virtual id or declared name reaches any log line at info.
The virtual-node declaration gained a disabled flag; the click help, the clickable key's help and the meanings of the covered and not-found causes were reworded; the a11y CI leg grew from nineteen to twenty files and the act-spans check's child call count from 43 to 46.
An inert lint skip was written and dropped on the operator's ruling; nothing was retired and no dependency, verb, cause, result field, exit or log field was added.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget contract, holding the virtual-node declaration a canvas makes (key, name, role, region, now disabled)
@task_file n=2 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the declared-node read of a canvas's virtual nodes, now also placing them on the page and giving each its accessibility tree id
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the document's accessibility tree, now holding each canvas's virtual nodes under the canvas node
@task_file n=4 path=packages/dioxus-native-dom/src/snapshot.rs
what: the snapshot and its nodes (id, role, name, enabled, clickable, bounds) that the snapshot text, diff and every answer read
@task_file n=5 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph answer, which skips every virtual node
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id and its reserved virtual-node id form, whose docs now say the form is served
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver's command execution and target resolution, where click takes a virtual id and the other id verbs read it not-found
@task_file n=8 path=packages/escher-driver/src/lint.rs
what: the lint answer and its stated limits, now saying a canvas's virtual nodes are not linted
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: the verb and node-key help texts, where the click help and the clickable key's help changed
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the unit check pinning each verb's help, arguments and result
@task_file n=11 path=packages/escher-driver/src/refusal.rs
what: the refusal cause names and their meanings, where covered and not-found were reworded
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs
what: the new integration check that virtual nodes stand in the tree, snapshot, click, refusals, MCP role and kept checks, and not in the graph, actions or lint
@task_file n=13 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration check of the in-process declared-node read
@task_file n=14 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the check that the driver's command spans hold no call content and no virtual id or declared name on any log line
@task_file n=15 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the test fixtures, now with a canvas page declaring five virtual nodes
@task_file n=16 path=.github/scripts/ci-leg.sh
what: the CI leg runner, whose a11y leg now runs twenty files
@task_file n=17 path=.github/scripts/test_ci_workflows.py
what: the check pinning each CI leg's file list
@task_file n=18 path=.github/scripts/test_capability_statement.py
what: the check pinning the capability statement's leg counts in words
@task_file n=19 path=docs/capabilities.md
what: the capability statement, updated by hand: the virtual-node id form now produced, its not-built list and stated limits
@task_item n=1 part=touches
decision/declared-node-read-logs-nothing widens: the accessibility tree, the snapshot and a click on a virtual node add no log site, span or field, and no virtual id or declared name is on any log line.
@task_item n=2 part=touches
contract/snapshot-node/clickable and contract/snapshot-text/clickable: a virtual node reads clickable false while click still takes it.
@task_item n=3 part=touches
contract/refusal-cause/covered and contract/refusal-cause/not-found: their meanings were reworded; a virtual node reads covered only by an element over its region, and every id verb but click reads a virtual id not-found.
@task_item n=4 part=touches
contract/graph-answer/content and contract/graph-answer/shape: the graph answer excepts virtual nodes, which the snapshot now holds.
@task_item n=5 part=touches
contract/actions-answer/content and contract/actions-answer/shape: the actions answer holds no virtual node; its resolve reads a virtual id not-found.
@task_item n=6 part=touches
contract/command-answer/graphed and contract/command-answer/linted: graphed nodes and lint findings hold no virtual id.
@task_item n=7 part=touches
contract/layout-answer/content and decision/layout-read-opens-no-span, decision/canvas-read-opens-no-span: layout and canvas now refuse a virtual id not-found, opening no new span.
@task_item n=8 part=touches
contract/kept-check/answer, contract/kept-check/arguments and contract/kept-check/answer-digest: a kept check now holds a virtual id as a click argument and the declared name in its answer, and replays.
@task_item n=9 part=touches
decision/committed-kept-checks-clean and decision/committed-checks-hold-no-private-data: a kept check may now hold author-declared virtual-node names.
@task_item n=10 part=touches
contract/mcp-role-stdout/json-rpc and contract/escher-session-stdout/command-answer: the MCP results and command answers carry a virtual node's id and declared name.
@task_item n=11 part=touches
decision/session-logs-carry-no-page-content and decision/driver-spans-carry-no-content, decision/command-span-no-call-content: declared names now cross into answers but stay off logs and spans; the click span carries verb and cause only.
@task_item n=12 part=touches
contract/recording-file/answer: a recorded answer may now hold a virtual node's id and declared name.
@task_item n=13 part=touches
contract/snapshot-diff/open-list-row-bounds and the snapshot's bounds rows: a virtual node's bounds are its region moved by its canvas's content-box origin, and the diff names a virtual node's changes.
@task_item n=14 part=touches
decision/exploration-adds-no-span and decision/graph-answer-never-logged: exploration's candidates read the graph and so hold no virtual node.
@task_item n=15 part=touches
decision/capability-statement-updated-by-hand: the statement now says the reserved virtual-node form is produced, drops it from not built and gains five stated limits; the a11y leg count moved nineteen to twenty.
@task_item n=16 part=touches
contract/command-span/snapshot-part and contract/command-span/verb: the act-spans check now reads a snapshot, a click and a refused layout on the declaring fixture, 46 child calls in all.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: A canvas's virtual node crosses into the snapshot text, its diff, MCP results and kept checks as its owner///key id and its author-declared name, and onto no log line or span field.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: A virtual node in the snapshot reads enabled from its declared state, clickable false and highlighted false, with no checked, value, selected or pressed and no children.
@task_fact n=3 slug=span-trace-coverage
fact: click is the one verb that takes a virtual id, refused disabled, off-screen, covered, not-found or stale in the one order; type, scroll, screenshot with an id, layout and canvas read it not-found.
@task_fact n=4 slug=log-coverage
fact: The graph, actions, lint and exploration hold no virtual node; the lint excludes them because a virtual entry has no element, with no skip in code.
@task_item n=1 part=untouched
Telemetry backends, OTel export, metric emission and metrics exporters: nothing was added or removed.
@task_item n=2 part=untouched
escher's telemetry sink and its log line: format, stream, RUST_LOG, allowlist, scrub set, service name and version, panic hook are unchanged.
@task_item n=3 part=untouched
The MCP tool_call span, its tool and ending fields, and the command span's field set: no field, span or ending was added.
@task_item n=4 part=untouched
The cold-agent and agent-run harnesses: their events, verdict fields, counted verbs, transcripts and cleanup are unchanged.
@task_item n=5 part=untouched
Script error capture, the window error event, WPT reports and the WASM panic hook: not reached.
@task_item n=6 part=untouched
Session state and recording directory privacy, recording bounds, idle expiry, the host wire, exit codes and the build-directory sweep line: unchanged.
@task_item n=7 part=untouched
Engine tracing behind its cargo feature and escher crates' ungated tracing: no dependency or feature changed.
