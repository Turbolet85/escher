@row_set n=1 row=decision/declared-node-read-logs-nothing cell=rule seen=1 detector=D-obs-instrumentation
value: The in-process read of a canvas's declared virtual nodes, its validation and its answer, and the virtual nodes' place in the accessibility tree, the snapshot and a click on one, have no log site, print, span or field; a refused declaration holds a closed reason and its index alone.
basis: fct-klrtrzdi
because: The chunk puts the declared virtual nodes into the accessibility tree, the snapshot and click with no log site, span or field added, so the decision widens from the read alone to those readers.
@row_add n=1 row=decision/virtual-node-id-and-name-crossing area=pii-scrubbing-compliance fact=F1 detector=D-obs-pii
name: Virtual node id and declared name crossing
rule: A canvas's virtual node crosses into the snapshot text, its diff, MCP results and kept checks as its owner///key id and its author-declared name, and onto no log line or span field.
why: A boundary widening, ratified.
guard: stand_act_spans
basis: fct-uelmicw4
because: The snapshot text, diff, MCP results and kept checks now carry a virtual node's id and declared name, a widened crossing no row of the section states.
@row_add n=2 row=contract/snapshot-node/virtual-node area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
boundary: the snapshot node
item: virtual node
promise: A canvas's virtual node reads enabled from its declared state, clickable false and highlighted false, with no checked, value, selected or pressed and no children.
basis: fct-w2gujpju
because: The snapshot now holds virtual nodes, and what such a node carries, no control value among it, is a new promise of the snapshot node.
@row_add n=3 row=decision/click-alone-takes-virtual-id area=span-trace-coverage fact=F3 detector=D-obs-instrumentation
name: click alone takes a virtual node id
rule: click is the one driver verb that takes a canvas's virtual node id, refused disabled, off-screen, covered, not-found or stale in that one order; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
basis: fct-w2gujpju
because: The driver now resolves a virtual id for click alone and refuses it through the closed causes its command span records, a contract no row of the section states.
@row_add n=4 row=decision/virtual-nodes-off-graph-actions-lint area=log-coverage fact=F4 detector=none
name: Virtual nodes kept off the graph, actions and lint
rule: The graph, actions, lint and exploration hold no virtual node; the lint excludes them because a virtual node's entry has no element, with no skip in code.
guard: the_graph_actions_and_lint_hold_no_virtual_node
state: provisional
basis: fct-kgfnnhll
because: The graph skips every virtual node and the lint's exclusion was measured to rest on the entry having no element, the inert skip dropped on a delegate's ruling, a decision no row states.
