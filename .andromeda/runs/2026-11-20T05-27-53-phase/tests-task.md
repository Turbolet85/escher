@task_prose part=chunk
Brings our own Rust files over the audit's size bar (800 code lines) and our own functions over its cognitive-complexity ceiling (15) under them, by moves and extractions that change no behaviour and no public API.
First the driver's JSON answer tests split into one test child per JSON answer module; then the snapshot serializer and the driver command layer split; then the command-line read, the MCP server run loop and five stand/probe test functions extracted below the ceiling.
Re-measures the whole tree at HEAD by the audit's recipe (551 files, 25 over 800, 101 functions over the ceiling) and reads again the CI run on the last wrap's commit.
Every upstream-authored file and function over a bar is measured and put to the founder in this run's founder-forks record, left as it is; the upstream duplicated test pairs watch is restated there, untouched.
Seven of our integration-test files over 800 are left for a later entry, since splitting them touches the one-target-per-file layout.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs mark=edits
what: the driver's JSON answer unit tests, one module of tests over every JSON answer — edits: split into one test child per JSON answer module
@task_file n=2 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot serializer of the Dioxus document — the snapshot node tree the driver answers with
@task_file n=3 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command layer — the verbs, their argument kinds and the call each runs
@task_file n=4 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line — reading the command form and its arguments — edits: the function read
@task_file n=5 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session MCP role — the MCP server's request loop over stdio — edits: the function run
@task_file n=6 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the Dioxus runtime probe check, walking the runtime's vdom — edits: the test function visit
@task_file n=7 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph checks — served ids against snapshot nodes and declared writes against task code — edits: the test functions every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form and every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run
@task_file n=8 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actions checks — each served screen's actionable nodes with the verb table's verbs — edits: the test function each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand's accessibility checks — every interactive role element reads a name — edits: the test function every_interactive_role_element_reads_a_name
@task_item n=1 part=needs
The restructure's invariant: the JSON answer tests, the snapshot serializer's parts, the command layer's parts, the command-line read, the MCP run loop and the five stand/probe test functions move or are extracted and each does exactly what it did — same answers, same refusals, same assertions; the existing workspace tests, stand checks in both layout modes and the stand-repeat rounds hold it, every check staying green.
@task_item n=2 part=needs
The measure the chunk is read by: a file is oversize past 800 code lines, a function over the ceiling past cognitive complexity 15 over every function space closures included; the cognitive-complexity ceiling is read on the host, not in CI, and the chunk's own files and functions must each read under it after the change.
@task_item n=3 part=needs
Re-measurement of the tree at HEAD by the audit's recipe — file count, files over the size bar, functions over the ceiling, split upstream-authored versus ours by presence at the upstream merge base — written as this run's evidence.
@task_item n=4 part=needs
No public API change: a moved or extracted unit may expose no more than it did; tests reach a moved unit without widening its visibility, as the bridge tests do from a child module.
@task_item n=5 part=needs
No new verb, field, carrier, log site or span: the driver's log line, command span and tool_call span keep their sites as they are, and an extracted helper adds no logging of its own.
@task_item n=6 part=needs
Extractions inside stand and probe test files stay inside the one-target-per-file layout of blitz-tests: helpers live in the same file or in the shared session/stand modules that are no target; no new test target, so the CI pin on blitz-tests targets and the stand-repeat leg's named targets stay as they are.
@task_item n=7 part=needs
Upstream-authored files are not edited for size or complexity: each upstream file and function over a bar goes to the founder record with its measurement and the least-change option; clippy over the workspace and upstream blitz-dom must still pass with nothing upstream touched.
@task_item n=8 part=needs
The upstream duplicated test pairs watch (cdp-server and vibey-script tests) is restated for the founder and left as written, nothing presuming his answer.
@task_item n=9 part=needs
Per-crate line coverage floors still hold after code moves between files of the driver and dioxus-native-dom crates; the coverage leg gated by the floors stays green.
@task_item n=10 part=needs
A doc comment on a moved public item moves with it unchanged; the doc leg gates any public doc comment edit.
@task_item n=11 part=needs
Green read per the workspace, not per package: every moved part is proved by the whole workspace test run and the fast CI leg, never by one package's green alone.
@task_item n=12 part=needs
The CI run on the last wrap's commit is read again, its verdict never assumed green while in progress.
@task_standing n=1
convention: a test split out of a source file stays an inline cfg(test) child module of the source it tests, unit tests sitting in their source file
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: a split test module keeps its tests' paths, and moved table-driven tests keep asserting their row count, their failures printing no argument value
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=3
convention: Rust tests stay on the built-in harness with shared fixtures and stand tables in one shared module
home: 3-test-harness-contract
@task_item n=1 part=not_needed
WPT conformance runner, its CLI, outputs, status set and pinned commit: the chunk touches no WPT run.
@task_item n=2 part=not_needed
Cold-agent pipe and agent-run harness rows — their events, verbs, exit codes, verdict and isolation: no change reaches them.
@task_item n=3 part=not_needed
Session wire protocol, session host lifecycle, session state directory and session recording rows: their code is not moved by this chunk.
@task_item n=4 part=not_needed
Mocking and stubbing discipline and the virtual clock, timer ticks and advance rows: no stand-in or time driving changes.
@task_item n=5 part=not_needed
Harness contrast, lint defects and screenshot pixel rows: no reader or painter is touched.
@task_item n=6 part=not_needed
Mutation testing rows: no mutation baseline is read or run by this chunk; the browser's surviving mutants are another entry.
@task_item n=7 part=not_needed
Windowed end-to-end absence and real MCP client operator pass: nothing launches a window or a live client.
@task_item n=8 part=not_needed
CI action pinning, secrets, permissions, cache and apt-install script rows: the chunk adds or edits no CI job or workflow.
