@row_add n=1 row=decision/declared-node-read-logs-nothing area=6-log-coverage fact=F1 detector=D-obs-instrumentation
name: The declared-node read logs nothing
rule: The in-process read of a canvas's declared virtual nodes, its validation and its answer, has no log site, print or field, and a refused declaration is a closed reason with its index alone, nothing of it logged.
state: provisional
basis: fct-wkaf2evl
because: The chunk adds the declared-node read with no log site, print, span or field and a refusal holding a closed reason and its index alone, a delegate's answer, and no row of the area says it.
@row_add n=2 row=decision/declared-node-read-opens-no-span area=4-span-trace-coverage fact=F2 detector=D-obs-instrumentation
name: The declared-node read opens no span
rule: The in-process read of a canvas's declared virtual nodes opens no span of its own, as the canvas read and the layout read do not.
basis: fct-wkaf2evl
because: The chunk adds the declared-node read opening no span, and the canvas-read row has no room under its bound to name a third read.
@row_add n=3 row=decision/virtual-node-names-stay-in-process area=8-pii-scrubbing-compliance fact=F3 detector=D-obs-pii
name: Declared virtual node names and keys stay in process
rule: A declared virtual node's name, key and reserved id stay in process: no snapshot, graph, span or log line carries them until their crossing is decided.
why: The crossing of a virtual node's name and key is a fork deferred, not answered.
guard: a_declaring_canvas_puts_nothing_on_the_snapshot_or_graph
state: provisional
basis: fct-jw55vkbf
because: The chunk adds app-declared names and keys that the read holds in process and no carrier crosses, deferred to the founder's fork, and no PII row says it.
