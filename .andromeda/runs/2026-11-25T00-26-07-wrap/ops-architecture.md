@row_set n=1 row=contract/lint-verb/truncated cell=promise seen=1 detector=none
value: A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`; a canvas element reads `truncated` once per label whose ink box does not lie inside its region.
basis: fct-ya64mcmm (Defect::Truncated and Session::lint now name a label its canvas widget paints); fct-nolgh3rx
because: The lint now reads a canvas widget's labels and names each one whose ink box leaves its region `truncated`, so the contract of `truncated` must say so.
@row_set n=2 row=contract/lint-verb/finding cell=promise seen=1 detector=none
value: The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order, a canvas element's `truncated` read once per label in paint order.
basis: fct-nolgh3rx (one finding per defect read, a canvas's truncated once per label, in paint order); fct-ya64mcmm
because: A canvas element now gives one `truncated` finding per label, chained in label order, so the finding contract's count and order must follow.
@row_set n=3 row=contract/driver-verbs/lint cell=promise seen=1 detector=none
value: No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines, a canvas widget's labels read by their ink boxes; dispatches nothing.
basis: fct-ya64mcmm (schema.rs: the lint verb's help and its findings field's help name the canvas label and its ink box); fct-nolgh3rx
because: The lint verb's fixed help now names the canvas label and its ink box, so the verb's contract must state that reading.
@row_add n=1 row=decision/canvas-label-region area=established-decisions fact=F1 detector=none
name: Canvas label region and ink box
rule: A canvas label's region is the widget's content box cut by each pushed clip, layer or mask path's bounding box, mapped through its transform, and by the element's clip region; its ink box is the union of its glyphs' outline bounds, and a label reads truncated when that box is not inside the region.
state: provisional
basis: fct-nolgh3rx (Established Decisions, new row: the region, PROVISIONAL, F2 A); fct-ya64mcmm; fct-vwzynsu7 (F2 A the delegate's)
because: The chunk chose how a canvas label's region and ink box are read for the lint, a delegate's answer, so it lands as a provisional decision.
@row_add n=2 row=decision/skrifa-glyph-outline-bounds area=stack-and-technologies fact=F2 detector=D-arch-decisions
name: skrifa for glyph outline bounds in the driver
rule: escher-driver takes the workspace's skrifa to read a glyph's outline bounds for the canvas lint.
why: skrifa is already in the graph through blitz-dom and blitz-paint, so no package is added.
state: provisional
basis: fct-xisnxkeg (escher-driver takes skrifa = { workspace = true }, no package added); fct-nolgh3rx (Stack and Technologies, new row, PROVISIONAL, F3 B)
because: escher-driver takes a library for a new role, which the stack must record, on a delegate's answer and so provisional.
@row_add n=3 row=absence/canvas-lint-checks-beyond-truncation area=standard-contracts fact=F3 detector=none
capability: Lint checks in a canvas beyond label truncation: label overlap, label contrast, out-of-region commands, no-area shapes
basis: fct-qxarr44d (the four checks left out on F4 A); fct-xovyxjw7 (inputs#I5: the four checks left out under F4 A)
because: The canvas lint reads only a label's truncation, and the four other checks inside a canvas are a whole capability left out.
@row_none n=1 fact=F4 type=9
what: The deferral of a label's region as the shape or virtual node under it to Circle Drawer's circles as virtual nodes is an owed item the route holds; the region's definition stands in the F1 add.
@row_none n=2 fact=F5 type=4
what: How the canvas label reader handles a font skrifa cannot read is the code's, held by its unit test; no reason is stated.
