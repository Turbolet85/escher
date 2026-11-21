@task_prose part=chunk
Plans three driver debts (capability v040-05) as founder forks, each fixed and proven on the stand or kept as a residual by his word.
`press` with Ctrl: whether to add a Ctrl (and by kin Meta/Alt) modifier to `press` beside `--shift`, on which carriers, so a non-adjacent multiple select set can be built, or keep the stated limit.
The UI graph read in parts: `graph --part N`, paged as `snapshot` is, as already ruled; every part within the 1,048,576-byte answer bound.
Handler coverage under a stopped bubbling dispatch: read a stopPropagation in the reached record so an ancestor the dispatch stopped short of no longer reads exercised, or keep the stated limit of handler coverage and the coverage line.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's verb table, argument kinds and bounds — edits: the `press` verb's modifier argument, the `graph` verb's `part` argument
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the verb specification: each verb's arguments, their closed value sets and bounds — edits: the `press` and `graph` entries
@task_file n=3 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command: parsing a call's verb and arguments into a command
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description and `help` text, which states the `press` limit
@task_file n=5 path=packages/escher-driver/src/execute.rs mark=edits
what: executing a driver command: the `press` key with its modifiers, the `graph` answer and its parts
@task_file n=6 path=packages/escher-driver/src/cli.rs mark=edits
what: escher-session's command line: the verbs' flags such as `--shift` and `--part`
@task_file n=7 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session mcp role: the MCP tool listing and each tool's argument schema
@task_file n=8 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording and the kept check file: a kept check's argument shapes
@task_file n=9 path=packages/escher-driver/src/replay.rs mark=edits
what: replay of a kept check: reading back its arguments
@task_file n=10 path=packages/escher-driver/src/coverage.rs mark=edits
what: the escher-session coverage role and its handler coverage line
@task_file n=11 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph: the nodes and states the `graph` answer carries, cut into parts
@task_file n=12 path=packages/dioxus-native-dom/src/reached.rs mark=edits
what: the reached record: each element of a dispatch's chain whose listeners hold its event, by id and event name
@task_file n=13 path=packages/blitz-dom/src/events/driver.rs mark=edits
what: the engine's event dispatch: the bubbling chain and where a stop is decided
@task_file n=14 path=docs/capabilities.md mark=edits
what: the capability statement: the `press` limit, the graph's form and handler coverage's stated limit
@task_file n=15 path=.github/scripts/test_capability_statement.py mark=edits
what: the check that pins the capability statement
@task_file n=16 path=tests/blitz-tests/tests/stand_graph.rs mark=edits
what: the stand check of the UI graph answer
@task_file n=17 path=tests/blitz-tests/tests/stand_handler_coverage.rs mark=edits
what: the stand check of handler coverage and the coverage line
@task_file n=18 path=tests/blitz-tests/tests/stand_mcp_tools.rs mark=edits
what: the stand check of the MCP tool listing and its argument schemas
@task_file n=19 path=tests/blitz-tests/tests/stand_act_select.rs mark=edits
what: the stand check of acting on a select, its multiple-key check
@task_file n=20 path=tests/blitz-tests/tests/stand_act_keys.rs mark=edits
what: the stand check of `press` keys and their modifiers
@task_item n=1 part=needs
The `graph` verb gains a `part` argument, paged as the snapshot's parts are: the graph answer's shape changes from one whole answer to numbered parts, each within the 1,048,576-byte answer bound and `MAX_PART` (1000).
@task_item n=2 part=needs
A new argument of a closed value set on a ratified verb (`press`'s modifier, `graph`'s `part`) crosses no new carrier: it rides the session socket's call, the command line, MCP and the kept check, and stays inside the security plan's crossing rule.
@task_item n=3 part=needs
The `press` verb may gain a Ctrl (and Meta/Alt) modifier beside `--shift`: the verb's arguments, its MCP tool schema, its `help` text and the kept check's argument shapes change with it, or the limit stays stated.
@task_item n=4 part=needs
A multiple select's chosen set built with a modifier press: the snapshot's value of a multiple select, the form entries and the change events a page script sees.
@task_item n=5 part=needs
The reached record may read a stopped propagation: it stays ids and event names only, and handler coverage and the coverage line from `escher-session coverage` then stop counting an ancestor's handler the dispatch never reached.
@task_item n=6 part=needs
The coverage line's shape, content and owner, and the coverage role's command shape, accepted, malformed and boot-failure answers, as a fix of the stop reading reaches them.
@task_item n=7 part=needs
The graph answer, a part's answer and the coverage line are never logged; driver spans carry no call content, so a new argument value (a modifier, a part number) is no span field unless the command span's closed field set says so.
@task_item n=8 part=needs
One command span per driver command and the MCP tool_call span with its tool and ending: a `graph` part read and a modified `press` are each one command under the same spans, with the command span's verb and part-read fields as they stand for a snapshot part read.
@task_item n=9 part=needs
Refusal causes for a malformed or out-of-range argument (a part past the last, past `MAX_PART`, an unknown modifier) use the existing closed cause names; escher-session's error object, exit codes and fixed diagnostics carry no caller-typed value.
@task_item n=10 part=needs
The cold-agent pipe's counted verbs and verdict fields: a `graph` call with a part is still counted as a graph call, and part reads as the verdict counts them.
@task_item n=11 part=needs
The recording file and kept check file hold the new arguments under the recording bounds and the private recording directory; committed kept checks hold no private data.
@task_item n=12 part=needs
The capability statement is updated by hand for each debt the founder rules fixed or kept, and its pinning check follows it.
@task_item n=13 part=needs
The answer echoes nothing the caller typed; the driver library installs no subscriber and emits no event of its own.
@task_standing n=1
convention: each new stand check of a driver verb runs on the headless stand with no telemetry sink, silent, and with RUST_LOG unset where it reads an empty stderr
home: 3-observability-harness-contract
@task_standing n=2
convention: each new kept check or fixture committed for a stand test holds no secrets and no private data
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Metric coverage: no counter, gauge, histogram or metrics exporter is added, and WPT score publishing is untouched.
@task_item n=2 part=not_needed
Error capture: the window error event, window.onerror, script error source labels, the WPT report and the panic hooks are not reached.
@task_item n=3 part=not_needed
The agent-run harness events and its log, and the build-directory sweep's stdout line, are not reached.
@task_item n=4 part=not_needed
Script diagnostics drain (take_js_errors, take_messages) and the telemetry sink's log line format, resource keys and RUST_LOG configuration are not changed.
@task_item n=5 part=not_needed
Engine debts, the single select sized as a list box, the open list's painted rows and optgroup labels, and any canvas work are out of scope.
@task_item n=6 part=not_needed
The layout and actions answers, the screenshot (shot) answer and lint findings are not changed by this chunk.
@task_item n=7 part=not_needed
Session lifecycle idle expiry, the host wire version and the session state directory are not changed.
