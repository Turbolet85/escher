@row_set n=1 row=contract/command-span/settled cell=promise seen=2 detector=D-obs-instrumentation
value: Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout, canvas and a part read record verb alone, and a refusal records verb and cause alone.
basis: fct-uhfo5wt4, fct-ez6ffs55
because: The canvas verb's command span records verb alone and, refused, verb and cause, so the verb-alone list must name canvas.
@row_set n=2 row=contract/command-span/verb cell=promise seen=2 detector=D-obs-instrumentation
value: The verb table's own word for a known verb, run or refused, one of its sixteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
basis: fct-snllpxfi, fct-uhfo5wt4
because: The verb table moved from fifteen to sixteen verbs with canvas, so the count the cell holds must follow.
@row_set n=3 row=contract/tool-call-span/tool cell=promise seen=2 detector=D-obs-instrumentation
value: The verb table's word for the tool, one of its sixteen; unrecorded for an unknown tool.
basis: fct-snllpxfi
because: The MCP tool listing and the verb table moved from fifteen to sixteen with canvas, so the count the cell holds must follow.
@row_set n=4 row=decision/sink-host-read-at-trace cell=rule seen=2 fact=F5 detector=D-obs-pii
value: A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel, layout-answer key or canvas needle in what they write.
basis: fct-ez6ffs55, fct-ol62onmg
because: host_log and mcp_log now read zero canvas needles at trace, so the needle list the decision names must include them.
@row_set n=5 row=decision/part-answer-never-logged cell=rule seen=2 detector=D-obs-pii
value: A part's answer, a snapshot part ({"text":…,"part":n,"parts":m}), a graph part ({"nodes":[…],"states":[…],"part":n,"parts":m}) or a canvas part, is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
basis: fct-x7cb4p36, fct-gr2bwf56, fct-f4jjdetj
because: The canvas full dump now answers in parts (Outcome::CanvasPart), so the decision's list of part answers is incomplete without it.
@row_add n=1 row=contract/command-span/canvas area=log-coverage fact=F1 detector=D-obs-instrumentation
boundary: The driver's command span
item: canvas
promise: A canvas's line, a part read's included, is `span="command" verb="canvas"` (with `cause` when refused) and holds no id, full flag, part number or anything of its answer.
basis: fct-uhfo5wt4, fct-ez6ffs55, fct-ol62onmg
because: The chunk adds the canvas verb, whose command span records verb alone and, refused, verb and cause, as its siblings' per-verb line rows state.
@row_add n=2 row=contract/canvas-answer/content area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
boundary: the canvas answer (canvas)
item: content
promise: The summary, the full dump and a part carry closed words, numbers and 64-bit digests only, never a glyph id, a glyph position, text, a font's bytes or a family name.
basis: fct-f4jjdetj, fct-ol62onmg
because: The chunk adds the canvas answer as a new content-bearing answer, and what it may hold is the PII contract the plan keeps for each answer.
@row_add n=3 row=contract/canvas-answer/logging area=pii-scrubbing-compliance fact=F3 detector=D-obs-pii
boundary: the canvas answer (canvas)
item: logging
promise: The answer leaves on the session socket, the command line, MCP and a kept check, kept whole in the recorded check, so canvas content crosses as the answer and not as shot pixels alone; it is never logged or fielded.
basis: fct-x7cb4p36, fct-fzvmboq3, fct-ol62onmg
because: Canvas content now crosses every carrier as the canvas answer, a new crossing the plan must state beside its never-logged promise.
@row_add n=4 row=decision/canvas-read-opens-no-span area=4-span-trace-coverage fact=F4 detector=D-obs-instrumentation
name: The canvas read opens no span
rule: The driver's in-process canvas read opens no span of its own, as the layout read does not; reached by the canvas verb it runs inside that call's command span.
why: No span of its own by design.
basis: fct-ol62onmg, fct-x7cb4p36
because: The chunk adds Session::canvas with no span by design, a decision future instrumentation work must respect.
@row_add n=5 row=decision/cold-agent-canvas-uncounted area=log-coverage fact=F6 detector=none
name: Canvas listed but not counted by the cold-agent pipe
rule: The cold-agent pipe lists canvas among its verbs but counts none of its calls, and the verdict holds no canvas counter until a served app holds a canvas.
why: No app the pipe serves holds a canvas yet.
basis: fct-snllpxfi, fct-rrwh4urw, fct-re2xbmyu
because: The pipe's verb list took canvas while its counted verbs stayed unchanged, a chosen limit the verdict rows must not be read past.
