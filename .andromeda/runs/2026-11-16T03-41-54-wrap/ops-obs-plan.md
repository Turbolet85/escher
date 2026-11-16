@row_set n=1 row=decision/checks-silent cell=except seen=2 detector=none
value: Telemetry checks install escher's sink, the MCP tools and layout stand checks their own recorder, the paint-tree bench reads one opt-in env var, a re-run child prints ids or digests for its parent.
basis: fct-dbtpzdmk, fct-iqboitei
because: stand_layout installs a subscriber of its own for its TRACE capture of the layout read, a check the row's named exceptions did not cover.
@row_set n=2 row=decision/headless-stand-no-sink cell=except seen=1 detector=none
value: The MCP tools and layout stand checks install their own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.
basis: fct-dbtpzdmk, fct-iqboitei
because: stand_layout's TRACE capture installs a recording subscriber over an in-process headless boot, an exception the row did not name.
@row_add n=1 row=decision/layout-read-opens-no-span area=4-span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: The layout read opens no span
rule: The driver's in-process layout read opens no span and emits no event, runs outside the command span, and writes no escher_ line at TRACE.
why: No span by design, under the decision that the driver emits no event.
guard: stand_layout's the_read_writes_no_escher_line_at_trace
basis: fct-iqboitei, fct-igtskrq7, fct-x3q3dkxj
because: A new driver read was added with no instrumentation by design, and §4 must say so, so the absence is not read as drift.
@row_add n=2 row=decision/layout-read-changes-nothing area=log-coverage fact=F2 detector=none
name: The layout read changes nothing
rule: The layout read runs no step, pump, tick or settle: the snapshot, both focus readings, the diff and the clock stay unchanged, an open list stays open and delivered Timer ticks stay unapplied.
guard: stand_layout's the_read_changes_nothing, the_read_runs_no_pass_that_would_apply_pending_ticks
basis: fct-5qpmkurf, fct-x3q3dkxj
because: A new read was added whose promise is that it changes nothing, which future work on the driver must keep.
@row_add n=3 row=decision/layout-answer-never-logged area=pii-scrubbing-compliance fact=F3 detector=D-obs-pii
name: The layout answer holds no content and is never logged
rule: The layout read's answer holds string ids, closed words for placer, shift, size cause and length, and numbers only, never page text, a value or a NodeId, and is never logged.
basis: fct-x3q3dkxj, fct-igtskrq7, fct-5qpmkurf
because: A new public answer type was added, and its content bound and its staying off every log line are a PII decision §8 must hold.
@row_add n=4 row=decision/layout-read-in-process-only area=new fact=F4 detector=none
section: Layout read carriers
name: The layout read stays in process only
rule: No verb, MCP tool, socket call or kept check reaches the layout read, and the verb table stays at fourteen.
state: provisional
basis: fct-x3q3dkxj, fct-svl7n53z, fct-u7f3beyp, fct-woiwjiy7
because: The absence of every carrier for the new read is applied ahead of the founder's word, and no section of obs-plan holds it.
@row_none n=1 fact=F5 type=9
what: An owed item routed to the «Cold-agent bar» entry by the approved card; the route holds it.
@row_none n=2 fact=F6 type=4
relied: The driver's layout read relies on the engine reader answering None for an unresolved id, a non-element, an unstyled element or a boxless one, and never panicking.
