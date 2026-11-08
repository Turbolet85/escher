@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and its sRGB conversion; the coverage command and replay test paint and compare no colour
@answer n=2 entity=color-palette#1.2 v=no
why: the system colour scheme feeding prefers-color-scheme; the chunk sets no viewport scheme and reads none
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on a colour-scheme change; the chunk changes no scheme and runs no cascade of its own
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe colour-scheme copy; the UI graph, snapshot and replay read no iframe
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor attribute mapping; the chunk sets and reads no background colour
@answer n=6 entity=color-palette#1.6 v=no
why: the frame clear colour base_color; the chunk paints no frame and changes no renderer config
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background colour; the chunk paints nothing
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill; the WPT runner is outside the chunk's reach
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour; apps/browser is not a chunk file and the chunk paints no focus
@answer n=10 entity=color-palette#2.2 v=no
why: the browser about:history link colour; a browser app stylesheet the chunk does not touch
@answer n=11 entity=color-palette#2.3 v=no
why: the stand's accent and select-border colours; the coverage count and owed test compare no colour
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default link, focus and button colours; the chunk paints nothing and the default sheet is untouched
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple select's chosen-row colours; the chunk measures no select row or contrast
@answer n=14 entity=color-palette#2.6 v=no
why: the open select list's highlighted-row colours; the select list's engine-painted colours are not needed by the chunk
@answer n=15 entity=color-palette#2.7 v=no
why: the open select list's disabled-row text colour; the coverage count and owed test paint no select list
@answer n=16 entity=color-palette#2.8 v=no
why: the open select list's border colour; the chunk draws no select list
@answer n=17 entity=color-palette#2.9 v=no
why: the open select list's plain-row pair; the chunk re-measures no select list colour
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight; the chunk paints no text selection
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent example button colours; neither example is a chunk file or a behaviour the chunk checks
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface colours; apps/browser is outside the chunk's reach
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar colours; the chunk builds no overlay or chrome
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page background colour; apps/browser is outside the chunk's reach
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background per scheme; apps/readme is outside the chunk's reach
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background colour; examples/todomvc is outside the chunk's reach
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette; the web-spa WASM host is not built by the chunk
