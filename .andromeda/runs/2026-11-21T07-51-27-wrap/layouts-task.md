@task_prose part=session
The driver's press verb gained an optional ctrl flag, the host's action key (Meta on macOS, Control elsewhere), so a focused multiple select builds a set of rows that are not adjacent.
The driver's graph verb gained an optional part argument 1 to 1000: a graph within the 10,000-byte snapshot budget answers whole as before; a larger one, or any asked part, answers whole nodes per part, the states in part 1 alone, the JSON graph keys plus part and parts, and a part past the last answers empty nodes and states.
Handler coverage now reads a stopped dispatch: ids resolved before the handlers run, written after; an unstopped bubbling dispatch counts every listener, one an ancestor or the target stops counts the innermost listener alone, a non-bubbling event its target alone, and a handler's panic writes the dispatch as stopped.
Measured: a Cells cell's click opens its editor unfocused, so a key needs a click on cell-input first; on a multiple select a space after plain arrows flips the chosen row out, the plain sequence ending with no row chosen.
Retired the stated limits that press holds no Ctrl, that the graph answer is unpaged, and that a stopPropagation is not read; no verb, refusal cause, session error, node key or exit status added, the verb table still fifteen verbs.
@task_file n=1 path=packages/escher-driver/src/schema.rs
what: the driver's verb table, every verb's arguments, answer fields and help, which the command line, the MCP tools listing, the actions answer and the kept-check reader all read
@task_file n=2 path=packages/escher-driver/src/schema_tests.rs
what: the checks on the verb table's arguments, fields and help text
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver's command forms, the press and graph commands and their arguments
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's run of each verb, press with its modifiers and the graph answer cut into parts at the snapshot budget
@task_file n=5 path=packages/escher-driver/src/json.rs
what: the driver's JSON writer of every answer, the graph answer included
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the checks on the JSON form of each answer, the graph part's form included
@task_file n=7 path=packages/escher-driver/src/cli_tests.rs
what: the checks on the escher-session command line's parsing, its usage line and verb flags
@task_file n=8 path=packages/escher-driver/src/mcp_tests.rs
what: the checks on the MCP server's tools/list, each tool's input schema
@task_file n=9 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture checks building commands
@task_file n=10 path=packages/escher-driver/src/recording_tests.rs
what: the checks on session recording, a kept check-{n}.json written and read back
@task_file n=11 path=packages/escher-driver/src/coverage.rs
what: the escher-session coverage role's reading of which handlers a run reached, its stated limits
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs
what: the reached-handler record behind handler coverage, which listeners a dispatch reached, resolved before and written after it
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the document's event dispatch into the vdom, the bridge that records the reached handlers
@task_file n=14 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand checks on the graph answer, Cells read in parts within the budget
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand checks on driving a select, a multiple select's rows chosen by keys
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the stand checks on handler coverage, the Cells grid's keydown under the editor's stop
@task_file n=17 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand checks on the MCP server's tools, a graph part and a press with ctrl round trip
@task_file n=18 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand checks that a kept check replays, a kept graph part step included
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand checks on each command's span, recording the verb alone and no argument
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand checks' shared session helpers, building calls and reading answers
@task_file n=21 path=examples/seven_guis/tests/cli_commands.rs
what: the process checks of the escher-session command line against the served tasks, Cells graph parts, press with ctrl and the coverage role
@task_file n=22 path=docs/capabilities.md
what: the capability statement, what the driver can do and its stated limits
@task_item n=1 part=touches
The escher-session graph answer: it is no longer one unpaged line; a graph past the 10,000-byte budget or an asked part answers part n of parts, whole nodes, the states in part 1 alone, the JSON graph keys plus part and parts; a row saying one unpaged uncoloured line is stale.
@task_item n=2 part=touches
The escher-session command line's command form: press takes a bare --ctrl beside --shift, and graph takes --part <part>; a usage or command-form row naming press --key [--shift] or a graph with no argument is stale.
@task_item n=3 part=touches
The escher-session snapshot --part: the same part argument kind, bound 1 to 1000, now pages the graph too; a row stating the part bound or that only the snapshot pages is touched.
@task_item n=4 part=touches
The escher-session MCP server's tools/list: the press tool lists ctrl as a boolean and the graph tool an optional integer part 1 to 1000.
@task_item n=5 part=touches
The escher-session MCP server's result bound: a graph answer now stays within the 10,000-byte snapshot budget a part at a time; a row holding that bound for the graph is touched.
@task_item n=6 part=touches
The escher-session help and actions answers: press's help names ctrl as the host's action key, ⌘ (Meta) on macOS and Ctrl elsewhere, moving the current row while space flips it, and graph's help states the paging and that a later part's empty states means read part 1; the verb count stays fifteen.
@task_item n=7 part=touches
The escher-session coverage role's answer: a dispatch its target or an ancestor stops counts the innermost listener alone, a non-bubbling event its target alone, an unstopped bubbling dispatch every listener, and a handler's panic is recorded as a stopped dispatch; the limit that a stopPropagation is not read is retired.
@task_item n=8 part=touches
Session recording as a replayable check: a kept check may now hold press with ctrl and graph with part, its format still 1, and a changed kept graph part stops naming parts.
@task_item n=9 part=touches
Caller input never written back: neither new argument reaches a log; a graph part's command span records the verb alone, no part or count.
@task_item n=10 part=touches
The served Cells task and its ids cell-input and cells-grid: a cell's click opens the editor unfocused, so a key reaches the editor only after a click on cell-input; the editor's Enter stops the dispatch and leaves the grid's keydown unexercised.
@task_item n=11 part=touches
A keyboard region over a list or multiple select: with press --ctrl the arrows move the current row without choosing it, so space builds a set of rows that are not adjacent; plain arrows choose the row and a following space flips it out.
@task_item n=12 part=touches
The escher-session refusals and exit codes: a part of 0, 1001 or text is refused malformed under the existing status; no refusal cause or exit status is added.
@task_item n=13 part=touches
The cold agent driving through the driver's MCP server: the tools it reads now carry press ctrl and graph part.
@task_fact n=1 slug=surface-cli
fact: escher-session press takes an optional bare --ctrl, the host's action key (Meta on macOS, Control elsewhere), moving the current row without choosing it so space builds a set of rows that are not adjacent.
@task_fact n=2 slug=surface-cli
fact: escher-session graph takes an optional --part 1 to 1000; unasked and within 10,000 bytes the graph answers whole, else each part holds whole nodes within the budget (a node over it a part alone), part 1 alone carries the states, a later part's empty states means read part 1, and a part past the last answers empty nodes and states.
@task_fact n=3 slug=surface-cli
fact: The coverage role counts a dispatch an ancestor or its target stops by its innermost listener alone, a non-bubbling event by its target alone, and a dispatch a handler's panic unwinds as stopped.
@task_fact n=4 slug=surface-desktop-native
fact: A Cells cell's click opens its editor unfocused, so a key reaches the editor only after a click on cell-input.
@task_fact n=5 slug=surface-desktop-native
fact: On a focused multiple select a space after plain arrows flips the chosen row out, so the plain sequence ends with no row chosen.
@task_item n=1 part=untouched
The ShellProvider window chrome controls and the desktop window's layout: no window call changed.
@task_item n=2 part=untouched
The stand's look and boot decisions, its accent colour, pinned viewport, fixed document skeleton, lints clean at boot and a11y by attributes: no stand page or style changed.
@task_item n=3 part=untouched
The stand element ids other than the Cells ones: no id was taken, freed or renamed.
@task_item n=4 part=untouched
The agent-run, cold-agent, multi_doc and paint_bench command lines: their usage, flags, stdout and exit codes are unchanged.
@task_item n=5 part=untouched
The escher-session host log's service.name, the MCP server's stdio, end and protocol revisions, and the screenshot, layout, lint and snapshot answers' forms: none changed.
@task_item n=6 part=untouched
The mobile-native and web-spa surfaces and the Decisions Log: nothing measured reaches them.
@task_item n=7 part=untouched
The served tasks other than Cells, and the load, order and flight pages: their controls and boot readings are unchanged.
