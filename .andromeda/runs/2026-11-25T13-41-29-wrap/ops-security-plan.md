@row_set n=1 row=decision/virtual-node-crossing cell=rule seen=1 fact=F2 detector=none
value: A canvas's virtual node id `{owner}///{key}` and its author-declared name cross the accessibility tree, snapshot text, diff, MCP results, kept checks and socket and command-line answer line; the graph, actions and layout answers carry the id, never the name; the lint and canvas answer hold none.
basis: fct-wbnxhw6m, fct-w25w7t3u, fct-vxspd4cu, fct-llzzxhy4
because: The graph, actions and layout answers now carry a virtual node's id and no declared name, leaving only the lint and the canvas answer holding none, so the row's list of reads holding none is false.
@row_set n=2 row=contract/actions-answer/verbs cell=promise seen=3 detector=none
value: A list of entries each holding verb, a word of the verb table, and cause, a closed cause word left out when absent; every listed element carries layout with no cause, and canvas with no cause on an app that can paint text; a listed virtual node reads not-found on every verb but click and layout.
basis: fct-x6ohav4t, fct-lwbdf7ug
because: The actions answer now lists virtual nodes, whose target is found through their canvas and which read not-found on every verb that takes no virtual id, canvas included, so every listed node no longer carries canvas with no cause.
@row_set n=3 row=contract/stable-element-id/virtual-node-form cell=promise seen=3 detector=none
value: The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/` and given only to a canvas's taken declared nodes, served in the accessibility tree and, from it, the snapshot, its diff and the graph, actions and layout answers; no element's id holds `///`.
basis: fct-wbnxhw6m, fct-x6ohav4t
because: The graph, actions and layout answers now carry a virtual node's id, so the row's list of where the id is served is incomplete.
@row_add n=1 row=decision/virtual-id-verbs area=input-validation fact=F1 detector=none
name: Verbs that take a virtual node's id
rule: Only click and layout take a virtual node's id; type, scroll and screenshot with an id and canvas refuse a virtual id as not-found.
why: A decided limit.
state: provisional
basis: fct-x6ohav4t, fct-jzelteld, fct-uopwvezd, fct-w4id572v
because: The chunk widens the verbs taking a virtual id from click to click and layout and keeps the other id verbs refusing it, a decided limit no row of the section states.
@row_add n=2 row=decision/virtual-node-keyed-clickable-false area=input-validation fact=F5 detector=none
name: Virtual node reads keyed and clickable false
rule: A canvas's virtual node reads keyed false and clickable false wherever it is listed.
why: Decided limits.
state: provisional
basis: fct-x6ohav4t, fct-lwbdf7ug, fct-jzelteld, fct-uopwvezd, fct-w4id572v
because: The chunk lists virtual nodes in the actions answer with keyed false and keeps clickable false in the snapshot, decided limits no row of the section states.
@row_none n=1 fact=F3 type=9
what: A stated limit owned by a CARRY on Circle Drawer's circles as virtual nodes; the route holds it.
@row_none n=2 fact=F4 type=9
what: A stated limit owned by a CARRY on Circle Drawer's circles as virtual nodes; the route holds it.
