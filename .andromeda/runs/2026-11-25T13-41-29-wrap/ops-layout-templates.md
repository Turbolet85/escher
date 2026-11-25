@row_set n=1 row=contract/escher-session-actions/answer cell=promise seen=1 fact=F3 detector=none
value: No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId; a virtual node is listed owned by its canvas, keyed false, each verb's cause what its call returns.
basis: fct-x6ohav4t (Session::actions), fct-lwbdf7ug
because: The actions answer now lists a canvas's virtual nodes, owned by the canvas, keyed false, with each verb's cause equal to its call's, so the answer's promise must say so.
@row_set n=2 row=contract/escher-session-lint/answer cell=promise seen=1 fact=F5 detector=none
value: No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only; holding no virtual node, so no overlap, clip or off-screen finding on a declared region.
basis: fct-w25w7t3u, fct-llzzxhy4
because: The lint is now the one read besides the canvas answer that holds no virtual node, a stated limit its answer's promise must carry.
@row_add n=1 row=decision/virtual-id-verbs area=surface-cli fact=F1 detector=none
name: Only click and layout take a virtual node id
rule: Of the escher-session id verbs only click and layout take a virtual node id; type, scroll, screenshot with an id and canvas refuse one not-found.
why: A decided limit, not owed.
state: provisional
basis: fct-x6ohav4t (execute::takes_virtual_id), fct-jzelteld, fct-uopwvezd (inputs#I3 row 3)
because: The verbs taking a virtual id moved from click alone to click and layout, the rest kept as a decided limit on the delegate's word, and no row of the CLI surface says which verbs take one.
@row_add n=2 row=contract/escher-session-layout/virtual-node area=surface-cli fact=F2 detector=none
boundary: escher-session layout
item: virtual node
promise: A virtual node id is answered placed by app, container its canvas's id, at the node's bounds minus the canvas's, with the canvas's shifts and both sizes caused by layout; the engine is never asked to explain a region.
basis: fct-x6ohav4t (Session::layout, fn declared), fct-lrqd5vrd
because: The layout verb now answers a virtual id instead of refusing it, and the standing layout answer row has no room within its bound for what that answer reads.
@row_add n=3 row=contract/escher-session-graph/virtual-node area=surface-cli fact=F3 detector=none
boundary: escher-session graph
item: virtual node
promise: A virtual node is one graph node under its own id, owned by its canvas with that canvas's component, carrying no handlers, no reads and no declared name.
basis: fct-x6ohav4t (DioxusDocument::graph), fct-nbmlbttg, fct-wbnxhw6m
because: The graph answer no longer drops a virtual node, and the standing graph answer row is at its bound, so what it promises of one stands as its own row.
@row_add n=4 row=contract/escher-session-coverage/virtual-node-click area=surface-cli fact=F4 detector=none
boundary: escher-session coverage role
item: virtual node click
promise: A click on a virtual node counts its canvas's click listener under the canvas's id; a virtual node carries no handler of its own, and no coverage line holds a virtual id.
basis: fct-x6ohav4t (coverage.rs), fct-kiimqirg
because: Handler coverage now counts a replayed virtual click against its canvas's listener, and the coverage answer row has no room within its bound for that rule.
