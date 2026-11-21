@row_set n=1 row=decision/ui-graph-crossing cell=rule seen=1 detector=none
value: The UI graph crosses socket call, command line and MCP and rests in a kept check as the graph verb's answer, whole or in parts: stable ids, owner ids, component, state and handler event names, declared writes, flags undeclared and unconfirmed, booleans, part counts; no value, typed text or NodeId.
basis: fct-xyhdhgxl
because: The graph verb now answers a graph over the snapshot budget in parts carrying part and parts, so the row's single text-block crossing no longer states the whole crossing.
@row_set n=2 row=contract/graph-verb/arguments cell=promise seen=1 detector=D-arch-resources
value: The graph verb takes one optional part, a whole number from 1 to 1000; a part of 0, of 1001 or of text is refused malformed with nothing run.
basis: fct-xyhdhgxl
because: GRAPH.args gains part as ArgKind::Part 1 to 1000, so the promise that graph takes no argument is false.
@row_set n=3 row=decision/large-screen-in-parts cell=rule seen=1 detector=none
value: A screen's snapshot or UI graph over the snapshot budget crosses in parts, one within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line.
basis: fct-xyhdhgxl
because: The graph now pages at the snapshot budget under the same ratified widening, so the rule naming only the screen's text is incomplete.
@row_set n=4 row=decision/reached-recorded-before-handlers cell=rule seen=1 detector=none
value: Where a reached record is switched on, the chain's nodes whose current listeners hold the event's name are resolved by stable id before the handlers run and written after: all while it propagates, the innermost alone when stopped or unwound by a panic, a non-bubbling event's target alone.
basis: fct-xyhdhgxl
because: The reached record is now resolved before the dispatch and written after it with the stop read, so recording every node before the handlers run is false.
@row_set n=5 row=decision/reached-record-before-dispatch cell=rule seen=1 fact=F1 detector=none
value: The reached record resolves the chain's nodes whose current listeners hold the event before it runs, holding no borrow across the render, and writes them after: all while it propagates, the innermost alone when stopped or a handler's panic unwinds, a non-bubbling event's target alone.
basis: fct-i44kzdy2
because: The record is written after the dispatch with the stop read and a drop guard writes an unwound dispatch as stopped, so the row's write before the event runs is false.
@row_set n=6 row=contract/handler-coverage/bubbling cell=promise seen=1 detector=none
value: A bubbling dispatch that still propagates counts every listener it passed; a dispatch a listener stops, or a handler's panic unwinds, counts its innermost listener alone.
basis: fct-vb3cx46e
because: The limit that a stopPropagation is not read is retired, the reached write now reading the stop.
@row_set n=7 row=contract/graph-verb/argument cell=promise seen=1 detector=D-arch-resources
value: `graph` takes one optional `part`, a whole number from 1 to 1000; a part of 0, of 1001 or of text is refused `malformed` with nothing run.
basis: fct-xyhdhgxl
because: GRAPH.args gains part as ArgKind::Part 1 to 1000, so the promise that graph takes no argument is false.
@row_set n=8 row=contract/graph-verb/no-values cell=promise seen=1 detector=none
value: The answer `{"nodes":[…],"states":[…]}`, a part's adding `part` and `parts`, holds ids, component, state and event names, closed flag words, booleans and the part counts only, never a value.
basis: fct-xyhdhgxl
because: A graph part is written as the graph keys plus part and parts, so the row's answer form is incomplete.
@row_set n=9 row=contract/driver-graph/answer cell=promise seen=1 detector=none
value: On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed, booleans and a part's part and parts counts; never a value, typed text or NodeId.
basis: fct-xyhdhgxl
because: A graph part's answer adds the closed counts part and parts, so the row's list of what the answer holds is incomplete.
@row_set n=10 row=contract/driver-graph/argument cell=promise seen=1 detector=D-arch-resources
value: graph takes one optional part, a whole number from 1 to 1000; a part of 0, of 1001 or of text is refused malformed with nothing run.
basis: fct-xyhdhgxl
because: GRAPH.args gains part as ArgKind::Part 1 to 1000, so the promise that graph takes no argument is false.
@row_set n=11 row=contract/driver-verbs/graph cell=promise seen=1 detector=D-arch-resources
value: One optional part (1 to 1000); result nodes and states, plus part and parts when read in parts, the screen's UI graph whole when within the snapshot budget and no part is asked; dispatches nothing, runs no step.
basis: fct-xyhdhgxl
because: GRAPH gains the part argument and the part and parts fields, so the row's no-argument whole read is false.
@row_set n=12 row=contract/driver-verbs/press cell=promise seen=1 detector=D-arch-resources
value: Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere.
basis: fct-xyhdhgxl
because: PRESS.args gains the flag ctrl joining the platform action modifier, so the row's carrying no Ctrl is false.
@row_set n=13 row=contract/escher-session/graph cell=promise seen=1 detector=D-arch-resources
value: The verb graph takes an optional --part beside --session.
basis: fct-xyhdhgxl
because: The command line's usage line now reads graph [--part <part>], so the row's no argument beside --session is false.
@row_set n=14 row=contract/command-line/verb-flags cell=promise seen=1 detector=D-arch-resources
value: --id, --text, --key, --ms and --part each take one value, --shift and --ctrl are bare, and --session <dir> exactly once; the call is validated before the session is asked for.
basis: fct-xyhdhgxl
because: The command line takes --ctrl as a bare flag beside --shift, so the row's flag list is incomplete.
@row_set n=15 row=contract/argument-kinds/part cell=promise seen=1 detector=D-arch-resources
value: A whole number from 1 to 1000, the snapshot or graph part asked for.
basis: fct-xyhdhgxl
because: The part kind now also serves the graph verb's part, so naming the snapshot alone is incomplete.
@row_set n=16 row=contract/graph-verb/unpaged cell=promise seen=1 fact=F2 detector=D-arch-resources
value: graph answers whole when no part is asked and the answer is within the 10,000-byte snapshot text budget, else in parts of whole nodes within it; the states ride in part 1 alone, a later part's empty states meaning read part 1, and a part past the last answers empty nodes and states.
basis: fct-xyhdhgxl
because: The unpaged-graph limit is retired and the graph pages at the snapshot budget with the states in part 1 alone, so the row's unpaged promise is false.
@row_retire n=1 row=contract/escher-session/press seen=1 heir=contract/escher-session/press-key-shift-ctrl detector=D-arch-resources
why: The press verb's form on the command line now carries --ctrl beside --shift, and the item cell naming the form cannot be set.
basis: fct-xyhdhgxl
because: The usage line gains [--ctrl] on press, so the row's item press --key [--shift] is incomplete and its heir names the new form.
@row_add n=1 row=contract/escher-session/press-key-shift-ctrl area=standard-contracts detector=D-arch-resources
boundary: escher-session command line
item: press --key [--shift] [--ctrl]
promise: The verb press takes --key and the optional bare flags --shift and --ctrl.
basis: fct-xyhdhgxl
because: The command line's press gains the bare --ctrl flag, the heir of the retired press row.
@row_none n=1 fact=F3 type=4
relied: The stand checks and kept checks that reach the Cells editor with a key click the cell input first, since a cell click opens the editor unfocused.
