@row_set n=1 row=decision/headless-stand-no-sink cell=except seen=3 fact=F6 detector=none
value: The MCP tools and layout stand checks install their own recording subscriber; the command-span check's re-run children and the error-text founder check install escher's sink over an in-memory writer.
basis: fct-v77omrom
because: The error-text founder check installs escher's own sink in process over an in-memory writer at trace, a new exception to the headless stand running with no sink.
@row_set n=2 row=decision/checks-silent cell=except seen=3 detector=none
value: Telemetry and error-text checks install escher's sink, MCP tools and layout checks their own recorder, the paint-tree bench reads an opt-in env var, a re-run child prints ids or digests to its parent.
basis: fct-v77omrom
because: The error-text founder check installs escher's sink in process, so the silent-checks exception must name it beside the telemetry checks.
@row_set n=3 row=decision/answers-echo-nothing-typed cell=except seen=1 detector=D-obs-pii
value: A caused item's message, chain and kept fields hold what their author wrote, a content-named field's value reading [redacted].
basis: fct-xz256kkb
because: The caused answer now carries each record's text, scrubbed by field name only, so a message holds what its author wrote and the row needs that stated exception.
@row_set n=4 row=decision/driver-library-installs-no-subscriber cell=rule seen=2 detector=none
value: The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step and its chaining panic hook, both printing nothing; only a binary's own main installs the sink.
basis: fct-z2b5kqi7
because: The capture the driver opens now installs a once-per-process chaining panic hook, so «setting only the scoped capture» no longer says all the driver library sets.
@row_set n=5 row=decision/driver-emits-no-event cell=rule seen=2 detector=none
value: escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which keeps the step's records and panics in process and prints nothing; its telemetry is its spans alone.
basis: fct-wed52rrj
because: The capture's count-only contract is retired: it now keeps each record's text and the step's panics, so «counts records» is false.
@row_set n=6 row=decision/capture-framework-targets-warn-up cell=rule seen=1 detector=none
value: The scoped log capture admits the Dioxus framework targets (dioxus*, generational_box, warnings) and the engine targets only at warn and error, escher_ targets never, and every other outside target at every level.
basis: fct-z2b5kqi7
because: The capture now admits engine records at warn and above, which the row says it never counts.
@row_set n=7 row=contract/telemetry-scrub-set/error cell=promise seen=1 detector=D-obs-pii
value: An `error` field is in the scrub set, so the sink prints its value as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item; a failed fetch's error text reaches no carrier as a field value.
basis: fct-xz256kkb
because: Records now cross into the caused answer, scrubbed by the same field-name set, so «no carrier beyond its log line» no longer holds as written.
@row_set n=8 row=contract/telemetry-scrub-set/url cell=promise seen=1 detector=D-obs-pii
value: A `url` field is in the scrub set, so the sink prints its value as `[redacted]` and the scoped capture keeps it as `[redacted]` in a caused item.
basis: fct-xz256kkb
because: The capture scrubs every source with the sink's content-named set, so a url field is hidden in caused as on the sink's lines.
@row_set n=9 row=decision/escher-target-content-named-set cell=rule seen=1 detector=D-obs-pii
value: Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error, password and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
basis: fct-wbcflx2v
because: CONTENT_FIELDS gained password, so the row's list of redacted names is incomplete.
@row_retire n=1 row=decision/scrub-at-sink-not-call-sites seen=1 heir=decision/scrub-by-field-name detector=D-obs-pii
why: Scrubbing is no longer the sink's alone and engine call sites were edited to carry their values in named fields.
basis: fct-xz256kkb
because: The capture now scrubs every source by field name and the engine's warn and error sites were rewritten, both of which the row denies.
@row_add n=1 row=decision/scrub-by-field-name area=8-pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Scrubbing by field name in the sink and the capture
rule: Scrubbing is by field name, in escher's own sink, a closed span's line judged pair by pair by the same rule, and in the scoped capture for every source, where a content-named field reads [redacted] and the message and other fields are kept as written.
basis: fct-xz256kkb
because: The capture now scrubs every source with the sink's content-named set, keeping a message as its author wrote it, which replaces the sink-only scrubbing decision.
@row_add n=2 row=decision/engine-sites-fixed-messages area=8-pii-scrubbing-compliance fact=F7 detector=D-obs-pii
name: Engine log sites write fixed messages
rule: Engine and framework warn and error sites write fixed messages and carry their values in named fields (href, url, src, text, request), so the field-name scrub reaches them.
basis: fct-g5qt4zja
because: The six interpolating engine and framework sites now write fixed messages with named fields, a convention the field-name scrub relies on.
@row_add n=3 row=decision/capture-keeps-bounded-records area=6-log-coverage fact=F1 detector=none
name: Capture keeps bounded records
rule: The scoped log capture keeps each admitted record's message, target, level, file, line and up to 8 fields, cuts a text at 1024 characters saying how much it cut, and keeps up to 16 records per source, counting the rest per source.
guard: a_text_is_cut_on_a_char_boundary_and_says_how_much, fields_and_records_past_their_bounds_are_counted_by_source
basis: fct-z2b5kqi7
because: The capture's count-only contract is retired for kept, bounded records, and no row of the area states those bounds.
@row_add n=4 row=decision/capture-chaining-panic-hook area=7-error-capture-reporting fact=F3 detector=none
name: Capture's chaining panic hook
rule: escher-telemetry's scoped capture installs once per process a chaining panic hook that keeps a panic's payload and location on the capture open on the panicking thread, does nothing when none is open, always runs the hook it replaced, and prints, logs and fields nothing.
guard: the_hook_installs_once_and_a_panic_outside_a_capture_reads_as_before
basis: fct-z2b5kqi7
because: A second, process-wide panic hook now stands beside the sink's, and no row of the area says it exists or what it may do.
@row_add n=5 row=decision/caused-carries-error-text area=7-error-capture-reporting fact=F4 detector=none
name: Caused answer carries each failure's text
rule: The caused answer of click, type, press, advance and scroll carries one item per panic, boundary error and log, in that order, with message, location, an error's chain of up to 8 links and its boundary root or own, keeping the first 16 and counting the rest in caused_past by kind and source.
basis: fct-z2b5kqi7
because: Errors are now reported in the acting verbs' answer with their text and bounds, replacing the closed-words count, and no row of the area says so.
@row_add n=6 row=decision/boundary-registry-document-life area=7-error-capture-reporting fact=F5 detector=none
name: Every error boundary registered for the document's life
rule: Every error boundary, root or own, is registered for the document's life, and an acting step reads the errors new since its marks from all of them.
why: A dropped boundary cannot be told, the framework's boundary fields being private, so an app remounting many boundaries grows the registry.
basis: fct-os6dpgyz
because: A step now reads every boundary's errors through a registry that only grows, a stated trade no row of the area holds.
@row_none n=1 fact=F8 type=4
relied: the error-text founder check reads the engine's compiled-in tracing flag to tell a tracing build from a plain one
