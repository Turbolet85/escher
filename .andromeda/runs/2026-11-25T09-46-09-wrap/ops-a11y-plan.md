@row_set n=1 row=decision/ui-graph-one-node-per-snapshot-node cell=rule seen=1 fact=F4 detector=none
value: The read-only UI graph holds one node per snapshot node but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them; names and closed words only.
basis: fct-w2gujpju
because: The graph now skips every snapshot node whose id holds the virtual separator, so one node per snapshot node no longer holds.
@row_set n=2 row=contract/node-id/reserved-separator cell=promise seen=1 detector=none
value: The form {owner}///{key} is reserved for a canvas's virtual nodes and served in the accessibility tree and, from it, the snapshot and its diff; no element's id holds ///.
basis: fct-w2gujpju
because: The reserved form is now served as a virtual node's author id and snapshot id, so «no served id holds ///» is false.
@row_set n=3 row=contract/snapshot/enabled cell=promise seen=1 fact=F9 detector=none
value: The snapshot's enabled field reads the presence of the element's disabled attribute, and on a virtual node the declaration's disabled flag, enabled false where declared disabled.
basis: fct-w2gujpju
because: A virtual node declaration now states disabled, which the snapshot reads as enabled false, so the field no longer reads an attribute alone.
@row_set n=4 row=contract/driver-click-refusals/covered cell=promise seen=1 fact=F6 detector=none
value: A driver click on an element under an open select list is refused `covered`; a virtual node reads `covered` only when an element lies over its region's centre.
basis: fct-xn5aba5k
because: A click now takes a virtual id and its covered reading is its region's centre alone, a stated limit the row must carry.
@row_add n=1 row=contract/accessibility-tree/virtual-nodes area=7-screen-reader-support fact=F1 detector=none
boundary: the accessibility tree
item: a canvas's virtual nodes
promise: A canvas's taken virtual nodes are children of its tree node, after its own, with the declared role, the declared name as label, author_id {owner}///{key} and disabled where declared; no bounds are written on them, and a hidden, collapsed or inert canvas, or one whose read errs, takes none.
basis: fct-w2gujpju
because: The Dioxus document's tree build now adds each canvas's virtual nodes under the canvas node, a new promise of the tree.
@row_add n=2 row=contract/snapshot/virtual-node area=7-screen-reader-support fact=F3 detector=none
boundary: snapshot
item: a virtual node
promise: A virtual node reads id, role, name, enabled from its declared disabled flag, clickable false, highlighted false, no checked, value, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
basis: fct-w2gujpju
because: The snapshot now reads virtual nodes from the tree, a new promise of the snapshot node.
@row_add n=3 row=contract/driver-verbs/virtual-id area=screen-reader-support fact=F5 detector=none
boundary: driver verbs
item: a virtual id
promise: Only click takes a virtual id, located at its canvas and refused in the one order disabled, off-screen, covered, not-found, stale; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
basis: fct-xqti77pe
because: The driver's click now resolves a virtual id at its canvas while every other id verb reads it not-found, a new promise of the verbs.
@row_add n=4 row=decision/virtual-node-crosses-carriers area=screen-reader-support fact=F8 detector=none
name: Virtual node id and name cross the agent carriers
rule: A virtual node's {owner}///{key} id and declared name cross the snapshot text, its diff, MCP results and kept checks, which replay a virtual click; no log line at info carries either.
why: A boundary widening, ratified.
guard: stand_act_spans
basis: fct-uelmicw4
because: The snapshot text, diff, MCP results and kept checks now carry a virtual node's id and declared name, a decision future carriers must respect.
@row_add n=5 row=decision/transformed-canvas-virtual-bounds area=screen-reader-support fact=F10 detector=none
name: Transformed canvas's virtual node bounds
rule: A virtual node's bounds are its region moved by its canvas's content-box origin alone, so a transformed canvas's virtual nodes read bounds with no transform applied.
basis: fct-xmpdsfkb
because: The capability statement now states this as a limit of the virtual node's bounds, which a planner must take into account.
@row_add n=6 row=decision/lint-holds-no-virtual-node area=visual-design-verification fact=F7 detector=none
name: Visual lint holds no virtual node by construction
rule: The visual lint excludes a canvas's virtual nodes by construction: a virtual node's lint entry has no element, so no region and no visible part, and no overlap or contrast finding names a virtual id; no skip in code holds this.
why: An inert skip that no input can turn red is not kept; a behaviour reading holds the fact instead.
guard: the_graph_actions_and_lint_hold_no_virtual_node
state: provisional
basis: fct-vgccdrgz
because: The planned lint skip was measured inert and dropped, so the exclusion rests on entries having no element, a statement no row holds yet.
@row_add n=7 row=decision/declaration-change-reaches-adapter-late area=a11y-strategy fact=F11 detector=none
name: Declaration change reaches a windowed adapter late
rule: A change of what a canvas declares marks nothing in the changed set; the snapshot rebuilds the tree per read, while a windowed adapter sees the change late.
basis: fct-wxv5k2j2
because: Virtual nodes now enter the tree while their declarations mark no change, a stated limit of the tree's refresh on change.
@row_none n=1 fact=F2 type=4
relied: a virtual node's tree id equals no DOM node's id
