@row_retire n=1 row=decision/circle-drawer-canvas-pixels-only seen=4 heir=decision/circle-drawer-circle-nodes detector=none
why: Circle Drawer's circles are no longer pixels only: each is a declared virtual node read in the tree, the snapshot and the UI graph.
basis: fct-5wc4mrm4
because: The chunk makes each circle a virtual node of the canvas, so a row keyed and ruled as pixels-only is false and its heir states the circle nodes.
@row_add n=1 row=decision/circle-drawer-circle-nodes area=screen-reader-support fact=F1 detector=none
name: Circle Drawer's circles as virtual nodes
rule: Circle Drawer's canvas, one node named Drawing area, declares one virtual node per circle: id circle-canvas///c{id}, role Image, name Circle {id}, {d} px, bounds its cut region, the id app-given from 1 and stable across undo and redo, an undone circle reading stale.
guard: stand_circle_nodes
state: provisional
basis: fct-5wc4mrm4
because: The chunk declares a virtual node per circle on a delegate's provisional word, replacing the pixels-only reading of the circles.
@row_add n=2 row=contract/canvas-answer/node-tag area=screen-reader-support fact=F2 detector=none
boundary: the canvas answer
item: node tag
promise: Each line of the canvas answer, full and in parts, ends node={k}, the 1-based index of the canvas's virtual node whose region holds the command's centre, nearest centre first, or node=- for none; no line carries a virtual node's id or name.
basis: fct-yf6akjxu
because: The chunk ties every draw command to a virtual node by index alone on the canvas answer, a promise at that boundary no row states.
@row_add n=3 row=contract/handler-coverage/virtual-node-click area=screen-reader-support fact=F3 detector=none
boundary: handler coverage
item: a virtual node's click
promise: A handler coverage answer lists a virtual node's click under its virtual id, owned by its canvas, exercised where a replayed click named that id; a canvas click inside a node counts for the canvas alone.
basis: fct-5wc4mrm4
because: The chunk lists a virtual node's click in handler coverage under its id, which no standing coverage row states.
@row_add n=4 row=absence/keyboard-virtual-nodes area=keyboard-navigation fact=F5 detector=D-a11y-surface
capability: A keyboard route to a canvas's virtual nodes; Circle Drawer's circles are reached by click, tree and snapshot alone
basis: fct-l3p3ykbb
because: The chunk adds clickable circle nodes with keyboard coverage marked absent by the plan's boundary, so the missing keyboard route is recorded as an absence.
@row_add n=5 row=absence/lint-virtual-nodes area=visual-design-verification fact=F6 detector=none
capability: Visual lint of a canvas's virtual nodes: no off-screen, clipped or zero-size check reads one; overlap skips either side
basis: fct-vufwaraf
because: The chunk leaves the lint reading no virtual node, its three checks carried to a later entry, an absence no row states.
@row_set n=1 row=contract/driver-click-refusals/covered cell=promise seen=2 fact=F4 detector=none
value: A driver click on an element under an open select list is refused `covered`; a virtual node reads `covered` only when an element lies over its region's centre, a sibling virtual node over it covering nothing.
basis: fct-lsa44c6h
because: The chunk witnesses that a circle under another circle is not refused covered, which the row implied only through its element clause.
@row_set n=2 row=contract/node-id/reserved-separator cell=promise seen=3 detector=none
value: The form {owner}///{key} is reserved for a canvas's virtual nodes and served in the accessibility tree and, from it, the snapshot and its diff, the UI graph, the actions, layout and handler coverage answers, these four never carrying the node's declared name; no element's id holds ///.
basis: fct-yf6akjxu
because: The coverage line now carries a virtual node's {owner}///{key} id, so the list of answers serving the form must name it.
@row_set n=3 row=decision/ui-graph-one-node-per-snapshot-node cell=rule seen=3 detector=none
value: The read-only UI graph holds one node per snapshot node, a canvas's virtual nodes among them, each owned by its canvas with no reads and no handler but its click, so the actions answer, exploration and handler coverage hold them too; names and closed words only.
basis: fct-vhzvumhp
because: A virtual node's click now stands among the graph's handlers that coverage lists, so the row's no-handlers clause is false.
