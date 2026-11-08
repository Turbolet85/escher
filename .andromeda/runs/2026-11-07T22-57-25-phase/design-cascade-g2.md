@carried n=1 label=design-system/surface fact=fct-grd6lupt
why: The graph must key on the agent-visible node as the snapshot is built, so an open list's row, a snapshot node with its own bounds and no DOM element, must be keyed with the select as its optional owner element and keep its bounds and its crop unchanged.
quote: A row of an open list carries its own `bounds` in the snapshot, and a `screenshot` of a row crops to it and leaves the list open
@carried n=2 label=design-system/surface fact=fct-d47jlprw
why: The reserved virtual-node id form of canvas element plus virtual key must be shaped for this surface, where the whole app is one canvas, while the chunk produces no virtual node and adds no canvas renderer.
quote: On wasm32 the same surface is a canvas appended to the page body, sized from host CSS
@carried n=2 run=2 label=design-system fact=fct-x3p4qjpb
quote: on wasm32 dioxus-native's surface is a canvas appended to the page body, sized from host CSS
@binding n=1
**surface (an open list's row in the snapshot) ↔ surface (the web-spa canvas)**: both are agent-visible nodes with no DOM element of their own, so the graph must key each on the accesskit node with its bounds and an optional owner element (the select, the canvas) whose one listener serves the sub-targets, rather than on `Target.node` alone (design-system/surface fct-grd6lupt) (design-system/surface fct-d47jlprw)
