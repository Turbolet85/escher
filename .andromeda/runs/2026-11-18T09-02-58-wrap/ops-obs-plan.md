@row_add n=1 row=contract/cold-agent-counted-verbs/graph area=log-coverage fact=F1 detector=none
boundary: The cold-agent pipe's counted verbs
item: graph
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.
basis: fct-wyr2e6ga
because: COUNTED now holds graph beside lint, screenshot and advance, so the counted-verb list gains its item.
@row_add n=2 row=contract/cold-agent-counted-verbs/actions area=log-coverage fact=F1 detector=none
boundary: The cold-agent pipe's counted verbs
item: actions
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.
basis: fct-wyr2e6ga
because: COUNTED now holds actions beside lint, screenshot and advance, so the counted-verb list gains its item.
@row_add n=3 row=contract/cold-agent-counted-verbs/layout area=log-coverage fact=F1 detector=none
boundary: The cold-agent pipe's counted verbs
item: layout
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.
basis: fct-wyr2e6ga
because: COUNTED now holds layout beside lint, screenshot and advance, so the counted-verb list gains its item.
@row_add n=4 row=contract/cold-agent-verdict/graph-calls area=6-log-coverage fact=F2 detector=none
boundary: The cold-agent verdict
item: graph_calls
promise: The count of accepted tool_call span lines of the graph verb in the live server child's server.log, written after advance_calls.
basis: fct-wyr2e6ga
because: The verdict gains graph_calls, a new field promised outward beside lint_calls, screenshot_calls and advance_calls.
@row_add n=5 row=contract/cold-agent-verdict/actions-calls area=6-log-coverage fact=F2 detector=none
boundary: The cold-agent verdict
item: actions_calls
promise: The count of accepted tool_call span lines of the actions verb in the live server child's server.log, written after advance_calls.
basis: fct-wyr2e6ga
because: The verdict gains actions_calls, a new field promised outward beside lint_calls, screenshot_calls and advance_calls.
@row_add n=6 row=contract/cold-agent-verdict/layout-calls area=6-log-coverage fact=F2 detector=none
boundary: The cold-agent verdict
item: layout_calls
promise: The count of accepted tool_call span lines of the layout verb in the live server child's server.log, written after advance_calls.
basis: fct-wyr2e6ga
because: The verdict gains layout_calls, a new field promised outward beside lint_calls, screenshot_calls and advance_calls.
@row_add n=7 row=contract/cold-agent-verdict/list-opens area=6-log-coverage fact=F3 detector=none
boundary: The cold-agent verdict
item: list_opens
promise: The count of accepted click or press answers whose changed list holds a ListBoxOption differing in bounds alone; an answer that does not decode as JSON is skipped.
basis: fct-wyr2e6ga
because: The verdict gains list_opens, a new field promised outward, with its decode rule stated in the report's coverage.
@row_add n=8 row=contract/cold-agent-verdict/refused-by-cause area=6-log-coverage fact=F4 detector=none
boundary: The cold-agent verdict
item: refused_by_cause
promise: An object of the eight closed refusal cause words to counts, read from the command span lines' cause field in the server log; a word outside the eight is not counted.
basis: fct-wyr2e6ga
because: The verdict gains refused_by_cause, a new field promised outward, holding closed words and counts only.
@row_set n=1 row=contract/cold-agent-verdict-reasons/capability-unused cell=promise seen=1 fact=F5 detector=none
value: Given when a counted verb, a part read or a list opening went unused.
basis: fct-wyr2e6ga
because: capability-unused now also reads list_opens at 0, so the reason's promise must name the list opening beside the counted verbs and part reads.
@row_set n=2 row=contract/cold-agent-verdict/capability-unused cell=promise seen=1 detector=none
value: The reason given when lint_calls, screenshot_calls, advance_calls, graph_calls, actions_calls, layout_calls, part_reads or list_opens reads 0.
basis: fct-wyr2e6ga
because: The capability-unused requirement set widened from four to eight, so the row naming only three of its fields is false.
@row_set n=3 row=decision/cold-agent-server-log-counted-only cell=rule seen=1 detector=none
value: The cold-agent pipe reads server.log only to count tool_call span lines, their refused and error endings, the accepted lines of each counted verb (lint, screenshot, advance, graph, actions, layout) and the command span lines' cause words, never printing or copying it.
basis: fct-wyr2e6ga
because: The counted verbs moved from three to six and the pipe now counts the command span's cause field, so the row's list of what is read is false.
@row_set n=4 row=decision/cold-agent-server-log-read-by-span-name cell=rule seen=1 detector=none
value: The cold-agent pipe reads the server log by span name, the tool_call span's tool and ending and the command span's cause word only, and never prints or copies it.
basis: fct-wyr2e6ga
because: The pipe now reads the command span's existing cause field for refused_by_cause, so a read by span name alone no longer holds.
@row_set n=5 row=decision/verdict-fields-closed cell=rule seen=1 detector=none
value: Each cold-agent verdict field is a count, a boolean, a closed word, an object of closed words to counts, the task, the client's tool names, version and model, or the transcript file's path, never transcript text, a tool argument, a tool answer, a reply or the check.
basis: fct-w53lgzmr
because: refused_by_cause is an object of closed words to counts, a field kind the row's closed list did not name.
@row_set n=6 row=decision/cold-agent-verdict-holds-no-content cell=rule seen=1 detector=none
value: Every field of the cold-agent verdict is a count, a boolean, a closed word or an object of closed words to counts, holding no transcript text, tool argument, tool answer, reply or check.
basis: fct-w53lgzmr
because: refused_by_cause is an object of closed words to counts, so the row's list of field kinds is incomplete.
@row_retire n=1 row=decision/graph-actions-layout-not-counted-verbs seen=1 detector=none
why: The decision is reversed: graph, actions and layout are now counted verbs, each with a verdict field, and required by capability-unused.
basis: fct-pkm3ir2p
because: COUNTED now holds graph, actions and layout and capability-unused reads their counts, so the row says the opposite of the code's contract.
