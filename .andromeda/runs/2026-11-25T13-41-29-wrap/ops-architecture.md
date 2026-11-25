@row_set n=1 row=contract/driver-verbs/click-virtual-node cell=promise seen=1 detector=none
value: click takes a virtual node id, as layout does: it acts on the canvas at the virtual region and is refused in the one order disabled, off-screen, covered, not-found, stale with nothing run; type, scroll, screenshot with an id and canvas read a virtual id not-found.
basis: fct-w25w7t3u, fct-x6ohav4t
because: The stated limit that only click takes a virtual id is retired, layout now taking one too, so the row's claim that layout reads a virtual id not-found is false.
@row_set n=2 row=contract/refusal-causes/not-found cell=promise seen=2 detector=none
value: The id names no element on the screen as it reads now and the session's record does not hold it, or names a virtual node to a verb other than click and layout; nothing runs.
basis: fct-x6ohav4t
because: not-found's fixed meaning now names click and layout as the verbs taking a virtual id, so the cause's promise must follow.
@row_set n=3 row=contract/ui-graph/graph cell=promise seen=2 detector=none
value: Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
basis: fct-x6ohav4t, fct-nbmlbttg
because: The filter that dropped a virtual node from the graph is lifted, so every snapshot node, a virtual node included, is one graph node and the exception is false.
@row_set n=4 row=contract/ui-graph/owner cell=promise seen=1 detector=none
value: The stable id of the select an option or an optgroup belongs to, or of the canvas that declares a virtual node.
basis: fct-x6ohav4t
because: A virtual node in the graph is now owned by its canvas, so the owner promise naming only a select is incomplete.
@row_set n=5 row=contract/handler-coverage/canvas cell=promise seen=1 detector=none
value: A virtual node a canvas declares carries no handler of its own; a click on one counts its canvas's listener under the canvas's id.
basis: fct-x6ohav4t, fct-kiimqirg
because: A replayed virtual click now records the canvas's click listener under the canvas's id, so the row saying handlers inside a canvas are not counted is false.
@row_set n=6 row=contract/dioxus-bridge/virtual-node-id cell=promise seen=3 detector=none
value: A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk; DioxusDocument::virtual_nodes gives it in process to a canvas's taken nodes, and the accessibility tree serves it, and from it the snapshot, its diff, the UI graph, actions and layout.
basis: fct-wbnxhw6m, fct-w25w7t3u
because: The graph, actions and layout answers now carry a virtual node's id, so the reads the row names as serving it are incomplete.
@row_set n=7 row=contract/stable-element-id/virtual-node-id cell=promise seen=3 detector=none
value: A reserved fifth form {owner}///{key}, given a canvas's taken nodes and served in the accessibility tree and from it the snapshot, its diff, the UI graph, actions and layout, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
basis: fct-wbnxhw6m, fct-w25w7t3u
because: The graph, actions and layout answers now carry a virtual node's id, so the reads the row names as serving it are incomplete.
@row_set n=8 row=contract/driver-session/canvas cell=promise seen=1 detector=none
value: A read answering a CanvasReading of one canvas in process, resolving its id as the run path does with a virtual id read not-found, refusing fonts-unavailable where the document cannot paint text, changing nothing and opening no span of its own.
basis: fct-x6ohav4t
because: Session::layout now resolves a virtual id while canvas still refuses one not-found, so resolving as Session::layout does is no longer true.
@row_add n=1 row=contract/driver-session/layout-virtual-node area=standard-contracts fact=F1 detector=D-arch-resources
boundary: escher-driver session
item: app placer of a virtual node
promise: Layout on a virtual node id answers it placed by app in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and both size causes layout; the engine is never asked for a region's reason.
basis: fct-x6ohav4t, fct-lrqd5vrd
because: Layout now explains a virtual node instead of refusing it, a new answer at the session's boundary no row states.
@row_add n=2 row=decision/virtual-id-verb-table area=standard-contracts fact=F2 detector=D-arch-decisions
name: One table of the verbs taking a virtual id
rule: The verbs that may take a virtual node id are named in one table keyed on the verb, which the run path, the in-process layout and actions all read, so no reader keeps its own list of them.
why: Keyed on the verb rather than a command, so the in-process layout, which holds no command, reads the same table as click and actions.
state: provisional
basis: fct-x6ohav4t, fct-qbim7ywh
because: The chunk chose one shared table for the verbs taking a virtual id, a decision future verbs must respect and no row holds.
@row_add n=3 row=contract/actions-verb/virtual-node area=standard-contracts detector=none
boundary: The actions verb
item: virtual node
promise: A listed virtual node's verbs are read against its canvas, each cause the one its call returns: a verb that takes no virtual id reads not-found, and click and layout meet the checks their call meets.
basis: fct-x6ohav4t, fct-lwbdf7ug
because: Actions now lists a virtual node with its target found through its canvas, which the target-verbs row cannot hold within its bound.
@row_none n=1 fact=F3 type=9
what: The lint reading no virtual node is a decided limit carried by a CARRY on Circle Drawer's circles as virtual nodes; the route holds it.
