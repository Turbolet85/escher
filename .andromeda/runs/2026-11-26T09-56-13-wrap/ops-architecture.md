@row_set n=1 row=contract/nodes-widgets/virtual-nodes cell=promise seen=2 detector=D-arch-resources
value: Behind custom-widget, a defaulted method returning an empty list through which a widget declares regions, each a VirtualNodeDeclaration of key, AccessKit Role, name, region as a BoundingRect, a disabled flag and an optional selected state; the engine never calls it.
basis: fct-yen3arbg
because: VirtualNodeDeclaration gains the public field selected, None stating nothing and Some(b) written as the node's selected state, so the declaration's contract must name it.
@row_set n=2 row=contract/snapshot/virtual-node cell=promise seen=1 detector=none
value: A virtual node reads its id, declared role and name, enabled as not declared disabled, selected as its tree node reads it, invalid and focused, clickable false, highlighted false, no checked, value or pressed, bounds its region moved by its canvas's content-box origin, and no children.
basis: fct-yen3arbg
because: The snapshot's virtual branch now reads selected from its tree node as the element branch does, so the row's «no selected» is false.
@row_set n=3 row=contract/refusal-causes/covered cell=promise seen=2 detector=none
value: Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings, and a virtual node reads covered by an element over its region's centre or by a later-declared node of its canvas whose region holds that centre.
basis: fct-wndwrhax
because: The covered meaning is restated so a node its canvas declares later whose region holds the target's centre also covers it, so «covered only by an element» is false.
@row_set n=4 row=contract/node-keys/selected cell=promise seen=1 detector=none
value: Read from aria-selected, or on a virtual node from its declaration, written only where the element or the declaration states it.
basis: fct-7l3clzz3
because: A virtual node's declared selected now reads selected or unselected in the snapshot and its text, and no word where it states none, so the key no longer reads from aria-selected alone.
@row_add n=1 row=decision/widget-tab-pass-through area=established-decisions fact=F1 detector=none
name: Tab passes through a focused custom widget
rule: A focused custom-widget element does not trap Tab: the widget hears the KeyDown first, then the engine's Tab default action moves focus forward, Shift+Tab backward, so a widget cannot keep Tab for itself.
why: A direct engine change for every document behind custom-widget, flagged upstreamable; upstream's widget arm returns before every default action, so a focused widget element trapped Tab.
guard: stand_canvas_click
state: provisional
basis: fct-hssrbcfb
because: The engine's widget arm now runs the Tab default action after the widget hears the key, a new engine decision no row states.
@row_add n=2 row=decision/widget-focused-on-pointer-down area=established-decisions fact=F2 detector=none
name: A custom widget is focused on pointer down
rule: The engine focuses a custom-widget element on pointer down, so a click on it focuses it within the click's own settled step, blur and focusout firing on the prior node and focus and focusin on the widget, and an app makes no focus call of its own.
guard: stand_circle_nodes
state: provisional
basis: fct-h4d3faes
because: The pointer-down focus of a custom-widget element was measured to make an app's own focus call redundant, a behaviour apps now rely on that no row states.
@row_add n=3 row=decision/circle-drawer-canvas-single-tab-stop area=standard-contracts fact=F3 detector=none
name: Circle Drawer canvas is one Tab stop with arrows and Enter
rule: Circle Drawer's canvas is one Tab stop whose arrows move the selection over circles in declared order, clamped, the first arrow with none selected selecting the first, and whose Enter opens the diameter dialog on the selected circle; a press with the dialog open changes nothing.
guard: stand_keyboard, stand_circle_nodes
state: provisional
basis: fct-blofjpfd
because: Circle Drawer's canvas gains a keyboard path, its circles reached, selected and acted on by keys, as the Cells grid's one Tab stop is stated.
@row_add n=4 row=contract/accessibility-tree/virtual-node-selected area=standard-contracts fact=F4 detector=none
boundary: Accessibility tree
item: virtual node selected
promise: A canvas's virtual node takes selected as its declaration states it, none where it states none; a canvas's current node is conveyed by selected alone, the tree's focus staying on the canvas and no active descendant naming a virtual node.
basis: fct-yen3arbg
because: The tree now writes a virtual node's declared selected state and leaves its focus on the canvas, a promise the full virtual-nodes row cannot take within its bound.
@row_add n=5 row=decision/covered-by-later-node-region area=standard-contracts fact=F5 detector=none
name: A later node covers by its declared bounding box
rule: A virtual node's covered reading by a later-declared node of its canvas reads that node's declared bounding-box region, so a later node's corner refuses covered where the shape it paints does not cover.
guard: stand_circle_nodes
state: provisional
basis: fct-wndwrhax
because: The later-node covered reading is region-based, and its recorded limit is that a bounding-box corner covers where the painted shape does not.
