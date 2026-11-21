@row_set n=1 row=contract/handler-coverage-limits/bubbling-dispatch cell=promise seen=1 detector=none
value: A bubbling dispatch that still propagates counts every listener it passed; one its target or an ancestor stops counts its innermost listener alone, and a non-bubbling event its target alone.
basis: fct-xyhdhgxl, fct-vb3cx46e, fct-63kznktn
because: The reached record now reads the stop, so the stated limit that a bubbling dispatch counts every ancestor listener is retired and the promise must state the stop-read count.
@row_set n=2 row=decision/handler-coverage-bubbling-counts-ancestors cell=rule seen=1 fact=F1 detector=none
value: A bubbling dispatch that still propagates counts every listener it passed as exercised; a dispatch stopped by its target or an ancestor counts its innermost listener alone, and a handler's panic unwinding out of the dispatch counts as a stopped dispatch.
basis: fct-xyhdhgxl, fct-i44kzdy2, fct-aiuylfmm
because: The coverage record now reads stopPropagation and writes an unwound dispatch as stopped, so the rule stating the stop unread is false.
@row_set n=3 row=contract/graph-command/argument cell=promise seen=1 detector=none
value: graph takes one optional argument, part, a whole number from 1 to 1000; any other argument, or a part of 0, 1001 or a non-number, is refused malformed with nothing run.
basis: fct-xyhdhgxl, fct-jc4dsh7s
because: graph gained the part argument, so an argument to graph is no longer refused as such.
@row_set n=4 row=contract/escher-session-command-line/graph cell=promise seen=1 detector=none
value: `graph` takes `--part` with one value, a whole number 1 to 1000, anything else refused `malformed`; a whole answer is one line `{"nodes":[…],"states":[…]}`, a part one line `{"nodes":[…],"states":[…],"part":n,"parts":m}` with `states` in part 1 alone, status `0`.
basis: fct-xyhdhgxl, fct-63kznktn
because: graph is now read in parts at the snapshot budget through `--part`, so the stated no-argument form and its one answer shape are false.
@row_set n=5 row=contract/escher-session-command-line/command-form cell=promise seen=2 detector=none
value: `escher-session <verb> … --session <dir>` is the whole command line, fifteen verbs; the instance-level verbs take their arguments as flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` and `--part` each with one value, `--shift` and `--ctrl` bare).
basis: fct-xyhdhgxl, fct-uq4pdh6a
because: press gained the bare flag `ctrl`, so the list of bare flags must name `--ctrl` beside `--shift`.
@row_set n=6 row=contract/escher-session-command-line/usage-line cell=promise seen=2 detector=none
value: Usage reads `snapshot [--part <part>]`, `graph [--part <part>]` after `lint`, press's `[--ctrl]`, `start <app> [--record <dir>]`, `serve <app> --session <dir> [--record <dir>]`, `mcp [--record <dir>]` and `coverage <app> <check>...`, `layout --id <id>` after `actions`, `help` after `stop`.
basis: fct-xyhdhgxl
because: The usage line now carries ` [--ctrl]` and `graph [--part <part>]`, which the stated usage line lacks.
@row_set n=7 row=contract/driver-session/graph cell=promise seen=1 detector=none
value: Returns `Outcome::Graphed` holding the screen's whole `UiGraph` when no part is asked and the answer is within the snapshot budget; else `Outcome::GraphPart`, one part of whole nodes with its `part` and `parts`, the states in part 1 alone and a part past the last empty.
basis: fct-xyhdhgxl
because: A new public `Outcome::GraphPart` arm answers a graph cut into parts, so a graph no longer always returns `Outcome::Graphed`.
@row_set n=8 row=contract/session-wire/graph-answer cell=promise seen=1 detector=none
value: Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed`, booleans and a part's `part` and `parts` counts — never a value or typed text, and no `NodeId`.
basis: fct-xyhdhgxl, fct-jc4dsh7s
because: A graph part's answer adds the `part` and `parts` numbers, which the closed list of what the answer carries omits.
@row_set n=9 row=decision/graph-answer-holds-no-value cell=rule seen=1 detector=none
value: A graph answer carries ids, component names, handler event names, state names, closed flag words, booleans and a part's part and parts counts only, never a value such as typed text.
basis: fct-xyhdhgxl, fct-jc4dsh7s
because: A paged graph answer now carries the part and parts numbers, which the rule's closed list of what an answer carries omits.
@row_add n=1 row=contract/driver-session/press-ctrl area=session-lifecycle fact=F5 detector=none
boundary: Driver session
item: press ctrl
promise: `press` takes the optional bare flag `ctrl`, held as the host's action key: Meta on macOS, Control elsewhere.
basis: fct-xyhdhgxl, fct-y6no544q
because: press gained the ctrl flag joining the host's action modifier, witnessed on the macOS and Windows CI legs, and no row of the area states it.
@row_none n=1 fact=F2 type=4
relied: a test comparing call arguments after a kept check's round trip through Check::from_bytes compares them by name, the reader returning them in name order.
@row_none n=2 fact=F3 type=4
relied: a stand step reaching the Cells cell editor with a key clicks cell-input first, the cell's click opening the editor unfocused.
@row_none n=3 fact=F4 type=4
relied: a check building a set of rows that are not adjacent on a focused multiple select uses press with ctrl, press space after plain arrows flipping the current row out.
