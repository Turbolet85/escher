@task_prose part=session
The driver's press verb gained an optional ctrl flag, held as the host's action key (Meta on macOS, Control elsewhere), so a press with ctrl moves the current row of a multiple select and press space flips it, building a set of rows that are not adjacent.
The driver's graph verb gained an optional part argument (1 to 1000): a graph whose answer exceeds the 10,000-byte snapshot text budget is cut into parts of whole nodes, the states carried in part 1 alone, a later part's empty states meaning read part 1, a part past the last answering empty nodes and states; a graph within the budget with no part asked still answers whole.
Handler coverage now reads a stopped dispatch: the reached record resolves the listening chain before the dispatch and writes after it, every listener when a bubbling dispatch still propagates, the innermost alone when stopped, the target alone for a non-bubbling event, and a handler's panic unwinding out of the dispatch is written as stopped through a drop guard.
Measured: a Cells cell's click opens its editor unfocused, so Enter reaches the editor only after a click on cell-input; plain arrows then press space on a focused multiple select end with no row chosen; Cells' graph at boot reads in more than one part.
Retired: the stated limits that press holds no Ctrl, that the graph answer is unpaged, and that a stopPropagation is not read by handler coverage.
Counts moved: driver argument rows across the verb table 11 to 13, press arguments 2 to 3, graph arguments 0 to 1 and result fields 2 to 4, the span check's driven calls per mode 40 to 41; no verb, refusal cause, session error, exit status, CI step or log field added.
@task_file n=1 path=packages/escher-driver/src/schema.rs
what: the driver verb table — the press and graph rows, their arguments, result fields and help, and the argument kinds with the part bound
@task_file n=2 path=packages/escher-driver/src/schema_tests.rs
what: the driver verb table's unit tests — argument and field counts per verb and the graph and press help
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver command form — the Press and Graph commands and their parsing from arguments
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver executor — press with the action modifier, the graph answer whole or cut into parts within the snapshot text budget, and the command span's field units
@task_file n=5 path=packages/escher-driver/src/json.rs
what: the driver JSON answers writer — the graph answer and a graph part's form with part and parts
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the driver JSON answers unit tests — a graph part's written form
@task_file n=7 path=packages/escher-driver/src/cli_tests.rs
what: the driver command line unit tests — the usage line and verb flags (--ctrl, --part)
@task_file n=8 path=packages/escher-driver/src/mcp_tests.rs
what: the MCP server unit tests — the tools/list input schema of the press and graph tools
@task_file n=9 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's shared unit test fixtures — command builders
@task_file n=10 path=packages/escher-driver/src/recording_tests.rs
what: the session recording unit tests — a kept check's round trip of press with ctrl and graph with part
@task_file n=11 path=packages/escher-driver/src/coverage.rs
what: the handler coverage reader — its module doc of the coverage limits
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs
what: the reached handler record — resolved before the dispatch, written after it with the stop read, a panic's unwind written as stopped
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the bridge — the dioxus document's dispatch to the vdom, which records the reached handlers
@task_file n=14 path=tests/blitz-tests/tests/stand_graph.rs
what: the UI graph stand check — Cells' graph read in parts within the budget, together the whole graph
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs
what: the select stand check — a press with ctrl building a set of rows that are not adjacent, both layout modes
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the handler coverage stand check — the grid's keydown unexercised under the cell editor's stop
@task_file n=17 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the MCP tools stand check — a graph part and a press with ctrl round-tripped over MCP
@task_file n=18 path=tests/blitz-tests/tests/stand_replay.rs
what: the check replay stand check — a kept graph part step replays and a changed one stops
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the command span stand check — driven calls per mode and the span fields of a graph part
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session check module — call builders and the graph reader that reads a paged graph whole
@task_file n=21 path=examples/seven_guis/tests/cli_commands.rs
what: the command-line checks run through the package's own binary — graph in parts, press with ctrl, the coverage role under the editor's stop
@task_file n=22 path=docs/capabilities.md
what: the capability statement pinned to the driver schema — its stated limits of press, graph and handler coverage
@task_item n=1 part=touches
The handler coverage limit on a bubbling dispatch is stale: a stopped dispatch is now read, counting the innermost listener alone (the target alone when the target stops it), a non-bubbling event its target alone, and only an unstopped bubbling dispatch every listener.
@task_item n=2 part=touches
The decision that a bubbling dispatch counts every ancestor listener is broken as stated: it holds only while the dispatch propagates.
@task_item n=3 part=touches
The graph command's argument contract moved: graph now takes an optional part, 1 to 1000, where it took none.
@task_item n=4 part=touches
The escher-session command line's graph item moved: graph [--part <part>], a part's answer the graph keys plus part and parts.
@task_item n=5 part=touches
The escher-session command line's command form moved: press now carries a bare --ctrl beside --shift.
@task_item n=6 part=touches
The driver argument kind Part now bounds a graph part as well as a snapshot part (1 to 1000).
@task_item n=7 part=touches
Driver verb contracts for graph (driver verbs, driver session graph, the escher_driver Session graph, the session wire's graph answer) gain the paged form: whole within the 10,000-byte budget, else parts of whole nodes, states in part 1, a part past the last empty.
@task_item n=8 part=touches
The snapshot text budget of 10,000 bytes now also bounds each graph part, not the snapshot alone.
@task_item n=9 part=touches
The command span of graph: a graph part's span records verb alone, no part and no count.
@task_item n=10 part=touches
The MCP server's tools/list: the press tool lists ctrl as a boolean, the graph tool an optional integer part 1 to 1000.
@task_item n=11 part=touches
The escher-session usage line and help: press gains [--ctrl], graph gains [--part <part>], both taken from the verb table.
@task_item n=12 part=touches
Session recording check file and Check::from_bytes: a kept check may now hold press with ctrl and graph with part, its format unchanged; replay of a kept graph part step.
@task_item n=13 part=touches
The capability statement pinned to the driver schema: its press, unpaged graph and stopPropagation limits were removed or narrowed; its pinned counts are unchanged.
@task_item n=14 part=touches
Any guard naming the test the_whole_cells_answer_is_within_the_answer_bound: it is replaced by cells_reads_in_parts_each_within_the_budget_and_together_the_whole_graph.
@task_item n=15 part=touches
Any row stating the Cells graph answer is whole within the answer bound: Cells' graph at boot now reads in more than one part.
@task_item n=16 part=touches
Listener reading for handler coverage (the vdom walk, recorded before handlers): the record is still resolved before the handlers run but written after the dispatch.
@task_item n=17 part=touches
Graph answer holds no value: unchanged in content, now also true of each graph part.
@task_item n=18 part=touches
The coverage role's line through escher-session coverage cells now reports the grid's keydown unexercised under the editor's stop.
@task_item n=19 part=touches
Driver verb table counts: argument rows 11 to 13, press arguments 2 to 3, graph arguments 0 to 1 and result fields 2 to 4, where a row's cell holds them.
@task_item n=20 part=touches
The command span stand check's driven calls per mode 40 to 41, where a row's cell holds that count.
@task_fact n=1 slug=integration-test-strategy
fact: A handler's panic unwinding out of the dispatch is recorded as a stopped dispatch — the innermost listener alone — so a panicking step keeps its reached record.
@task_fact n=2 slug=session-lifecycle
fact: A kept check's arguments read back by Check::from_bytes arrive in name order, not schema order, so a test comparing call arguments after a round trip compares them by name.
@task_fact n=3 slug=test-data-fixtures
fact: A Cells cell's click opens its editor unfocused, so a stand step reaching the editor with a key needs a click on cell-input first.
@task_fact n=4 slug=integration-test-strategy
fact: On a focused multiple select, press space after plain arrows flips the current row out, so the plain sequence ends with no row chosen; a set of rows not adjacent needs press with ctrl.
@task_fact n=5 slug=session-lifecycle
fact: The press verb's ctrl flag is the host's action key — Meta on macOS, Control elsewhere — witnessed on the macOS and Windows CI legs.
@task_item n=1 part=untouched
WPT conformance runner, its CLI, statuses, outputs and pinned commit.
@task_item n=2 part=untouched
CI legs, jobs, caches, package installs, actions pinning and permissions: no CI step, leg or script changed.
@task_item n=3 part=untouched
Coverage floors, mutation testing and the cognitive-complexity ceiling.
@task_item n=4 part=untouched
Agent-run and cold-agent contracts, events, verdicts and exit codes.
@task_item n=5 part=untouched
Lint defects and findings, harness contrast and harness screenshot.
@task_item n=6 part=untouched
The virtual clock, advance, timer stepping and the harness's settle.
@task_item n=7 part=untouched
The refusal cause set, the session error set and exit statuses: no cause, error or status added.
@task_item n=8 part=untouched
The session wire's hello, version and request bounds, and the session state directory.
@task_item n=9 part=untouched
Log fields, scrub sets and telemetry target allowlists: no log or span field changed.
@task_item n=10 part=untouched
Pixel tests, fonts, layout and incremental layout oracles.
