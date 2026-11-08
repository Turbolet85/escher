@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and sRGB conversion are paint-side, and the chunk paints nothing and the graph read carries no colour value
@answer n=2 entity=color-palette#1.2 v=no
why: the viewport colour scheme feeding prefers-color-scheme is not carried by the graph read, and the chunk moves no viewport state
@answer n=3 entity=color-palette#1.3 v=no
why: the cascade-time recascade on scheme change is paint-side, and the chunk adds no colour and triggers no recascade
@answer n=4 entity=color-palette#1.4 v=no
why: iframe scheme copying in resolve.rs is not reached, as the chunk has no iframe or sub-document reach
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor attribute mapping is a style mapping, and the new reacts-to-a-click field reads handlers, not styles
@answer n=6 entity=color-palette#1.6 v=no
why: the optional frame clear colour in the dioxus-native config is not reached, as the chunk changes no renderer config and adds no colour
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background fallback is a paint rule, and the graph read does no painting and no hit-testing
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill belongs to the WPT ref-test runner, which the chunk does not reach
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour is a browser app value, and the chunk touches no browser app surface
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is a browser app asset value, and the chunk touches no browser app surface
@answer n=11 entity=color-palette#2.3 v=no
why: the seven_guis stand's colours are held in layout-templates and not as design-system values, and the signal move onto the registry adds no colour and changes no stand accent
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default link, focus and button colours are default-sheet styling, which the chunk does not change, and the reacts-to-a-click reading comes from handlers, not from the a[href] rule
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple select chosen row is default-sheet select painting, and the chunk touches no select list painting
@answer n=14 entity=color-palette#2.6 v=no
why: the open select list highlighted-row constants are select list painting, and the chunk touches no select list painting
@answer n=15 entity=color-palette#2.7 v=no
why: the open select list disabled-row constant is select list painting, and the chunk touches no select list painting
@answer n=16 entity=color-palette#2.8 v=no
why: the open select list border constant is select list painting, and the chunk touches no select list painting
@answer n=17 entity=color-palette#2.9 v=no
why: the open select list plain-row pair is select list painting measured elsewhere, and the chunk touches no select list painting
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight is an engine paint default, and the chunk paints nothing
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent example button colours belong to example apps the chunk does not reach
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface scale is browser app CSS, and the chunk touches no browser app surface
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar colours are browser app CSS, and the chunk touches no browser app surface
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page backgrounds are browser app assets, and the chunk touches no browser app surface
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background is readme app CSS, which the chunk does not reach
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is todomvc example CSS, which the chunk does not reach
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette lives in an example's lib.rs that the chunk does not reach, and the web-spa canvas direction adds no colour and no canvas renderer
