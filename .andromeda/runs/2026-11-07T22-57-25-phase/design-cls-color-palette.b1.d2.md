@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and Stylo-to-sRGB conversion is paint plumbing the chunk does not touch; the graph read paints nothing
@answer n=2 entity=color-palette#1.2 v=no
why: the system colour scheme enum feeding prefers-color-scheme is not read or changed by the graph read
@answer n=3 entity=color-palette#1.3 v=no
why: a colour-scheme change recascade is engine behaviour the chunk does not move or trigger
@answer n=4 entity=color-palette#1.4 v=no
why: copying the parent colour scheme into iframe sub-documents is engine behaviour the chunk does not touch
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor-to-background-color mapping is a paint rule the chunk does not change
@answer n=6 entity=color-palette#1.6 v=no
why: the optional frame clear base_color is dioxus-native paint configuration the chunk does not change
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background rule is paint, and the web-spa canvas renderer is out of this chunk (task item 7)
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill belongs to the WPT runner, which the chunk does not touch
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour is a browser app value, and the chunk touches no browser app surface
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is a browser app value, and the chunk touches no browser app surface
@answer n=11 entity=color-palette#2.3 v=no
why: the stand's accent and border colours are paint held in layout-templates, and the chunk adds no colour or paint; the signals moving onto the registry does not change where these values are recorded
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default link, input focus and button colours are default stylesheet paint the chunk does not touch; the reacts-to-click field is read from handlers, not from anchor styling
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair is select painting the chunk does not touch and no measurement the chunk re-pins
@answer n=14 entity=color-palette#2.6 v=no
why: the open select list highlighted-row constants are engine select-list painting the chunk does not touch
@answer n=15 entity=color-palette#2.7 v=no
why: the open select list disabled-row text constant is engine select-list painting the chunk does not touch
@answer n=16 entity=color-palette#2.8 v=no
why: the open select list border constant is engine select-list painting the chunk does not touch
@answer n=17 entity=color-palette#2.9 v=no
why: the open select list plain-row pair is measured select painting, and the chunk performs no painting or measurement
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight is engine paint, and the graph read paints and measures no text
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent example button colours are example app values the chunk does not touch
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface scale is a browser app value, and the chunk touches no browser app surface
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar colours are browser app values, and the chunk touches no browser app surface
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page backgrounds are browser app values, and the chunk touches no browser app surface
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background is a readme app value the chunk does not touch
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is an example app value the chunk does not touch
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette is an example app value the chunk does not touch
