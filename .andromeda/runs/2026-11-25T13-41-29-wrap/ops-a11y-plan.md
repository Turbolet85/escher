@row_set n=1 row=decision/ui-graph-one-node-per-snapshot-node cell=rule seen=2 fact=F2 detector=none
value: The read-only UI graph holds one node per snapshot node, a canvas's virtual nodes among them, each owned by its canvas with no handlers or reads, so the actions answer, handler coverage and exploration hold them too; names and closed words only.
basis: fct-x6ohav4t (Symbols / APIs: `DioxusDocument::graph`'s snapshot filter lifted); fct-w25w7t3u; fct-vxspd4cu
because: The graph no longer excepts a canvas's virtual nodes, and the actions answer and exploration now hold them, so the row's exception and its consequence are false.
@row_set n=2 row=contract/node-id/reserved-separator cell=promise seen=2 fact=F4 detector=none
value: The form {owner}///{key} is reserved for a canvas's virtual nodes and served in the accessibility tree and, from it, the snapshot and its diff, the UI graph, the actions answer and the layout answer, these three never carrying the node's declared name; no element's id holds ///.
basis: fct-wbnxhw6m; fct-llzzxhy4
because: The graph, actions and layout answers now carry a virtual node's {owner}///{key} id and no declared name, so the places the reserved form is served grew.
@row_add n=1 row=decision/virtual-id-verbs area=screen-reader-support fact=F1 detector=none
name: Verbs that take a virtual node's id
rule: Only click and layout take a canvas virtual node's id; type, scroll, screenshot with an id and canvas read a virtual id not-found, a decided limit and not an owed one.
state: provisional
basis: fct-x6ohav4t (Symbols / APIs: `execute::takes_virtual_id`); fct-uopwvezd (inputs#I3, decided limits, F16 A); fct-jzelteld
because: The chunk decides which driver verbs accept a virtual id, a delegate's answer, and no a11y-plan row states that set.
@row_add n=2 row=contract/layout-answer/virtual-node area=screen-reader-support fact=F3 detector=none
boundary: the layout answer
item: a virtual node
promise: Layout explains a virtual node as placed by the app in its canvas, at its bounds' offset from the canvas, with the canvas's shifts and size cause layout; the engine is never asked for a region's reason.
basis: fct-x6ohav4t (Symbols / APIs: `Session::layout`, `fn declared`); fct-lrqd5vrd
because: The layout answer now answers a virtual id with a fixed placing promise that no a11y-plan row states.
@row_none n=1 fact=F5 type=9
what: A stated limit owned by a CARRY on Circle Drawer's circles as virtual nodes; the route holds it.
@row_none n=2 fact=F6 type=9
what: A stated limit owned by a CARRY on Circle Drawer's circles as virtual nodes; the route holds it.
