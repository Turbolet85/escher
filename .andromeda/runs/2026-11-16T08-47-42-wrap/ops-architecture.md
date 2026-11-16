@row_add n=1 row=contract/acting-verb-results/caused area=standard-contracts fact=F1 detector=D-arch-resources
boundary: acting verb results
item: caused
promise: Present only when the step caused something: one item per kind (panic, error, log) and level (error, warn, info, debug, trace), with the first reached handler's id, owner id and event name where one was reached, and a count; limited to the app root boundary and tracing records, not the log crate.
basis: fct-vzyy52tx (Schema / config), fct-xxn4lhqe (Symbols / APIs)
because: The five acting verbs gain the caused result field, a new contract item at the acting answer that no row holds.
@row_add n=2 row=decision/caused-crosses-the-carriers area=standard-contracts fact=F1 detector=D-arch-resources
name: caused crosses the carriers
rule: An acting answer's caused crosses socket call, command line and MCP and rests in a kept check, holding closed kind and level words, handler ids, owner ids, event names and counts, never a message, a payload or a NodeId.
why: A boundary widening, ratified, narrower than the ruling allows; no new verb, cause, session error, node key, exit or span field.
basis: fct-vzyy52tx (Schema / config), fct-xdxtksgn (Inputs: the PROVISIONAL lifted)
because: The caused member now crosses every carrier and rests in kept checks on standing ratification, a crossing no decision row records.
@row_add n=3 row=decision/capture-framework-targets-warn-and-error area=cross-cutting-patterns fact=F2 detector=D-arch-decisions
name: Framework targets captured at warn and error only
rule: The log capture counts records from targets outside the engine and escher prefix sets at every level, except the Dioxus framework targets (dioxus, generational_box, warnings), counted only at warn and error.
why: The framework traces on every re-render and signal write, so counting it at every level would put caused on every ordinary call.
basis: fct-qepwv2ae (Deviation 1), fct-noe4u5rr (Decisions & corrections)
because: The capture's target rule is a ruled decision new with this chunk that no row holds.
@row_add n=4 row=decision/capture-dispatcher-pair area=standard-contracts fact=F4 detector=D-arch-decisions
name: Capture's process-wide dispatcher pair
rule: Where the held default has no capture layer, the log capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls into its one-dispatcher interest mode.
why: Per-thread dispatchers die with their threads and leave tracing-core in its one-dispatcher mode; they were rejected.
basis: fct-phkejc5f (Deviation 3), fct-smn7bfku (Reverted / negative API facts)
because: The capture's dispatcher form is a chosen design with a rejected alternative, and no row records it.
@row_add n=5 row=decision/acting-step-caught-and-captured area=standard-contracts fact=F5 detector=D-arch-decisions
name: Acting step caught and captured
rule: The driver runs an acting step and its settle inside one log capture under a panic catch, so the session keeps answering after a caught panic, and a second panic in the settle reads busy render.
basis: fct-xxn4lhqe (Symbols / APIs: settled_step)
because: The run path now catches a handler panic and captures the step's records, behaviour every acting call relies on that no row states.
@row_add n=6 row=decision/root-boundary-err-reads-one-error area=standard-contracts fact=F6 detector=D-arch-decisions
name: Root boundary Err reads one error item
rule: A handler Err the app root error boundary takes reads one error item in caused, and the boundary's rendering of its Display reaches the screen as ordinary screen content, never caused; an Err the app's own boundary catches leaves no mark.
why: Ratified as a narrowing: the rendering is an existing screen-content crossing.
basis: fct-nnu5onnh (Decisions & corrections), fct-r6izsq5e (Expected amendments)
because: The error kind of caused rests on the root boundary alone, a ruled limit no row states.
@row_add n=7 row=contract/reached-record/take-step-reached area=standard-contracts fact=F7 detector=D-arch-resources
boundary: Reached record
item: take_step_reached
promise: Drains the handlers reached since the last drain, each once, in first-reached order.
basis: fct-xxn4lhqe (Symbols / APIs: dioxus-native-dom)
because: A new public method of the Dioxus document lands on the reached record's boundary and is registered nowhere.
@row_add n=8 row=contract/dioxus-native-dom-api/root-error-mark area=standard-contracts fact=F7 detector=D-arch-resources
boundary: dioxus-native-dom public API
item: root_error_mark and RootErrorMark
promise: Public and re-exported: root_error_mark reads the root error boundary's error context; RootErrorMark compares by identity and its Debug prints its name alone.
basis: fct-xxn4lhqe (Symbols / APIs: dioxus-native-dom)
because: A new public reader and type land on dioxus-native-dom's public API and are registered nowhere.
@row_add n=9 row=decision/capture-before-own-recorder area=conventions fact=F8 detector=none
name: Capture opened before a check's own recorder
rule: A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder.
why: So no call site's interest is cached from a thread with no subscriber.
basis: fct-y36qbbsj (Changes after implement: 22ffa0e85)
because: The operator's CI fix sets a rule every recorder-installing stand check must follow, and no row holds it.
@row_add n=10 row=decision/driver-depends-on-telemetry area=standard-contracts fact=F9 detector=D-arch-resources
name: Driver depends on escher-telemetry for the step capture
rule: escher-driver depends on escher-telemetry, a workspace path crate, for the log capture of its acting step.
basis: fct-wrfoebd6 (Dependencies), fct-xqptp2lr (Crates / modules)
because: A new crate edge lands between two workspace crates and is registered nowhere.
@row_set n=1 row=decision/escher-telemetry-sink cell=rule seen=1 fact=F3 detector=D-arch-decisions
value: Escher binaries install escher-telemetry at start: one stderr-only sink stamping the service identity, bridging `log` records and chaining the panic hook, the EnvFilter filtering the sink layer alone beside an unfiltered capture layer that counts by level and prints nothing.
basis: fct-xxn4lhqe (Symbols / APIs: escher-telemetry)
because: The subscriber is now a registry with the EnvFilter on the sink layer and a capture layer beside it, which the row's one-sink picture leaves out.
@row_set n=2 row=contract/telemetry/init cell=promise seen=1 detector=D-arch-resources
value: init installs once a subscriber holding one stderr sink under the RUST_LOG filter and an unfiltered capture layer beside it, answering ForeignSubscriber, never a panic, when another subscriber is global.
basis: fct-xxn4lhqe (Symbols / APIs), fct-r6izsq5e (Expected amendments: contract/telemetry/init)
because: init's subscriber now holds a capture layer the filter does not reach, so the one-sink promise is incomplete.
@row_set n=3 row=contract/telemetry/target-filter cell=promise seen=1 detector=D-arch-decisions
value: The sink drops whole at every level a target outside the engine and escher prefix sets; the capture layer counts such records and prints none.
basis: fct-xxn4lhqe (Symbols / APIs), fct-qepwv2ae (Deviation 1)
because: Records of outside targets are no longer dropped by the whole subscriber, since the capture layer counts them.
@row_set n=4 row=decision/driver-library-side-effect-free cell=rule seen=1 detector=D-arch-decisions
value: The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var.
basis: fct-xxn4lhqe (Symbols / APIs: settled_step), fct-xqptp2lr (Crates / modules)
because: The driver now runs each acting step inside escher-telemetry's capture, which sets a scoped default where none holds a capture layer.
@row_set n=5 row=decision/driver-spans-only cell=rule seen=2 detector=D-arch-decisions
value: The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
basis: fct-xxn4lhqe (Symbols / APIs: settled_step)
because: An acting step now runs under escher-telemetry's scoped capture default, so «installs no subscriber» holds only of a global one.
@row_set n=6 row=decision/escher-driver-spans-ungated cell=rule seen=1 detector=D-arch-decisions
value: escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no global subscriber.
basis: fct-xxn4lhqe (Symbols / APIs: settled_step)
because: An acting step now runs under escher-telemetry's scoped capture default, so «installs no subscriber» holds only of a global one.
@row_set n=7 row=contract/driver-explore/path-ending cell=promise seen=1 detector=none
value: An explored path ends accepted (settled or not), refused with its cause, panicked, or diverged at a step; a handler panic, caught by the run path and read from its answer's panic item, ends the path panicked and the exploration goes on.
basis: fct-xxn4lhqe (Symbols / APIs: explore's run_caught), fct-r6izsq5e (Expected amendments)
because: A panicked path is now read from the accepted answer's panic item rather than a per-path catch alone.
@row_set n=8 row=contract/answer-json/outcome cell=promise seen=1 detector=none
value: An object keyed by its verb's result fields in the table's order, busy present only when not settled and caused only when the step caused something.
basis: fct-vzyy52tx (Schema / config: written form)
because: The outcome object gains a conditional caused member, written last and only when non-empty.
@row_set n=9 row=contract/busy-classes/render cell=promise seen=1 detector=none
value: A busy class word of an instance that did not go quiet, and of an acting step whose settle panicked a second time.
basis: fct-xxn4lhqe (Symbols / APIs: settled_step)
because: A second panic in the settle now reads busy render, a new meaning of the word.
@row_set n=10 row=decision/accessibility-feature-gating cell=rule seen=2 detector=D-arch-resources
value: The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated.
basis: fct-xqptp2lr (Crates / modules: dioxus_native_dom::boundary, ungated)
because: The new boundary module is ungated, which the row's closed list of ungated modules leaves out.
