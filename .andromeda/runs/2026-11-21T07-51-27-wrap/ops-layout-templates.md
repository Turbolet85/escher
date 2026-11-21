@row_set n=1 row=contract/escher-session-graph/answer cell=promise seen=1 detector=none
value: One uncoloured line {"nodes","states"} for a whole graph, {"nodes","states","part","parts"} for a part, part 1 alone carrying the states and a later part's empty states meaning read part 1; ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
basis: fct-xyhdhgxl
because: graph now takes --part and answers a part with part and parts beside nodes and states, so the row's «No argument» and single form are false.
@row_add n=1 row=contract/escher-session-graph/part area=surface-cli fact=F2 detector=none
boundary: escher-session graph
item: --part
promise: Optional whole number 1 to 1000; unasked, a graph within 10,000 bytes answers whole, else each part holds whole nodes within that budget, a node over it a part alone; a part past the last answers empty nodes and states; outside the bound refused malformed, exit 1.
basis: fct-xyhdhgxl
because: graph gains the optional part argument at the snapshot budget, a new item of the command line's contract that no row of the area holds.
@row_add n=2 row=contract/escher-session-press/ctrl area=surface-cli fact=F1 detector=none
boundary: escher-session press
item: --ctrl
promise: Optional bare flag, the host's action key (Meta on macOS, Control elsewhere), moving the current row without choosing it, so space builds a set of rows that are not adjacent.
basis: fct-xyhdhgxl
because: press gains the optional ctrl flag on the command line, MCP and kept checks, a new item of the command line's contract that no row of the area holds.
@row_add n=3 row=contract/escher-session-coverage/stopped-dispatch area=surface-cli fact=F3 detector=none
boundary: escher-session coverage role
item: stopped dispatch
promise: A dispatch an ancestor or its target stops counts its innermost listener alone, a non-bubbling event its target alone, and a dispatch a handler's panic unwinds counts as stopped.
basis: fct-xyhdhgxl
because: the coverage role now reads a stopped dispatch instead of counting every listener on the chain, a narrowed stated limit of what unexercised means that no row of the area holds.
@row_none n=1 fact=F4 type=4
relied: A Cells cell's click opens its editor unfocused, so a driver's key reaches the editor only after a click on cell-input; the Cells task's code holds it and no reason is stated.
@row_none n=2 fact=F5 type=4
relied: On a focused multiple select a space after plain arrows flips the chosen row out, so the plain sequence ends with no row chosen; the engine's select handling holds it and no reason is stated.
