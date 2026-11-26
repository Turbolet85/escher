@row_set n=1 row=contract/refusal-causes/covered cell=promise seen=2 detector=none
value: The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first, and that a virtual node reads covered by an element or a later node of its canvas over its region's centre.
basis: fct-wndwrhax
because: The covered cause's fixed meaning now says a canvas's virtual node also reads covered by a node its canvas declares later whose region holds its region's centre, so the row's «only by an element» no longer holds.
@row_add n=1 row=decision/custom-widget-tab-pass-through area=input-validation fact=F1 detector=none
name: Tab passes through a focused custom widget
rule: After a custom widget hears a KeyDown of Tab, the engine runs the Tab default action on the focused custom-widget element, Shift reading backwards, so a focused widget element never traps Tab and a widget cannot keep Tab for itself.
why: A keyboard trap is an engine defect, fixed where it is; upstream's widget arm returns before every default action.
guard: stand_canvas_click tab_and_shift_tab_leave_a_focused_widget_which_still_hears_them
state: provisional
basis: fct-hssrbcfb
because: The engine's widget arm now runs the Tab default action after the widget hears Tab, an engine change for every document behind the feature that no row states.
@row_add n=2 row=decision/virtual-node-selected-closed area=input-validation fact=F3 detector=none
name: Virtual node selected state as a closed boolean
rule: A canvas's virtual node declaration states selected only as a closed optional boolean: none states nothing, a value is written as the node's selected state in the accessibility tree and read by the snapshot as an element's is; the tree's focus never names a virtual node.
basis: fct-yen3arbg
because: The virtual node declaration gains a selected field that crosses into the tree and the snapshot, a new declared field no row states.
@row_add n=3 row=decision/circle-drawer-canvas-keyboard area=input-validation fact=F4 detector=none
name: Circle Drawer canvas keyboard model
rule: Circle Drawer's canvas is one Tab stop: arrows move the selection in declared order, clamped, the first arrow with none selected selecting the first; Enter opens the dialog on the selected circle; with the dialog open no key acts and every circle but the adjusted one is declared disabled.
state: provisional
basis: fct-yen3arbg
because: Circle Drawer's canvas now takes keys through a closed key model, beside the Cells grid's row, and no row states it.
@row_add n=4 row=decision/covered-region-bounding-box area=error-handling fact=F5 detector=none
name: Covered read against a declared bounding box
rule: A virtual node's region in the covered reading is the bounding box its canvas declares, so a later node's corner refuses covered where the shape it paints does not cover.
state: provisional
basis: fct-wndwrhax
because: The covered cause's later-node reading takes a declared region as its bounding box, a recorded residual the restated contract row cannot hold within its bound.
@row_none n=1 fact=F2 type=4
relied: The canvas click's focus relies on the engine focusing a custom-widget element on pointer down, within the click's own settled step, with no app focus call.
