@task_prose part=chunk
An acting answer's caused item stops redacting a record's field named exactly `error`: the per-step capture judges fields by a scrub set of its own, the log sink's content set without `error`.
Every other content name (url, href, src, html, text, value, attrs, path, request, panic.payload, password) still reads `[redacted]` in a caused item; the sink's content set, its two readers and its stderr lines are unchanged.
The tool's self-description follows: the caused result-field help in the driver schema (served by help and as the MCP tools' descriptions) and the caused module doc say `error` crosses as its author wrote it.
It proves the opening on the error-text stand check and the capture unit tests, and pins the sink's own `error` rule under an escher_* target with a telemetry unit test.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the per-step capture that keeps each record's fields for the acting answer's caused items, and its inline unit tests — edits: Values::keep, the KeptField doc, the answer's own scrub set beside the sink's, the capture unit tests
@task_file n=2 path=packages/escher-telemetry/src/format.rs mark=edits
what: the log sink's line format and its content-field scrub set, read by the event rule and the span-field store — edits: its unit test pinning the sink's `error` rule under an escher_* target
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, the verb and result-field help served by help and as the MCP tools' descriptions — edits: caused's result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs mark=edits
what: the caused answer item built from captured records, crossing on every carrier — edits: its module doc, the fields doc
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the error-text stand check of the acting answer's caused items, holding the founder check that installs escher's sink over an in-memory writer
@task_item n=1 part=needs
The acting call result's caused item: a record's field named exactly `error` now holds the error's text, not `[redacted]`, on every carrier that carries caused — the command line, the socket's call, MCP and a kept check's kept answer.
@task_item n=2 part=needs
Every other content-named field on a caused item (url, href, src, html, text, value, attrs, path, request, panic.payload, password) still reads `[redacted]`; a record carrying both `error` and `url` or `path` shows the text in one and the redaction in the other.
@task_item n=3 part=needs
The sink's stderr lines are unchanged: a field named `error` still prints `[redacted]` under an escher_* target and under every target; the sink drops an app target's record whole, so an app record's error text occurs zero times on the sink's lines.
@task_item n=4 part=needs
The sink's content-field set itself and its two readers (the event rule, the span-field store) are unchanged; only the capture's read moves to the answer's own set.
@task_item n=5 part=needs
The text cut (1024 bytes), the fields kept per record and the records kept per capture are unchanged; the opened `error` text is scrubbed and bounded by the same cut as any kept text.
@task_item n=6 part=needs
An item's message, location, boundary and cause chain are untouched — they already cross whole.
@task_item n=7 part=needs
No new verb, argument, node key, refusal cause, session error, exit status, carrier or log field; the wire, the MCP tool list and the session errors keep their shape.
@task_item n=8 part=needs
The help text for caused changes: the schema's help is served by help on the command line and as the MCP tools' descriptions, so help parity across command line and MCP and any check pinning the capability statement or claim names to the schema must still read it.
@task_item n=9 part=needs
The control on the engine's Resource load failed warn (fields url, waiting_nodes, error): the answer's item reads the error text while the sink's line reads error=[redacted] and url=[redacted]; that engine item is asserted only on a build that compiled the engine's tracing.
@task_item n=10 part=needs
The stand check installs escher's sink over an in-memory writer: the sink host's output is drained, the sink's capture counted by target, and log silence read on a host with a sink.
@task_item n=11 part=needs
An MCP result's caused comparison decodes the tool result text whole before reading the `error` field.
@task_item n=12 part=needs
No ids, names, typed text or screen content reach the stand logs or the session host's stderr; a typed password never leaves the instance — the opening is of the `error` field alone, and password stays hidden.
@task_item n=13 part=needs
The sink's `escher_*` rule for `error` is pinned by escher-telemetry's own unit test in its inline test module, with the expected line as a literal.
@task_item n=14 part=needs
New tests are controlled red-before-green: each assertion that `error` reads its text, and that another content name and the sink's line stay redacted, fails against the old shared scrub set.
@task_item n=15 part=needs
Per-crate coverage floors for escher-telemetry and escher-driver hold with the new answer set and its tests.
@task_standing n=1
convention: each new unit test sits inline in its source file's test module, uses the built-in harness, and prints no argument value on failure
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=2
convention: each new test is controlled red-before-green with its expected value pinned as a literal; a table-driven test asserts its row count
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=3
convention: each new or extended stand check runs on the bundled font with no skip, owns its host, reads no wall time or sleep, and runs in both layout modes
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: each stand check holds a zero-failure budget across the repeated stand rounds in CI
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_standing n=5
convention: test data is stated in code, nothing seeded; shared stand tables and fixtures live in one shared module that is no test target
home: 3-test-harness-contract
@task_item n=1 part=not_needed
WPT conformance, the WPT runner and its statuses, expectations and pinned commit: the chunk touches no layout or conformance behaviour.
@task_item n=2 part=not_needed
The cold-agent pipe, its events, verdict, isolation and exit codes, and the agent-run verbs, selections and events: no harness-metadata or agent-run surface changes.
@task_item n=3 part=not_needed
The harness's contrast, lint defects and findings, screenshot and pixel tests: no reading of the screen changes.
@task_item n=4 part=not_needed
Driver refusal causes, their order and remedies: no refusal is added or changed.
@task_item n=5 part=not_needed
The virtual clock, advance, timer ticks and settle: no time or settle behaviour changes.
@task_item n=6 part=not_needed
Handler coverage and its coverage line, the graph and actions commands: no listener or graph reading changes.
@task_item n=7 part=not_needed
CI action pinning, secrets, permissions, package installs and artifacts: no workflow file or CI job changes.
@task_item n=8 part=not_needed
Mocking discipline and network stand-ins: no stand-in is added.
@task_item n=9 part=not_needed
Windowed end-to-end runs and the buffered-render and select-list paint decisions: out of reach of the answer's scrub.
