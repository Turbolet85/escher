@row_set n=1 row=decision/graph-answer-crossing cell=rule seen=1 detector=none
value: A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words, booleans and a part's part and parts numbers, never a control value, typed text or NodeId.
basis: fct-xyhdhgxl
because: A graph answer may now be a part written with the numbers part and parts beside its nodes and states, so the crossing's carried set must name them.
@row_set n=2 row=decision/reached-handler-record-without-log cell=rule seen=1 fact=F3 detector=none
value: Where recording of reached handlers is switched on, each reached handler's stable id and event name is resolved before the dispatch and written into a root-context record after it, a handler's panic unwinding out of the dispatch written as stopped, with no log site and no NodeId.
basis: fct-xyhdhgxl
because: The record is no longer written ahead of the handlers but resolved before and written after the dispatch through a drop guard, so the row's rule is false as it stands.
@row_set n=3 row=decision/reached-record-deduplicated-survives-removal cell=rule seen=1 detector=none
value: Handler coverage relies on the reached record holding, deduplicated, the stable id and event of each node whose listeners a dispatch reached, the innermost listener alone when the dispatch was stopped, so a handler whose run removes its element still counts.
basis: fct-xyhdhgxl
because: A stopped bubbling dispatch now records its innermost listener alone and a non-bubbling event its target alone, so the record no longer holds every node whose listeners held the event.
@row_set n=4 row=contract/driver-verbs/graph cell=promise seen=1 detector=D-security-input
value: Takes an optional part from 1 to 1000 (anything else refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]}, a part adding part and parts, of ids, owner ids, component, state and handler event names, flag words and booleans, never a control value, typed text or NodeId.
basis: fct-xyhdhgxl
because: The graph verb gained the bounded part argument and a part answer, so its promise of taking no argument is false.
@row_set n=5 row=contract/graph-answer/nodes-states cell=promise seen=1 detector=none
value: `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId; a part adds `part` and `parts`, its states empty past part 1.
basis: fct-xyhdhgxl
because: A graph part is written as the graph keys plus part and parts, so the answer's form must name them.
@row_set n=6 row=contract/refusal-causes/malformed cell=promise seen=1 detector=D-security-input
value: The one cause that carries a fault; a call breaking the schema, such as an `actions` call with an argument or a `graph` part outside 1 to 1000, is refused with it and nothing runs.
basis: fct-jc4dsh7s
because: A graph call with a part is no longer malformed, so the row's example of a graph call with an argument is false.
@row_set n=7 row=decision/kept-check-call-content cell=rule seen=1 fact=F5 detector=none
value: A kept check holds call content: target ids, press keys and the ctrl flag, graph parts, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN.
basis: fct-wewx4j7o
because: A kept check may now hold press with ctrl and graph with part, its format unchanged, so its call-content class must name them.
@row_set n=8 row=decision/kept-check-graph-answer cell=rule seen=1 detector=none
value: A kept graph answer holds ids, owner ids, component names, registered state names, handler event names, closed flag words, booleans and a part's part and parts numbers, never a control value or typed text.
basis: fct-wewx4j7o
because: A kept check may now hold a graph part step whose answer carries part and parts, so the kept graph answer's content must name them.
@row_set n=9 row=contract/command-span/verb cell=promise seen=2 fact=F4 detector=none
value: Records the instance-level verb; a graph call, a graph part's included, an actions or layout call records the verb alone, plus cause when refused, never the part asked or the part count.
basis: fct-wewx4j7o
because: A graph part's command span records verb alone and neither the part asked nor the part count reaches a log, which the row must state for the new argument.
@row_retire n=1 row=decision/graph-answer-unpaged seen=1 heir=decision/graph-answer-paged detector=D-security-input
why: The graph verb now takes a part and a large graph is answered in parts, so an unpaged graph no longer stands.
basis: fct-xyhdhgxl
because: The graph answer is paged under the snapshot budget, contradicting the row's whole-graph-in-one-answer rule and its key.
@row_add n=1 row=decision/graph-answer-paged area=api-security fact=F2 detector=D-security-input
name: The graph answer is paged
rule: The graph verb takes an optional part from 1 to 1000; a graph within the 10,000-byte snapshot budget with no part asked is one whole answer, else parts of whole nodes each within the budget, a node over it a part alone, the states in part 1 alone and a part past the last empty.
why: A boundary widening, ratified.
guard: stand_graph cells_reads_in_parts_each_within_the_budget_and_together_the_whole_graph
basis: fct-xyhdhgxl
because: The graph answer is now paged by a bounded part argument, a decision future work on the graph verb must respect.
@row_add n=2 row=contract/cli-flags/ctrl area=input-validation fact=F1 detector=D-security-input
boundary: escher-session flags
item: --ctrl
promise: Bare, taking no value; on press it holds the host's action key, Meta on macOS and Control elsewhere.
basis: fct-jc4dsh7s
because: press gained the closed optional flag ctrl, bare on the command line like --shift, and the flag sheet promises each flag's form.
