@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and its sRGB conversion in blitz-dom bind no line of the chunk, which adds no colour and paints nothing (task not_needed 1)
@answer n=2 entity=color-palette#1.2 v=no
why: the viewport colour scheme feeding prefers-color-scheme is read by no graph field and changed by no chunk line
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on a colour-scheme change is engine behaviour the graph read neither moves nor reads, and the chunk changes no scheme
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe scheme copy concerns sub-documents the chunk does not render, and the chunk changes no scheme (task not_needed 1)
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor mapping is a styling rule the chunk neither touches nor paints through (task not_needed 1)
@answer n=6 entity=color-palette#1.6 v=no
why: the optional frame clear colour base_color is a renderer config the chunk does not change, and it paints nothing (task not_needed 1)
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background fallback is a paint rule, and the chunk adds no canvas renderer and paints nothing (task item 7 and not_needed 1)
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill lives in the wpt runner, which the chunk does not touch
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour is a browser app value, and the chunk changes no browser app surface (task not_needed 8)
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is a browser about-page value the chunk does not change (task not_needed 8)
@answer n=11 entity=color-palette#2.3 v=no
why: the stand's colours are recorded in layout-templates rather than as design-system values, and the chunk's signal move on stand.rs adds or changes no colour (task not_needed 1)
@answer n=12 entity=color-palette#2.4 v=no
why: the default link and focus colours are style rules keyed on href, and the new boolean reads click handlers, not styling; the chunk paints nothing (task not_needed 1)
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple select chosen-row pair is engine painting the chunk does not touch (task not_needed 2)
@answer n=14 entity=color-palette#2.6 v=no
why: the open select list highlighted-row constants are engine-painted and the chunk touches no select list painting (task not_needed 2)
@answer n=15 entity=color-palette#2.7 v=no
why: the open select list disabled-row constant is engine-painted and the chunk touches no select list painting (task not_needed 2)
@answer n=16 entity=color-palette#2.8 v=no
why: the open select list border constant is engine-painted and the chunk touches no select list painting (task not_needed 2)
@answer n=17 entity=color-palette#2.9 v=no
why: the open select list plain-row pair is measured painting the chunk does not touch (task not_needed 2)
@answer n=18 entity=color-palette#2.10 v=no
why: the engine default text-selection highlight is paint behaviour the chunk does not touch (task not_needed 1)
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent example button colours sit in examples the chunk does not touch
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface values are browser app paint the chunk does not change (task not_needed 8)
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar values are browser app paint the chunk does not change (task not_needed 8)
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page backgrounds are browser app paint the chunk does not change (task not_needed 8)
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background sits in the readme app, which the chunk does not touch
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background sits in the todomvc example, which the chunk does not touch
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette sits in the wasm_hello example, which the chunk does not touch
