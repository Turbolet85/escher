@task_prose part=session
The session made an acting call's answer say what its step caused: click, type, press, advance and scroll gain a caused member, last among their fields and present only when the step caused something.
Each caused item is one kind (panic, error, log) at one level (error, warn, info, debug, trace) with the id, owner and event of the first handler the step reached and a count; it holds no message, payload or caller input.
A handler panic is now caught inside the step and the session keeps answering; a handler Err the app's root default boundary takes reads as one error item, and that boundary renders the error text on screen in place of the app.
The Dioxus framework's own trace records count only at warn and error; the telemetry crate gains a log capture beside its sink, the sink's printed output and filter unchanged.
Exploration now ends a panicking path as Panicked from the answer's panic item and its kept check holds the panicked step; help and the MCP tool listing describe caused from the verb table.
No verb, refusal, exit code, node key, stand element id, served task, window chrome control or command line flag was added, renamed or retired.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: new: the telemetry crate's log capture that counts the log records a step caused by level, beside the sink, printing nothing
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: edited: the telemetry crate's init, the host log subscriber: the filter now on the sink alone, the capture layer beside it; crate doc link fixed
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs
what: new: the document's reading of the app's root error boundary mark
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: edited: the native document crate's module list and re-exports
@task_file n=5 path=packages/dioxus-native-dom/src/reached.rs
what: edited: the reached-handlers record, now also a per-step list of handlers reached, drained each step
@task_file n=6 path=packages/escher-driver/Cargo.toml
what: edited: the driver crate's manifest, gaining the telemetry crate as a dependency
@task_file n=7 path=Cargo.lock
what: edited: the workspace lockfile, one dependency line for the driver's new edge
@task_file n=8 path=packages/escher-driver/src/caused.rs
what: new: the caused item of an acting answer, its closed kind and level words
@task_file n=9 path=packages/escher-driver/src/execute.rs
what: edited: the session's acting step, now run under the capture and a panic catch, yielding caused on click, type, press, advance and scroll
@task_file n=10 path=packages/escher-driver/src/session.rs
what: edited: the session start, turning on the reached record on every held session
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: edited: the verb table, the source of help and the MCP tool listing, gaining the caused field on the five acting verbs
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: edited: the verb table's checks, the acting verbs' field counts and the field kinds table
@task_file n=13 path=packages/escher-driver/src/json.rs
what: edited: the answer line's written form, writing caused last and only when non-empty
@task_file n=14 path=packages/escher-driver/src/explore.rs
what: edited: exploration, ending a path Panicked when its answer carries a panic item
@task_file n=15 path=packages/escher-driver/src/fixture_tests.rs
what: edited: the driver's fixture checks, matching the acting outcomes' new field
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: edited: the session recording's checks, matching the acting outcomes' new field
@task_file n=17 path=packages/escher-driver/src/replay.rs
what: edited: the replay of a kept check, matching the acting outcomes' new field
@task_file n=18 path=packages/escher-driver/src/lib.rs
what: edited: the driver crate's module list and re-exports of the caused types
@task_file n=19 path=tests/blitz-tests/tests/stand_act_errors.rs
what: new: the stand check of caused on every carrier, both layout modes, on its own fixture page, with the command span check
@task_file n=20 path=tests/blitz-tests/tests/stand_explore.rs
what: edited: the stand check of exploration, a panicking path ending Panicked and its kept check
@task_file n=21 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited: the stand session checks' shared helpers, matching the acting outcomes' new field
@task_file n=22 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited: the stand check of the command span around acting calls, matching the acting outcomes' new field
@task_item n=1 part=touches
contract/escher-session-actions/answer: the answer of click, type, press, advance and scroll gains a caused member, last, present only when the step caused a panic, an error or a log record.
@task_item n=2 part=touches
contract/escher-session-help/answer: help, read from the verb table, now lists caused on the five acting verbs with its keys, closed words and limits.
@task_item n=3 part=touches
contract/escher-session-mcp/tools: the tools/list entries of the acting tools carry the caused field from the same verb table.
@task_item n=4 part=touches
contract/escher-session-mcp/result-bound: an acting tool's result can now be longer by the caused member; check any bound a row states against it.
@task_item n=5 part=touches
contract/escher-session-record/check-file and decision/session-recording-replayable-check: a kept check now records and replays caused, a differing count stops the replay naming caused, and a panicked exploration path's check holds the panicked step.
@task_item n=6 part=touches
decision/no-caller-input-echoed: caused writes back handler ids, owner ids, event names, closed words and counts, never caller input, a log message or a panic payload; the row's rule is held and newly exercised.
@task_item n=7 part=touches
decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot: every held session now starts with the reached-handlers record on and drains it each step.
@task_item n=8 part=touches
contract/escher-session-host/log-service-name: the host log's subscriber is rebuilt with the filter on the sink alone and a counting capture layer beside it; the sink's output and service name stay as they were.
@task_item n=9 part=touches
contract/escher-session-exit-codes/1 and contract/escher-session-exit-codes/3: a handler panic during a step is now caught and the session answers instead of failing; no exit code was added or reassigned.
@task_item n=10 part=touches
contract/escher-session-command/form: the command form and its verbs are unchanged, but an acting verb's result field count moved from 5 or 6 to 6 or 7; name it where a row holds that count.
@task_item n=11 part=touches
decision/mcp-server-local-stdio and contract/escher-session-mcp/stdio: serve_mcp in process carries the same caused member as the line carrier; no transport changed.
@task_item n=12 part=touches
decision/cold-agent-uses-driver-mcp: a cold agent driving through the MCP server now reads caused on its acting calls.
@task_fact n=1 slug=surface-cli
fact: An acting verb's answer carries caused last and only when non-empty: one item per kind (panic, error, log) and level (error, warn, info, debug, trace) with the first reached handler's id, owner and event where known and a count; no message, payload or Err text crosses.
@task_fact n=2 slug=surface-cli
fact: Log records from the Dioxus framework targets (dioxus, generational_box, warnings) count in caused only at warn and error; every other target outside the engine and escher prefixes counts at every level.
@task_fact n=3 slug=surface-desktop-native
fact: A handler Err the app's own root default boundary takes replaces the whole app on screen with the boundary's rendering of the error text; that rendering crosses as screen content, never through caused.
@task_fact n=4 slug=surface-cli
fact: A handler panic during an acting step is caught; the session answers that call with a panic item and keeps answering later calls.
@task_item n=1 part=untouched
The stand stable element ids: no stand id was added, renamed or removed; the new check runs on its own fixture page.
@task_item n=2 part=untouched
The ShellProvider window chrome controls: no window or chrome behaviour moved.
@task_item n=3 part=untouched
The multi_doc and paint_bench command lines: their arguments, environment and output are untouched.
@task_item n=4 part=untouched
The agent-run.sh and cold-agent.sh pipes: their usage, exit codes, logs and record dirs are unchanged.
@task_item n=5 part=untouched
The refusal rows (fonts-unavailable, off-screen, unknown-verb) and recording failures: no refusal or session error was added, and a refused call carries no caused.
@task_item n=6 part=untouched
The snapshot, screenshot, graph, lint and coverage answers and the served tasks: no field of theirs moved.
@task_item n=7 part=untouched
The stand layout decisions (pinned viewport, document skeleton, scroll into view, clipping boxes, open list rows, accent colour, a11y by attributes): no layout, scroll or paint rule changed.
@task_item n=8 part=untouched
The absence of arbitrary index.html templates: still absent.
