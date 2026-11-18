@row_set n=1 row=contract/cold-agent-verdict-reasons/capability-unused cell=promise seen=1 fact=F4 detector=D-arch-resources
value: The verdict fails with `capability-unused` unless `lint`, `screenshot`, `advance`, `graph`, `actions` and `layout` are each used, a part is read and a list is opened.
basis: fct-wyr2e6ga
because: The pipe now counts six verbs and its capability-unused requirement set reads graph, actions, layout and list openings beside lint, screenshot, advance and part reads, so the row's three verbs and one read no longer state the reason.
@row_set n=2 row=decision/cold-agent-six-apps cell=rule seen=1 detector=none
value: The cold-agent prompt names six apps (counter, flight booker, timer, CRUD, cells and the load page) and their goals, among them the counter's UI graph, the flight booker's actions and its flight type list opened, CRUD's layout, and Cells kept off graph and actions; it names no stand id.
basis: fct-wyr2e6ga
because: The fixed stand prompt gains goals asking for 0.3.0's three carried reads and the painted list, so the row naming the apps' goals must name them.
@row_set n=3 row=decision/cold-agent-run-carries-stand-screen cell=rule seen=1 fact=F5 detector=none
value: The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts, diffs, and graph and actions answers, from the driver's own MCP server.
basis: fct-xfqd7f22
because: The founder's F4 answer lets graph and actions answers cross to the model provider on the live run, so the row listing what crosses must hold them.
@row_add n=1 row=contract/cold-agent-verdict/graph-calls area=standard-contracts fact=F1 detector=D-arch-resources
boundary: cold-agent verdict.json
item: graph_calls
promise: The cold-agent verdict and its run.end event carry `graph_calls`, the count of the agent's `graph` calls read from the tool_call span, which the `passed` bar requires to be used.
basis: fct-wyr2e6ga
because: The verdict and its run.end event gain graph_calls, a new field of the pipe's outward verdict, which the registry does not yet hold.
@row_add n=2 row=contract/cold-agent-verdict/actions-calls area=standard-contracts fact=F1 detector=D-arch-resources
boundary: cold-agent verdict.json
item: actions_calls
promise: The cold-agent verdict and its run.end event carry `actions_calls`, the count of the agent's `actions` calls read from the tool_call span, which the `passed` bar requires to be used.
basis: fct-wyr2e6ga
because: The verdict and its run.end event gain actions_calls, a new field of the pipe's outward verdict, which the registry does not yet hold.
@row_add n=3 row=contract/cold-agent-verdict/layout-calls area=standard-contracts fact=F1 detector=D-arch-resources
boundary: cold-agent verdict.json
item: layout_calls
promise: The cold-agent verdict and its run.end event carry `layout_calls`, the count of the agent's `layout` calls read from the tool_call span, which the `passed` bar requires to be used.
basis: fct-wyr2e6ga
because: The verdict and its run.end event gain layout_calls, a new field of the pipe's outward verdict, which the registry does not yet hold.
@row_add n=4 row=contract/cold-agent-verdict/list-opens area=standard-contracts fact=F2 detector=D-arch-resources
boundary: cold-agent verdict.json
item: list_opens
promise: The cold-agent verdict carries `list_opens`, the count of accepted click or press answers whose changed list holds a ListBoxOption row whose differs is exactly bounds; an answer that does not decode as JSON is skipped.
basis: fct-pf5rizdr
because: The verdict gains list_opens, a new outward field the passed bar requires, which the registry does not yet hold.
@row_add n=5 row=contract/cold-agent-verdict/refused-by-cause area=standard-contracts fact=F3 detector=D-arch-resources
boundary: cold-agent verdict.json
item: refused_by_cause
promise: The cold-agent verdict carries `refused_by_cause`, an object of the eight closed refusal cause words in the schema's order, each counted from the command span's cause field; a word outside the eight is not counted, and a refused session verb counts no cause.
basis: fct-pf5rizdr
because: The verdict gains refused_by_cause, a new outward field of closed words to counts, which the registry does not yet hold.
