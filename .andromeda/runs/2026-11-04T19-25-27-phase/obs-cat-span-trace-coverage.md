@carried n=1 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
why: The probe must add no span, so escher-driver must still hold exactly its 2 `info_span!` sites and `command` its eight fields, and `stand_act_spans` must still pass.
quote: `packages/escher-driver` and `packages/escher-telemetry` are untouched, escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`)
@carried n=2 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
why: The MountedData hazard sits on the mounted-event path. Any fix the founder chooses for it must leave that guard opening no span, and this chunk must not decide or land that fix.
quote: the mounted-event guard in `flush_queued_mounted_events` and `Harness::text_contrast`'s blend read open no span
@carried n=3 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: A probe that drives the stand through `Session::run` must not field or `Debug`-print what it reads. Component names, handlers and signal values join the driver's never-fielded set: masked, never logged.
quote: never fielded — `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its byte size or its `width` / `height`, a lint's findings or their count, the session's label, the record of ids, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`: an argument fielded by its schema name, or a `Command` or an `Outcome` fielded with `Debug`, would print an id, typed text, an accessible name or a control's value
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: The probe reads dioxus-native-dom, the bridge, and its bridge-side reads must open no span, emit no event and record no field, the same as the bridge reads already recorded.
quote: dioxus-native-dom's `Snapshot::text_parts`, the part read's cut, which open no span, emit no event and record no field
@carried n=4 run=2 label=obs-plan/span-trace-coverage fact=fct-gf4hwplk
quote: dioxus-native-dom's `is_select` and the driver's `focus_select` open no span
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-j47hr7dn
why: A probe check under `tests/blitz-tests/` must keep the test crate slice free of spans and traces. If it adds a file to the slice, the slice count it lands under must be re-measured, not carried over.
quote: observed absent — spans or traces · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files (test crate slice)
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-sicgzqzx
why: Probe code kept as example code under `examples/seven_guis/` must keep the examples slice free of spans.
quote: observed absent — spans · searched: `span!|instrument` over the 32 slice files (examples slice)
@carried n=7 label=obs-plan fact=fct-wa4xq2qh
why: If the probe reaches the stand over MCP, `tool_call` must keep its two closed-word fields and no other.
quote: two fields and no other, each a closed word — `tool`, the verb table's word, and `ending`, `accepted` · `refused` · `error`
@carried n=8 label=obs-plan fact=fct-dfgrfrux
why: The body's absence line must hold after the chunk: the probe instruments no function and opens no trace in any slice.
quote: Spans, instrumented functions and traces observed absent over the reading's apps, examples, s05, s06, layout, s09, blitz-vibey-script / blitz, s11, test crate and wpt runner slices and the 16 listed files, each search standing for its slice.
@binding n=1
**Span / Trace Coverage ↔ the chunk scope's boundary «Nothing crosses»**: the scope bars the probe from adding any span or field. This category's side is the measured count the chunk must still match: 2 `info_span!` sites, `command` with eight fields, `tool_call` with two (obs-plan/span-trace-coverage fct-xswdmupu) (obs-plan fct-wa4xq2qh).
@binding n=2
**Span / Trace Coverage ↔ obs-plan §8 PII Scrubbing and §6 Log Coverage**: the driver's never-fielded set (obs-plan/span-trace-coverage fct-zuu6oxcq) and the sink's scrub (obs-plan fct-cbcilbst) bind together. Nothing the probe reads (a component name, a handler, a signal's value) may become a span field, a log field or a `Debug` print.
@binding n=3
**Span / Trace Coverage ↔ the MountedData fork (the founder's)**: the mounted-event guard opens no span (obs-plan/span-trace-coverage fct-xswdmupu). The fix's options go to the founder undecided, and whichever option he picks must keep the span absence. This chunk lands no fix.
@binding n=4
**Span / Trace Coverage ↔ test-plan (the census counts and `stand_act_spans`)**: a probe file added under `tests/blitz-tests/` or `examples/seven_guis/` moves the slice counts that the absence searches stand for (obs-plan/span-trace-coverage fct-j47hr7dn) (obs-plan/span-trace-coverage fct-sicgzqzx). The wrap must re-measure them, and `stand_act_spans` must stay green.
