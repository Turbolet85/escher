@row_set n=1 row=contract/driver-session/acting-verbs cell=promise seen=2 fact=F1 detector=none
value: `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the snapshot diff, with `caused` where the step caused something: at most 16 items in order, each text cut at 1024 bytes, the rest counted in `caused_past`; `scroll` adds `in_view`.
basis: fct-z2b5kqi7
because: The acting verbs' caused is now bounded to 16 kept items with cut texts and gains `caused_past` counting the rest, so the acting-verbs promise must state that bound.
@row_set n=2 row=decision/socket-carries-commands cell=rule seen=1 detector=none
value: The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff, a step's caused with its error text, and lint findings out.
basis: fct-f4fi3clh
because: A step's caused now carries each record's message and chain across the session socket, so the list of what the socket carries out is incomplete without it.
@row_set n=3 row=decision/root-boundary-err-on-own-session cell=rule seen=1 detector=none
value: A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain.
basis: fct-smxs2rkb
because: The split onto a session of its own is kept but the sentinel reading is inverted, the root Err's text now crossing in caused, so the row's caused-alone sentinel reading is false.
@row_add n=1 row=contract/driver-session/payload-unread area=session-lifecycle fact=F2 detector=none
boundary: Driver session
item: <payload unread>
promise: A caught panic the capture's hook did not see still answers a `panic` item in `caused` whose message is the fixed text `<payload unread>`, so the panic count never loses an item.
basis: fct-pdldgk3u
because: The answer gains a fixed-text panic item for a panic the hook missed, a promise at the driver session's boundary no row states.
@row_add n=2 row=decision/caused-fields-scrubbed-by-name area=integration-test-strategy fact=F3 detector=none
name: Caused fields scrubbed by name on every source
rule: A field of a caused record whose name is in the content-field scrub set, `password` among them, reads redacted on every source, the engine's included, while the message and every other field are kept whole.
guard: stand_act_error_text
basis: fct-xz256kkb
because: The capture now keeps record text and redacts by field name on every source, a scrub rule the checks assert that no row of the area states.
@row_add n=3 row=decision/mcp-result-decoded-whole area=integration-test-strategy fact=F4 detector=none
name: MCP tool result text decoded whole
rule: A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper.
why: Un-escaping quotes alone leaves a Windows path's backslashes doubled.
guard: stand_act_errors
basis: fct-xxjvdom5
because: The Windows red of CI#43940101161 set a convention for every MCP comparison check, held by one shared helper, which no row states.
@row_add n=4 row=decision/caused-text-kept-off-sink-lines area=integration-test-strategy fact=F5 detector=none
name: Caused text kept off the sink's lines
rule: No panic payload, error text or raw href a step caused reaches a line of escher's sink at any level up to trace.
guard: stand_act_error_text
except: The error-text check installs escher's own sink in process over an in-memory writer at trace, where the headless stand otherwise runs with no sink.
basis: fct-v77omrom
because: The error-text check reads the sink's lines at trace through an in-process sink, an exception to the sink-less headless stand that no row of the area states.
@row_add n=5 row=decision/engine-items-asserted-on-tracing-build area=test-strategy fact=F6 detector=none
name: Engine caused items asserted on a tracing build only
rule: A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds.
guard: stand_act_error_text
basis: fct-gvjd2ehp
because: The founder check runs on both builds and asserts the engine's warnings only under tracing, a build-conditional assertion rule no row states.
@row_add n=6 row=decision/expected-value-pinned-as-literal area=unit-test-strategy fact=F7 detector=none
name: Expected value pinned as a literal
rule: A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal.
why: A check computing its expectation from the mutated constant reads green under the mutation.
basis: fct-lkive45y
because: The cut control read does not hold until the flood check pinned its measured value, a rule on building controlled checks that no row states.
@row_none n=1 fact=F8 type=4
relied: A panic outside a capture reads as before, the capture's hook always running the hook it replaced.
