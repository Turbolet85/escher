@row_set n=1 row=decision/stand-lints-clean-at-boot cell=rule seen=2 detector=none
value: At boot the counter, the flight booker, the timer, CRUD, the load page, the order page and Circle Drawer read zero visual-lint findings, in both layout modes.
basis: fct-aphn5qn7, fct-km6xkrby
because: Circle Drawer now boots with lint 0 in both layout modes, so the set of tasks that lint clean at boot gains it.
@row_set n=2 row=contract/stand-element-ids/circle-canvas cell=promise seen=1 detector=D-layout-surface
value: Stable author element id of the Circle Drawer's canvas, an object element with role group named "Drawing area".
basis: fct-t6akh6q6, fct-km6xkrby
because: The drawing area is now one object element with role group and the name Drawing area in place of a div holding circle nodes, so the id's promise must name what it marks.
@row_add n=1 row=decision/circle-drawer-circles-painted-on-canvas area=surface-desktop-native fact=F1 detector=D-layout-surface
name: Circle Drawer circles painted on a canvas
rule: Circle Drawer's circles are painted by a canvas widget from one data-circles attribute (x,y,d per circle, semicolon-joined, malformed text read as no circle), never as DOM nodes, so they reach no snapshot, tree, graph or lint and cross as shot pixels alone.
state: provisional
basis: fct-t6akh6q6, fct-ryoj2i3n, fct-qhin2lqw, fct-yshqj5md
because: The chunk replaced the circle DOM nodes with a canvas widget read from one attribute, a choice resting on a delegate's word, so the surface gains it as a provisional decision.
@row_add n=2 row=decision/widget-content-placed-against-content-box area=surface-desktop-native fact=F2 detector=none
name: Widget content placed against the content box
rule: An app placing widget content from a click reads the point against the widget's content box, subtracting border and padding from the client rect.
why: The client rect is the border box while a widget paints in its content box.
basis: fct-c4pe4eil, fct-hzprrvfq
because: The chunk measured the client rect off from the widget's paint box by the border and made the app read clicks against the content box, a convention every widget-placing app must follow.
@row_add n=3 row=decision/canvas-text-glyph-origins-whole-pixels area=surface-desktop-native fact=F3 detector=none
name: Canvas text on whole device pixels
rule: Text an app paints on a canvas places each glyph origin on a whole device pixel, so the canvas reads byte-equal across hosts; Circle Drawer's diameter label is drawn so.
why: The CPU renderer snaps glyph origins differently on Apple hosts than elsewhere.
guard: stand_canvas_cross_os
state: provisional
basis: fct-x64dkiaq, fct-yshqj5md
because: The macOS digest red was caused by glyph-origin snapping and fixed by drawing the label on whole device pixels, a rule resting on a delegate's word for every canvas painting text.
@row_add n=4 row=decision/circle-drawer-diameter-label area=surface-desktop-native fact=F4 detector=none
name: Circle Drawer diameter label
rule: Circle Drawer's canvas shows each circle's diameter as an integer, 12 px sans-serif, centred on the circle.
state: provisional
basis: fct-t6akh6q6, fct-yshqj5md
because: The diameter as the canvas text was chosen at a fork on a delegate's word, so the surface gains it as a provisional decision.
@row_add n=5 row=decision/widget-element-click-synthesis area=surface-desktop-native fact=F6 detector=none
name: Click synthesized on a custom-widget element
rule: An app's onclick and ondoubleclick on a custom-widget element fire, the engine synthesizing click, context menu and double click after the widget hears each pointer event, in every document behind custom-widget.
why: Ratified; upstream returned before the synthesis.
guard: stand_canvas_click
basis: fct-t6akh6q6, fct-km6xkrby, fct-b3zdv2v7
because: The engine now synthesizes clicks on a custom-widget target, a ratified change for every document, so the surface's interaction rules gain it.
@row_none n=1 fact=F5 type=9
what: The missing keyboard path into Circle Drawer's canvas is named a later entry, an owed item the route holds.
