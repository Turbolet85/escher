@row_add n=1 row=decision/circle-drawer-canvas-keyboard-region area=surface-desktop-native fact=F1 detector=none
name: Circle Drawer canvas as one keyboard region
rule: Circle Drawer's canvas is one Tab stop whose arrows move the selected circle in declared order and stop at the ends, the first arrow with none selected selecting the first circle; Enter opens the dialog on the selected circle, and a key pressed while the dialog is open changes nothing.
basis: fct-yen3arbg, fct-blofjpfd, fct-da5yvz32
because: The chunk gives Circle Drawer's canvas a keyboard path, and no row of the surface states it.
@row_add n=2 row=decision/circle-drawer-click-selects-or-adds area=surface-desktop-native fact=F2 detector=none
name: Circle Drawer click selects or adds
rule: A click on Circle Drawer's canvas selects the topmost circle whose disc holds the point and adds none; only a click on no circle adds a circle.
basis: fct-yen3arbg, fct-bj6tkrbc
because: A click on a circle now selects it instead of adding one, and no row of the surface states the click's meaning.
@row_add n=3 row=decision/circle-drawer-dialog-disables-other-circles area=surface-desktop-native fact=F3 detector=none
name: Circle Drawer dialog disables other circles
rule: While Circle Drawer's diameter dialog is open, every circle but the one being adjusted declares disabled and is refused disabled.
basis: fct-yen3arbg, fct-a6uxljfm
because: The chunk makes the open dialog disable every other circle, a state no row of the surface holds.
@row_add n=4 row=decision/circle-drawer-canvas-keeps-size area=surface-desktop-native fact=F4 detector=none
name: Circle Drawer canvas keeps its size
rule: Circle Drawer's canvas keeps 700 x 400 and never shrinks, so on the 600 px stand view the dialog's Close lies below the fold and is scrolled into view before a click.
basis: fct-pobztq6c, fct-fpvtcznd, fct-7kzlkcnh
because: The canvas keeping its full size was chosen and moves Close below the stand view, which no row of the surface states.
@row_add n=5 row=decision/circle-drawer-canvas-selection-attributes area=surface-desktop-native detector=none
name: Circle Drawer canvas selection attributes
rule: Circle Drawer's canvas reads the selected circle and the circle being adjusted from two attributes beside data-circles, data-selected and data-adjusting, each a circle's id or empty.
basis: fct-wndwrhax, fct-pobztq6c
because: The canvas gains two attributes beside data-circles, and decision/circle-drawer-circles-painted-on-canvas's rule cannot take them within its bound.
@row_add n=6 row=decision/widget-tab-pass-through area=new fact=F7 detector=none
section: ## Engine: custom-widget input
name: Tab passes through a focused custom widget
rule: A focused custom-widget element hears a Tab keydown, and then the engine moves focus forward, or backward with Shift, so a widget cannot keep Tab for itself.
why: A focusable widget that traps Tab is an engine defect, a keyboard trap; upstream's widget arm returns before every default action, and the fix is flagged upstreamable.
state: provisional
basis: fct-hssrbcfb, fct-o2pekkuo, fct-yen3arbg
because: The engine now moves focus past a focused widget on Tab, a decision no area of layout-templates holds.
@row_add n=7 row=contract/escher-session-refusals/covered area=surface-cli fact=F9 detector=none
boundary: escher-session refusals
item: covered
promise: A canvas's virtual node is refused covered by an element over its region's centre, or by a node its canvas declares later whose declared bounding box holds that centre.
basis: fct-wndwrhax, fct-arwtq2r5
because: The covered meaning served by help and MCP now counts a later-declared node of the same canvas, and the surface holds no covered row.
@row_add n=8 row=contract/escher-session-snapshot/virtual-node-selected area=surface-cli fact=F10 detector=none
boundary: escher-session snapshot
item: virtual node selected state
promise: A virtual node's declared selected state, stated true, stated false or not stated, reads as selected, unselected or no word in the tree, the snapshot and its text.
basis: fct-7l3clzz3, fct-yen3arbg
because: A virtual node's declaration now carries a selected state that the snapshot reads, and no row of the surface states it.
@row_none n=1 fact=F5 type=9
what: An open limit carried on A11y re-read: the route holds it.
@row_none n=2 fact=F6 type=9
what: A recorded residual limit beside the CRUD list's, owned by the route and the a11y plan's limit row, not a layout row.
@row_none n=3 fact=F8 type=4
relied: An app's canvas click needs no focus call of its own, because the engine focuses a custom-widget element on pointer down.
