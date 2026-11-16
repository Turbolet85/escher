@row_set n=1 row=decision/headless-stand-no-sink cell=rule seen=2 detector=none
value: A headless boot made in process installs no telemetry sink and reads no environment variable, its acting steps running only under escher-telemetry's scoped capture, which prints nothing; a headless boot made by the escher-session binary runs under that binary's sink.
basis: fct-r6izsq5e, fct-xxn4lhqe
because: An in-process session's acting step now runs under escher-telemetry's scoped capture default, so a plain «installs no telemetry subscriber» no longer holds and the row must name the capture as the one subscriber that prints nothing.
@row_set n=2 row=decision/driver-library-installs-no-subscriber cell=rule seen=1 detector=none
value: The driver library, its command line, MCP server and coverage reader included, installs no sink and reads no environment variable, setting only escher-telemetry's scoped capture around an acting step, which prints nothing; only a binary's own main installs the sink.
basis: fct-r6izsq5e, fct-xxn4lhqe
because: The driver now runs each acting step under escher-telemetry's scoped capture, a scoped default dispatcher, which contradicts the row's «installs no subscriber».
@row_set n=3 row=decision/driver-emits-no-event cell=rule seen=1 detector=none
value: escher-driver emits no log event and installs no sink, setting only escher-telemetry's scoped capture around an acting step, which counts records and prints nothing; its telemetry is its spans alone.
basis: fct-xxn4lhqe, fct-qyzrvae7
because: The driver now sets escher-telemetry's scoped capture during an acting step, contradicting «installs no subscriber», while it still emits no event and its command span is unchanged.
@row_set n=4 row=contract/telemetry-sink/rust-log cell=promise seen=1 detector=D-obs-stack
value: The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset; it filters the sink layer alone, the capture layer beside it unfiltered.
basis: fct-xxn4lhqe, fct-r6izsq5e
because: The subscriber is now the sink layer under the RUST_LOG EnvFilter beside an unfiltered capture layer, so the filter's reach must be stated as the sink's alone.
@row_set n=5 row=contract/escher-sink-env/rust-log cell=promise seen=1 detector=D-obs-stack
value: Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable, and apply to the sink layer alone, never to the capture layer beside it.
basis: fct-xxn4lhqe, fct-r6izsq5e
because: The EnvFilter now filters the sink layer alone with the capture layer unfiltered beside it, which the promise must say.
@row_add n=1 row=decision/acting-step-scoped-capture area=3-observability-harness-contract fact=F1 detector=D-obs-instrumentation
name: Acting steps run under a scoped log capture
rule: During each acting step of an in-process session the driver runs the step under escher-telemetry's scoped capture, which counts tracing records per level and prints nothing, using one process-wide pair of capture dispatchers, built once and never dropped, where the default holds no capture layer.
why: Two registered dispatchers keep tracing-core from taking a callsite's interest from one thread's default alone; a per-thread dispatcher died with its thread.
basis: fct-xxn4lhqe, fct-phkejc5f, fct-smn7bfku, fct-y36qbbsj
because: The chunk adds the scoped capture and its process-wide dispatcher pair, a decision with a stated reason that no row of the harness contract holds.
@row_add n=2 row=contract/acting-answer/caused area=7-error-capture-reporting fact=F2 detector=none
boundary: The acting answer (click, type, press, advance, scroll)
item: caused
promise: Present only when the step caused something: one item per kind (panic, error, log) and level, with the first reached handler's id, owner and event and a count, never a message, payload or error text; it sees only errors the root boundary takes and only tracing records.
basis: fct-vzyy52tx, fct-xxn4lhqe
because: The five acting verbs gain the caused result field, an outward promise at the answer boundary that no row holds.
@row_add n=3 row=decision/acting-step-catches-panics area=7-error-capture-reporting fact=F3 detector=none
name: A panicking acting step is caught
rule: A panic in an acting step's handler or settle is caught and the session keeps answering: the panic reads as a caused panic item, and a second panic in the settle reads busy render.
basis: fct-xxn4lhqe, fct-itglb53b
because: The run path now catches a panic in the acting step and settle, a behaviour the error-capture section did not hold beyond exploration.
@row_add n=4 row=decision/capture-framework-targets-warn-up area=6-log-coverage fact=F4 detector=none
name: Capture counts framework targets at warn and above
rule: The scoped log capture counts the Dioxus framework targets (dioxus*, generational_box, warnings) only at warn and error, engine and escher_ targets not at all, and every other outside target at every level.
why: The framework traces on every re-render and signal write, so counting it at every level would mark every ordinary step.
basis: fct-noe4u5rr, fct-qepwv2ae, fct-mj3euqii
because: The operator ruled the capture's target set, a decision with its reason that no log-coverage row holds.
@row_add n=5 row=decision/root-boundary-err-crosses-as-screen area=pii-scrubbing-compliance fact=F5 detector=D-obs-pii
name: A root-boundary Err crosses as screen content only
rule: An Err the root default boundary takes reaches the screen as the boundary's rendering of its Display, an existing screen-content crossing, and never reaches caused; an Err an app's own boundary catches leaves no mark.
basis: fct-nnu5onnh, fct-r6izsq5e, fct-ifrruoof
because: The operator narrowed the Err case so its text crosses only as the boundary's screen rendering, a crossing decision the PII section must hold.
@row_add n=6 row=contract/caused-answer/content area=pii-scrubbing-compliance fact=F6 detector=D-obs-pii
boundary: the caused member of an acting answer
item: content
promise: Carries ids, owner ids, event names, the closed kind and level words and counts only, never a message, payload or error text; a boundary widening, ratified.
basis: fct-xdxtksgn, fct-vxqzynyw
because: The caused crossing as built now stands on the founder's own word, no longer provisional, and no PII row holds it.
