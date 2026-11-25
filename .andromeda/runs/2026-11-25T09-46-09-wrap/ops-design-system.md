@row_set n=1 row=contract/actions-line/nodes cell=promise seen=1 fact=F5 detector=none
value: Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with. A virtual node is never listed.
basis: fct-w2gujpju
because: The snapshot now holds a canvas's virtual nodes while the actions answer, read from the graph that skips them, holds none, so the row must stop implying every snapshot node of an interactive role is listed.
@row_add n=1 row=contract/canvas-virtual-nodes/accessibility-tree area=surface fact=F1 detector=none
boundary: A canvas's virtual nodes
item: accessibility tree
promise: Each declared virtual node of a canvas is a child of the canvas's tree node, after its own, with the declared role, the declared name as label and author id {owner}///{key}, no bounds and no focus; a hidden, collapsed or inert canvas, or one whose read errs, gives none.
basis: fct-w2gujpju
because: The accessibility tree now serves a canvas's virtual nodes, a promise at the tree boundary no row states.
@row_add n=2 row=contract/canvas-virtual-nodes/snapshot area=surface fact=F2 detector=none
boundary: A canvas's virtual nodes
item: snapshot and diff
promise: The snapshot and its diff read a virtual node with its id, role, declared name, enabled from its declared state, clickable false, highlighted false, no checked, value, selected or pressed, no children, and bounds its region moved by its canvas's content-box origin.
basis: fct-w2gujpju
because: The snapshot now reads virtual nodes with a fixed set of fields, a promise at the snapshot boundary no row states.
@row_add n=3 row=contract/canvas-virtual-nodes/id-verbs area=surface fact=F3 detector=none
boundary: A canvas's virtual nodes
item: click and the other id verbs
promise: Only click takes a virtual id, acting on the node's region in its canvas and refused in the one order disabled, off-screen, covered, not-found, stale; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
basis: fct-w2gujpju
because: Click now resolves a virtual id at its canvas while every other id verb reads it not-found, a verb contract no row states.
@row_add n=4 row=contract/canvas-virtual-nodes/covered area=surface fact=F4 detector=none
boundary: A canvas's virtual nodes
item: covered
promise: A virtual node reads covered only by an element over its region's centre.
basis: fct-w2gujpju
because: The covered cause's meaning was rewritten for virtual nodes, a refusal promise no row states.
@row_add n=5 row=contract/canvas-virtual-nodes/carriers area=surface fact=F6 detector=none
boundary: A canvas's virtual nodes
item: crossing to the carriers
promise: A virtual node's id and author-declared name cross to the snapshot text, its diff, the MCP results and kept checks, and a kept check holding a virtual click replays.
basis: fct-uelmicw4
because: The carriers now hold a virtual node's id and declared name, a boundary widening no row states.
@row_add n=6 row=contract/canvas-virtual-nodes/disabled area=surface fact=F7 detector=none
boundary: A canvas's virtual nodes
item: disabled state
promise: A virtual node declaration carries a disabled state, which the accessibility tree reads as disabled and the snapshot as enabled false.
basis: fct-w2gujpju
because: The declaration gained a disabled field read by the tree and the snapshot, a promise no row states.
