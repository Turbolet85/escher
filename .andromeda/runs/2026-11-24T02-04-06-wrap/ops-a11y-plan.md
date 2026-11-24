@row_add n=1 row=decision/widget-click-synthesis area=keyboard-navigation fact=F1 detector=D-a11y-surface
name: Engine click synthesis on a custom-widget target
rule: A click on a custom-widget element reaches the widget first, then the engine fires the UA click on main-button pointerup, a context menu on the secondary button and a double click on the second click, with no click walk and no focus clear, so an app's onclick and ondoubleclick there fire.
why: Ratified; upstream's widget arm returned before any click synthesis, so an app's click handlers on a widget element never fired.
guard: stand_canvas_click
basis: fct-t6akh6q6
because: The chunk adds the engine's click synthesis on a custom-widget target in every document behind custom-widget, a ratified decision on how a click reaches a widget element that the section holds no row for.
@row_add n=2 row=decision/circle-drawer-canvas-pixels-only area=screen-reader-support fact=F2 detector=D-a11y-surface
name: Circle Drawer's drawing area as one named canvas node
rule: Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable, and its circles and diameter labels reach no DOM, snapshot, accessibility tree, UI graph or lint, leaving only as shot pixels.
guard: stand_circle_drawer_canvas
state: provisional
basis: fct-t6akh6q6
because: The chunk replaces Circle Drawer's circle nodes with one canvas node read by role and name alone, a delegate's choice that lands provisional and that the screen-reader section must state.
@row_add n=3 row=absence/canvas-keyboard-path area=keyboard-navigation fact=F3 detector=D-a11y-surface
capability: A keyboard path into a canvas widget: Circle Drawer's circles are added, selected or resized only by pointer
basis: fct-k4xssamx
because: The chunk adds an interactive canvas with no keyboard path into it, a whole capability left for a later entry that keyboard coverage must record as absent.
@row_add n=4 row=absence/canvas-paint-contrast-reading area=visual-design-verification fact=F4 detector=D-a11y-surface
capability: A contrast or lint reading of the text and fills a widget paints on a canvas
basis: fct-k4xssamx
because: The chunk paints Circle Drawer's diameter labels and fills on a canvas, outside the stand's contrast reading and the lint, with colours fixed in the widget and not taken from tokens.
