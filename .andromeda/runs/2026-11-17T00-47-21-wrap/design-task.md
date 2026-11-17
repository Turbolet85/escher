@task_prose part=session
A caused item's error field in an acting verb's answer now shows the text its author wrote, such as a failed fetch's URL, on every answer carrier: command line, session socket call, MCP role and a kept check's replayed answer.
The answer's per-step capture now hides eleven field names (url, href, src, html, text, value, attrs, path, request, panic.payload, password) in place of the sink's twelve; error left the hidden set, cut at the usual text bound.
The sink's log lines are unchanged: error still prints as [redacted] there, and its twelve names stand.
The help line for caused's result fields was rewritten, six lines for six: error no longer reads as [redacted], and a field named error and every message cross as their author wrote them; help serves it, and so does each acting MCP tool's description.
After implement, two doc comments name the answer's hidden set as plain code text instead of a link, because CI's documentation job refused a public doc linking a private item.
No verb, argument, node key, cause, exit status, carrier, endpoint or log field was added or removed.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: the per-step capture that gathers each step's logged records into the answer's caused fields and hides content-named field values as [redacted]; edited (new answer hidden set without error, doc lines)
@task_file n=2 path=packages/escher-telemetry/src/format.rs
what: the telemetry sink's line formatter and its content-name redaction rule for log lines; edited (one doc line, behaviour unchanged)
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the driver's verb schema and help text, served by the help command and as each acting MCP tool's description, including caused's result-field help; edited
@task_file n=4 path=packages/escher-driver/src/caused.rs
what: the caused item an acting verb's answer carries, its level, target, message and fields; edited (doc text only)
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand test that checks each failure's caused text across the command line, MCP in process, the session socket and a kept check; edited (one app warn case and one failed-fetch engine case added)
@task_item n=1 part=touches
The help text the escher-session help command prints and each acting MCP tool's description carries changed: caused's result-field help no longer names error among the fields that read [redacted], and it says error and messages cross as their author wrote them.
@task_item n=2 part=touches
The MCP help tool result and the other MCP tool results now carry a caused item whose error field holds its author's text, not [redacted].
@task_item n=3 part=touches
The command line output line that agents read for an acting verb now carries a caused item's error field as written, such as a failed fetch's URL.
@task_item n=4 part=touches
The rule that escher-session's output is read by an agent, not a person, now has an answer whose caused error text is the author's own words, unscrubbed except for its length cut.
@task_item n=5 part=touches
Any row saying a caused item's content-named fields, error among them, read [redacted] in an answer is now stale for error; the answer hides eleven names, the log sink still twelve.
@task_item n=6 part=touches
Any row that counts the hidden field names of the answer as twelve, or names the sink's set as the answer's set, holds a moved value: the answer's set is now its own, eleven names.
@task_item n=7 part=touches
A kept check that replays an acting verb's answer now replays caused items whose error field holds text, so a row on what a kept check's replayed answer holds is reached.
@task_item n=8 part=touches
The stdout line of the mcp role and of the command line keep their shape; only the value inside a caused item's error field changed, so a row on their content is reached but not its form.
@task_item n=1 part=untouched
Color palette, typography, spacing, depth, border radius and iconography rows: no colour, font, spacing, paint order, radius or cursor changed.
@task_item n=2 part=untouched
Motion rows, the virtual clock, scrolling and animation: no timing or verb behaviour moved.
@task_item n=3 part=untouched
Select list rows, its events, colours, overlay, accessibility and snapshot bounds: untouched.
@task_item n=4 part=untouched
WPT runner output and result colours, the shell provider, the JS window object, the dioxus-native configuration and the web-spa surface: untouched.
@task_item n=5 part=untouched
The screenshot and snapshot verbs, the actions, coverage and UI graph lines, the escher-session tasks and their arguments: no argument, node key or answer shape changed.
