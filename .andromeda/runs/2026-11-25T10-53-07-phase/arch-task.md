@task_prose part=chunk
Makes a canvas's taken virtual nodes (id {owner}///{key}) read by every driver read that today holds DOM elements alone: the UI graph, the actions answer, handler coverage, the layout explanation and exploration.
The UI graph gains one graph node per virtual snapshot node, with its owner; actions lists a virtual node with the verbs it takes and the refusal cause each would meet.
Handler coverage reads a virtual node's reach where a kept check acts on it; the layout verb answers for a virtual id; exploration takes a virtual node as a candidate it acts on.
Settles the two provisional limits of virtual nodes as elements it owns (only click takes a virtual id, others reading not-found; a virtual node's clickable reads false) and lifts the exclusion of virtual nodes from graph, actions, lint and exploration.
Sweeps the literal pins with the words: counts, help texts, the self-description, the verb table's length, the capability statement's counts and the guards asserting no virtual id.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph reader of the Dioxus DOM bridge, one graph node per snapshot node with its owner — edits: the exclusion of virtual nodes from the graph
@task_file n=2 path=packages/escher-driver/src/actions.rs mark=edits
what: the actions verb's answer, each actionable node with the verbs it takes and the refusal cause each would meet
@task_file n=3 path=packages/escher-driver/src/lint.rs mark=edits
what: the lint verb's defect reading over the snapshot — edits: the exclusion of virtual nodes from the lint
@task_file n=4 path=packages/escher-driver/src/explore.rs mark=edits
what: the driver's exploration, the candidates it acts on and its explored verbs
@task_file n=5 path=packages/escher-driver/src/coverage.rs mark=edits
what: the handler coverage reader behind the command line's coverage role, the reached handlers a kept check acts on
@task_file n=6 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's acting step and per-verb target check — edits: resolve, locate and check_target, the arm refusing a virtual id as not-found on every verb but click
@task_file n=7 path=packages/escher-driver/src/layout.rs mark=edits
what: the session's layout explanation, Session::layout answering why an element sits where it does from blitz-dom's layout reason
@task_file n=8 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot of the Dioxus DOM bridge, its nodes and node state — edits: clickable of a virtual node
@task_file n=9 path=packages/dioxus-native-dom/src/reached.rs mark=edits
what: the reached record of the Dioxus DOM bridge, the handlers a dispatch reached, resolved before dispatch and written after
@task_file n=10 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's one self-description for the command line and MCP — edits: its help texts and counts
@task_file n=11 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command and refusal schema and the verb table VERBS — edits: the literal pins on VERBS.len and help texts
@task_file n=12 path=docs/capabilities.md mark=edits
what: the capability statement, held to the driver schema — edits: its counts and their pin, the limits pinned by a check
@task_file n=13 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand UI graph check — edits: its guard asserting no virtual id in the graph
@task_file n=14 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand actions check — edits: its guard asserting no virtual id in the actions answer
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_declared.rs mark=edits
what: the virtual nodes declared check, a canvas declaring keyed, named, role-bearing regions — edits: its guard asserting no virtual id
@task_file n=16 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs mark=edits
what: the virtual nodes as elements check on a fixture canvas in both layout modes — edits: the_graph_actions_and_lint_hold_no_virtual_node
@task_item n=1 part=needs
The UI graph: one graph node per snapshot node, now including a virtual snapshot node with its owner; the carve-out that kept virtual nodes out of the graph is lifted, and the graph verb's node and answer carry the virtual id.
@task_item n=2 part=needs
The actions verb's answer: a virtual node listed with the verbs it takes and the refusal cause each would meet, its node fields and keyed reading, and the actions answer still changing nothing.
@task_item n=3 part=needs
Handler coverage and the coverage role: a virtual node's reach read where a kept check acts on it, through the reached record and kept-check replay, with canvas handlers and unreached screens read as today.
@task_item n=4 part=needs
The layout explanation: Session::layout and the layout verb give an answer for a virtual id, its answer JSON and the unexplained-placer reading, over blitz-dom's layout reason.
@task_item n=5 part=needs
Exploration: a virtual node is a candidate exploration acts on, within its explored verbs, step budget, path expansion and path ending; exploration stays off the driver's carriers.
@task_item n=6 part=needs
The per-verb target check and the refusal of a virtual id: only click takes a virtual id today and type, scroll, screenshot with an id, layout and canvas refuse it not-found; the chunk settles which verbs take it, inside the refusal causes closed at eight, with no new refusal cause.
@task_item n=7 part=needs
A virtual node's clickable reading in the snapshot node state, the driver snapshot and the bridge's clickable, now false, and the one actionable rule it is read against.
@task_item n=8 part=needs
The lint verb and a virtual node: the lint's defect set and findings under the exclusion of virtual nodes from graph, actions, lint and exploration that this chunk owns.
@task_item n=9 part=needs
The virtual node crossing: a virtual id or name added to a carrier is read against the ratified exits it already rides; no new carrier, verb, exit, session error or span field, and a new crossing class is a fork.
@task_item n=10 part=needs
Virtual node ids and declarations: the stable virtual_node_id {owner}///{key}, DioxusDocument::virtual_nodes, the virtual node refusal, the accessibility tree's virtual nodes and the snapshot's virtual node, read as declared; a declaration change marks and signals nothing.
@task_item n=11 part=needs
Literal pins swept with the words: the verb table VERBS and its length, the help verb and the help texts, the one self-description for command line and MCP, the MCP tool listing and its count, the capability statement's counts held to the schema, and the command line usage line.
@task_item n=12 part=needs
The guards asserting no virtual id in graph, actions and lint are rewritten to the new reading, each a control entered with its red reading on a guard some input can turn red.
@task_item n=13 part=needs
The proving surface is a fixture canvas a test declares, in both layout modes; the stand declares no virtual node, and the stand's graph and actions readings stay as they are.
@task_item n=14 part=needs
No NodeId crosses a session and the snapshot holds no process-local values: a virtual node reaches every read by its stable id alone.
@task_item n=15 part=needs
The validated verb set widens only by ratification, and the driver runs commands only after validation; the chunk adds no verb.
@task_item n=16 part=needs
The driver logs no content and opens spans only: no snapshot text, name or diff content of a virtual node enters a span or log line.
@task_item n=17 part=needs
Answers are written by the driver's own JSON writer with string escaping and no serde_json type in the driver's public API; a virtual node's fields in the graph, actions and layout answers follow it.
@task_item n=18 part=needs
Driver unit tests live in path children of their module and stay children when split; a new driver test reaches its unit through that placement.
@task_standing n=1
convention: one behavior per integration test file, one test target per file in blitz-tests, the fixture's condition asserted first, with descriptive test names
home: conventions
@task_standing n=2
convention: every scenario run in both layout modes
home: conventions
@task_standing n=3
convention: expect and unwrap justified, module and item docs, public item docs, rustdoc warnings as errors, fmt and clippy gates
home: conventions · infrastructure-patterns
@task_standing n=4
convention: per-crate line coverage floors and the cognitive complexity ceiling hold for new and changed code
home: stack-and-technologies
@task_item n=1 part=not_needed
The engine's style cascade, paint, text shaping and scrolling decisions: the chunk changes no engine behaviour.
@task_item n=2 part=not_needed
Form controls and the select list (select model, open list, options, range, password masking): no virtual node is a form control.
@task_item n=3 part=not_needed
Browser app, browser history, about pages, persistence and networking: outside the driver and the bridge.
@task_item n=4 part=not_needed
WPT runner, its outputs, statuses, CI artifacts and the PR body section: the chunk touches no WPT test.
@task_item n=5 part=not_needed
Script runtime and its globals, fetchers and Boa: no script reads a virtual node.
@task_item n=6 part=not_needed
Cold-agent and agent-run scripts, their events, exit codes, verdict fields and state directories: the chunk runs no cold-agent run.
@task_item n=7 part=not_needed
Session socket wire versions and bounds, session host process and idle expiry, and recording directory mechanics: no new carrier, wire field or session error is added.
@task_item n=8 part=not_needed
Telemetry bootstrap, its line format, scrubbing and target filter: no new span field or log line.
@task_item n=9 part=not_needed
Keyboard reach of a canvas, pan and zoom of a rotated or scaled canvas, and Circle Drawer's circles as virtual nodes with their binary-level reading: other entries.
@task_item n=10 part=not_needed
CI workflow pins, package install scripts, the Nix flake, Skia archive checks and upstream-only workflows: the chunk adds no CI job.
