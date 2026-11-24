@row_add n=1 row=decision/canvas-text-glyph-origins-whole-pixels area=typography fact=F1 detector=none
name: Canvas text glyph origins on whole device pixels
rule: Text an app paints on a canvas is byte-equal across hosts only with each glyph origin on a whole device pixel, so canvas text checked by digest across hosts is drawn with its glyph origins so placed.
why: The CPU renderer snaps glyph origins on Apple hosts to a different grid than elsewhere.
guard: stand_canvas_cross_os
state: provisional
basis: fct-x64dkiaq, fct-yshqj5md
because: The chunk's fix found canvas text differing on macOS and made the label byte-equal on all three hosts by whole-pixel glyph origins, a rule future canvas text must follow, applied ahead of ratification.
@row_add n=2 row=decision/widget-element-click-synthesis area=surface fact=F2 detector=none
name: Click synthesis on a custom-widget element
rule: On a custom-widget element the engine delivers every pointer event to the widget first, then synthesises the app's click on the main button, context menu on the secondary button and double click on a second click within the window, with no click walk and no focus clear.
why: So an app's onclick and ondoubleclick on a widget element fire for every app behind custom-widget; upstream's widget arm returned before the synthesis.
guard: stand_canvas_click
basis: fct-t6akh6q6
because: The chunk changed the engine so a widget element's click and double-click handlers fire, a decision every Dioxus native app behind custom-widget now relies on.
@row_add n=3 row=decision/circle-drawer-circles-painted area=surface fact=F3 detector=none
name: Circle Drawer circles painted on one canvas
rule: Circle Drawer's drawing area is one canvas element of role group named Drawing area, keyed and lint clean at boot; its circles are carried by one data-circles attribute, x,y,d per circle joined by semicolons with malformed text read as no circle, and are painted, never DOM nodes.
state: provisional
basis: fct-t6akh6q6, fct-ryoj2i3n, fct-yshqj5md
because: The chunk replaced the circle DOM nodes with one painted canvas element, so circles now cross as shot pixels alone, a decision applied ahead of ratification.
@row_add n=4 row=contract/custom-widget-element/paint-box area=surface fact=F4 detector=none
boundary: custom-widget element
item: paint box
promise: A widget paints in its element's content box while the element's client rect is its border box, so an app placing widget content from a click point subtracts the element's border and padding.
basis: fct-c4pe4eil, fct-hzprrvfq
because: The chunk measured the widget paint box against the click point and Circle Drawer now subtracts its border, a promise every app placing widget content relies on.
@row_add n=5 row=absence/canvas-keyboard-path area=surface fact=F5 detector=none
capability: A keyboard path into a canvas widget
basis: fct-k4xssamx
because: The chunk's canvas takes circles only by pointer and no keyboard path into a canvas widget is built.
@row_add n=6 row=decision/circle-drawer-canvas-colours-in-code area=color-palette fact=F6 detector=D-design-tokens
name: Circle Drawer canvas colours as widget constants
rule: Circle Drawer's canvas colours, the stand accent stroke, the dark label and the accent fill at alpha 38/255, are constants in the sample's widget code, as the removed CSS rules held them.
why: The canvas paints from widget code, not the cascade; an exception to colours living in stylesheets.
basis: fct-k4xssamx, fct-t6akh6q6
because: The chunk moved the circle colours out of CSS into the widget, a hardcoded exception the palette must record.
