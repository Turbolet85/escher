@account n=1 entity=color-palette#2.7 v=not_in_scope
why: the disabled row's paint colour; the read detects disabled through action_point, never through paint
@account n=2 entity=typography#1.6 v=not_in_scope
why: form controls' default font families; the chunk sets no type and measures no text value
@account n=3 entity=typography#1.12 v=not_in_scope
why: the text editor's base size; the chunk neither edits text inputs nor measures their text
@account n=4 entity=typography#1.13 v=not_in_scope
why: the engine's generic base sizes; no row the chunk re-measures reads a font size
@account n=5 entity=typography#2.3 v=not_in_scope
why: a WASM font context; the chunk's proof runs on the stand, not the web-spa canvas
@account n=6 entity=typography#2.4 v=not_in_scope
why: dioxus-native's font feature list; the chunk adds or changes no feature
@account n=7 entity=typography#2.6 v=carried
why: the stand proof of off-screen and covered causes needs the stand's font context to measure text, which the fonts-unavailable check witnesses
@account n=8 entity=typography#2.8 v=not_in_scope standing=S2
why: a test depending on text layout runs with blitz-dom's system-fonts feature
@account n=9 entity=typography#3.1 v=not_in_scope
why: generic family mapping to Parley; the chunk sets no type
@account n=10 entity=typography#3.4 v=not_in_scope
why: line-height mapping; the chunk measures no line height
@account n=11 entity=typography#3.5 v=not_in_scope
why: a select row's height is the engine's; the read only reads a row's hit and visible region
@account n=12 entity=spacing#2.7 v=not_in_scope
why: a multiple select's size hint; the chunk adds no layout or styling
@account n=13 entity=spacing#3.4 v=carried
why: rows past MAX_ROWS lie outside the open list's scrollport, the off-screen cause the read reports for owner-served rows
@account n=14 entity=spacing#3.5 v=not_in_scope
why: the open list's placement is the engine's; the read only reads the hit answer it yields
@account n=15 entity=depth-strategy#3.1 v=carried
why: the hit walk decides the covered cause the read reports through action_point and why an owner-served row is not clipped
@account n=16 entity=depth-strategy#3.2 v=carried
why: the open list answers the hit first, so the driver's covered reads it, the covered case the read must report
@account n=17 entity=depth-strategy#3.3 v=carried
why: a non-invertible transform answers no hit, a covered outcome the read's shared refusal path returns
@account n=18 entity=depth-strategy#3.4 v=carried
why: visible_region decides the off-screen cause, and for an open list's row answers that list's scrollport
@account n=19 entity=depth-strategy#4.9 v=not_in_scope
why: how the open list paints; the read reports hit and region causes, not paint
@account n=20 entity=border-radius#2.2 v=not_in_scope
why: clip-path radius handling in paint; the chunk adds no styling and reads no clip-path
