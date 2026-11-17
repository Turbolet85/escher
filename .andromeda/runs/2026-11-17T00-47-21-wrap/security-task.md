@task_prose part=session
In a caused item of an acting answer, a field named error now crosses as its author wrote it, cut at the text bound, on every answer carrier: command line, session socket call, MCP, and a kept check's kept answer.
The per-step capture now judges fields by the answer's own hidden set of eleven names (url, href, src, html, text, value, attrs, path, request, panic.payload, password); error left that set, so the capture's hidden names went from twelve to eleven.
The sink's content-named set (twelve names, error among them) and its stderr lines are unchanged: the sink still prints error=[redacted] and the error text appears 0 times on its lines.
The widening rests on the founder's own word, relayed (inputs I1): a failed fetch's error text, which can hold its URL, now crosses the answer while the url field stays [redacted].
The caused result-field help served by help and as each acting MCP tool's description now says error and every message cross as written; no verb, argument, cause, exit, carrier, endpoint, env var, span or log field was added, and no dependency moved.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: the scoped per-step capture that builds each caused item's fields for the answer, now holding the answer's own hidden field set apart from the sink's content names; its doc comments edited after implement to name that set as code text
@task_file n=2 path=packages/escher-telemetry/src/format.rs
what: escher's allowlist stderr telemetry sink, its content-named field set (twelve names) and the redaction of content-named fields on escher_ targets; edited by one line, its rule unchanged
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the closed driver verb schema and its help text, served by help and as each acting MCP tool's description; the caused result-field help edited
@task_file n=4 path=packages/escher-driver/src/caused.rs
what: the caused items an acting verb's answer carries (what failed, where, from which source, with its fields); doc text edited
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: the stand check that caused error text crosses the command line, MCP in process, the session socket and a kept check, and that no sentinel or raw href reaches the sink's lines, including a failed fetch on the tracing build
@task_item n=1 part=touches
Caused text crossing: a content-named field no longer always reads [redacted] in a caused item; error crosses as written, holding what its author wrote such as a failed fetch's URL, a boundary widening ratified on the founder's word.
@task_item n=2 part=touches
Value mask holds on every carrier: its exception that caused text is scrubbed by field name now means the answer's own eleven-name set, error not among them.
@task_item n=3 part=touches
Session socket carries what the screen reads: its claim that caused text is scrubbed by field name is stale in its set; the socket now carries error text as written.
@task_item n=4 part=touches
escher's allowlist telemetry sink: its exception or claim that the answer's capture shares the sink's content-name list is stale; the capture reads its own set, the sink's twelve names and lines unchanged.
@task_item n=5 part=touches
Boundary widening rule: this session is a widening of what crosses an answer carrier (error text in caused), ratified by the founder's relayed word; a row that records ratified widenings gains this one.
@task_item n=6 part=touches
MCP server help contract: the help text and each acting tool's description changed in the caused result-field lines (error no longer listed as reading [redacted]); byte-equal on both surfaces, line count unchanged.
@task_item n=7 part=touches
Resource load failure log error and network tracing log error: the sink still prints error=[redacted], but the same record's error text now crosses the answer's caused item; a row saying that text never leaves the process is stale.
@task_item n=8 part=touches
MCP result crossing and the session socket reply rows: a caused item in an MCP result or socket reply now holds error text as written, still bounded by the text cut and the result size bound.
@task_item n=9 part=touches
Kept check rows (kept check keeps each answer whole, kept check call-content class, recorded check answer): a kept answer's caused item now holds error text as written, replayed and compared whole.
@task_item n=10 part=touches
No echo of caller input, command line echoes nothing typed, session output holds no caller input: indirect, through the caused item; error text is what the app or engine author wrote, never caller-typed, but any row reading caused as fully scrubbed by content names needs its exception updated.
@task_item n=11 part=touches
Escher targets scrub or withhold content-named fields and log sink redacts and allowlists: true for the sink still, but a row that extends that scrub to the caused answer now misstates it; the answer's set is eleven names.
@task_item n=12 part=touches
Screen content stays out of logs: indirect; the sink keeps error redacted, the change widens only the answer, so the row holds unless it claims the answer and the log share one set.
@task_fact n=1 slug=threat-model-summary
fact: Two scrub sets stand apart: the answer's capture hides eleven field names (url, href, src, html, text, value, attrs, path, request, panic.payload, password) in a caused item, while the stderr sink redacts twelve content names, error among them; error therefore reads [redacted] on sink lines and crosses the answer as written, cut at the text bound.
@task_item n=1 part=untouched
Authentication and authorization rows, and the CI token, publish and signing rows: no auth, CI workflow or release surface moved.
@task_item n=2 part=untouched
Dependency security rows: no dependency, pin, audit ignore or lockfile change.
@task_item n=3 part=untouched
Secret management and environment variable rows: no env var read or added, no secret touched.
@task_item n=4 part=untouched
Socket and MCP transport, bound, idle expiry, listener and request-grammar rows: no endpoint, port, verb, argument or request kind added; the sole listener and its bounds stand.
@task_item n=5 part=untouched
Error handling rows for refusal causes, refusal JSON, session errors, exit codes and MCP protocol errors: no cause, exit or error kind added or moved.
@task_item n=6 part=untouched
Input validation rows on select lists, ranges, stable element ids, accessibility names, password and file control masking: the control-value mask and the id rules are unchanged.
@task_item n=7 part=untouched
Data protection storage rows (history database, HTTP cache, cookies, TLS, clipboard, file dialog) and outbound request rows (per-host limits, schemes, GET-only fetches): none reached.
@task_item n=8 part=untouched
Command span and tool_call span contracts: no span or span field added; span lines unchanged.
