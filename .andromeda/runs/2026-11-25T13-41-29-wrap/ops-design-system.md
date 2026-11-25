@row_set n=1 row=contract/actions-line/nodes cell=promise seen=2 detector=none
value: Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with. A virtual node is listed.
basis: fct-x6ohav4t, fct-w25w7t3u, fct-lwbdf7ug
because: The actions answer now lists each virtual node, owned by its canvas, so the row's «A virtual node is never listed» is false.
@row_set n=2 row=contract/actions-line/content cell=promise seen=1 fact=F5 detector=none
value: The line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text, NodeId or a virtual node's declared name: a virtual node is carried by its owner///key id.
basis: fct-wbnxhw6m, fct-llzzxhy4
because: The actions answer now carries virtual nodes, by their owner///key id and never their declared name, so the line's content promise must name it.
@row_set n=3 row=contract/ui-graph-line/content cell=promise seen=2 fact=F5 detector=none
value: The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a graph read in parts its part and parts numbers only, never a value, typed text or a virtual node's declared name.
basis: fct-wbnxhw6m, fct-nbmlbttg, fct-llzzxhy4
because: The graph now holds each virtual node by its owner///key id and never its declared name, so the line's content promise must exclude the name.
@row_set n=4 row=contract/coverage-line/content cell=promise seen=1 fact=F3 detector=none
value: The line carries ids, owner ids, event names and two counts only, owner left out when absent, never a virtual node's id: a virtual node carries no handler, and a click on one counts its canvas's listener under the canvas's id.
basis: fct-x6ohav4t, fct-kiimqirg
because: A replayed click on a virtual node now counts as its canvas's listener under the canvas's id, so the line's content promise must say what it carries for one.
@row_add n=1 row=decision/virtual-id-verbs-click-and-layout area=surface fact=F1 detector=none
name: Only click and layout take a virtual node's id
rule: Of the id-taking verbs only click and layout take a virtual node's id; type, scroll, screenshot with an id and canvas read a virtual node's id not-found.
why: A decided limit, stated in the capability statement and owed by no later work.
guard: click_and_layout_alone_take_a_virtual_id, the_other_id_verbs_read_a_virtual_id_not_found
state: provisional
basis: fct-w25w7t3u, fct-uopwvezd, fct-jzelteld, fct-w4id572v
because: The verbs taking a virtual id moved from click alone to click and layout, and the rest stay refused as a decided limit future work must respect.
@row_add n=2 row=contract/escher-session-commands/layout-answer-virtual-node area=surface fact=F2 detector=none
boundary: escher-session commands
item: layout answer of a virtual node
promise: layout on a virtual node's id answers it placed by app in its canvas, at the region's offset from the canvas's border box, its width and height caused by layout, never an engine reason.
basis: fct-x6ohav4t, fct-lrqd5vrd, fct-wbnxhw6m
because: The layout answer now explains a virtual node instead of refusing it, and the layout-answer row has no room within its bound for how one is placed.
@row_none n=1 fact=F4 type=9
what: The lint and canvas answer holding no virtual node is a limit owned by a CARRY on Circle Drawer's circles as virtual nodes; the route holds it, and the canvas verb's not-found is the click-and-layout decision's.
