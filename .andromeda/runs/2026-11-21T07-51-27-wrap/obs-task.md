@task_prose part=session
The driver's press verb gained an optional ctrl flag, the host's action key (Meta on macOS, Control elsewhere), so a press can move the current row of a multiple select and build a set of rows that are not adjacent.
The driver's graph verb gained an optional part argument (1 to 1000): a graph whose answer exceeds the 10,000-byte snapshot budget, or any graph asked by part, is answered in parts of whole nodes, each part the graph keys plus part and parts, the states in part 1 alone, a part past the last answering empty nodes and states.
A graph part's command span records verb alone, as a snapshot part read does; no span field, log field, scrub set, verb, refusal cause, session error, node key or exit status was added; the verb table still holds fifteen verbs while its argument rows went from 11 to 13.
The handler coverage reached record is now resolved before the dispatch and written after it: a bubbling dispatch that still propagates counts every listener, one a listener stops counts the innermost listener alone, a non-bubbling event its target alone, and a dispatch a handler's panic unwinds is written as stopped.
The capability statement's limits on Ctrl, on an unpaged graph and on an unread stopPropagation were retired or narrowed by hand; its pinned counts did not move; stand_act_spans' driven calls per mode went from 40 to 41.
Measured: a plain arrow sequence then space on a multiple select ends with no row chosen, and a Cells cell click opens its editor unfocused so a key needs a click on the editor input first.
@task_file n=1 path=packages/escher-driver/src/schema.rs
what: the driver's verb table (the schema rows): each verb's arguments, result fields and help, the argument kinds including the part bound, read by the command line, the MCP listing, the actions answer and the kept-check reader
@task_file n=2 path=packages/escher-driver/src/schema_tests.rs
what: the unit checks pinning the verb table's argument and field counts and the press and graph help text
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver's command enum and its parsing from call arguments (press with shift and ctrl, graph with its part)
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's command execution and its command span: press joining the action modifier, graph answering whole or in parts under the snapshot text budget, and the unit pinning the span's eight fields with none an argument name
@task_file n=5 path=packages/escher-driver/src/json.rs
what: the driver's answer writer: the graph answer's JSON (nodes, states, and a part's part and parts) and the written lengths of each node
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the unit checks of the answer writer, including a graph part's shape
@task_file n=7 path=packages/escher-driver/src/cli_tests.rs
what: the unit checks of the escher-session command line's verb flags and usage lines
@task_file n=8 path=packages/escher-driver/src/mcp_tests.rs
what: the unit checks of the MCP tool listing's input schemas (press ctrl a boolean, graph part an integer)
@task_file n=9 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture checks building commands
@task_file n=10 path=packages/escher-driver/src/recording_tests.rs
what: the unit checks of the kept check file's round trip of call arguments
@task_file n=11 path=packages/escher-driver/src/coverage.rs
what: the escher-session coverage role's module (handler coverage over the reached record), its doc of the stop limit
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs
what: the reached record of handler coverage: which listeners a dispatch reached, by node id and event name, resolved before the handlers run and written after with a drop guard
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the bridge dispatching a document event to the vdom, keeping the target node and finishing the reached record with whether the dispatch still propagates
@task_file n=14 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand check of the graph verb: Cells read in parts within the budget, a small screen whole, part bounds refused malformed
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand check of select interaction: a press with ctrl building a non-adjacent set of rows
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the stand check of handler coverage: the grid's keydown unexercised under the cell editor's stop
@task_file n=17 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of the MCP tools: a graph part and a press with ctrl round trip, their tool_call and command spans
@task_file n=18 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check of kept-check replay: a kept graph part step replays and a changed one stops naming parts
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check of the command span per driven call, its driven calls per mode (now 41), a graph part's span recording verb alone
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand checks' shared session helpers (press with ctrl, a graph part, reading a paged graph whole)
@task_file n=21 path=examples/seven_guis/tests/cli_commands.rs
what: the escher-session binary's command line checks: graph in parts, press with ctrl, the coverage role under the editor's stop
@task_file n=22 path=docs/capabilities.md
what: the capability statement, updated by hand: its stated limits on Ctrl, an unpaged graph and the coverage stop read
@task_item n=1 part=touches
The graph answer's shape: a graph is now answered whole, or as a part holding the graph keys plus part and parts, states in part 1 alone, a part past the last empty.
@task_item n=2 part=touches
The driver's command span for graph: a graph part read records verb alone, with no part number or count, as the snapshot part read does.
@task_item n=3 part=touches
The rule that a part's answer is never logged now reaches a graph part as well as a snapshot part.
@task_item n=4 part=touches
The command answer for a graph (nodes and states) gains a paged form beside the whole one.
@task_item n=5 part=touches
The rules that a graph answer is never logged and that the UI graph answer holds no control value or typed text: the writer is unchanged but the answer is now cut into parts.
@task_item n=6 part=touches
The rules that the command span carries no call content and no argument name: two new arguments, ctrl and part, are kept out of the span; the span's field count stays eight while the verb table's argument rows went from 11 to 13.
@task_item n=7 part=touches
Any guard or row naming the stand check of command spans per driven call, whose driven calls per mode moved from 40 to 41.
@task_item n=8 part=touches
The kept check file's arguments: a kept check may now hold press with ctrl and graph with part; its format is unchanged, and arguments read back arrive in name order.
@task_item n=9 part=touches
The refusal cause malformed now covers a graph part of 0, over 1000 or not a number, and a ctrl flag of the wrong kind or repeated.
@task_item n=10 part=touches
The reached record holding ids and event names only: it is now resolved before the dispatch and written after it, still ids and event names alone.
@task_item n=11 part=touches
The handler coverage line and the escher-session coverage role's answers: a stopped dispatch now counts its innermost listener alone, so an ancestor's listener under a stop reads unexercised.
@task_item n=12 part=touches
The cold-agent pipe's counted graph verb, graph calls and part reads in its verdict: a graph may now be read in parts, which those counts may need to say how they treat.
@task_item n=13 part=touches
The capability statement updated by hand: three of its stated limits were retired or narrowed by hand, its pinned counts unchanged.
@task_item n=14 part=touches
The MCP role's stdout tool listing: the press tool lists ctrl as a boolean and the graph tool an optional integer part.
@task_item n=15 part=touches
The rule that the select chosen set is never logged: a press with ctrl now builds a set of non-adjacent chosen rows, still in no span or log.
@task_item n=16 part=touches
The escher-session stdout command answer and the command line: usage lines now read press with an optional --ctrl and graph with an optional --part.
@task_fact n=1 slug=log-coverage
fact: Handler coverage counts a bubbling dispatch that still propagates at every listener it reached, one a listener stops at the innermost listener alone, a non-bubbling event at its target alone, and a dispatch a handler's panic unwinds as stopped.
@task_item n=1 part=untouched
Telemetry backends, OTel export, metric emission and metrics exporters: none added.
@task_item n=2 part=untouched
escher's telemetry sink and its log line: format, stream, RUST_LOG, resource keys, target allowlist and the scrub set are unchanged.
@task_item n=3 part=untouched
escher-telemetry's startup and panic events and its panic hook: not reached.
@task_item n=4 part=untouched
The agent-run and cold-agent harness events, their log, transcript and cleanup: no harness or script changed.
@task_item n=5 part=untouched
CI integration: no workflow step, leg, artifact upload or script changed.
@task_item n=6 part=untouched
The recording and session state directories' privacy, recording bounds, idle expiry and the host wire version: unchanged.
@task_item n=7 part=untouched
escher-session exit codes, session error objects and the MCP role exit statuses: none added or changed.
@task_item n=8 part=untouched
Script error surfaces, the window error event, script diagnostics drains and the WPT report: not reached.
@task_item n=9 part=untouched
The build-directory sweep line and the package-install diagnostic line: not reached.
@task_item n=10 part=untouched
The tool_call span's fields and endings: unchanged; only the command span's per-verb rows move.
