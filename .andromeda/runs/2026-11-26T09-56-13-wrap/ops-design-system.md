@row_set n=1 row=decision/circle-drawer-canvas-colours-in-code cell=rule seen=1 detector=D-design-tokens
value: Circle Drawer's canvas colours, the stand accent stroke, the dark label, the accent fill at alpha 38/255 and the selected circle's fill at alpha 96/255, are constants in the sample's widget code, the first three as the removed CSS rules held them.
basis: fct-pobztq6c, fct-oayiy3a5
because: The canvas gains a selected circle's fill, a constant in widget code flagged hardcoded, so the row's list of canvas colours held in code must name it.
@row_add n=1 row=decision/later-virtual-node-covers-earlier area=depth-strategy fact=F7 detector=none
name: Later-declared virtual node covers an earlier one
rule: A canvas's virtual node reads covered where a node its canvas declares later has a region holding the target's region centre; a region is the declared bounding box, so a later shape's corner covers where its painted shape does not.
state: provisional
basis: fct-wndwrhax, fct-arwtq2r5, fct-ti5ufy76
because: Declaration order now stands as depth among a canvas's virtual nodes for the covered refusal, where before another node of its canvas never covered one.
@row_add n=2 row=decision/circle-drawer-canvas-keyboard area=surface fact=F1 detector=none
name: Circle Drawer canvas keyboard path
rule: Circle Drawer's canvas is one Tab stop; arrows move the selected circle in declared order, clamped, the first arrow with none selected selecting the first; Enter opens the diameter dialog on the selected circle; a press while the dialog is open changes nothing.
state: provisional
basis: fct-yen3arbg, fct-da5yvz32, fct-blofjpfd, fct-ti5ufy76
because: The canvas sample gains a keyboard path chosen on a delegate's word, which future work on the canvas must respect.
@row_add n=3 row=decision/circle-drawer-click-selects-topmost area=surface fact=F2 detector=none
name: Circle Drawer click selects the topmost circle
rule: A click on Circle Drawer's canvas selects the topmost circle whose disc holds the point and adds none; only a click on no circle adds a circle.
basis: fct-yen3arbg, fct-bj6tkrbc, fct-ti5ufy76
because: A click on a circle no longer adds a circle but selects it, the founder's choice of selection on the canvas.
@row_add n=4 row=decision/widget-never-traps-tab area=surface fact=F3 detector=none
name: A focused custom widget never traps Tab
rule: A focused custom-widget element never traps Tab: the widget hears the keydown, then the engine's Tab and Shift+Tab focus order runs, so a widget cannot keep Tab for itself.
why: A focusable widget that traps Tab is a keyboard trap; differs from upstream, whose widget arm returns before every default action.
guard: tab_and_shift_tab_leave_a_focused_widget_which_still_hears_them
state: provisional
basis: fct-hssrbcfb, fct-o2pekkuo, fct-ti5ufy76
because: The engine's widget arm now passes Tab on to the focus order, a provisional fork from upstream that binds every custom widget.
@row_none n=1 fact=F4 type=4
relied: The engine focuses a custom-widget element on pointer down, so an app's canvas click needs no focus call of its own.
@row_add n=5 row=contract/virtual-node-accessibility/selected area=surface fact=F5 detector=none
boundary: The tree and snapshot of a canvas's virtual nodes
item: selected
promise: A virtual node's declared selected state reads selected for Some(true), unselected for Some(false) and no word for None, alike in the accessibility tree, the snapshot and a diff; an assistive technology learns Circle Drawer's current circle from selected alone.
basis: fct-yen3arbg, fct-7l3clzz3, fct-ti5ufy76
because: A virtual node's declaration gains a selected state promised outward through the tree, the snapshot and a diff.
@row_add n=6 row=decision/circle-drawer-dialog-disables-other-circles area=surface fact=F6 detector=none
name: Circle Drawer dialog disables the other circles
rule: While Circle Drawer's diameter dialog is open every circle but the adjusted one is declared disabled and refused disabled by the driver.
basis: fct-yen3arbg, fct-a6uxljfm
because: The open diameter dialog now declares the other circles disabled, a choice the canvas sample's interaction rests on.
