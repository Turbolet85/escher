@task_prose part=session
The session paid three driver debts on the founder's word: press with Ctrl, graph in parts, and handler coverage under a stopped dispatch.
The press verb gains an optional closed flag ctrl, held as the host's action key (Meta on macOS, Control elsewhere); on the command line it is a bare --ctrl beside --shift, over MCP a boolean.
The graph verb gains an optional part argument (1 to 1000, outside refused malformed) and the result fields part and parts; a graph over the 10,000-byte snapshot budget, or any graph asked by part, answers in parts of whole nodes, states in part 1 alone, a part past the last empty.
The reached-handler record is now resolved before the dispatch and written after it: a stopped bubbling dispatch records its innermost listener alone, a non-bubbling event its target alone, and a handler's panic unwinding out of the dispatch is written as stopped by a drop guard.
No verb, refusal cause, session error, node key, exit status, log field, span field or scrub set was added; the verb table still holds fifteen verbs and a graph part's command span records verb alone.
The capability statement's limits on Ctrl, the unpaged graph and the unread stopPropagation were retired or narrowed; CI went green 17 of 17 on the pre-CI commit.
@task_file n=1 path=packages/escher-driver/src/schema.rs
what: the closed driver verb schema: the verb table, each verb's argument rows, argument kinds and bounds, result fields and help text
@task_file n=2 path=packages/escher-driver/src/schema_tests.rs
what: the unit checks of the driver verb schema: argument and field counts per verb and the help text
@task_file n=3 path=packages/escher-driver/src/command.rs
what: the driver command type and its validation from call arguments, the single command validation gate
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's verb execution: key presses with modifiers, the graph read and its cut into parts within the answer budget, the outcome arms
@task_file n=5 path=packages/escher-driver/src/json.rs
what: the driver's answer writer: the JSON form of each outcome, the graph answer's nodes and states and a part's form
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the unit checks of the driver's answer JSON, the graph answer and the graph part answer
@task_file n=7 path=packages/escher-driver/src/cli_tests.rs
what: the unit checks of the escher-session command line: verb flags, usage lines and argv parsing
@task_file n=8 path=packages/escher-driver/src/mcp_tests.rs
what: the unit checks of the MCP server's tool listing and tool argument value kinds
@task_file n=9 path=packages/escher-driver/src/fixture_tests.rs
what: the unit checks building driver commands against fixtures
@task_file n=10 path=packages/escher-driver/src/recording_tests.rs
what: the unit checks of the recorded check file: its arguments read back and its round trip
@task_file n=11 path=packages/escher-driver/src/coverage.rs
what: the handler coverage reader behind the coverage answer and the coverage role, and its stated limits
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs
what: the reached-handler record of the Dioxus native bridge: which listeners a dispatch reached, resolved before the handlers and written after them
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus native bridge's document: the dispatch of engine events to the vdom by data-dioxus-id
@task_file n=14 path=tests/blitz-tests/tests/stand_graph.rs
what: the stand checks of the graph verb: the whole graph, the graph in parts within the budget
@task_file n=15 path=tests/blitz-tests/tests/stand_act_select.rs
what: the stand checks of select interaction: a multiple select's chosen set built by keys
@task_file n=16 path=tests/blitz-tests/tests/stand_handler_coverage.rs
what: the stand checks of handler coverage: listeners exercised and unexercised under a stopped dispatch
@task_file n=17 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand checks of the MCP tools: round trips of each tool and their logged spans
@task_file n=18 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand checks of kept check replay: a kept step replays and a changed one stops
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand checks of the command span: fields recorded per driven call, no argument name among them
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared stand test helpers that build driver calls and read outcomes
@task_file n=21 path=examples/seven_guis/tests/cli_commands.rs
what: the process checks of the escher-session binary's command line and coverage role on the seven GUIs app
@task_file n=22 path=docs/capabilities.md
what: the committed capability statement: what the driver does and its stated limits
@task_item n=1 part=touches
decision/graph-answer-unpaged is now false: the graph answer is paged by part at the 10,000-byte snapshot budget, whole only when no part is asked and it fits.
@task_item n=2 part=touches
contract/driver-verbs/graph moved: graph takes an optional part argument (1 to 1000) and answers the fields part and parts besides nodes and states.
@task_item n=3 part=touches
contract/graph-answer/nodes-states moved: a part answers the graph keys plus part and parts, its nodes whole, states in part 1 alone, a later part's empty states meaning read part 1, a part past the last empty.
@task_item n=4 part=touches
contract/driver-argument-kinds/part now covers a graph part as well as a snapshot part, bounded 1 to 1000 with 0, 1001 and text refused malformed.
@task_item n=5 part=touches
contract/cli-flags/part now applies to graph too (graph [--part <part>]); contract/cli-flags/shift gains a sibling bare flag --ctrl on press.
@task_item n=6 part=touches
contract/driver-argument-kinds/flag gains a second flag argument: press's ctrl, a closed flag, wrong kind or repeat refused malformed.
@task_item n=7 part=touches
contract/mcp-tool-arguments/value-kinds: the press tool lists ctrl as a boolean and the graph tool an optional integer part 1 to 1000.
@task_item n=8 part=touches
decision/closed-driver-verb-schema, decision/single-validation-gate and decision/one-validation-path-for-every-carrier: both new arguments enter through the schema rows alone and every carrier (in process, command line, MCP, kept check) takes them from there; no verb added, fifteen stand.
@task_item n=9 part=touches
decision/reached-handler-record-without-log: the record is now resolved before the dispatch and written after it, reading the stop, with a drop guard writing a panicking dispatch as stopped; still no log and no NodeId.
@task_item n=10 part=touches
decision/reached-record-deduplicated-survives-removal and decision/dispatch-recorded-only-with-signal-registry: rules over the reached record, whose write moved after the dispatch; the bridge keeps the target node it found the data-dioxus-id on.
@task_item n=11 part=touches
decision/coverage-answer-crossing, contract/coverage-line/unexercised and contract/escher-session-coverage/answer-line: a listener above a stopping one now counts unexercised (the Cells grid's keydown under the editor's stop), through the coverage role too.
@task_item n=12 part=touches
decision/graph-answer-crossing and decision/kept-check-graph-answer: a new answer arm, the graph part, crosses the same carriers and is kept in a kept check; the graph writer is unchanged, no control value or typed text.
@task_item n=13 part=touches
contract/recorded-check/arguments and contract/kept-check-file/contents: a kept check may now hold press with ctrl and graph with part, its format unchanged; read-back arguments arrive in name order.
@task_item n=14 part=touches
decision/command-span-content-free, contract/command-span/verb and contract/driver-command-span/verb: a graph part's command span records verb alone, no part or count; the span-check guard's driven calls per mode moved 40 to 41.
@task_item n=15 part=touches
contract/mcp-tool-result/size-bound and contract/driver-command-line/answer-bounds: the Cells graph now reads in more than one part, each within the 10,000-byte budget, replacing the check that the whole Cells answer fits the answer bound.
@task_item n=16 part=touches
decision/paged-answer-closed-decoding: the graph answer is now a paged answer, read by the closed fields part and parts.
@task_item n=17 part=touches
decision/multiple-select-keys-and-clicks and decision/multiple-select-set-exits: press with ctrl moves the current row and space flips it, building a set of rows that are not adjacent; plain arrows then space ends with no row chosen (measured).
@task_item n=18 part=touches
decision/cells-grid-keyboard: measured that a Cells cell's click opens its editor unfocused, so a key reaches the editor only after a click on cell-input.
@task_item n=19 part=touches
contract/escher-session-stdout/command-line: the graph result line may now be one part with part and parts.
@task_item n=20 part=touches
decision/actions-answer-unpaged stands as written: only the graph is paged, the actions answer is not.
@task_fact n=1 slug=input-validation
fact: press takes an optional closed flag ctrl, the host's action key (Meta on macOS, Control elsewhere), as a bare --ctrl on the command line and a boolean over MCP; wrong kind or repeat refused malformed.
@task_fact n=2 slug=api-security
fact: The graph answer is paged: whole when no part is asked and it fits the 10,000-byte snapshot budget, else parts of whole nodes each within the budget (a node over the budget a part alone), states in part 1 alone, a part past the last empty; part is bounded 1 to 1000.
@task_fact n=3 slug=input-validation
fact: The reached-handler record is resolved before the dispatch and written after it: a stopped bubbling dispatch records its innermost listener alone, a non-bubbling event its target alone, and a handler's panic unwinding out of the dispatch is written as stopped.
@task_fact n=4 slug=logging-monitoring
fact: A graph part's command span records verb alone; neither the part asked nor the part count reaches a log.
@task_fact n=5 slug=data-protection
fact: A kept check may hold press with ctrl and graph with part, its format unchanged.
@task_item n=1 part=untouched
Authentication and authorization, secret management and environment variables: no credential, secret or environment read changed.
@task_item n=2 part=untouched
Dependency security: no dependency, version, pin or audit gate changed.
@task_item n=3 part=untouched
Outbound network, script fetch, CORS, CSP, navigation and resource-load rows: no fetch, request or navigation path changed.
@task_item n=4 part=untouched
Session socket wire, grammar, bounds and lifecycle replies, and MCP protocol errors: no request form, reply, error or connection rule changed.
@task_item n=5 part=untouched
The closed refusal cause set, session errors and exit codes: none added or removed; the new bounds refuse with the existing malformed cause.
@task_item n=6 part=untouched
Masked input values, password painting and the value mask: no control value or typed text newly crosses.
@task_item n=7 part=untouched
Stable element id derivation and carriers, and accessibility adapter rows: no id form or accessible name changed.
@task_item n=8 part=untouched
Single select list rows (open list painting, rows, wheel, typeahead, closing): the open list was not changed.
@task_item n=9 part=untouched
CI workflow, WPT, cold-agent, agent-run and apt-install rows: no CI step, leg or script changed.
@task_item n=10 part=untouched
Engine log targets, the telemetry sink's allowlist and scrub set: no log field, target or scrub set changed.
