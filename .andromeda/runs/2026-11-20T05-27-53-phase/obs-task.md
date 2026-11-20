@task_prose part=chunk
Brings our own Rust files over the code audit's size bar (800 code lines) and our own functions over its complexity ceiling (cognitive complexity 15) under those bars, by moves and extractions that change no behaviour and no public API.
First the escher-driver JSON answer tests split into one test child per JSON answer, then the session snapshot file and the driver's command file, then the driver's CLI read, the MCP role's run loop and five stand and probe test functions.
Re-measures every tracked Rust file at HEAD by the audit's own recipe (file code lines, function cognitive complexity) and records each upstream-authored file and function over a bar for the founder with its measurement, left as it is.
Restates the duplication watch on upstream's cdp-server and vibey-script test pairs for the founder; no upstream-authored file is edited.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs mark=edits
what: the escher-driver's tests of its JSON answers (the command answer's encoding per answer kind) — edits: split into one test child per JSON answer, the shape the JSON encoder's own split took
@task_file n=2 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot of the served document — snapshot nodes, their text, clickable, select values and open list row bounds that the graph, actions and layout answers read
@task_file n=3 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command handling — the verbs, the command answer and the one command span per driver command
@task_file n=4 path=packages/escher-driver/src/cli.rs mark=edits
what: escher-session's command-line roles and their reading of a command — edits: the function read
@task_file n=5 path=packages/escher-driver/src/mcp.rs mark=edits
what: escher-session's mcp role — JSON-RPC lines on stdout and the tool_call span around each hosted call — edits: the function run
@task_file n=6 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the runtime probe check over the dioxus runtime's reads — edits: the test function visit
@task_file n=7 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's graph checks over the graph answer's nodes and states — edits: the test functions every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form and every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run
@task_file n=8 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's check of the actions answer over each served screen — edits: the test function each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand's accessibility name and id checks over interactive role elements — edits: the test function every_interactive_role_element_reads_a_name
@task_item n=1 part=needs
Restructure invariant: the JSON answer tests, the snapshot code, the driver's command handling, the CLI read, the mcp role's run loop and the five stand and probe test functions are moved or extracted and each does what it did — the command answer and its encodings, the snapshot and the answers read from it, the command span per driver command, the tool_call span with its tool and ending fields and command child, the mcp role's stdout and exit status, the coverage role's line — held by the existing checks (the driver's JSON answer tests, the stand checks, the runtime probe, the span and log-line checks), every check staying green.
@task_item n=2 part=needs
No new log site, span, span field, event, verb, answer field or carrier: an extracted helper of the command handling or the mcp run loop opens no span of its own and emits no event, the command span stays one per driver command and the tool_call span keeps its two closed fields.
@task_item n=3 part=needs
An extracted helper inside the driver's command or tool_call path must keep call content out of the spans — no typed text, answer, decoded screen text or page content as a span field or log value — and must keep the driver library installing no subscriber.
@task_item n=4 part=needs
Answer-bearing types (the command answer, the graph, actions and layout answers, the coverage line) stay unlogged in moved code: no Debug print or log of them is introduced by a split or an extraction.
@task_item n=5 part=needs
Moved test code and new shared test helpers stay silent checks: they print nothing on success, read no value they leak, and a check reading an empty stderr keeps unsetting RUST_LOG.
@task_item n=6 part=needs
The runtime probe's extracted visit helper keeps leaking no read value: whatever it reads from the dioxus runtime stays inside the assertion.
@task_item n=7 part=needs
The mcp role's run loop, once extracted, keeps the hosted call running through the session's run alone and keeps escher-session's fixed-string diagnostics and exit status codes unchanged.
@task_item n=8 part=needs
The measure the chunk is read by: tokei code lines per file (bar 800) and rust-code-analysis cognitive complexity per function space, closures included (ceiling 15), by the 0.3.0 audit's own summarizer, re-measured at HEAD.
@task_item n=9 part=needs
Authorship decides the class: a file standing in upstream/main at the merge base is upstream-authored and is not edited for size or complexity; each such file and function over a bar is recorded with its measurement for the founder.
@task_standing n=1
convention: every new test file or shared test module is a silent check — it prints nothing when it passes and logs no answer, page content or typed text
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Metric coverage and metric emission or exporters: the chunk adds no counter, gauge, timing or exporter.
@task_item n=2 part=not_needed
Error capture and reporting: script error surfaces, window error events, script diagnostics drain, panic hooks and WPT reports belong to the engine and upstream code the chunk leaves as it is.
@task_item n=3 part=not_needed
Telemetry strategy rows on engine tracing behind a cargo feature and escher crates taking tracing: the chunk changes no dependency or feature of any crate.
@task_item n=4 part=not_needed
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs, kept checks and transcripts are harness code the chunk does not reach.
@task_item n=5 part=not_needed
escher's telemetry sink and its log-line format, scrub set, resource keys and target allowlist: the sink's code is not moved or split by this chunk.
@task_item n=6 part=not_needed
CI integration rows on artifacts, the build-directory sweep line, the package-install diagnostic, session recording files, the recording directory and the session lifecycle: the chunk adds no CI job, artifact or recording behaviour.
@task_item n=7 part=not_needed
Select, optgroup, page script event and form entry rows under PII scrubbing: they rule engine behaviour of select lists and page events that the chunk leaves as it is.
