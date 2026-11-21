@task_prose part=session
The driver's press verb gained an optional ctrl flag, held as the host's action key (Meta on macOS, Control elsewhere), so a press with ctrl moves the current row of a multiple select and a press of space flips it; a set of rows that are not adjacent can now be built.
The driver's graph verb gained an optional part argument (1 to 1000): a graph whose written answer is over the 10,000-byte snapshot text budget, or any graph read with a part, is cut into parts of whole nodes, the states in part 1 alone, a later part's empty states meaning read part 1, a part past the last empty; the answer for a part carries the graph keys plus part and parts.
Handler coverage now reads a stopped dispatch: the reached record resolves the listening chain before the dispatch and writes after it, all listeners when a bubbling dispatch still propagates, the innermost alone when it was stopped, the target alone for a non-bubbling event; a handler's panic unwinding out of the dispatch is written as stopped by a drop guard.
Retired the stated limits that press holds no Ctrl, that the graph answer is unpaged, and that a stopPropagation is not read, in the capability statement, the press help and the reached and coverage module docs.
Measured: a Cells cell click opens its editor unfocused, so a key needs a click on the cell input first; plain arrows then space on a multiple select ends with no row chosen; a kept check's arguments read back in name order.
No verb, refusal cause, session error, node key, exit status, crate, dependency, log field, span field or CI step added; the verb table still holds fifteen verbs.
@task_file n=1 path=packages/escher-driver/src/schema.rs
what: edited — the driver verb table and its argument and field rows (press gains ctrl as a flag, graph gains part as a part argument and the part and parts fields, their help text), the one self-description the command line, MCP listing and kept-check reader read
@task_file n=2 path=packages/escher-driver/src/schema_tests.rs
what: edited — unit checks of the driver verb table: argument counts per verb, the graph's paging help and press's action-key help
@task_file n=3 path=packages/escher-driver/src/command.rs
what: edited — the driver's validated command enum and call parsing (Press gains ctrl, Graph becomes Graph with an optional part)
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: edited — the driver's acting step and outcomes: press joins the platform action modifier when ctrl, graph answers whole or cuts into parts within the snapshot text budget, the new GraphPart outcome
@task_file n=5 path=packages/escher-driver/src/json.rs
what: edited — the driver's own hand-written JSON answer writer: a graph part's answer form and the measured written lengths of a graph's nodes and states
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: edited — unit checks of the driver's JSON answer writer, now the graph part form
@task_file n=7 path=packages/escher-driver/src/cli_tests.rs
what: edited — unit checks of the escher-session command line's verb flags and usage line (--ctrl, graph --part)
@task_file n=8 path=packages/escher-driver/src/mcp_tests.rs
what: edited — unit checks of the MCP role's tool listing (press ctrl a boolean, graph part an integer 1 to 1000)
@task_file n=9 path=packages/escher-driver/src/fixture_tests.rs
what: edited — shared driver unit-test fixtures building commands
@task_file n=10 path=packages/escher-driver/src/recording_tests.rs
what: edited — unit checks of the kept check file and its strict reader (press with ctrl and graph with part kept and read back)
@task_file n=11 path=packages/escher-driver/src/coverage.rs
what: edited — the handler coverage reader's module docs, the stated limit on a stopped dispatch narrowed
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs
what: edited — the reached record of the Dioxus bridge: which handlers a dispatch reached, now resolved before the dispatch and written after it with the stop read, and a drop guard that writes a panicking dispatch as stopped
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited — the Dioxus DOM bridge's event dispatch to the vdom (handle_event registry dispatch by data-dioxus-id), now keeping the target node and finishing the reached write after the dispatch
@task_file n=14 path=tests/blitz-tests/tests/stand_graph.rs
what: edited — the stand check of the graph verb on the seven_guis stand: Cells read in parts within the budget, together the whole graph
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs
what: edited — the stand check of acting on selects: a press with ctrl builds a set of rows that are not adjacent
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: edited — the stand check of handler coverage: a keydown the Cells editor stops leaves the grid's keydown unexercised
@task_file n=17 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: edited — the stand check of the MCP role's tool calls: a graph part and a press with ctrl round trip
@task_file n=18 path=tests/blitz-tests/tests/stand_replay.rs
what: edited — the stand check of kept check replay: a kept graph part step replays and a changed one stops
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the stand check that the command span records the verb and no argument (driven calls per mode 40 to 41)
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — shared stand-check helpers in a non-target module (press_ctrl, graph_part, a paged graph read whole)
@task_file n=21 path=examples/seven_guis/tests/cli_commands.rs
what: edited — the seven_guis binary checks of the escher-session command line and its coverage role
@task_file n=22 path=docs/capabilities.md
what: edited — the capability statement: its stated measured limits (press Ctrl, unpaged graph, stopped dispatch) narrowed or removed
@task_item n=1 part=touches
The press verb's contract (driver verbs press, escher-session command line press --key [--shift]) now carries an optional ctrl flag, the host's action key; a row saying press carries no Ctrl is stale.
@task_item n=2 part=touches
The command line's verb flags: --ctrl is a bare flag beside --shift; the usage line reads press with [--ctrl] and graph [--part <part>].
@task_item n=3 part=touches
The graph verb's paging row (the graph verb is unpaged) is broken: graph now crosses in parts when over the snapshot text budget or when a part is asked.
@task_item n=4 part=touches
The graph verb's arguments and argument rows (the graph verb, the driver graph, driver verbs graph, escher-session graph): graph now takes part, 1 to 1000, and its result gains the part and parts fields.
@task_item n=5 part=touches
The graph verb's answer form: a part answers the graph keys plus part and parts, the states in part 1 alone, a later part's empty states meaning read part 1, a part past the last empty nodes and states; a small graph with no part asked still answers whole.
@task_item n=6 part=touches
The argument kind part now serves a snapshot part or a graph part, bounded by MAX_PART 1000.
@task_item n=7 part=touches
The decision that a screen over the snapshot text budget crosses in parts now holds for the graph answer too, at the same 10,000-byte budget.
@task_item n=8 part=touches
The decisions that the reached record is written before the dispatch or before the handlers run: it is now resolved before the dispatch and written after it, reading whether the dispatch was stopped.
@task_item n=9 part=touches
The handler coverage reader's bubbling dispatch: an unstopped bubbling dispatch counts every listener, one an ancestor or the target stops counts the innermost listener alone, a non-bubbling event its target alone.
@task_item n=10 part=touches
The Dioxus DOM bridge's reached record and its handle_event registry dispatch: the bridge keeps the target node it found the data-dioxus-id on and finishes the write after the vdom dispatch; the public reached record surface (record_reached, reached_handlers, take_step_reached) is unchanged.
@task_item n=11 part=touches
The platform action modifier and shortcut modifier conventions: the driver's press with ctrl now uses the same action modifier, Meta on macOS and Control elsewhere.
@task_item n=12 part=touches
The multiple select key model (keys choose alone, span or move): the driver can now reach the move-with-the-action-key and space-flips path, and measured plain arrows then space flip the chosen row out.
@task_item n=13 part=touches
The command span's graph call: a graph part read records verb alone, no part or count.
@task_item n=14 part=touches
The MCP role's tool listing and the one self-description for command line and MCP: press lists ctrl as a boolean, graph lists part as an integer 1 to 1000, taken from the verb table rows.
@task_item n=15 part=touches
The kept check file's arguments and kept verbs: a kept check may now hold press with ctrl and graph with part, its format unchanged.
@task_item n=16 part=touches
The capability statement and the rule that measured limits are stated where an agent reads the tool: the Ctrl, unpaged-graph and stopPropagation limits are retired and a narrower coverage limit stated; the statement's pinned counts unchanged.
@task_item n=17 part=touches
The answer JSON outcome row and the driver's hand-written answer writer: a new GraphPart outcome beside the snapshot part; the whole graph outcome stays.
@task_fact n=1 slug=standard-contracts
fact: A handler's panic that unwinds out of the vdom dispatch is recorded in the reached record as a stopped dispatch, the innermost listener alone, through a guard dropped on unwind.
@task_fact n=2 slug=standard-contracts
fact: A graph part's states ride in part 1 alone, and a later part's empty states means read part 1; a part past the last answers empty nodes and states.
@task_fact n=3 slug=standard-contracts
fact: A Cells cell click opens its editor unfocused, so a key reaches the editor only after a click on the cell input.
@task_item n=1 part=untouched
Layout, style, paint, text, scrolling and hit testing in the engine: no engine path changed.
@task_item n=2 part=untouched
The select list engine model and its default actions: no engine keyboard path changed, the existing Ctrl and Meta arms were reached as they stood.
@task_item n=3 part=untouched
The session socket wire, its version and reply bounds, session errors, refusal causes and faults, and the exit statuses: none added or changed.
@task_item n=4 part=untouched
Node keys, snapshot text and the snapshot diff: no node key or state token added.
@task_item n=5 part=untouched
Telemetry bootstrap, its scrub sets, line format and log fields: none changed.
@task_item n=6 part=untouched
CI legs, gates, coverage floors, the stand-repeat budget and the stack and dependency rows: no CI step, leg, script or dependency changed.
@task_item n=7 part=untouched
Occupied names (crates, binaries, servers, environment variables) and the absences of exploration, settle or fixture state on the carriers: no crate, binary or carrier added.
@task_item n=8 part=untouched
The UI graph registry, its writers, readers and graph state hooks: the graph's content is unchanged, only how its answer crosses.
@task_item n=9 part=untouched
Browser, WPT runner, script runtime and the browser history store: not reached.
