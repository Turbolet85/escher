@task_prose part=chunk
An acting answer's caused item gets a scrub set of its own: the log sink's content-field set without the field named error, so a record's error field crosses with its author's text instead of reading [redacted].
Every other content name (url, href, src, html, text, value, attrs, path, request, panic.payload, password) still reads [redacted] in a caused item's fields, on every carrier that carries caused: command line, socket call, MCP and a kept check's kept answer.
The log sink's stderr lines and its content-field set do not change: error still prints [redacted] there, now pinned by a sink unit test under an escher_* target.
The driver's help for caused's result fields and the caused module's docs are rewritten to say error crosses as written.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the per-step capture that keeps each record's fields, scrubbed and bounded, for the actions answer's caused item — edits: Values::keep, the KeptField doc, the answer's own scrub set beside the sink's
@task_file n=2 path=packages/escher-telemetry/src/format.rs mark=edits
what: escher's stderr log sink with its target allowlist and content-field redaction set — edits: its unit test pinning the sink's error rule under an escher_* target
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema's tool help, served by help and as the MCP tools' descriptions — edits: caused's result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs mark=edits
what: the actions answer's caused item, its causes and fields — edits: its module doc and the fields doc
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the error-text check of the actions answer, holding the founder check that installs escher's sink over an in-memory writer
@task_item n=1 part=needs
The actions answer's caused item fields: a field named exactly error now carries the record's text, still under the existing text cut, fields-per-record and records-per-capture bounds.
@task_item n=2 part=needs
Password and every other content-named field stay redacted in the caused item; the answer must never reveal password text or a value-bearing field.
@task_item n=3 part=needs
The log sink's redaction rule and target allowlist stay as they are: escher_* targets still redact error, url and the other content fields; other targets' records are still dropped whole.
@task_item n=4 part=needs
The driver's self-description: caused's result-field help in the driver schema, served by help and as MCP tool descriptions, changes its list of redacted names; no new verb, argument, node key, cause or session error.
@task_item n=5 part=needs
The keyed field on the carriers: the same caused fields cross unchanged in shape on command line, socket call, MCP results and a kept check's answer.
@task_item n=6 part=needs
The acceptance check: a real failure logged with error = %e from a handler the step reaches is read from the acting answer's caused alone with its error text, while a url or path field on the record still reads [redacted].
@task_item n=7 part=needs
The control: the sink's line for an engine record still prints error=[redacted] and url=[redacted], and an app record's error text occurs zero times on the sink's lines.
@task_item n=8 part=needs
Assertions are written as Rust tests: capture unit tests in the telemetry crate and the error-text check in the blitz test suite, reading the sink through an in-memory writer.
@task_standing n=1
convention: each new behaviour check is a Rust test asserted in the repo's test suites, run in the CI legs that hold the accessibility and stand checks
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new stand check survives the stand checks repeated three rounds
home: ci-integration
@task_item n=1 part=not_needed
Keyboard navigation: focus, Tab order, key activation, range input and select list key handling are not touched.
@task_item n=2 part=not_needed
Visual design verification: contrast, lint findings, colours, scrollbars and zoom are not touched.
@task_item n=3 part=not_needed
ARIA patterns and roles and the accessibility tree's states, hidden content and name reads: no role, state or accessible name changes.
@task_item n=4 part=not_needed
The snapshot, its diff, its text form and the UI graph: no snapshot reading or node key changes.
@task_item n=5 part=not_needed
The platform adapter, AccessKit wiring and the accessibility feature's defaults in the scope summary: no adapter or feature change.
@task_item n=6 part=not_needed
Cognitive accessibility: invalid-state presentation and stand focus order tables are not touched.
@task_item n=7 part=not_needed
Driver screenshots and driver click refusals: no screenshot or refusal cause changes.
