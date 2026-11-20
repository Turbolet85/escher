@task_prose part=chunk
Brings our own Rust files over the code audit's size bar (800 code lines) and our own functions over its complexity ceiling (cognitive complexity 15) under those bars by moves and extractions that change no behaviour and no public API.
First the driver's JSON answer tests split into one test child per JSON answer module, then the snapshot module and the driver command module, then the driver CLI reader, the MCP server loop and five stand and probe test functions.
Every upstream-authored oversize file and over-ceiling function is measured and put to the founder in this run's founder-forks list, with the duplicated upstream test pairs watch restated there; none is edited.
Re-measures sizes and complexity over every tracked Rust file at HEAD, and re-reads the CI verdict of the last wrap's commit.
Leaves the seven test files of ours over 800 lines (stand requirement sweep, select popup, stand keyboard, act error text, MCP tools, select interaction, seven_guis CLI commands) for a later entry.
@task_file n=1 path=packages/escher-driver/src/json_tests.rs mark=edits
what: the tests of the driver's JSON answers (snapshot, snapshot diff, actions answer, handler coverage, refusals) — edits: split into one test child per JSON answer module
@task_file n=2 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot model read from the accessibility tree — snapshot nodes and their value, checked, enabled, clickable and select list row readings
@task_file n=3 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's commands — click, type, screenshot, actions and their refusal causes
@task_file n=4 path=packages/escher-driver/src/cli.rs mark=edits
what: the driver's command-line front end — edits: the function `read`
@task_file n=5 path=packages/escher-driver/src/mcp.rs mark=edits
what: the driver's MCP server and its tool results — edits: the function `run`
@task_file n=6 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the Dioxus runtime probe check over the rendered tree and its reached handlers — edits: the test function `visit`
@task_file n=7 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand's UI graph checks — one graph node per snapshot node, served ids never holding the reserved separator, each stand task's state declarations agreeing with its code — edits: the test functions `every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form` and `every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run`
@task_file n=8 path=tests/blitz-tests/tests/stand_actions.rs mark=edits
what: the stand's actionable-nodes check — each served screen lists its actionable nodes with the expectation table's verbs — edits: the test function `each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs`
@task_file n=9 path=tests/blitz-tests/tests/stand_accessibility_ids.rs mark=edits
what: the stand's accessibility id and name checks — every interactive role element reads a name — edits: the test function `every_interactive_role_element_reads_a_name`
@task_file n=10 path=.andromeda/runs/2026-11-20T05-27-53-phase/founder-forks.md mark=edits
what: the run's list of questions left to the founder — edits: one entry per upstream-authored file over 800 lines and function over the ceiling with its measurement and least-change option, and the duplicated upstream test pairs watch
@task_item n=1 part=needs
The restructure's invariant: the driver's JSON answer tests, the snapshot model's readers, the driver commands, the CLI reader `read`, the MCP server loop `run` and the five stand and probe test functions are moved or have parts extracted, and each keeps what it does now — every snapshot, snapshot diff, actions, handler coverage and MCP tool answer, every refusal cause, every stand check's verdict; the existing driver unit tests, the stand checks and the probe check, all still green, hold it.
@task_item n=2 part=needs
The measure the chunk is read by: a file is oversize past 800 code lines (tokei code count) and a function over the ceiling past cognitive complexity 15 (rust-code-analysis, every function space, closures included), re-measured at HEAD over every tracked Rust file into the run's evidence.
@task_item n=3 part=needs
No public API change and no new verb, field, carrier, log site or span: a moved or extracted unit stays private to its crate or module, the keyed field on the carriers and the driver's schema unchanged.
@task_item n=4 part=needs
The moved snapshot readers stay behind the accessibility feature, as every snapshot reader is; a split module of the snapshot keeps the same feature gate.
@task_item n=5 part=needs
How a test reaches a moved unit: each split JSON answer test lives as a test-only child of its JSON answer module, reaching that module's private items as a child does, with no item widened for a test.
@task_item n=6 part=needs
A helper extracted from a stand test function reads the stand's expectation tables (roles, names, verbs, tab order) where they are stated once; no extraction restates an expected role, name or verb.
@task_item n=7 part=needs
An extraction from a stand or probe test stays inside its own integration-test target and the shared stand test setup; no new integration-test target, so the stand checks' three-round repeat leg and the CI's target pin read the same targets.
@task_item n=8 part=needs
No upstream-authored file (one standing in upstream main at the merge base) is edited for size or complexity; each goes to the founder-forks list with its measurement, and the plan leaves it as it is.
@task_item n=9 part=needs
The duplicated upstream test pairs watch (the cdp-server and vibey-script tests) rides as the upstream class does: restated in the founder-forks list, nothing written presuming the founder's answer.
@task_item n=10 part=needs
The CI verdict of the last wrap's commit, in progress at take-up, is read again and never taken as green until it reads so.
@task_item n=11 part=needs
The seven test files of ours over 800 lines left by this chunk are named as a later entry on the review card.
@task_standing n=1
convention: accessibility assertions are Rust tests over the AccessKit tree, run in the test suite
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: stand checks run three rounds in the slow-tier accessibility test leg, which gates CI
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Contrast, colour, lint contrast finding, scrollbar thumb and select list colour rows: no reached code computes or asserts colour.
@task_item n=2 part=not_needed
Range input, single-choice select key and pointer, multiple select and keyboard activation contracts: engine behaviour in upstream-authored code this chunk does not edit.
@task_item n=3 part=not_needed
Platform adapter, accesskit_xplat, accessibility tier and reach decisions: no adapter or feature default is changed.
@task_item n=4 part=not_needed
ARIA role mapping, ARIA property reads, accessibility tree states and hidden content rows: built by the engine's document, which the chunk leaves as it is.
@task_item n=5 part=not_needed
Focus movement, focusability, inertness and Tab order decisions: engine focus behaviour is not reached.
@task_item n=6 part=not_needed
seven_guis Cells grid and CRUD list contracts: the seven_guis app and its tests are left for a later entry.
@task_item n=7 part=not_needed
Cognitive accessibility rows (invalid state not by colour or class alone): no form state rendering is touched.
