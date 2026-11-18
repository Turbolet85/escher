@row_set n=1 row=contract/cold-agent-verdict/capability-unused cell=promise seen=2 detector=none
value: The reason given when a counted verb, a part read or a list opening went unused, or when any of panic, error and log in caused_by_kind reads 0.
basis: fct-vkogrj6o
because: The chunk makes capability-unused also fire when any of caused_by_kind's three kind counts reads 0, so the row's set of what goes unused must name it.
@row_set n=2 row=contract/cold-agent-verdict-reasons/capability-unused cell=promise seen=2 detector=none
value: Given when a counted verb, a part read or a list opening went unused, or when any of panic, error and log in caused_by_kind reads 0.
basis: fct-vkogrj6o
because: capability-unused's set gains caused_by_kind, each of its three kind counts at 0 giving the reason, so the row's condition must follow.
@row_set n=3 row=contract/escher-session-exit-codes/2 cell=promise seen=1 detector=none
value: Usage: one fixed line on stderr built from `VERBS` and the seven task names, stdout empty.
basis: fct-wnmdhla7
because: The served-task table gains order as its seventh entry and the usage line's apps name it, so the six task names the promise holds are now seven.
@row_set n=4 row=decision/checks-silent cell=except seen=4 detector=D-obs-instrumentation
value: Telemetry and error-text checks install escher's sink, MCP tools and layout checks a recorder; the paint-tree bench reads an opt-in env var; a re-run child prints ids or digests; the order page logs.
basis: fct-wnmdhla7
because: The order page the stand checks pin adds one fieldless info log site under its own target, so the rule that pinned code adds no log site must name it as an exception.
@row_add n=1 row=contract/cold-agent-verdict/caused-by-kind area=6-log-coverage fact=F1 detector=none
boundary: The cold-agent verdict
item: caused_by_kind
promise: An object of the closed kind words panic, error and log to the count of accepted click, type, press, advance and scroll answers whose caused list holds an item of that kind, each kind counted once per answer; an answer that is not JSON is skipped; also on run.end.
basis: fct-vkogrj6o
because: The verdict gains a new field at the pipe's boundary, an object of closed words to counts read from the acting answers, and no row states it.
@row_add n=2 row=decision/cold-agent-caused-read-by-kind-only area=log-coverage fact=F2 detector=D-obs-pii
name: Cold-agent pipe reads a caused item by its kind only
rule: The cold-agent pipe reads nothing of a caused item in an acting answer but its kind, and skips an acting answer that is not JSON rather than counting it.
basis: fct-wnmdhla7
because: The pipe now reads the transcript's acting answers for their caused items, and the bound on what it reads of them, the kind word alone, is a rule no row states.
@row_add n=3 row=contract/caused-item/owner area=pii-scrubbing-compliance fact=F4 detector=none
boundary: The caused item of an acting answer
item: owner
promise: The owner the UI graph reads for the component that caused the item, left out where the graph reads none, as for a component keyed by HTML ids under no keyed element.
basis: fct-sedefl7e
because: Measurement disproved the plan's claim that each caused item carries its control's owner, showing the item's owner is the graph's, and no row states that promise.
@row_add n=4 row=decision/app-crates-take-tracing-directly area=telemetry-strategy fact=F5 detector=D-obs-stack
name: App crates take tracing themselves
rule: An app crate that logs declares the tracing crate as its own dependency, since the native renderer's prelude re-exports no tracing.
basis: fct-mqjftslj
because: The example app crate began logging and had to take tracing itself as no re-export reaches it, a convention for every logging app crate that no row states.
@row_none n=1 fact=F3 type=4
relied: The order page writes one fieldless info record under its own target seven_guis::tasks::order on a good Submit, which the scoped capture reads into a caused log item and the stderr sink's allowlist drops; the stand's caused checks and host_log rely on it.
