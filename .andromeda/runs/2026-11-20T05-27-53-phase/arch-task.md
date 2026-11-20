@task_prose part=chunk
Brings our own Rust files over the code audit's size bar (800 code lines) and our own functions over its cognitive complexity ceiling (15) under them, by moves and extractions that change no behaviour and no public API.
First the driver's JSON answer tests split into one test child per answer writer, the shape the JSON writer itself took; then the Dioxus bridge's snapshot reader and the driver's command module; then the command line's read, the MCP role's run and five stand and probe test functions.
Every upstream-authored file over either bar is measured and put to the founder, and left as it is; the duplication watch on upstream's test pairs rides the same way.
Re-measures both bars over every tracked Rust file at HEAD; leaves our seven integration-test targets over 800 lines for a later entry.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs mark=edits
what: the driver's own JSON writer's unit tests, a path child of the JSON writer — edits: split into one test child per answer writer
@task_file n=2 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's own JSON writer, hand-written answers, its answer writers as path children — edits: its test module declaration
@task_file n=3 path=packages/escher-driver/src/json/actions.rs mark=edits
what: the JSON writer's actions answer writer — edits: its test child declaration
@task_file n=4 path=packages/escher-driver/src/json/caused.rs mark=edits
what: the JSON writer's caused answer writer — edits: its test child declaration
@task_file n=5 path=packages/escher-driver/src/json/diff.rs mark=edits
what: the JSON writer's diff answer writer — edits: its test child declaration
@task_file n=6 path=packages/escher-driver/src/json/graph.rs mark=edits
what: the JSON writer's graph answer writer — edits: its test child declaration
@task_file n=7 path=packages/escher-driver/src/json/layout.rs mark=edits
what: the JSON writer's layout answer writer — edits: its test child declaration
@task_file n=8 path=packages/escher-driver/src/json/lint.rs mark=edits
what: the JSON writer's lint answer writer — edits: its test child declaration
@task_file n=9 path=packages/escher-driver/src/json/snapshot.rs mark=edits
what: the JSON writer's snapshot answer writer — edits: its test child declaration
@task_file n=10 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the Dioxus DOM bridge's screen reader — snapshot, its nodes and node state, text form and diff
@task_file n=11 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's commands — the validated verb set, call validation and its refusal schema
@task_file n=12 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line — its roles, verb flags and usage — edits: the function read
@task_file n=13 path=packages/escher-driver/src/mcp.rs mark=edits
what: the mcp role's MCP server over stdio — initialize, tools/list, tools/call — edits: the function run
@task_file n=14 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the Dioxus runtime probe integration test target — edits: the test function visit
@task_file n=15 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph check — edits: the test functions every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form and every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run
@task_file n=16 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actions verb check — edits: the test function each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs
@task_file n=17 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand's accessible names and stable ids check — edits: the test function every_interactive_role_element_reads_a_name
@task_item n=1 part=needs
The restructure's invariant: the JSON writer's answer tests, the bridge's snapshot reader, the driver's command validation, the command line's argument read, the MCP role's request loop and the five stand and probe test functions move or are extracted and each does exactly what it did — same answers, refusals, exit statuses, tool results and assertions; the existing driver unit tests, the blitz-tests stand checks and every CI leg hold it, every check staying green.
@task_item n=2 part=needs
The measure the chunk is read by: a file is oversize past 800 code lines and a function is over the cognitive complexity ceiling past 15, closures included; each touched file and function ends under its bar, and both bars are re-measured over every tracked Rust file at HEAD.
@task_item n=3 part=needs
No public API change: helpers extracted from our crates stay crate-internal or private, nothing new exported from the driver or dioxus-native-dom, and the driver's public API still exposes no serde_json type.
@task_item n=4 part=needs
No new verb, field, carrier, log site or span: an extracted function opens no span and emits no event of its own, and the command span and tool_call span stay where they are opened, with the same fields.
@task_item n=5 part=needs
Split unit-test modules stay path children of the module they test and keep their test paths, so a test moved out of the JSON writer's tests reaches the writer's private items as before.
@task_item n=6 part=needs
Unit-test placement: each answer writer's test child is declared at the end of that writer's source file, the shape the JSON writer's split already took.
@task_item n=7 part=needs
A helper extracted from a stand or probe test function lives in the file itself or in a shared non-target test module, never as a new test target; one test target per file in blitz-tests and the single test table of its manifest stay as they are.
@task_item n=8 part=needs
An extracted assertion helper in a test is a track-caller helper, so a failure still points at the calling test line.
@task_item n=9 part=needs
Scenarios that run in both layout modes keep running in both after their test function is broken up.
@task_item n=10 part=needs
Clippy allows stay local and none is added to dodge the complexity ceiling; the fmt and clippy gate and the rustdoc warnings gate pass on the moved code.
@task_item n=11 part=needs
expect and unwrap carried into moved or extracted code keep their justification.
@task_item n=12 part=needs
Upstream-authored files — those standing in upstream at the merge base — get no edit for size or complexity; each over a bar is recorded for the founder with its measurement and left as it is, upstream's duplicated test pairs likewise.
@task_item n=13 part=needs
The per-crate coverage floors hold for escher-driver and dioxus-native-dom after the moves.
@task_standing n=1
convention: each new module or test file opens with a module doc stating what it holds
home: conventions
@task_standing n=2
convention: each test keeps a descriptive name stating the behaviour it checks
home: conventions
@task_standing n=3
convention: section delimiter comments mark the sections of a split file
home: conventions
@task_standing n=4
convention: Rust code passes the formatting and clippy gate before it lands
home: conventions · infrastructure-patterns
@task_item n=1 part=not_needed
Engine layout, style, paint, scroll and hit-testing decisions and contracts: the engine's files are upstream-authored and left as they are.
@task_item n=2 part=not_needed
The WPT runner, its outputs, statuses and CI artifacts: nothing in the WPT path is touched.
@task_item n=3 part=not_needed
The browser app — history, about pages, persistence, toolbar, publish: no browser code is touched.
@task_item n=4 part=not_needed
The script runtime — Boa, script globals, script fetcher and vibey-script API: not touched.
@task_item n=5 part=not_needed
Networking — the net provider, per-host caps, HTTP cache: not touched.
@task_item n=6 part=not_needed
Cold-agent and agent-run scripts, their events, exit codes and verdict fields: not touched.
@task_item n=7 part=not_needed
Stack choices of new crates, toolchain pins and the Nix flake: no dependency or toolchain changes.
@task_item n=8 part=not_needed
Upstream sync rules — two-parent merge, merge base, ratification per take: this chunk takes nothing from another tree.
@task_item n=9 part=not_needed
The session socket's wire, the session host process model and the recording directory: none of their code is moved.
@task_item n=10 part=not_needed
Versioning, publishing and release decisions: no version or publish change.
