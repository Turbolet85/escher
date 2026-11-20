@task_prose part=chunk
Brings our own Rust files over the audit's size bar (800 code lines) and our own functions over its complexity ceiling (cognitive 15) under them, by moves and extractions that change no behaviour and no public API.
First the driver's JSON-answer test file splits into one test child per JSON answer; then the snapshot reader and the driver command file split, and the CLI argv reader, the MCP server loop and five stand-test functions are cut by extraction.
Every upstream-authored file and function over a bar is measured and put to the founder in this run's founder-forks record, left as it is; the duplication watch on upstream's test pairs rides there too.
Re-measures the bars at HEAD over every tracked Rust file, and reads the last commit's CI verdict again.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs mark=edits
what: the driver's tests of its JSON answers (the actions, graph, layout and refusal answers and their fields) — edits: split into one test child per JSON answer
@task_file n=2 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot reader — the snapshot text's id and name fields, its state words, control state readings and the masked value
@task_file n=3 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command — the closed driver verb schema, the single command validation gate, target refusals and the command span
@task_file n=4 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line — its closed argv reader over the escher-session flags and roles, exit codes and usage messages — edits: the function read
@task_file n=5 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP server over stdio — bounded request lines and tool results, protocol errors, the tool_call span — edits: the function run
@task_file n=6 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the check probing the Dioxus runtime's built nodes — edits: the test function visit
@task_file n=7 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand checks of the graph answer, the stable element id and the signal registry's declarations — edits: the test functions every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form and every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run
@task_file n=8 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand checks of the actions answer and its verbs — edits: the test function each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand checks of accessible names and stable ids on interactive role elements — edits: the test function every_interactive_role_element_reads_a_name
@task_item n=1 part=needs
The restructure's invariant: the JSON-answer tests, the snapshot reader, the driver command (verb schema, validation gate, refusals, command span), the CLI argv reader, the MCP server loop and the five stand-test functions move or are extracted and each keeps doing what it did — a split moves code and tests, an extraction keeps its callers' answers — held by every existing check staying green.
@task_item n=2 part=needs
The measure the chunk is read by: a file is oversize past 800 code lines and a function over the ceiling past cognitive complexity 15, closures included, by the audit's own summarizer, re-measured at HEAD over every tracked Rust file.
@task_item n=3 part=needs
No public API change: no new verb, answer field, carrier, log site or span; a moved or extracted unit stays crate-private, widening no visibility of the driver or the snapshot reader to reach it.
@task_item n=4 part=needs
A function extracted from the CLI argv reader stays inside its closed grammar: it echoes nothing the caller typed, never panics on argv, and keeps the fixed usage messages and exit codes.
@task_item n=5 part=needs
A function extracted from the MCP server loop keeps the stdio-only transport and the bounded request and result lines, never prints to stdout outside protocol lines nor panics, and keeps protocol errors fixed with no echoed data.
@task_item n=6 part=needs
Code moved out of the snapshot reader keeps the value mask on every carrier and keeps the NodeId and attribute text off the answers; no moved part logs a stable id, accessible name or screen content.
@task_item n=7 part=needs
Code moved out of the driver command keeps the one validation path for every carrier, the fixed order of target refusals and one content-free command span per call.
@task_item n=8 part=needs
The JSON-answer test children reach the moved answers through the driver crate's own test modules, the shape the JSON answer code already took, with no new exported item for tests.
@task_item n=9 part=needs
Each extracted stand-test helper keeps its check's assertions and stays inside its own test target, so each stand check stays one integration-test target and the CI target pin and the stand-repeat leg's named targets are unchanged.
@task_item n=10 part=needs
No upstream-authored file is edited for size or complexity: each upstream file and function over a bar goes to the run's founder-forks record with its measurement and the least-change option, left as it is; the upstream duplicated-test-pairs watch is restated there, nothing presuming the founder's answer.
@task_item n=11 part=needs
The last commit's CI run is read again and an in-progress run is never read as green.
@task_standing n=1
convention: a new test file or test module takes no tracing and no style dependency
home: dependency-security
@task_standing n=2
convention: a new test's helpers and assertions put no screen content, stable id or accessible name in a log line
home: logging-monitoring
@task_item n=1 part=not_needed
Secret management and CI secrets, signing material, publish jobs and release guards: the chunk touches no workflow, secret or release step.
@task_item n=2 part=not_needed
Data protection of the browser shell — history in sqlite, the cookie store, the on-disk HTTP cache, TLS, the clipboard and file dialog: no code of those is moved.
@task_item n=3 part=not_needed
Outbound fetching and navigation — script and sub-resource fetchers, per-host limits, file-URL and external-navigation rules, CORS and same-origin absences, request timeouts and response size limits: the chunk moves no net or navigation code.
@task_item n=4 part=not_needed
Cold-agent, agent-run, apt-install, coverage-floor and build-directory sweep scripts, and the WPT runner and its results workflow: the chunk edits no script or runner.
@task_item n=5 part=not_needed
Recording and kept-check files — the recording directory, recorded check reading and replay, kept-check lifecycle and contents: the chunk moves no recorder or replay code.
@task_item n=6 part=not_needed
The opt-in devtools CDP server and the Dioxus scheme and JS evaluation in the browser app: not reached by the chunk's moves.
@task_item n=7 part=not_needed
Authentication and authorization absences and the threat model summary: nothing the chunk does adds or removes a trust boundary.
