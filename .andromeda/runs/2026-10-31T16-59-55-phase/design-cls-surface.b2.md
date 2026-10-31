# Classified — design · surface.b2

## Answers
surface#13.1 · no · WPT runner's result-colour tokens for terminal output; the WPT runner's terminal colours are Not needed and the chunk does not reach them.
surface#13.2 · no · WPT runner's terminal colours for test kind and summary headings; terminal output of the WPT runner is Not needed.
surface#13.3 · no · WPT runner's OSC 8 hyperlinks on test names, printed only to a terminal; the WPT runner's terminal output is Not needed.
surface#14.1 · no · paint_bench's printed stats lines; output of a binary the chunk does not touch, Not needed.
surface#14.2 · no · screenshot example's printed lines; terminal printing of the example is Not needed, and the driver's pixel read is carried by surface#15.1.
surface#15.1 · yes · the driver's screenshot answer `{"width","height","png"}`, its PNG as base64 in one uncoloured JSON line with status 0, is what the open list's pixel read through the driver reads; its no-colour and no-ESC clause is S4's, which the chunk adds no output to meet.
surface#15.2 · no · the six served tasks, the usage line and the MCP `start` enum; the chunk adds no task or verb, and the stand's flight booker is served unchanged.
surface#16.1 · yes · WASM builds register bundled DejaVu Sans for sans-serif, the family the list's row text and a closed select's label resolve to on the WASM canvas surface, which the list paints on.
surface#17.1 · no · wasm_hello's canvas size, radius, tabindex and outline are an example fixture's own page values, not engine defaults the list reads; Not needed.
