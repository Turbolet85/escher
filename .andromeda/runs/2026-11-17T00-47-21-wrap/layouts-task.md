@task_prose part=session
The session opened the error field of a caused item: on every answer carrier (command line, socket call, MCP, a kept check's replayed answer) a field named error now shows the text its author wrote, cut at the existing text bound, where it used to read [redacted].
The answer's per-step capture now judges fields by its own hidden-name set of eleven names (url, href, src, html, text, value, attrs, path, request, panic.payload, password); error left it, so twelve hidden names became eleven for the answer.
The log sink's own set of twelve content names, error among them, and every sink line are unchanged: the sink still prints error=[redacted].
The caused result-field help, served by help and as each acting MCP tool's description, was rewritten in place (six lines for six): error is no longer named among the redacted fields, and it says an error field and every message cross as their author wrote them, such as a failed fetch's URL.
The error-text stand test grew from seven to eight tests: an app warn with a parse error, and on the tracing build a failed stylesheet fetch whose engine error text reaches the caused item while the sink line keeps error=[redacted].
After implement, two doc comments were changed to name the answer's hidden set as plain code text rather than a link, after CI's documentation job failed on it; no crate, verb, argument, exit, cause, carrier, endpoint, env var or log field was added or retired.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: the telemetry per-step capture that gathers each acting step's caused records and scrubs their fields by name before they cross the answer; edited (new answer hidden-name set without error, doc comments)
@task_file n=2 path=packages/escher-telemetry/src/format.rs
what: the telemetry log sink's line format and its content-name scrub set; edited (one line, set and lines unchanged)
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the driver's verb and result schema, the help text served by help and as the acting MCP tools' descriptions; edited (caused result-field help)
@task_file n=4 path=packages/escher-driver/src/caused.rs
what: the driver's caused item, the per-step list of what the process logged that an acting answer carries; edited (doc comments only)
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand test of an acting step's error text in the answer across command line, MCP in process, session socket and kept check, with its own failing-fetch page; edited (8 tests, a parse-warn step, a failed-fetch case)
@task_item n=1 part=touches
Caller input never written back (the no-echo decision): its exception that caused text is scrubbed by field name now reads by the answer's own eleven-name set; a caused item's error field now carries author-written text, which can hold a failed fetch's URL.
@task_item n=2 part=touches
escher-session actions answer contract: the caused list on an acting answer now shows error as written (cut at the text bound) while url, path and the other content names still read [redacted].
@task_item n=3 part=touches
escher-session help answer contract: the caused result-field help text changed (error no longer listed among redacted names; error and every message cross as written); same line count.
@task_item n=4 part=touches
escher-session MCP server tools/list contract: each acting tool's description carries the same rewritten caused help, so the tool listing bytes changed.
@task_item n=5 part=touches
Session recording as a replayable check and the check-file contract: a kept check now replays caused items whose error field holds text, and a changed text stops the replay naming caused.
@task_item n=6 part=touches
escher-session MCP server result bound, and any row holding the text cut bound on a caused field value: the error text crosses cut at the existing text bound (no bound value moved).
@task_item n=7 part=touches
escher-session host log contract and any row over the sink's lines: the sink still prints error=[redacted] and its twelve content names are unchanged; a row saying the answer and the sink share one scrub set is now stale.
@task_item n=8 part=touches
Load page alone has a network provider: the error-text test builds its own failing load page with its own failing network provider outside the stand; the stand's load page is unchanged.
@task_item n=9 part=touches
escher-session served tasks load contract, read only where it names what a failed fetch answers: a failed fetch's engine error text now reaches the caused item on the tracing build.
@task_fact n=1 slug=surface-cli
fact: In an acting answer's caused item, a field named error crosses every carrier as its author wrote it, cut at the text bound, while url, href, src, html, text, value, attrs, path, request, panic.payload and password read [redacted]; the log sink still prints error as [redacted].
@task_item n=1 part=untouched
Stand stable element ids and every stand control's key: no id, key or element was added, renamed or removed.
@task_item n=2 part=untouched
ShellProvider window chrome controls and the desktop window layout: no window, chrome or viewport change.
@task_item n=3 part=untouched
Stand accessibility attributes, accent colour, lints and fixed document skeleton: no stand markup or style changed.
@task_item n=4 part=untouched
escher-session exit codes, refusals, snapshot, screenshot, graph, lint and coverage answers: no exit, refusal, part or answer shape moved.
@task_item n=5 part=untouched
agent-run.sh, cold-agent.sh, multi_doc and paint_bench command lines: no argument, env var, exit or output changed.
@task_item n=6 part=untouched
MCP server stdio transport, protocol revisions and end: the server's framing and lifecycle are unchanged; only tool description text moved.
@task_item n=7 part=untouched
Mobile-native and web-spa surfaces and the decisions log: nothing in this session reaches them.
