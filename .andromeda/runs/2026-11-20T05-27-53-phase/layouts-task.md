@task_prose part=chunk
Brings our own Rust files over the code audit's 800-code-line bar and our own functions over its cognitive-complexity ceiling of 15 under them, by moves and extractions that change no behaviour and no public API.
First the driver's JSON answer tests split into one test child per answer, then the stand snapshot builder and the driver's command module, then the escher-session read and MCP run functions and five stand test functions.
Every upstream-authored file and function over a bar is measured and put to the founder in this run's founder-forks record, left as it is; the duplication watch rides the same way.
Re-measures the whole tree at HEAD by the audit's recipe, and re-reads the CI verdict of the last wrap's commit.
Leaves the seven oversize stand and seven_guis integration-test files of ours for a later entry.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs mark=edits
what: the driver's tests of its JSON answers (snapshot, graph, actions, layout, lint and the rest), one file over the bar — edits: split into one test child per answer, as the answer encoders took the json module shape
@task_file n=2 path=packages/escher-driver/src/json mark=edits
what: the driver's JSON answer encoders, one module per answer, which gain a test child each — edits: the per-answer test children moved out of the JSON answer tests
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the stand document's snapshot reading that the escher-session snapshot answer is built from, one file over the bar
@task_file n=4 path=packages/escher-driver/src/command.rs mark=edits
what: the escher-session command line and its verbs, one file over the bar
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line reading of its arguments and roles — edits: the function read, over the complexity ceiling
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session MCP server over stdio, its tools and its end — edits: the function run, over the complexity ceiling
@task_file n=7 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the stand's runtime probe check over the dioxus document — edits: the test function visit
@task_file n=8 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand check of the escher-session graph answer and the served tasks' declarations — edits: the test functions every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form and every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run
@task_file n=9 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand check of the escher-session actions answer for each served screen — edits: the test function each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs
@task_file n=10 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand check that stand names, roles and states and stable element ids stand as attributes — edits: the test function every_interactive_role_element_reads_a_name
@task_item n=1 part=needs
The restructure's invariant: the JSON answer tests, the snapshot reading, the command line and its verbs, the escher-session argument reading, the MCP server's run loop and the five stand test functions are moved, split or extracted and each does exactly what it did — every answer, refusal, exit code and tool result unchanged — held by the existing checks: the driver's JSON answer tests, the stand checks of graph, actions, accessibility and ids and the runtime probe, the MCP server checks, all staying green.
@task_item n=2 part=needs
The measure the chunk is read by: a file is oversize past 800 code lines (tokei code count) and a function over the ceiling past cognitive complexity 15 (rust-code-analysis, every function space, closures included), by the 0.3.0 audit's own summarizer; each class-one file and function must read under its bar after the change.
@task_item n=3 part=needs
No public API change: a moved or extracted unit stays private to its crate or module (crate-private at most), and no item the driver or the dioxus-native-dom crate exports changes name, signature or path.
@task_item n=4 part=needs
No new verb, field, carrier, log site or span: the escher-session verbs, answers, MCP tools list, protocol revisions, result bound, exit codes and host log service name stay the same set.
@task_item n=5 part=needs
How a test reaches a moved unit: the split JSON answer tests live as a test child of each answer's own module under the json module, reaching that answer's encoder through the parent module and compiled only under test.
@task_item n=6 part=needs
An extracted helper of a stand test function stays inside that test's own integration-test target (or the shared stand test setup it already uses), so the integration-test target set the CI pin and the stand-repeat leg name is unchanged.
@task_item n=7 part=needs
No upstream-authored file is edited: a file standing in upstream/main at the merge base (the seven_guis stand's first files included) is left as it is; each of the 15 upstream files over 800 (document.rs 3,470 first) and the 94 upstream functions over the ceiling goes to this run's founder-forks record with its measurement and the least-change option standing.
@task_item n=8 part=needs
The duplication watch (3.74 % by the audit's jscpd recipe, upstream cdp-server and vibey-script tests 58 pairs / 611 lines) is restated in founder-forks with nothing written presuming the founder's answer; no upstream test pair is touched.
@task_item n=9 part=needs
Re-measures every tracked Rust file at HEAD by the audit recipe (551 files: 25 over 800, 101 functions over the ceiling) into the run's measurement evidence, and measures again after the change.
@task_item n=10 part=needs
Re-reads the CI verdict of the last wrap's commit, which was in progress at take-up and is never read as green until it reads so.
@task_item n=11 part=needs
The seven oversize integration-test files of ours (requirement sweep, select popup, keyboard, act error text, MCP tools, select interaction, seven_guis cli commands) are left whole and named as the later entry on the P5 card.
@task_standing n=1
convention: a split or extracted check keeps the same test names and asserts the same escher-session answers, refusals and exit codes it did before the move
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): the chunk touches no window chrome.
@task_item n=2 part=not_needed
The cold-agent run pipe and agent-run test contracts (their usage, exit codes, stdout, record dirs, Windows pass-through) and the cold-agent decisions: no script of those pipes is reached.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines (their arguments, environment variables, backend, stdout): benchmark tools outside the chunk.
@task_item n=4 part=not_needed
Stand page design decisions — the fixed document skeleton and main mount, arbitrary index.html templates, stand accent colour, order page built to cause, load page alone having a network provider, flight type select boot reading, CRUD list keyboard region, pinned headless viewport: the chunk changes no stand page and no stand task code.
@task_item n=5 part=not_needed
Scrolling and visible-region decisions (into-view scroll moving every scroller, visible region through clipping boxes, open list rows' own bounds): those rule over upstream-authored layout and DOM code this chunk leaves untouched.
@task_item n=6 part=not_needed
The mobile-native and web-spa surfaces: unmeasured, and nothing of this chunk reaches them.
