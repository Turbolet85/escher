@row_add n=1 row=decision/canvas-text-on-whole-device-pixels area=test-strategy fact=F1 detector=none
name: Canvas text on whole device pixels across hosts
rule: Text an app paints on a canvas that a cross-host shot comparison holds is drawn with its glyph origins on whole device pixels, so Circle Drawer's canvas with its diameter label reads one answer digest on linux, windows and macOS in both layout modes.
why: The CPU renderer snaps glyph origins differently on Apple hosts, so canvas text off whole device pixels differs in bytes across hosts.
guard: stand_canvas_cross_os
state: provisional
basis: fct-yshqj5md, fct-x64dkiaq, fct-xivjls5n
because: The chunk's cross-OS canvas table gained Circle Drawer's canvas painting text, held byte-equal on three hosts only by the whole-pixel glyph snap, a rule on the escher overseer's word that no row states.
@row_add n=2 row=decision/widget-element-click-reaches-app area=session-lifecycle fact=F2 detector=none
name: Click on a custom-widget element reaches the app
rule: A click on a custom-widget element reaches the app's click handler, and two clicks with no advance its double-click handler, in both layout modes, the widget hearing every pointer event first.
why: Differs from upstream, whose widget arm returned before the click synthesis.
guard: stand_canvas_click
basis: fct-t6akh6q6, fct-b3zdv2v7, fct-yshqj5md
because: The engine now synthesises click and double click on a custom-widget target after the widget handles the event, a ratified engine change no row of the area states.
@row_add n=3 row=contract/stand-boot/boot-circle-drawer area=3-test-harness-contract fact=F3 detector=none
boundary: seven_guis headless stand
item: stand::boot_circle_drawer
promise: stand::boot_circle_drawer(HarnessOptions) boots Circle Drawer in process alone, served by no escher-session role.
basis: fct-t6akh6q6, fct-xjtjetyy
because: The stand gained a public Circle Drawer boot beside the other stand::boot contracts, which state each boot one an item.
@row_add n=4 row=decision/canvas-circles-read-from-attribute-and-pixels area=integration-test-strategy fact=F4 detector=none
name: Circle Drawer's circles read from the canvas
rule: Circle Drawer's circles are no DOM nodes: a check reads them through the canvas's data-circles attribute (x,y,d per circle, semicolon-joined, malformed text read as no circle) and its shot pixels, never through snapshot, tree, graph or lint.
state: provisional
basis: fct-t6akh6q6, fct-ryoj2i3n, fct-yshqj5md
because: The chunk moved Circle Drawer's circles out of the DOM onto a canvas, so checks read them only through the attribute and shot pixels, on the escher overseer's word.
@row_add n=5 row=decision/click-point-against-content-box area=integration-test-strategy fact=F5 detector=none
name: Canvas click point read against the content box
rule: A click point placing canvas content is read against the canvas's content box, not its client rect.
why: The client rect is the border box while a widget paints in its content box, so a point read against the client rect is off by the border and padding.
guard: stand_circle_drawer_canvas
basis: fct-c4pe4eil, fct-hzprrvfq, fct-nk3wyknj
because: The chunk measured the client rect off by the canvas border and made the app and its check read the click against the content box, a convention no row states.
