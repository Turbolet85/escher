@task_prose part=chunk
An acting answer's caused item stops hiding a record's field named exactly error: the answer gets a scrub set of its own, the log sink's content set without error, and the per-step capture judges a field by that set.
Every other content name (url, href, src, html, text, value, attrs, path, request, panic.payload, password) still reads [redacted] in a caused item's fields; the text cut, fields per record and records per capture are unchanged.
The log sink's stderr lines and its content set do not change: a field named error still prints [redacted] there, pinned by the telemetry crate's own unit test.
The tool's self-description of caused follows: the result-field help served by help and as the MCP tools' descriptions, and the caused module's docs, say error crosses as its author wrote it.
A new check reads a real failure's error text from the acting answer's caused alone, with a control that the sink still scrubs it and that another content name stays hidden.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs mark=edits
what: the per-step capture of log records that feeds an acting answer's caused items, and its unit tests — edits: Values::keep, the KeptField doc, the answer's own scrub set beside the sink's, the capture unit tests
@task_file n=2 path=packages/escher-telemetry/src/format.rs mark=edits
what: the log sink's line format and its content-field scrub set, unchanged — edits: a unit test pinning the sink's error rule under an escher_* target
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's schema of verbs and result fields, served by help and as the MCP tools' descriptions — edits: caused's result-field help
@task_file n=4 path=packages/escher-driver/src/caused.rs mark=edits
what: the caused item of an acting answer, the records that crossed during the action — edits: the module doc, the fields doc
@task_file n=5 path=tests/blitz-tests/tests/stand_act_error_text.rs mark=edits
what: the error-text check of an acting answer's caused, with the founder check that installs escher's sink over an in-memory writer
@task_item n=1 part=needs
The caused item of an acting answer: a record's field named exactly error now carries the error's text as its author wrote it, bounded by the existing text cut, instead of [redacted].
@task_item n=2 part=needs
Every other content field name stays [redacted] in a caused item's fields; only the one name error opens, and only in the answer.
@task_item n=3 part=needs
The change reaches caused on every carrier that carries it alike — the command line answer, the socket's call, the MCP tool results and a kept check's kept answer — with no new verb, argument, node key, cause, session error, exit status, carrier or log field.
@task_item n=4 part=needs
The help answer and the MCP tools' descriptions, which serve caused's result-field help: their text changes to say error crosses unredacted while the other content names read [redacted].
@task_item n=5 part=needs
The escher-session output is read by an agent, not a person: the widened caused field stays plain machine-read text in the answer's existing shape, its field order and line shape unchanged.
@task_item n=6 part=needs
The log sink's stderr lines are not changed: a field named error still prints [redacted] under every target, and the sink's event rule and span-field store keep reading its own content set.
@task_item n=7 part=needs
An item's message, location, boundary and cause chain are untouched; they already cross whole, and the text cut, fields per record and records per capture keep their bounds.
@task_item n=8 part=needs
The new check proves a real failure whose text sits in a field named error is read from the acting answer's caused alone, with a control that the sink's line still prints error=[redacted] and that another content name on the same record (url or path) still reads [redacted] in the answer.
@task_item n=9 part=needs
The sink's rule for error under an escher_* target is pinned by a unit test of the telemetry crate, and the answer's own scrub set by capture unit tests.
@task_standing n=1
convention: a new check of escher-session's answer reads it as an agent reads it — plain machine text, no colour and no terminal test
home: surface
@task_item n=1 part=not_needed
Colour, typography, spacing, depth, border-radius and brand rows: the chunk paints nothing and sets no visual value.
@task_item n=2 part=not_needed
Motion rows — the virtual clock, advance, settle, scroll timing and animation: the chunk changes what a caused item carries, not when an action runs or settles.
@task_item n=3 part=not_needed
Iconography rows — cursors, the cursor-icon crate, favicons: untouched.
@task_item n=4 part=not_needed
Select list rows — its overlay, events, accessibility tree, colours, gestures and viewport placement: the chunk reaches no form control.
@task_item n=5 part=not_needed
Screenshot and snapshot rows, including fonts-unavailable refusals and select-list row bounds: neither verb's answer changes.
@task_item n=6 part=not_needed
The escher-session tasks rows (counter, crud, cells, timer, flight-booker, load, unknown task): no task is added or changed.
@task_item n=7 part=not_needed
The actions, coverage and UI graph lines and their arguments and answers: those answers carry no caused item and do not change.
@task_item n=8 part=not_needed
Shell provider, window, JS window object and dioxus-native configuration rows: no window, viewport or config behaviour changes.
@task_item n=9 part=not_needed
WPT runner output and result colour rows: the runner is not reached.
@task_item n=10 part=not_needed
MountedData-during-render and buffered render replay rows: the render path is not reached.
