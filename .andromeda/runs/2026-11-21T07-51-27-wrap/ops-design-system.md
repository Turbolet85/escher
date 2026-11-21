@row_set n=1 row=contract/escher-session-commands/graph-answer cell=promise seen=1 fact=F2 detector=none
value: graph answers one uncoloured line of JSON on stdout, status 0, the same on the session socket's call and in a kept check, over the mcp role the text block of that line: {"nodes":[…],"states":[…]} for a whole graph and {"nodes":[…],"states":[…],"part":n,"parts":m} for a graph read in parts.
basis: fct-xyhdhgxl
because: graph now answers a part's form beside the whole graph's, so a row promising the whole graph alone is false.
@row_set n=2 row=contract/escher-session-commands/graph-arguments cell=promise seen=1 detector=none
value: graph takes one optional --part, 1 to 1000; a part of 0, over 1000 or not a number, or any other argument, is refused malformed with nothing run.
basis: fct-xyhdhgxl
because: graph gained the optional part argument, so the row saying it takes no argument is false.
@row_set n=3 row=contract/ui-graph-line/content cell=promise seen=1 detector=none
value: The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a graph read in parts its part and parts numbers only, never a value or typed text.
basis: fct-xyhdhgxl
because: a graph part's line now carries the part and parts numbers, which the row's closed list of content leaves out.
@row_add n=1 row=contract/escher-session-commands/graph-parts area=surface fact=F2 detector=none
boundary: escher-session commands
item: graph parts
promise: graph is paged at the 10,000-byte snapshot budget: with no part and within the budget it answers whole, otherwise part n of parts, cut on whole nodes, the states in part 1 alone, a later part's empty states meaning read part 1, and a part past the last answering empty nodes and states.
basis: fct-xyhdhgxl
because: graph now pages its answer at the snapshot budget, a promise outward that no row of the surface states.
@row_add n=2 row=contract/escher-session-commands/press-ctrl area=surface fact=F1 detector=none
boundary: escher-session commands
item: press --ctrl
promise: press takes an optional ctrl flag held as the host's action key, Meta on macOS and Ctrl elsewhere; with it the arrows move a multiple select's current row without choosing and space flips that row, building a set of rows that are not adjacent.
basis: fct-xyhdhgxl
because: press gained the ctrl flag at the command boundary, a promise outward that no row of the surface states.
@row_add n=3 row=contract/escher-session-commands/coverage-stopped-dispatch area=surface fact=F3 detector=none
boundary: escher-session commands
item: coverage under a stopped dispatch
promise: coverage counts an unstopped bubbling dispatch as exercising every listener on the chain, one a listener stops the innermost listener alone, a non-bubbling event its target alone, and a handler panic's unwind as a stopped dispatch.
basis: fct-i44kzdy2
because: handler coverage now reads a stopped dispatch, which decides what the coverage answer reports as exercised and no row of the surface states.
@row_none n=1 fact=F4 type=4
what: how the engine's multiple select answers plain arrows then space, a measured behaviour with no reason stated; the code and the stand check hold it.
@row_none n=2 fact=F5 type=4
what: how the Cells editor opens unfocused on a cell click, a measured behaviour of one example app with no reason stated; the code holds it.
relied: a kept check that keys the Cells editor clicks the cell input first.
