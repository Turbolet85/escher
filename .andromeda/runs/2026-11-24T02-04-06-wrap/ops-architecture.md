@row_set n=1 row=decision/click-focuses-button cell=except seen=2 fact=F1 detector=D-arch-decisions
value: a select focuses through its own arm, and a list box's row focuses its select; a click on a custom-widget target runs no click walk and clears no focus
basis: fct-t6akh6q6
because: The engine now synthesises the click on a custom-widget target with no click walk and no focus clear, so the row's «a click on any other element still clears focus» no longer holds for a widget target.
@row_set n=2 row=contract/headless-stand/boot cell=promise seen=3 detector=D-arch-resources
value: seven_guis::stand, native only, boots the four lean tasks, Cells, Circle Drawer, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font; Circle Drawer's boot runs in process only, served by no escher-session role.
basis: fct-t6akh6q6
because: The stand gains a public Circle Drawer boot, in process only and served by no role, so the stand's boot contract must name it.
@row_add n=1 row=decision/widget-click-synthesis area=established-decisions fact=F1 detector=D-arch-decisions
name: Click synthesis on a custom-widget target
rule: On a custom-widget target the engine, behind the custom-widget feature, synthesises the click on a main-button pointerup, contextmenu on a secondary one and a double click on a second click, after the widget's own handling and the focus set, with no click walk and no focus clear.
why: Ratified as a direct engine change for every document behind custom-widget, flagged upstreamable; upstream's widget arm returned before any click, the widget still hearing every pointer event first.
guard: stand_canvas_click
basis: fct-t6akh6q6
because: An app's click and double-click handlers on a widget element now fire, a new engine change for every document that no standing row states and the engine-change tally row has no room to hold.
@row_add n=2 row=decision/canvas-text-whole-pixel-glyph-origins area=standard-contracts fact=F2 detector=none
name: Canvas text across hosts on whole device pixels
rule: Text an app paints on a canvas is byte-equal across hosts only with its glyph origins on whole device pixels, so a canvas shot holding such text is held to one cross-OS digest only when painted so.
why: The CPU renderer snaps glyph origins differently on Apple hosts.
guard: stand_canvas_cross_os
state: provisional
basis: fct-x64dkiaq
because: A canvas shot holding app-painted text now stands in the cross-OS digest table, byte-equal only by the whole-pixel snap, a limit stated on a delegate's word that no standing row holds and the cross-OS shot exception has no room to take.
@row_add n=3 row=decision/widget-click-point-content-box area=established-decisions fact=F3 detector=none
name: Widget click point read against the content box
rule: An app that places custom-widget content from a click point reads that point against the widget's content box, subtracting border and padding from the element's client rect.
why: A widget paints in its content box while an element's client rect is its border box.
guard: stand_circle_drawer_canvas
basis: fct-c4pe4eil
because: The chunk measured that a click point read against the client rect lands off by the border and padding, a rule for every app painting from a click that the content-box paint row has no room to hold.
@row_add n=4 row=decision/canvas-placement-at-click-centre area=standard-contracts fact=F4 detector=none
name: Canvas placement only at the click centre
rule: A driver click lands at the centre of its target element, so an agent places content on a canvas only at the canvas's centre, no verb landing a pointer elsewhere.
why: Stated as a measured limit of the driver where an agent reads it.
state: provisional
basis: fct-z4jrcd3w
because: The capability statement gains the limit that an agent places canvas content only at the centre, on a delegate's word, a limit the click contract's centre landing does not state.
@row_add n=5 row=absence/canvas-keyboard-path area=established-decisions fact=F5 detector=none
capability: A keyboard path into a custom-widget canvas: the Circle Drawer canvas is reached by pointer alone
basis: fct-k4xssamx
because: The chunk builds the canvas with no keyboard path into it, a whole capability the project does not have.
