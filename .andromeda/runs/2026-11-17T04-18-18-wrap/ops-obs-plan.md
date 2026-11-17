@row_set n=1 row=decision/layout-read-opens-no-span cell=rule seen=1 detector=D-obs-instrumentation
value: The driver's layout read opens no span and emits no event of its own: read in process it writes no escher_ line at TRACE, and reached by the layout verb it runs inside that call's command span.
basis: fct-yx7iahh5, fct-stryxpwu, fct-dxvnlriy
because: The layout verb now reaches the read through the session's run, so the read no longer runs outside the command span, and the row must say it runs inside the layout call's span while opening none of its own.
@row_set n=2 row=contract/command-span/verb cell=promise seen=1 detector=D-obs-instrumentation
value: The verb table's own word for a known verb, run or refused, one of its fifteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
basis: fct-w5rznwmn
because: The verb table grew from fourteen to fifteen words with layout, so the count the promise states moved.
@row_set n=3 row=contract/tool-call-span/tool cell=promise seen=1 detector=D-obs-instrumentation
value: The verb table's word for the tool, one of its fifteen; unrecorded for an unknown tool.
basis: fct-w5rznwmn
because: The MCP tools and the verb table grew from fourteen to fifteen with layout, so the count the promise states moved.
@row_set n=4 row=contract/command-span/settled cell=promise seen=1 fact=F2 detector=D-obs-instrumentation
value: Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions, layout and a part read record verb alone, and a refusal records verb and cause alone.
basis: fct-yx7iahh5, fct-dxvnlriy
because: The layout outcome joins the non-acting outcomes whose command span records verb alone, so the promise's list of verb-alone reads must name it.
@row_set n=5 row=decision/sink-host-read-at-trace cell=rule seen=1 detector=D-obs-pii
value: A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name, typed-sentinel or layout-answer key needle in what they write.
basis: fct-dxvnlriy, fct-nnxgcxnx
because: The trace reads of the host and the mcp role now also hold the layout answer's keys out of the log, so the row's needle set is incomplete without them.
@row_add n=1 row=contract/command-span/layout area=log-coverage fact=F1 detector=D-obs-instrumentation
boundary: The driver's command span
item: layout
promise: A layout's line is `span="command" verb="layout"` (with `cause` when refused) and holds nothing of its answer; the layout read the call reaches opens no span of its own.
basis: fct-dxvnlriy, fct-yx7iahh5
because: The new layout verb's command span records verb alone when accepted and verb and cause when refused, a per-verb line contract like those of graph, lint and actions.
@row_add n=2 row=contract/layout-answer/content area=pii-scrubbing-compliance fact=F3 detector=D-obs-pii
boundary: the layout answer (layout)
item: content
promise: Carries ids, closed words and numbers only, never a value or typed text; the sink at trace carries none of its keys (laid_out, placed, shifts).
basis: fct-xa2wojve, fct-rdlzvbev, fct-nnxgcxnx
because: The new layout answer is a carrier of screen reading whose content class is a privacy contract, stated like the actions and graph answers' content rows.
@row_add n=3 row=contract/layout-answer/logging area=pii-scrubbing-compliance fact=F4 detector=D-obs-pii
boundary: the layout answer (layout)
item: logging
promise: The answer leaves on the session socket, the command line and MCP and is kept whole in a recorded check; it is never logged or fielded.
basis: fct-zud3mrzc, fct-gd5fq3ek, fct-rdlzvbev
because: The layout answer now crosses four carriers and rests in a kept check, and the rule that it is an answer and never a log or span value must stand as its own contract.
