@task_prose part=session
The session paid three driver debts: press gains an optional ctrl flag, graph is read in parts, and handler coverage now reads a stopped dispatch.
press --ctrl holds the host's action key (Meta on macOS, Ctrl elsewhere); on a focused multiple select ctrl plus arrows move the current row and space flips it, so a set of rows that are not adjacent can be built.
graph takes an optional part 1 to 1000; a graph within the 10,000-byte snapshot budget with no part asked answers whole, otherwise it answers parts of whole nodes with part and parts fields, states in part 1 alone, a later part's empty states meaning read part 1, a part past the last empty.
The reached record is resolved before the dispatch and written after it: a dispatch a listener stops counts the innermost listener alone, a non-bubbling event its target alone, and a handler's panic unwinding is written as stopped.
Measured: plain arrows then space on a multiple select end with no row chosen; a Cells cell click opens its editor unfocused, so a key needs a click on the cell input first.
Retired the stated limits that press holds no Ctrl, that the graph is unpaged and that a stopPropagation is not read; no verb, exit status, session error or log field was added.
@task_file n=1 path=packages/escher-driver/src/schema.rs
what: the driver verb table, the schema rows each verb's arguments, fields and help are read from (press and graph rows edited)
@task_file n=2 path=packages/escher-driver/src/schema_tests.rs
what: the checks over the verb table's rows, arguments, fields and help (edited)
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver's command set, the verbs as parsed from the command line (press ctrl and graph part edited)
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's verb execution, where press joins the action modifier and graph is cut into parts at the snapshot budget (edited)
@task_file n=5 path=packages/escher-driver/src/json.rs
what: the driver's JSON answer writer, the graph answer's form and a graph part's form (edited)
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the checks over the JSON answer writer, the graph part form among them (edited)
@task_file n=7 path=packages/escher-driver/src/cli_tests.rs
what: the checks over escher-session's command line usage and verb flags (edited)
@task_file n=8 path=packages/escher-driver/src/mcp_tests.rs
what: the checks over the escher-session mcp role's tool listing (edited)
@task_file n=9 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture checks that build commands (edited)
@task_file n=10 path=packages/escher-driver/src/recording_tests.rs
what: the checks over kept check files, written and read back (edited)
@task_file n=11 path=packages/escher-driver/src/coverage.rs
what: the coverage role, the handler coverage the coverage line reports (its stated limit edited)
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs
what: the reached-handler record behind handler coverage, resolved before the dispatch and written after it (edited)
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the bridge from the native document's events to the vdom dispatch (edited)
@task_file n=14 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand checks of the graph verb and the UI graph answer, Cells read in parts (edited)
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand checks of acting on a select, the multiple select's set of rows built with ctrl (edited)
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the stand checks of handler coverage, a stopped dispatch on the Cells editor (edited)
@task_file n=17 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand checks of the mcp role's tools, graph part and press ctrl round trips (edited)
@task_file n=18 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand checks of replaying a kept check, a graph part step among them (edited)
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand checks of the command span's fields over the driven calls (edited)
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared stand-check helpers that build verb calls and read answers (edited)
@task_file n=21 path=examples/seven_guis/tests/cli_commands.rs
what: the checks of the escher-session binary's commands against the seven GUIs tasks, Cells among them (edited)
@task_file n=22 path=docs/capabilities.md
what: the capability statement, the stated limits of the driver and its coverage (edited)
@task_item n=1 part=touches
escher-session commands graph arguments: graph now takes an optional part, 1 to 1000; a row stating graph takes no argument is stale.
@task_item n=2 part=touches
escher-session commands graph answer: a graph over the 10,000-byte budget or asked a part answers a part, its nodes and states plus part and parts; states ride in part 1 alone and a part past the last is empty.
@task_item n=3 part=touches
The UI graph line content: the graph's stdout line is now one part of a paged graph where the graph is large; a row saying one unpaged line is stale.
@task_item n=4 part=touches
driver verbs and the virtual clock press: press carries the optional ctrl flag, the host's action key; its clock behaviour is unchanged.
@task_item n=5 part=touches
escher-session commands help: press's help now says ctrl moves the current row and space flips it; graph's help and its states field's help state the paging and that a later part's empty states means read part 1.
@task_item n=6 part=touches
escher-session commands snapshot [--part <part>]: the part argument kind and its 1 to 1000 bound are now shared by snapshot and graph.
@task_item n=7 part=touches
escher-session mcp role other tool results and the tool listing: press lists ctrl as a boolean and graph an optional integer part; a graph part's result is the part form.
@task_item n=8 part=touches
The coverage line content and the coverage answer: a dispatch a listener stops now counts the innermost listener alone, so an ancestor's handler under a stop reads unexercised.
@task_item n=9 part=touches
decision actions decided by the verbs' own checks: the actions line takes press ctrl and graph part from the same schema rows, no code of its own.
@task_item n=10 part=touches
coverage check files and kept checks: a kept check may now hold press with ctrl and graph with part, its format unchanged.
@task_item n=11 part=touches
escher-session tasks cells: Cells' graph at boot reads in more than one part; a key to a cell needs a click on the cell input after the cell click, since the editor opens unfocused.
@task_item n=12 part=touches
decision focused select with no box keeps focus and keys: on a focused multiple select ctrl plus arrows move the current row without choosing and space flips it; plain arrows choose the row and a following space flips it out.
@task_item n=13 part=touches
decision driver command line output read by agents and escher-session stdout command line output: the graph's output can now come as several parts the agent reads in turn.
@task_fact n=1 slug=surface
fact: press takes an optional ctrl flag held as the host's action key, Meta on macOS and Ctrl elsewhere; with it the arrows move a multiple select's current row without choosing and space flips that row, building a set of rows that are not adjacent.
@task_fact n=2 slug=surface
fact: graph is paged at the 10,000-byte snapshot budget: with no part and within the budget it answers whole; otherwise part n of parts, cut on whole nodes, states in part 1 alone, a later part's empty states meaning read part 1, a part past the last answering empty nodes and states, part 0 or over 1000 refused as malformed.
@task_fact n=3 slug=surface
fact: handler coverage reads a stopped dispatch: an unstopped bubbling dispatch counts every listener on the chain, one a listener stops counts the innermost listener alone, a non-bubbling event its target alone, and a handler panic's unwind counts as stopped.
@task_fact n=4 slug=surface
fact: on a focused multiple select, plain arrows choose the current row alone and a following space flips it out, so that sequence ends with no row chosen.
@task_fact n=5 slug=surface
fact: a Cells cell's click opens its editor unfocused; a key reaches the editor only after a click on the cell input.
@task_item n=1 part=untouched
Color palette, typography, spacing, depth strategy, border radius and iconography rows: no colour, font, spacing, paint order, radius or cursor changed.
@task_item n=2 part=untouched
Exit statuses and exit codes 0 to 3, the session error object, its remedy and stderr usage and session errors: no status, cause or session error was added.
@task_item n=3 part=untouched
The single select's list events, the select list overlay, its placement, colours, motion and hit order: the open list is untouched; only a multiple select's keys are read.
@task_item n=4 part=untouched
Screenshot rows, layout rows and the snapshot's select-list row bounds: their answers did not change.
@task_item n=5 part=untouched
Motion rows on scroll, animation and the virtual clock's advance: no clock or scroll behaviour moved.
@task_item n=6 part=untouched
WPT runner output and result colours, the shell provider, the JS window object, dioxus-native config and the web-spa surface: none was reached.
@task_item n=7 part=untouched
escher-session tasks other than cells, counter, crud, flight-booker, load, order and timer: their behaviour is unchanged.
