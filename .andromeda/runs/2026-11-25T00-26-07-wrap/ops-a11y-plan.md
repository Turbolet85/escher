@row_set n=1 row=decision/circle-drawer-canvas-pixels-only cell=rule seen=3 detector=none
value: Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable; its circles and diameter labels reach no DOM, snapshot, accessibility tree or UI graph, leaving only as shot pixels, canvas verb draw commands and lint truncated boxes, no label's text among them.
basis: fct-nolgh3rx; fct-fmp5esit; fct-xovyxjw7
because: Circle Drawer's labels now reach the lint as truncated ink boxes with no label text (F1 A), so the row's «reach no … lint» is false and is re-stated to what the lint now reads.
@row_add n=1 row=absence/canvas-label-contrast-reading area=visual-design-verification fact=F1 detector=none
capability: A contrast or overlap reading of canvas-painted labels; the visual lint reads a canvas label only for truncation
basis: fct-qxarr44d; fct-xovyxjw7
because: The lint-like checks left out under F4 A (a canvas label's contrast against the fill under it, labels overlapping one another) leave the visual lint with no contrast or overlap reading of a canvas label, a whole capability absent.
@row_add n=2 row=contract/visual-lint-finding/canvas-label area=screen-reader-support fact=F2 detector=none
boundary: visual lint finding
item: canvas label
promise: A label a canvas widget paints reaches the lint only as numeric ink boxes under truncated, its text and glyph ids crossing no carrier, and gives no accessibility tree node beyond the canvas element's own.
basis: fct-ya64mcmm; fct-xff2nsjd; fct-fmp5esit; fct-mbmku3j6
because: The chunk makes canvas labels a new input of the lint, and what crosses from them (four numbers, no text or glyph id, no tree node) is a promise at the lint's boundary future work must keep.
