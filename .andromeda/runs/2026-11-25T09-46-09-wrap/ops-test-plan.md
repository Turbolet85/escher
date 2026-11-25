@row_set n=1 row=decision/reserved-graph-id-form cell=rule seen=1 fact=F1 detector=none
value: No id of the UI graph, the actions answer or the lint answer holds the reserved `{owner}///{key}` form; the lint holds no virtual node by construction, its entry having no element and so no visible part to pair, with no skip in code.
basis: fct-klrtrzdi, fct-ldtw2iop, fct-kgfnnhll
because: Virtual node ids are now served in the accessibility tree, the snapshot and its diff, so the rule narrows to the graph, actions and lint answers, and the lint's exclusion rests on construction, not a skip.
@row_set n=2 row=contract/element-id/virtual-form cell=promise seen=1 detector=none
value: The form `{owner}///{key}` is reserved for a canvas's virtual nodes and served in the accessibility tree, the snapshot and its diff: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
basis: fct-klrtrzdi, fct-w2gujpju
because: The reserved form now has a producer, so the contract states where it is served while the walk still writes none.
@row_set n=3 row=contract/driver-session/stale-or-not-found cell=promise seen=1 detector=none
value: An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not; a virtual node's id is read `not-found` by every verb but `click`, which acts on a virtual node the screen lists.
basis: fct-w2gujpju
because: A read virtual id now reads not-found on every id verb but click, and click takes a listed virtual id, so the stale or not-found split no longer covers every id.
@row_set n=4 row=contract/driver-session/refusal-order cell=promise seen=1 detector=none
value: A `click` or `type` on an element, or a `click` on a virtual node, that cannot take it is refused with the first that holds of `disabled` · `off-screen` · `covered`, leaving the snapshot and both focus readings as they were.
basis: fct-7yeqdilt, fct-kjiqs4jp
because: A click on a virtual node is now refused through the same checks in the one order, so the contract names it beside the element case.
@row_set n=5 row=contract/driver-refusal-causes/not-found cell=promise seen=1 detector=none
value: An id no screen of the session read, or that no element carries, is refused not-found; every id verb but `click` refuses a virtual node's id not-found.
basis: fct-w2gujpju
because: The not-found cause now also answers a virtual id on type, scroll, screenshot, layout and canvas, and its meaning was reworded for it.
@row_set n=6 row=contract/refusal-cause/not-found cell=promise seen=1 detector=none
value: A call naming an id not on the screen, or a virtual node's id on any verb but `click`, is refused `not-found`, the instance unchanged.
basis: fct-w2gujpju
because: A virtual node's id stands on the screen yet reads not-found on every id verb but click.
@row_set n=7 row=contract/refusal-cause/covered cell=promise seen=1 detector=none
value: A target a hit at its centre does not reach, or a virtual node with an element over its region's centre, is refused `covered`; the fixed meaning also says an option of a closed select lies under it, to click the select first and then the option.
basis: fct-w2gujpju, fct-xn5aba5k
because: The covered cause now answers a click on a virtual node and its meaning was reworded for it.
@row_add n=1 row=contract/session-run/virtual-node-click area=integration-test-strategy fact=F2 detector=D-tests-coverage
boundary: escher_driver Session
item: click on a virtual node
promise: `click` is the one id verb that takes a virtual node, acting at its canvas region and focusing the canvas, refused `disabled`, `off-screen`, `covered`, `not-found` or `stale` in the one order; `type`, `scroll`, `screenshot` with an id, `layout` and `canvas` read a virtual id `not-found`.
basis: fct-w2gujpju, fct-7yeqdilt
because: The chunk opens click to virtual nodes and closes every other id verb to them, a contract no standing row states.
@row_add n=2 row=contract/snapshot-node/virtual-node area=unit-test-strategy fact=F3 detector=none
boundary: Snapshot node
item: virtual node
promise: A virtual node reads its id, declared role and name, `enabled` from its declared disabled, `clickable` false, `highlighted` false, no value, checked, selected or pressed, bounds its region moved by its canvas's content-box origin, and no children.
basis: fct-w2gujpju
because: The snapshot now holds virtual nodes, and what each reads is a promise of the snapshot's node shape no row states.
@row_add n=3 row=contract/accessibility-tree/virtual-nodes area=test-strategy fact=F4 detector=none
boundary: Accessibility tree
item: virtual nodes
promise: Each virtual node is a child of its canvas's tree node, after its own children, with author id `{owner}///{key}`, declared role and name and disabled where declared, under an even-versioned tree id equal to no DOM id; a hidden, collapsed or inert canvas, or one whose read errs, holds none.
basis: fct-w2gujpju, fct-7yeqdilt
because: The accessibility tree now carries a canvas's virtual nodes, a contract at the tree no row states.
@row_add n=4 row=decision/virtual-node-crosses-mcp-and-kept-check area=session-lifecycle fact=F5 detector=none
name: Virtual node crosses the MCP role and a kept check
rule: A virtual node's id and declared name cross the MCP role and a kept check, and the kept check replays its virtual click; the socket and command line carry the same run line by construction.
why: Ratified as a boundary widening.
guard: the_mcp_role_carries_the_virtual_node, a_kept_check_holds_and_replays_the_virtual_click
basis: fct-uelmicw4, fct-7yeqdilt
because: The chunk widens what crosses the carriers to a virtual node's id and declared name, a decision no session row states.
@row_add n=5 row=decision/inert-guard-dropped area=unit-test-strategy fact=F6 detector=none
name: Inert guard dropped for a behaviour reading
rule: A guard no input can turn red is not kept; where its control reads it inert, the guard is dropped and the fact it guarded is pinned by a behaviour reading instead.
state: provisional
basis: fct-fkpv7eyw, fct-kgfnnhll
because: The chunk's ruling sets how an inert guard is handled, a rule the mutation-control row does not state, landing provisional as a delegate's word.
