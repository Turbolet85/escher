@task_prose part=chunk
Makes a canvas's taken virtual nodes (id {owner}///{key}) read by every driver read that today holds DOM elements alone: the UI graph (in process and the graph verb), the actions answer, handler coverage, the layout explanation and exploration.
Takes up the two stated limits it owns: only click takes a virtual id while type, scroll, screenshot with an id, layout and canvas read it not-found (F10 B), and a virtual node's clickable reads false (F12 A); and lifts the exclusion of virtual nodes from the graph, actions, the lint and exploration (F9 A).
Sweeps the literal pins with the words — counts, help texts, the describe() self-description, VERBS.len(), the capability statement's counts — and the guards asserting no virtual node; adds no carrier, verb, exit, session error or span field, and a new refusal cause or crossing class is a fork for the founder.
@task_file n=1 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph builder, one graph node per snapshot node — edits: the exclusion of virtual nodes (F9 A), one graph node per virtual snapshot node with its owner
@task_file n=2 path=packages/escher-driver/src/actions.rs mark=edits
what: the actions read, listing each element with the verbs it takes and the refusal cause each would meet — edits: a virtual node's listing
@task_file n=3 path=packages/escher-driver/src/lint.rs mark=edits
what: the lint read of the page's findings — edits: the exclusion of virtual nodes
@task_file n=4 path=packages/escher-driver/src/explore.rs mark=edits
what: auto-exploration, choosing candidates to act on — edits: a virtual node as a candidate
@task_file n=5 path=packages/escher-driver/src/coverage.rs mark=edits
what: handler coverage (the coverage role), the reach a kept check's actions read — edits: a virtual node's reach
@task_file n=6 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command execution, target resolution and refusal — edits: resolve, locate, check_target, the not-found arm for a virtual id (F10 B)
@task_file n=7 path=packages/escher-driver/src/session.rs mark=edits
what: the driver session — edits: Session::layout, the layout read's answer for a virtual id
@task_file n=8 path=packages/blitz-dom/src/layout_reason.rs mark=edits
what: the engine's layout explanation read (layout_reason) — edits: an answer for a virtual id
@task_file n=9 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot and its nodes — edits: a virtual node's clickable (F12 A)
@task_file n=10 path=packages/dioxus-native-dom/src/reached.rs mark=edits
what: the reached record, the ids and event names a run reached
@task_file n=11 path=docs/capabilities.md mark=edits
what: the capability statement, updated by hand — edits: its counts and their pin, the stated limits pinned by a check
@task_file n=12 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph check — edits: its guard asserting no virtual id
@task_file n=13 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actions check — edits: its guard asserting no virtual id
@task_file n=14 path=tests/blitz-tests/tests/virtual_nodes_declared.rs mark=edits
what: the declared virtual nodes check — edits: its guard asserting no virtual id
@task_file n=15 path=tests/blitz-tests/tests/virtual_nodes_as_elements.rs mark=edits
what: the virtual nodes as elements check, on a fixture canvas a test declares in both layout modes — edits: the_graph_actions_and_lint_hold_no_virtual_node
@task_item n=1 part=needs
The UI graph answer gains one node per virtual snapshot node with its owner: the graph answer's shape and content, and the rule of one graph node per snapshot node with its virtual-node carve-out lifted.
@task_item n=2 part=needs
The actions answer lists a virtual node with the verbs it takes and the cause each would be refused with: the actions answer's shape, cause, owner and content.
@task_item n=3 part=needs
Handler coverage reads a virtual node's reach where a kept check acts on it: the coverage line's shape, owner and content, and the reached record that holds ids and event names only.
@task_item n=4 part=needs
The layout read gives an answer for a virtual id where today it reads not-found: the layout answer's content and logging, and the layout read opening no span.
@task_item n=5 part=needs
Exploration takes a virtual node as a candidate it acts on: exploration adds no span of its own and catches a panicking call.
@task_item n=6 part=needs
The refusal causes a virtual id meets — not-found for the verbs that do not take it (F10 B), and those click already meets — stay inside the ratified refusal-cause set; a new cause is a fork, not an addition.
@task_item n=7 part=needs
A virtual node's clickable in the snapshot node and the snapshot text (F12 A, provisional) is decided by this chunk; any change is a change to that contract.
@task_item n=8 part=needs
A virtual node's name and id cross only on today's exits (founder F3 A): the virtual-node crossing rule, under which the graph, actions answer, lint, canvas answer and exploration today hold none — each carrier that now takes one is read against it.
@task_item n=9 part=needs
The answers that now hold virtual names are still never logged: the graph answer, the actions answer, the coverage line and the layout answer stay out of every log and span; driver spans carry no content and the command span adds no field.
@task_item n=10 part=needs
No new verb, carrier, exit, session error or span field: the command span's per-verb rows (graph, actions, layout, lint, canvas) and its field count, the escher-session exit codes and error object stay as they are.
@task_item n=11 part=needs
The actions read still changes nothing in the document, and a declaration change marks and signals nothing (founder F11 A).
@task_item n=12 part=needs
Literal pins swept with the words: the escher-session commands' help text, the tool_call help, the describe() self-description and VERBS.len(), and the capability statement's counts updated by hand.
@task_item n=13 part=needs
The declared-node read still logs nothing; the canvas read opens no span.
@task_item n=14 part=needs
Each fork the research finds is written into the founder-forks evidence with its crossing, rule, measured costs, dependences, owner and most conservative option; no raw log carrying a temp or home path enters evidence.
@task_standing n=1
convention: each new test is silent — it prints nothing on success and reads empty stderr with RUST_LOG unset
home: 3-observability-harness-contract
@task_standing n=2
convention: each new log or span assertion that fails names a kind and a count, never the content it read
home: 3-observability-harness-contract
@task_standing n=3
convention: each new fixture, committed check or evidence file holds no private data, no secret and no raw log with a temp or home path
home: ci-integration · 8-pii-scrubbing-compliance
@task_item n=1 part=not_needed
Telemetry backends, OTel export, metric emission and exporters, and the sink's resource keys: the chunk adds no telemetry channel.
@task_item n=2 part=not_needed
Script error reporting — window error events, onerror arguments, script error source labels, WPT report messages and the panic hook: the chunk touches no script error path.
@task_item n=3 part=not_needed
The agent-run and cold-agent harness rows — their events, verdict fields, counted verbs, transcripts and cleanup: the chunk changes no harness and no counted verb.
@task_item n=4 part=not_needed
Select, optgroup, multiple-select form entries, open-list shots and page script change events: the chunk reaches no form control.
@task_item n=5 part=not_needed
Session recording files, the recording and state directories, idle expiry and the build-directory sweep line: the chunk changes no session lifecycle or recording.
@task_item n=6 part=not_needed
The telemetry sink's install order, target allowlist, line format and escaping, and engine tracing feature gating: the chunk adds no log site and changes no sink.
@task_item n=7 part=not_needed
WPT score publishing, the package-install diagnostic and CI artifact uploads: the chunk changes no workflow.
