@row_set n=1 row=contract/refusal-cause/not-found cell=promise seen=2 fact=F4 detector=none
value: A call naming an id not on the screen, or a virtual node's id on any verb but `click` and `layout`, is refused `not-found`, the instance unchanged.
basis: fct-vxspd4cu
because: The verbs taking a virtual id moved from `click` alone to `click` and `layout`, so `layout` no longer refuses a virtual id not-found.
@row_set n=2 row=contract/driver-refusal-causes/not-found cell=promise seen=2 detector=none
value: An id no screen of the session read, or that no element carries, is refused not-found; every id verb but `click` and `layout` refuses a virtual node's id not-found.
basis: fct-vxspd4cu
because: The verbs taking a virtual id moved from `click` alone to `click` and `layout`, so the row's exception must name both.
@row_set n=3 row=contract/driver-session/stale-or-not-found cell=promise seen=2 detector=none
value: An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not; a virtual node's id is read `not-found` by every verb but `click` and `layout`, which take a virtual node the screen lists.
basis: fct-x6ohav4t
because: `click` and `layout` now share the resolver that takes a virtual id, so `layout` no longer reads one not-found.
@row_set n=4 row=contract/handler-coverage-limits/canvas-handler cell=promise seen=1 detector=none
value: A canvas's virtual node carries no handler of its own; a click on one counts its canvas's listener under the canvas's id.
basis: fct-kiimqirg
because: A replayed virtual click now records the canvas's click listener under the canvas's id, so the stated limit that a canvas handler is not counted no longer holds.
@row_set n=5 row=contract/element-id/virtual-form cell=promise seen=2 detector=none
value: The form `{owner}///{key}` is reserved for a canvas's virtual nodes and served in the accessibility tree, the snapshot and its diff, the UI graph, the actions answer and the layout answer: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
basis: fct-wbnxhw6m
because: The graph, actions and layout answers now carry a virtual node's `{owner}///{key}` id, so the places the form is served grew.
@row_retire n=1 row=decision/reserved-graph-id-form seen=2 heir=decision/lint-holds-no-virtual-node detector=none
why: The UI graph and the actions answer now hold the reserved virtual id form; only the lint answer still holds none.
basis: fct-w25w7t3u
because: The stated limit that the graph, actions, the lint and exploration hold no virtual node is retired for all but the lint, so the row's rule is false.
@row_retire n=2 row=decision/handler-coverage-canvas-uncounted seen=1 heir=decision/handler-coverage-virtual-click-counts-canvas detector=none
why: Handler coverage now counts a click on a canvas's virtual node as its canvas's listener, so a canvas's handler is counted.
basis: fct-kiimqirg
because: A replayed virtual click reaches its canvas listener and is counted, contradicting the row's rule.
@row_add n=1 row=decision/lint-holds-no-virtual-node area=unit-test-strategy fact=F3 detector=none
name: The lint reads no virtual node
rule: The lint reads no canvas's virtual node: it makes no overlap, clipped or off-screen finding on a canvas's region, and no id of the lint answer holds the reserved `{owner}///{key}` form.
guard: the_lint_holds_no_virtual_node
state: provisional
basis: fct-llzzxhy4
because: The lint is now the one read holding no virtual node, a stated limit the chunk pins, replacing the retired graph-and-actions rule.
@row_add n=2 row=decision/handler-coverage-virtual-click-counts-canvas area=integration-test-strategy detector=none
name: Virtual click counted as its canvas's listener
rule: Handler coverage counts a click on a canvas's virtual node as its canvas's click listener, under the canvas's id, since a virtual node carries no handler of its own.
guard: a_replayed_virtual_click_reaches_its_canvas_listener
state: provisional
basis: fct-kiimqirg
because: Coverage now counts a virtual click through its canvas, replacing the retired rule that a canvas handler is not counted.
@row_add n=3 row=contract/session-run/layout-virtual-node area=integration-test-strategy fact=F1 detector=none
boundary: escher_driver Session
item: layout on a virtual node
promise: A `layout` on a canvas's virtual node answers it placed by `app` in its canvas, at the region's offset from the canvas's border box, with the snapshot's bounds, the canvas's shifts and size cause `layout`; the engine is never asked for a region's reason.
basis: fct-lrqd5vrd
because: `layout` now takes a virtual id and answers it from its canvas, a new promise of the layout verb.
@row_add n=4 row=contract/driver-verbs/actions-virtual-node area=integration-test-strategy fact=F2 detector=none
boundary: driver verbs
item: actions on a virtual node
promise: `actions` lists a canvas's virtual node with its canvas as owner and keyed false, finding its target through its canvas; a verb that takes no virtual id reads `not-found` for it, every other verb decided as for any node.
basis: fct-lwbdf7ug
because: The actions answer now lists virtual nodes and decides their verbs through the canvas, a new promise of the actions verb.
@row_add n=5 row=contract/exploration/virtual-nodes area=session-lifecycle fact=F6 detector=none
boundary: exploration
item: virtual nodes
promise: An exploration acts on a canvas's virtual nodes as on every node the actions answer lists, and its explored path's check replays those steps.
basis: fct-kytgando
because: Exploration no longer holds no virtual node; it now acts on them and its kept check replays them.
@row_none n=1 fact=F5 type=4
relied: A fixture page's absolute coordinates sit inside the body's default 8 px margin, which a check pinning a fixture's page coordinates measures rather than forecasts; the standing literal-pin rule already covers pinning as measured.
