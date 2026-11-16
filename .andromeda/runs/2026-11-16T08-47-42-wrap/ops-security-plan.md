@row_set n=1 row=decision/driver-installs-no-subscriber cell=except seen=1 fact=F6 detector=none
value: During an acting step escher-telemetry sets a scoped capture dispatcher as default where none holds a capture layer: one process-wide pair, never dropped, its twin unused, printing nothing.
basis: fct-xxn4lhqe
because: An acting step now sets escher-telemetry's scoped capture dispatcher as the thread's default, an exception the row's no-subscriber rule must name.
@row_set n=2 row=decision/driver-no-subscriber-no-env cell=except seen=1 detector=none
value: During an acting step escher-telemetry sets a scoped capture dispatcher as default where none holds a capture layer: one process-wide pair, never dropped, its twin unused, printing nothing.
basis: fct-xxn4lhqe
because: An acting step now sets escher-telemetry's scoped capture dispatcher as the thread's default, an exception the row's no-subscriber rule must name.
@row_set n=3 row=contract/driver-answer/settled cell=promise seen=1 detector=none
value: A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the busy reading names a class only, never a URL, request, node, id, name or value.
basis: fct-xxn4lhqe, fct-vzyy52tx
because: An acting outcome now also carries caused, holding handler ids, so the class-only promise must be scoped to the busy reading it is about.
@row_add n=1 row=decision/caused-answer-crossing area=threat-model-summary fact=F1 detector=none
name: Caused answer crossing
rule: Each acting verb's answer (click, type, press, advance, scroll) carries caused on every ratified carrier only when the step caused something: by kind and level, closed kind and level words, the first reached handler's id, owner and event, and a count, never a message, payload, target or field value.
why: A boundary widening, ratified, narrower than the ratified crossing allows; the text is left to a follow-up.
basis: fct-vzyy52tx, fct-xdxtksgn, fct-vxqzynyw
because: The chunk adds the caused field to the five acting verbs' answers on every carrier, a new crossing no row states.
@row_add n=2 row=decision/root-boundary-err-as-screen-content area=threat-model-summary fact=F2 detector=none
name: Root-boundary Err crosses as screen content only
rule: A handler Err the app's root error boundary takes crosses only as screen content, through Dioxus's default boundary rendering of its Display, never through caused, which reports it as one error item holding no byte of its text.
why: A narrowing: the boundary's rendering is an existing screen-content crossing.
basis: fct-nnu5onnh, fct-ifrruoof
because: The operator ruled that the root boundary's rendering of an Err is the existing screen crossing and caused carries none of its text, a decision no row holds.
@row_add n=3 row=contract/caused-field/limits area=error-handling fact=F3 detector=none
boundary: The caused answer field
item: limits
promise: Records are tied to the first handler the step reached; an error an app's own error boundary catches gives no item; records sent through the log crate are not counted.
basis: fct-vzyy52tx
because: The caused field's help states these limits outward, a contract no row holds.
@row_add n=4 row=decision/acting-step-panic-caught area=error-handling fact=F4 detector=none
name: Acting-step panics caught
rule: A panic in an app handler during an acting step is caught with its payload never read, the answer carries a panic item and the session keeps answering; a panic in the settle runs the settle once more, and a second panic reads busy render.
basis: fct-xxn4lhqe, fct-itglb53b
because: The run path now catches handler and settle panics, a new error-handling decision no row states.
@row_add n=5 row=decision/scoped-log-capture-counts-only area=logging-monitoring fact=F5 detector=none
name: Scoped log capture counts by level only
rule: escher-telemetry's scoped log capture counts records by level and prints nothing, holding no message, field or target; it counts the Dioxus framework targets at warn and error only, never the engine and escher_ prefixes, and every other outside target at every level.
why: The framework traces on every re-render and signal write, which would otherwise read as caused on every click.
basis: fct-xxn4lhqe, fct-noe4u5rr, fct-qepwv2ae
because: The chunk adds a log capture beside the sink with a ruled target and level rule, which no logging row states.
