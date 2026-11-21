@row_set n=1 row=decision/part-answer-never-logged cell=rule seen=1 detector=D-obs-pii
value: A part's answer, a snapshot part ({"text":…,"part":n,"parts":m}) or a graph part ({"nodes":[…],"states":[…],"part":n,"parts":m}), is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
basis: fct-xyhdhgxl
because: The graph verb now answers in parts with its own part form, so the rule that a part's answer is never logged must name the graph part beside the snapshot part.
@row_set n=2 row=contract/command-answer/graphed cell=promise seen=1 detector=none
value: A graph answer is one JSON object of nodes and states, a part adding its part number and part count, holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
basis: fct-xyhdhgxl
because: A graph answer may now be a part carrying part and parts beside nodes and states, so the promise of its form must say so.
@row_set n=3 row=contract/graph-answer/shape cell=promise seen=1 detector=none
value: `{"nodes":[…],"states":[…]}` for a whole graph; a part read is `{"nodes":[…],"states":[…],"part":n,"parts":m}`, its states held in part 1 alone.
basis: fct-xyhdhgxl
because: The graph answer gains a part form written with part and parts, states carried in part 1 alone, so the shape contract must state it.
@row_set n=4 row=contract/command-span/graph cell=promise seen=1 detector=D-obs-instrumentation
value: A graph's line, a part read's included, is `span="command" verb="graph"` (with `cause` when refused, an argument or a `part` outside 1 to 1000 refused `malformed`) and holds no node, state, id, name, part number or count.
basis: fct-wewx4j7o
because: The graph verb gains the part argument 1 to 1000, and a graph part's command span records verb alone, so the span contract must cover the part read and its refusal.
@row_set n=5 row=contract/graph-answer/content cell=promise seen=1 detector=D-obs-pii
value: Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a part its part number and part count only, never a state's value; a boundary widening, ratified.
basis: fct-xyhdhgxl
because: A graph part answer now carries part and parts, numbers the content contract did not list, so the contract must name them.
@row_set n=6 row=decision/hosted-call-through-session-run cell=rule seen=1 detector=D-obs-instrumentation
value: A hosted snapshot or graph part read runs through the session's run alone, so its spans are command spans recording verb alone.
basis: fct-wewx4j7o
because: A graph part read is now a part read whose command span records verb alone, so the rule must cover it beside the snapshot part.
@row_add n=1 row=contract/escher-session-coverage/reached-listeners area=log-coverage fact=F1 detector=none
boundary: escher-session's coverage role
item: reached listeners
promise: A bubbling dispatch that still propagates counts every listener it reached, one a listener stops counts the innermost listener alone, a non-bubbling event counts its target alone, and a dispatch a handler's panic unwinds counts as stopped.
basis: fct-xyhdhgxl
because: Handler coverage now reads whether a dispatch was stopped, so what the coverage role counts as exercised is a promise at its boundary that no row states.
