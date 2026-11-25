@row_add n=1 row=contract/accessibility-tree/virtual-nodes area=standard-contracts fact=F1 detector=D-arch-resources
boundary: Accessibility tree
item: virtual nodes
promise: Under the accessibility feature the Dioxus document's tree holds each canvas's taken virtual nodes as children of the canvas's node, after its own: role and label as declared, author_id {owner}///{key}, disabled where declared; a hidden, collapsed or inert canvas, or one whose read errs, takes none.
basis: fct-w2gujpju
because: The Dioxus document's accessibility tree now adds each canvas's taken virtual nodes, a promise at the tree's boundary no row states.
@row_add n=2 row=decision/virtual-node-tree-id area=established-decisions fact=F2 detector=D-arch-decisions
name: A virtual node's accessibility tree id
rule: A virtual node's accessibility tree id is its canvas's slot index low and 2·(n+1) high, and a canvas past 2^31 − 1 virtual nodes takes no tree node.
why: an even high half equals no DOM node id
basis: fct-w2gujpju
because: The chunk chose how a virtual node is numbered in the accessibility tree so it can never collide with a DOM node, a decision no row holds.
@row_add n=3 row=contract/snapshot/virtual-node area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Snapshot
item: virtual node
promise: A virtual node reads its id, declared role and name, enabled as not declared disabled, invalid and focused, clickable false, highlighted false, no checked, value, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
basis: fct-w2gujpju
because: The snapshot now reads a canvas's virtual nodes with a fixed reading of each field, a contract no snapshot row states.
@row_add n=4 row=contract/driver-verbs/click-virtual-node area=standard-contracts fact=F4 detector=D-arch-resources
boundary: driver verbs
item: click on a virtual node id
promise: click is the one verb taking a virtual node id: it acts on the canvas at the virtual region and is refused in the one order disabled, off-screen, covered, not-found, stale with nothing run; type, scroll, screenshot with an id, layout and canvas read a virtual id not-found.
basis: fct-w2gujpju
because: click now resolves a virtual node id while every other id verb reads it not-found, a verb contract no row states.
@row_add n=5 row=decision/virtual-node-crossing area=established-decisions fact=F6 detector=D-arch-decisions
name: Virtual node ids and names cross the carriers
rule: A virtual node's id and author-declared name cross the snapshot text, the diff, MCP results and kept checks, with no carrier, verb, key, field or cause added.
why: A boundary widening of what the existing carriers hold, ratified.
basis: fct-uelmicw4
because: The carriers now hold a virtual node's id and declared name, a ratified widening no crossing row names.
@row_add n=6 row=decision/declaration-change-marks-nothing area=cross-cutting-patterns fact=F8 detector=none
name: A virtual node declaration change marks nothing
rule: A change of what a canvas declares writes no changed-set mark and sends no invalidation signal, so a windowed accessibility adapter sees it only at its next rebuild, while each snapshot reads the declarations afresh.
why: a mark or a signal would amend the ratified changed-set write sites and marks for every document
basis: fct-klrtrzdi
because: The provisional rule left the mark to this chunk, which added none and has each snapshot read the declarations afresh, so the rule stands without «yet».
@row_add n=7 row=contract/stable-element-id/virtual-node-form-served area=standard-contracts detector=D-arch-resources
boundary: Stable element ids
item: virtual node form served
promise: The reserved virtual node form {owner}///{key} is served in the accessibility tree and, from it, in the snapshot and its diff.
basis: fct-w2gujpju
because: The form is now served beyond the in-process declared-node read, and the standing row cannot say so within its bound.
@row_retire n=1 row=decision/virtual-node-declaration-marks-nothing seen=1 heir=decision/declaration-change-marks-nothing detector=none
why: Its name, rule and reason await a refreshing reader that has now landed without a mark, so the decision is restated as settled in its heir.
basis: fct-klrtrzdi
because: The report's expected amendment drops «yet» and the pending reader from the rule, more than one cell of the row.
@row_set n=1 row=contract/ui-graph/graph cell=promise seen=1 fact=F5 detector=D-arch-resources
value: Read-only under accessibility, returns one node per snapshot node in pre-order, a virtual node excepted, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
basis: fct-w2gujpju
because: The graph now skips every snapshot node whose id holds the virtual separator, so it no longer holds one node per snapshot node.
@row_set n=2 row=contract/refusal-causes/covered cell=promise seen=1 fact=F7 detector=D-arch-resources
value: Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings, and a virtual node reads covered only by an element over its region's centre.
basis: fct-xn5aba5k
because: The covered meaning was reworded for virtual nodes, read covered only at their region's centre, a stated limit.
@row_set n=3 row=contract/snapshot/bounds cell=promise seen=1 detector=D-arch-resources
value: A row's or group's place in an open list, a virtual node's region moved by its canvas's content-box origin and read untransformed, else the element's viewport-relative, unclamped client bounding rect.
basis: fct-xmpdsfkb
because: The snapshot now gives virtual nodes bounds from their region, untransformed under a transformed canvas, which the row does not state.
@row_set n=4 row=contract/snapshot/membership cell=promise seen=1 detector=D-arch-resources
value: A node is an accessibility tree node with an author_id whose element resolves, or a canvas's taken virtual node; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
basis: fct-w2gujpju
because: The snapshot now holds virtual nodes, which have no element, so membership by a resolving element alone is no longer whole.
@row_set n=5 row=contract/refusal-causes/not-found cell=promise seen=1 detector=D-arch-resources
value: The id names no element on the screen as it reads now and the session's record does not hold it, or names a virtual node to a verb other than click; nothing runs.
basis: fct-w2gujpju
because: Every id verb but click now reads a virtual node id not-found, and the not-found meaning was reworded with it.
@row_set n=6 row=contract/dioxus-bridge/virtual-node-id cell=promise seen=2 detector=D-arch-resources
value: A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk; DioxusDocument::virtual_nodes gives it in process to a canvas's taken nodes, and the accessibility tree serves it, and from it the snapshot and its diff.
basis: fct-w2gujpju
because: The form is now served in the accessibility tree, snapshot and diff, so «no carrier serves it» is false.
@row_set n=7 row=contract/nodes-widgets/virtual-nodes cell=promise seen=1 detector=D-arch-resources
value: Behind custom-widget, a defaulted method returning an empty list through which a widget declares regions, each a VirtualNodeDeclaration of key, AccessKit Role, name, region as a BoundingRect and a disabled flag; the engine never calls it.
basis: fct-w2gujpju
because: VirtualNodeDeclaration gained a public disabled field, a change of the public declaration contract.
@row_set n=8 row=decision/canvas-placement-at-click-centre cell=rule seen=2 detector=D-arch-decisions
value: A driver click lands at the centre of its target element, or on a canvas at a declared virtual node's region, so an agent places content on a canvas only at the canvas's centre or a declared region, no verb landing a pointer elsewhere.
basis: fct-w2gujpju
because: click on a virtual node id now acts on the canvas at the virtual region, so the canvas's centre is no longer the only place a click lands on it.
