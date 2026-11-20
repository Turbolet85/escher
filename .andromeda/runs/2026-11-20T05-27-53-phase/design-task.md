@task_prose part=chunk
Brings our own Rust files over the code audit's size bar (800 code lines) and our own functions over its complexity ceiling (cognitive complexity 15) under those bars, by moves and extractions that change no behaviour and no public API.
First the driver's JSON answer tests split into one test child per answer, the shape the JSON answers module already took; then the native DOM snapshot and the driver's command file, then seven functions over the ceiling (the command line reader, the mcp role's run loop, five stand and probe test functions).
Every upstream-authored file and function over a bar is measured and put to the founder in this run's founder forks, and left as it is; the upstream duplicated test pairs watch rides the same way.
Re-measures both bars over every tracked Rust file at HEAD, and re-reads CI for the last wrap's commit; seven of our test files over 800 are left for a later entry.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs mark=edits
what: the unit tests of the driver's JSON answers (actions, caused, diff, graph, layout, lint, snapshot) — edits: split into one test child per answer
@task_file n=2 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's JSON answers module, the parent that declares its test module — edits: its test child declarations
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the dioxus-native DOM snapshot — the snapshot and accessibility tree the driver serves
@task_file n=4 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's escher-session commands — their arguments and answers
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line role — its arguments, stdout, stderr usage error and exit status — edits: the function read
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session mcp role — its tool results, Err ending and stdout — edits: the function run
@task_file n=7 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the dioxus runtime probe check — edits: the test function visit
@task_file n=8 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph checks — edits: the test functions every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form and every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run
@task_file n=9 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actions line checks — edits: the test function each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs
@task_file n=10 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand's accessibility name checks — edits: the test function every_interactive_role_element_reads_a_name
@task_item n=1 part=needs
The restructure's invariant: the driver's JSON answer tests, the native DOM snapshot, the driver's commands, the command line role's read, the mcp role's run and the five stand and probe test functions are moved or extracted and each answers as before — the escher-session commands' arguments and answers, its stdout, stderr and exit statuses, the mcp role's tool results and Err ending, the snapshot and accessibility tree, the actions line and UI graph line — held by the existing checks, every one staying green.
@task_item n=2 part=needs
The measure the chunk is read by: a file is oversize past 800 code lines and a function is over the ceiling past cognitive complexity 15, every function space counted, closures included; whether a plan row states either bar is to be checked.
@task_item n=3 part=needs
Where a moved or extracted unit may live and what it may expose: a split file's parts become private child modules of the module they leave, adding no public item, no new verb, field, carrier, log site or span.
@task_item n=4 part=needs
How a test reaches a moved unit: the JSON answer tests stand as test children beside each answer's own child module, declared from their parent as the existing test modules are, reaching only what they reached before.
@task_item n=5 part=needs
An extraction from the command line reader or the mcp role's run loop keeps every caller's answer, the exit status read from the schema's table and the mcp role's stdout and Err ending as they are; the output stays read by an agent, with no colour added.
@task_item n=6 part=needs
An extraction from a stand or probe test function keeps what that test asserts and its one integration-test target; no target is added, removed or renamed.
@task_item n=7 part=needs
No upstream-authored file is edited for size or complexity: each of the 15 upstream files over 800 and 94 upstream functions over the ceiling is recorded in this run's founder forks with its measurement and the least-change option, nothing written presuming the founder's answer.
@task_item n=8 part=needs
The upstream duplicated test pairs watch is restated in this run's founder forks with its reading, unanswered and untouched.
@task_item n=9 part=needs
The re-measurement of both bars over every tracked Rust file at HEAD, written as the run's evidence, and the CI verdict for the last wrap's commit read again, never read as green while in progress.
@task_item n=10 part=needs
What the chunk leaves is named: our seven test files over 800 stay for a later entry, their integration-test target layout untouched.
@task_standing n=1
convention: every test of the engine is built with system fonts, as the existing tests are
home: typography
@task_item n=1 part=not_needed
Colour rows — engine colours, select list colours, colour scheme recascade, colour tokens: the chunk paints nothing and changes no colour.
@task_item n=2 part=not_needed
Spacing and depth rows — presentational spacing hints, select list placement, z-index paint levels, hit-testing at paint clips: no layout, paint or hit-test code is reached.
@task_item n=3 part=not_needed
Motion rows — the virtual clock, scroll instantness, overlay scrollbar fades, smooth scroll: no timing behaviour is reached.
@task_item n=4 part=not_needed
Iconography rows — cursor icons, cursor resolution, favicon: not reached.
@task_item n=5 part=not_needed
The shell provider's window contracts (maximize, minimize, decorations, drag), the JS window object's sizes and the dioxus-native config's alpha mode and base colour: no window or script runtime code is reached.
@task_item n=6 part=not_needed
WPT runner output and result colour contracts: the runner is not reached.
@task_item n=7 part=not_needed
The web-spa surface's WASM canvas pages and the WASM bundled font context: not reached.
@task_item n=8 part=not_needed
Brand identity and the empty anti-pattern, self-validation and decisions log sections: they hold no rows.
