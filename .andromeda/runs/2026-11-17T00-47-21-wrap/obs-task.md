@task_prose part=session
The session let the error field of a caused item in an acting answer show its text as written, cut at the text bound, on every answer carrier: the command line, the session socket, MCP and a kept check's kept answer.
The per-step capture now hides fields by its own answer set of eleven names (url, href, src, html, text, value, attrs, path, request, panic.payload, password); error left that set, which was the sink's twelve-name content set before.
The telemetry sink is unchanged: its twelve content names, error among them, still print [redacted] on its lines, and a failed fetch's error text appears 0 times on the sink's lines.
The caused result help served by help and as each acting MCP tool's description now says a field named error and every message cross as their author wrote them, such as a failed fetch's URL; the help kept its line count.
Measured: an app warn item reads error as its text with url and path redacted; on the tracing build the engine's Resource load failed item holds the failed fetch's error text while the sink line reads error=[redacted].
The widening was ratified by the founder's word; no span, log field, event, cause, exit, verb, carrier or dependency was added or retired.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: escher-telemetry's per-step capture behind the caused items of an answer, holding the answer's own hidden field set and the kept field value it judges by field name
@task_file n=2 path=packages/escher-telemetry/src/format.rs
what: escher's telemetry sink line format, the scrub by field name with its content-named set, applied to event lines and closed span lines
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the driver's served help and acting MCP tool descriptions, including the caused result-field help naming which fields read [redacted]
@task_file n=4 path=packages/escher-driver/src/caused.rs
what: the driver's caused items of an acting answer, each record's level, target, message and fields as captured during the step
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check that every acting failure answers its error text in caused, crossing MCP in process, the session socket and a kept check, and that no sentinel reaches the sink's lines
@task_item n=1 part=touches
The scrub-set error item of escher-telemetry's scrub set: the sink still prints error=[redacted], but the capture now keeps error's text in a caused item, so a failed fetch's error text crosses the answer carriers as a field value.
@task_item n=2 part=touches
The decision on scrubbing by field name in the sink and the capture: in the capture a content-named field no longer reads [redacted] by the sink's set; the capture judges by its own eleven-name answer set, error not among them.
@task_item n=3 part=touches
The decision that answers echo nothing the caller typed: its exception for caused text scrubbed by field name now rests on the answer's own set, and error crosses as written, which can carry a URL the page or caller supplied.
@task_item n=4 part=touches
The scrub-set url item: url stays [redacted] in both the sink and the caused item; a row that reads both sets as one set is now stale even where url's value holds.
@task_item n=5 part=touches
Any row that holds the count of hidden names for the capture or the answer: twelve became eleven for the capture; the sink's twelve is unchanged.
@task_item n=6 part=touches
Rows on Escher targets redacting the content-named set and on the reach of the scrubbing sink: true for the sink's lines, but a row that extends that rule to the caused items of an answer now has error as an exception.
@task_item n=7 part=touches
The kept check file's answer and answer digest items: a kept answer's caused fields now hold error's text, so a check recorded with the old redacted value replays as a changed answer.
@task_item n=8 part=touches
The escher-session help command and the help served as acting MCP tool descriptions: the caused field help text changed in content, same line count.
@task_item n=9 part=touches
The failed-stylesheet error capture rows: a failed engine resource load now answers its error text, holding the fetch's URL, in the caused item's error field on the tracing build, while url itself stays redacted.
@task_item n=10 part=touches
Rows on session logs carrying no page content and on engine log sites writing fixed messages: they hold for the sink's lines, but the answer now carries error text that can hold page-supplied content such as a URL.
@task_fact n=1 slug=8-pii-scrubbing-compliance
fact: The answer's caused items are judged by their own hidden set of eleven field names, apart from the sink's twelve-name content set; error is the one name the sink redacts and the answer shows as written, cut at the text bound, by the founder's ratified widening.
@task_item n=1 part=untouched
Span coverage rows: the command and tool_call spans, their fields, endings and the refusal cause names did not change; no span or span field was added.
@task_item n=2 part=untouched
Metric coverage and the absences of a metrics exporter, a telemetry backend or an error-reporting service: still absent.
@task_item n=3 part=untouched
Agent-run and cold-agent harness rows: their events, verdict fields, transcripts, artifacts and cleanup are not reached.
@task_item n=4 part=untouched
CI integration rows: workflows, artifact uploads, recording bounds and private directories are unchanged.
@task_item n=5 part=untouched
The sink's installation, target allowlist, RUST_LOG handling, output stream, resource keys service.name and service.version, panic hook and telemetry installed event are unchanged.
@task_item n=6 part=untouched
Script error surface rows, the window error event and window.onerror arguments, the script diagnostics drain and the WPT report are not reached.
@task_item n=7 part=untouched
escher-session exit codes, the host wire, the coverage role and refusal causes are unchanged; no cause, exit or carrier was added.
