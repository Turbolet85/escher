@row_set n=1 row=contract/acting-verb-results/caused cell=promise seen=1 fact=F2 detector=none
value: Present only when the step caused something: an item per panic, error or record: kind, level, source, target, message cut at 1024 bytes, location, an error's boundary and chain, a log's scrubbed fields, first reached handler; panics, then errors new since the boundary marks, then records; 16 kept
basis: fct-z2b5kqi7, fct-wed52rrj, fct-pdldgk3u
because: The closed-words caused shape counted per kind and level is retired for one item per record carrying its text, ordered and bounded, so the row's promise no longer holds.
@row_set n=2 row=decision/driver-library-side-effect-free cell=except seen=2 fact=F4 detector=D-arch-decisions
value: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced.
basis: fct-z2b5kqi7, fct-smxs2rkb
because: The capture the driver opens now installs a process-wide panic hook, the stated exception to a side-effect-free driver library, which the row must name.
@row_set n=3 row=decision/no-node-id-across-session cell=rule seen=1 detector=D-arch-decisions
value: Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, closed words, booleans and counts cross a driver session, never a `NodeId`.
basis: fct-z2b5kqi7, fct-xz256kkb
because: The acting verbs' caused now carries record messages, locations, cause chains and scrubbed fields across every carrier, so the row's closed list of what crosses is false.
@row_set n=4 row=decision/no-echo-of-typed-input cell=except seen=1 detector=none
value: caused carries an app's own messages and fields as their author wrote them, scrubbed by field name, so an app that writes typed text into a record echoes it.
basis: fct-s5bltahf, fct-xz256kkb
because: A step's caused now crosses with the app's record text kept as written, scrubbed only by field name, so an ending can hold what the caller typed where the app wrote it.
@row_set n=5 row=contract/answer-json/outcome cell=promise seen=2 detector=none
value: An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16.
basis: fct-z2b5kqi7, fct-xz256kkb
because: The five acting verbs gain caused_past written after caused only when non-empty, a conditional field the row's list of conditional fields must hold.
@row_add n=1 row=contract/acting-verb-results/caused-past area=standard-contracts fact=F1 detector=D-arch-resources
boundary: acting verb results
item: caused_past
promise: Written after caused only when non-empty on click, type, press, advance and scroll: one entry per kind and source counting the items past caused's 16, a panic or error entry writing no source.
basis: fct-z2b5kqi7, fct-pi3u54rh
because: A new result field of the five acting verbs lands on every carrier and stands beside its sibling acting verb results nowhere yet.
@row_add n=2 row=contract/telemetry-capture/sources area=standard-contracts fact=F3 detector=D-arch-resources
boundary: escher-telemetry capture
item: sources
promise: The capture keeps text from three sources: app targets at every level, framework and engine targets at warn and error, escher targets never.
basis: fct-z2b5kqi7, fct-wed52rrj
because: The capture's count-only contract is retired for kept text admitted by source, engine records at warn and above now admitted, and no row states it.
@row_add n=3 row=contract/telemetry-capture/scrub area=standard-contracts fact=F3 detector=none
boundary: escher-telemetry capture
item: scrub
promise: A field named in CONTENT_FIELDS reads [redacted] on every source, the message and every other field kept.
basis: fct-xz256kkb
because: The capture's kept text is scrubbed by field name for every source, apart from the sink's own engine rule, and no row states it.
@row_add n=4 row=contract/telemetry-capture/bounds area=standard-contracts fact=F3 detector=none
boundary: escher-telemetry capture
item: bounds
promise: Each kept text is cut at 1024 bytes, saying how much; at most 8 fields a record and 16 records a source are kept, the rest counted.
basis: fct-z2b5kqi7
because: The capture's kept text is bounded by fixed cuts and counts that callers read, and no row states them.
@row_add n=5 row=contract/dioxus-native-dom-api/boundary-marks area=standard-contracts fact=F5 detector=D-arch-resources
boundary: dioxus-native-dom public API
item: boundary_marks and BoundaryMark
promise: Public and re-exported: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and boundary_marks reads them by identity; the registry grows for the document's life, since a dropped boundary cannot be told.
basis: fct-z2b5kqi7, fct-os6dpgyz
because: A new public read of every error boundary lands beside root_error_mark and is what the driver now reads, so the public API rows must register it.
@row_add n=6 row=contract/telemetry/subscriber area=standard-contracts fact=F6 detector=D-arch-resources
boundary: escher-telemetry bootstrap
item: subscriber
promise: Public: returns the sink's subscriber with the capture layer, built uninstalled, at a given LevelFilter.
basis: fct-z2b5kqi7, fct-q3lnt7nx
because: A private builder became a public entry point of escher-telemetry, a new API the bootstrap's contract rows must register.
@row_add n=7 row=contract/blitz-dom-crate-root/tracing-compiled area=standard-contracts fact=F7 detector=D-arch-resources
boundary: blitz-dom crate root
item: TRACING_COMPILED
promise: Public: true when the engine's tracing feature is compiled in.
basis: fct-z2b5kqi7
because: blitz-dom's crate root exports a new public constant, which the crate root's contract rows must register.
@row_add n=8 row=decision/engine-log-values-in-named-fields area=conventions fact=F8 detector=none
name: Engine log values in named fields
rule: Engine warn and error log sites carry their values in named fields under fixed messages and never interpolate a value into the message.
basis: fct-z2b5kqi7, fct-g5qt4zja, fct-wbcflx2v
because: The engine's interpolating warn and error sites went to none, each value moved into a named field under a fixed message, a convention future engine log sites must keep.
@row_add n=9 row=decision/mcp-result-text-decoded-whole area=conventions fact=F9 detector=none
name: MCP result text decoded whole
rule: A stand check comparing an MCP tool result's text against the in-process answer decodes the result's JSON string whole through the one shared stand-check helper.
why: Un-escaping quotes alone leaves a Windows path's backslashes doubled.
basis: fct-xxjvdom5, fct-zbtw6u7h
because: A Windows CI red showed a partial un-escape leaves the comparison unmatched, and the fix made whole decoding through one shared helper the rule for every such check.
@row_none n=1 fact=F10 type=5
what: The membership and count of the content field set is held by the code and the observability plan's scrub rows; architecture names the set only by its name.
