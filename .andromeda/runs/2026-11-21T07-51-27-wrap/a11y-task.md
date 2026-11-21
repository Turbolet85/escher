@task_prose part=session
The driver's press verb gained an optional ctrl flag, held as the host's action key (Meta on macOS, Control elsewhere), so a driven Ctrl+arrow then Space builds a set of rows that are not adjacent on a focused multiple select, in process, over MCP, on the command line and in a kept check.
The graph verb gained an optional part argument (1 to 1000): a graph over the 10,000-byte snapshot budget is answered in parts of whole nodes, the states in part 1 alone, a later part's empty states meaning read part 1, a part past the last empty; the whole-graph answer stays for a small screen.
The reached-handler record behind handler coverage now resolves the listening chain before the dispatch and writes after it: an unstopped bubbling dispatch counts every listener, one stopped by its target or an ancestor counts the innermost listener alone, a non-bubbling event its target alone, and a handler's panic unwinding out of the dispatch is written as stopped (a drop guard); the public reached record surface is unchanged.
Measured: on a focused multiple select, plain arrows choose the row and a closing Space flips it out, so the plain sequence ends with no row chosen; a Cells cell's click opens its editor unfocused, so a key reaches the editor only after a click on the editor input.
Retired: the stated limits that press carries no Ctrl, that the graph is unpaged and that a stopPropagation is not read, from the capability statement and the verb help; the capability statement's pinned counts unchanged, no verb, cause, node key, log field or span field added.
@task_file n=1 path=packages/escher-driver/src/schema.rs
what: the driver schema, the verb table with each verb's arguments, fields and help (press and graph rows)
@task_file n=2 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's unit checks of verb arguments, fields and help text
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver's command form, the verbs a session carries and their parsed arguments
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's verb execution against the harness: press with its modifiers and the graph answer cut into parts
@task_file n=5 path=packages/escher-driver/src/json.rs
what: the driver's JSON writer of answers, the UI graph answer and its written lengths
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the driver JSON writer's unit checks, the graph part answer form
@task_file n=7 path=packages/escher-driver/src/cli_tests.rs
what: the driver command line's unit checks, the verb flags and usage lines
@task_file n=8 path=packages/escher-driver/src/mcp_tests.rs
what: the driver's MCP tool listing checks, the press and graph tools' input schema
@task_file n=9 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture checks building commands
@task_file n=10 path=packages/escher-driver/src/recording_tests.rs
what: the kept check recording round trip checks
@task_file n=11 path=packages/escher-driver/src/coverage.rs
what: handler coverage, the coverage answer read from the reached-handler record (its module doc on stopped dispatches)
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs
what: the Dioxus document reached-handler record, resolved before the dispatch and written after it, with the stop read and the panic drop guard
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus document bridge dispatching engine events to the vdom, keeping the target node for the reached record
@task_file n=14 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand's UI graph checks, Cells' graph read in parts within the budget
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand's select acting checks, the multiple select set of non-adjacent rows built with ctrl
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the stand's handler coverage checks, the Cells grid keydown left unexercised under the editor's stop
@task_file n=17 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand's MCP tool checks, a graph part and a press with ctrl round trip and the non-adjacent row set
@task_file n=18 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand's kept check replay checks, a kept graph part step
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand's command span checks, the driven calls per mode and a graph part's span recording the verb alone
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand session test helpers (press with ctrl, a graph part, reading a paged graph whole)
@task_file n=21 path=examples/seven_guis/tests/cli_commands.rs
what: the seven_guis command line checks of the session binary, graph parts, press ctrl and coverage through the command line
@task_file n=22 path=docs/capabilities.md
what: the capability statement, its stated limits (Ctrl on press, the unpaged graph, the stop not read) and its accessibility reach
@task_item n=1 part=touches
The Dioxus document reached-handler record (record_reached, reached_handlers): its public surface is unchanged but what it records moved: resolved before the handlers run, written after the dispatch, a stopped dispatch counting the innermost listener alone, a non-bubbling event its target alone, a panicking handler's dispatch written as stopped.
@task_item n=2 part=touches
The handler coverage answer: a listener above a stop is now reported unexercised (the Cells grid's keydown under the editor's Enter stop), where before every ancestor listener on the chain counted as reached.
@task_item n=3 part=touches
Listeners read from the vdom walk: the reached record now resolves the chain's elements whose current listeners hold the event before the dispatch, by node and id; a rule over which listeners count reached is touched.
@task_item n=4 part=touches
The multiple select keys and row clicks contract: the driver can now drive the host action key with arrows (press with ctrl), building a set of non-adjacent rows; measured, plain arrows choose a row and a closing Space flips that row out, ending with no row chosen.
@task_item n=5 part=touches
Keyboard activation by Space and Enter: on a focused multiple select a Space toggles the current row, measured through the driver, including flipping a just-chosen row out.
@task_item n=6 part=touches
The seven_guis Cells grid editor Enter and Enter contracts: measured, a cell click opens the editor unfocused, so Enter reaches the editor only after a click on the editor input; the editor's Enter stop now leaves the grid's keydown unexercised in coverage.
@task_item n=7 part=touches
UI graph rows (the graph mirrors the snapshot one node per node, read-only and carrying no values): the graph answer may now come in parts of whole nodes at the 10,000-byte budget, part and parts fields added, the nodes of all parts together the whole graph.
@task_item n=8 part=touches
UI graph state registration rows (use_graph_state, undeclared, unconfirmed, no registered state): the states now travel in part 1 alone of a paged graph, a later part's empty states meaning read part 1.
@task_item n=9 part=touches
The driver schema defines no role or name set: the verb table gained two argument rows (press ctrl, graph part) and still defines no role or name set.
@task_item n=10 part=touches
Tab and Shift+Tab move focus: press's modifiers grew from shift alone to shift and ctrl; the engine's existing Ctrl and Meta arms are what the ctrl flag reaches, no engine keyboard path changed.
@task_item n=11 part=touches
The actions verb changes nothing and the actions answer: actions lists press's ctrl and graph's part from the schema rows; its answer stays unpaged and its causes unchanged.
@task_item n=12 part=touches
MCP tool results and listing: the press tool lists ctrl as a boolean and the graph tool an optional integer part 1 to 1000; the snapshot result unchanged.
@task_item n=13 part=touches
Capability statement reach edited by hand: the capability statement was edited (three limits narrowed or removed) while its accessibility reach and pinned counts stayed unchanged.
@task_item n=14 part=touches
Stand checks repeated three rounds: the stand repeat ran three rounds green on CI after the new MCP set check, not locally re-run by the operator's word.
@task_fact n=1 slug=keyboard-navigation
fact: The reached-handler record counts a dispatch stopped by its target or an ancestor as reaching the innermost listener alone, a non-bubbling event its target alone, and writes a dispatch a handler's panic unwinds out of as stopped; its listener ids resolve before the handlers run.
@task_fact n=2 slug=keyboard-navigation
fact: The driver's press carries an optional ctrl flag held as the host's action key (Meta on macOS, Control elsewhere), so driven Ctrl+arrows then Space build a non-adjacent set of rows on a focused multiple select.
@task_fact n=3 slug=5-keyboard-navigation
fact: A Cells cell's click opens its editor unfocused, so a key reaches the editor only after a click on the editor input; the grid's keydown stays unexercised under the editor's Enter stop.
@task_fact n=4 slug=screen-reader-support
fact: The UI graph answer is paged at the 10,000-byte snapshot budget into parts of whole nodes, part 1 alone carrying the states and a later part's empty states meaning read part 1; a graph within the budget with no part asked is answered whole.
@task_item n=1 part=untouched
Accessibility tree construction, AccessKit adapter, platform adapter and role mapping rows: no accessibility tree node, role, name or state reading changed.
@task_item n=2 part=untouched
ARIA property reads and accessibility tree states (aria-checked, aria-selected, aria-expanded and the rest): nothing read from ARIA attributes moved.
@task_item n=3 part=untouched
Contrast, visual lint, colour pairs, scrollbar, motion and hidpi rows: no visual or styling rule changed.
@task_item n=4 part=untouched
Snapshot node fields, snapshot text form, snapshot diff and select list rows in the snapshot: the snapshot answer and its paging are unchanged; only the graph gained parts.
@task_item n=5 part=untouched
Single-choice select list open and closed key contracts, typeahead, optgroup and range input keys: no select list or range input key behaviour changed.
@task_item n=6 part=untouched
Focusability, inert, hidden content, disabled-ness and focus-visible rules: no focus or hit-testing rule moved.
@task_item n=7 part=untouched
Password masking, value carriage and the keyed wire form: no value or PII path changed, no log field, span field or scrub set changed.
@task_item n=8 part=untouched
The a11y CI leg, accessibility checks gate and test leg tiers: no CI step, leg or script changed.
