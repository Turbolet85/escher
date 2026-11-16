@task_prose part=chunk
An acting answer's `caused` item stops redacting a record's field named exactly `error`: the per-step capture gets a scrub set of its own, the log sink's content set without `error`, so a Rust failure's text logged as `error = %e` reaches the answer.
Every other content-named field stays `[redacted]` in the answer, and the log sink's stderr lines, its content set and its two readers of that set stay unchanged.
The tool's self-description of `caused` (its result-field help, served by help and as the MCP tools' descriptions, and the caused module's docs) is made to say that `error` crosses as its author wrote it.
It re-asserts, under a unit test, that the sink still prints `error=[redacted]` under an `escher_*` target, and proves the opening with the error-text stand check.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the per-step capture that keeps each record's fields for the `caused` answer and judges which field values are scrubbed — edits: `Values::keep`, the `KeptField` doc, a new scrub set for the answer beside the sink's, the capture unit tests
@task_file n=2 path=packages/escher-telemetry/src/format.rs mark=edits
what: escher's stderr log sink formatter, home of the content-field scrub set `CONTENT_FIELDS`, the event rule and the span-field store that read it — edits: its unit test pinning the sink's `error` rule under an `escher_*` target
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the closed driver verb schema whose fixed help text is served by `help` and as the MCP tools' descriptions — edits: `caused`'s result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs mark=edits
what: the `caused` answer of an acting command, carrying each captured record's message, location, boundary, cause chain and kept fields — edits: its module doc, the `fields` doc
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the error-text stand check, holding the founder check that installs escher's log sink over an in-memory writer (a named exception) — edits: the acceptance check that a field named `error` crosses in `caused` while the sink's line stays redacted
@task_item n=1 part=needs
The answer's own scrub set: the log sink's content set less the one name `error`, read by the capture's field keeper; only the field named exactly `error` opens, and only in the `caused` answer.
@task_item n=2 part=needs
Every other content name stays hidden in a `caused` item's fields: `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `panic.payload`, `password` still read `[redacted]`; a field of another content name on the same record as an opened `error` (such as `url` or `path`) still reads `[redacted]`.
@task_item n=3 part=needs
The log sink's scrub is unchanged: its content-field set keeps `error`, a field named `error` under an `escher_*` target and under every target still prints `[redacted]` on stderr, the event rule and the span-field store stay on that set, and a unit test pins the `escher_*` rule for `error`.
@task_item n=4 part=needs
The sink drops a record of an app's target whole, writing no line, so an app record's error text occurs zero times on the sink's lines on every build; the engine's resource-load-failure warn (fields `url`, `waiting_nodes`, `error`) is written only in a build that compiled the engine's tracing, and there the sink's line reads `error=[redacted]` and `url=[redacted]` while the answer's item reads the error text.
@task_item n=5 part=needs
The caused-text crossing widens: the text of a field named `error` crosses on every carrier that carries `caused` — the command line, the session socket's `call` answer, the MCP tool result and a kept check's kept answer — and on no other carrier, log line, span or field.
@task_item n=6 part=needs
The widening's price: an error's text can hold what its author put into it, such as a failed fetch's URL, and it now reaches the answer and a kept check's kept answer whole; this is a boundary widening whose amendment of the masters stating the scrub is a card at the wrap, not this chunk's.
@task_item n=7 part=needs
The bounds on what the answer carries are unchanged: the text cut of 1024 bytes applies to the opened `error` text as to every kept text, and the fields kept per record and records kept per capture stay as they are.
@task_item n=8 part=needs
An item's message, location, boundary and cause chain stay as they already cross, whole; this chunk touches none of them.
@task_item n=9 part=needs
No new verb, argument, node key, refusal cause, session error, exit code, carrier or log field: the closed schema, cause set and session-error table take nothing.
@task_item n=10 part=needs
The `caused` result-field help is fixed program text in the closed schema, served by `help` and as the MCP tools' descriptions; it stops listing `error` among the names that read `[redacted]` and says `error` crosses as written, holding no caller input.
@task_item n=11 part=needs
Acceptance: a real failure logged with `error = %e` from a handler the step reaches is read from the acting answer's `caused` alone, its `error` field holding the error's text, not `[redacted]`; the control reads the sink's line for the same failure still printing `error=[redacted]`.
@task_item n=12 part=needs
The proving check installs escher's own log sink over an in-memory writer inside a stand test, the named founder exception to the test harness taking no tracing; the capture's own unit tests prove the answer's set and the sink's set apart.
@task_item n=13 part=needs
The masked control values and the masked stand-in are not opened by this: a field named `password` or `value` stays redacted in the answer, and every carrier keeps the value mask.
@task_standing n=1
convention: a new stand test takes no tracing subscriber and no style except where the scope names the exception — here the error-text check's sink over an in-memory writer
home: dependency-security
@task_standing n=2
convention: a new unit or stand test reads only closed content and prints no page content, caller input or captured text beyond what the answer it checks already carries
home: logging-monitoring
@task_item n=1 part=not_needed
Authentication and authorization rows: the chunk changes no access, token, guard or peer check.
@task_item n=2 part=not_needed
Secret management rows (CI secrets, signing material, the operator's login, the environment the driver reads): the chunk reads no new environment and touches no secret.
@task_item n=3 part=not_needed
Outbound request rows — the HTTP clients, fetch methods, per-host limits, CORS, same-origin, request timeouts, response size limits, file and data URL rules: the chunk makes and changes no request.
@task_item n=4 part=not_needed
Select list, form control, range, password painting, accessibility name and stable element id rows: the chunk reads and changes no control, element or id.
@task_item n=5 part=not_needed
HTML, CSS and SVG parsing, layout, paint, hit-testing and serialization rows: no document handling changes.
@task_item n=6 part=not_needed
Recording directory, state directory and session socket file-system rows (owner-only modes, cleanup, path length): no file or directory the session writes changes.
@task_item n=7 part=not_needed
Cold-agent, agent-run, apt-install, coverage-floor and WPT runner script rows: no script or harness outside the named check changes.
@task_item n=8 part=not_needed
Browser app storage rows — history database, cookie store, HTTP cache, clipboard, file dialog, TLS: none is reached.
