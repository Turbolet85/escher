@row_set n=1 row=decision/circle-drawer-canvas-pixels-only cell=rule seen=2 fact=F1 detector=none
value: Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable, and its circles and diameter labels reach no DOM, snapshot, accessibility tree, UI graph or lint, leaving only as shot pixels and as the canvas verb's draw commands, no label's text among them.
basis: fct-orfcp2mx, fct-hbtyqmii, fct-gekogvs7, fct-f4jjdetj
because: Circle Drawer's canvas is now read in process and on every carrier as draw commands with no text, so «only as shot pixels» no longer holds while the canvas stays one named node.
@row_set n=2 row=decision/readers-of-one-snapshot-model cell=rule seen=1 detector=none
value: The text form, the diff, the text parts, MCP tool results, the node keys' help and the screenshot read only the one snapshot model, with no second role or name mapping; the canvas read, which reads one canvas element's draw commands, is no reader of the snapshot model.
basis: fct-orfcp2mx, fct-x7cb4p36
because: The canvas verb's MCP result reads draw commands rather than the snapshot model, so the row must say the canvas read is no snapshot reader.
@row_set n=3 row=decision/snapshot-model-single-source cell=rule seen=1 detector=none
value: The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else; the canvas read, which reads one canvas element's draw commands, is no reader of the snapshot model.
basis: fct-orfcp2mx, fct-x7cb4p36
because: The canvas verb's MCP result reads draw commands rather than the snapshot model, so the row must say the canvas read is no snapshot reader.
@row_add n=1 row=contract/canvas-answer/glyph-run area=screen-reader-support fact=F2 detector=none
boundary: the canvas answer
item: glyph run
promise: A glyph run crosses as its count, size, brush and 64-bit digests; no canvas answer, MCP result or kept check holds its text, a glyph id, a glyph position, font bytes or a family name.
basis: fct-f4jjdetj, fct-ol62onmg
because: The canvas verb carries a canvas's text-painting commands outward, and the promise that no text or glyph identity crosses is a contract at that boundary.
@row_add n=2 row=decision/canvas-commands-carry-no-identity area=screen-reader-support fact=F4 detector=none
name: Canvas draw commands carry no identity
rule: Draw commands carry no identity, so a changed command reads as one line removed and one added, and Circle Drawer's circles are no nodes of the snapshot or the accessibility tree, until virtual nodes give them one.
basis: fct-yvyptbkp, fct-re2xbmyu
because: The chunk states this limit of the canvas read as standing until virtual nodes land, and future work on canvas content must respect it.
@row_none n=1 fact=F3 type=0
what: an instance of the standing contract/actions-answer/causes, which already answers every id-taking verb with the cause a check would refuse it with; the canvas verb's fonts-unavailable refusal is the architecture master's refusal-causes contract.
