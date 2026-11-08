@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and Stylo sRGB conversion live in blitz-dom util.rs, a file the session does not reach
@answer n=2 entity=color-palette#1.2 v=no
why: the colour scheme enum and its prefers-color-scheme wiring sit in blitz-traits and blitz-dom files the session does not reach
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on scheme change is in blitz-dom document.rs, which the session does not reach
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe scheme copy is in blitz-dom resolve.rs, which the session does not reach
@answer n=5 entity=color-palette#1.5 v=no
why: the bgcolor attribute mapping is in blitz-dom stylo.rs, which the session does not reach
@answer n=6 entity=color-palette#1.6 v=no
why: the frame base_color lives in dioxus-native config.rs and dioxus_renderer.rs, which the session does not reach, and the session's Dioxus event-path change sets no clear colour
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background rule is in blitz-paint render.rs, which the session does not reach
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill is in the wpt ref_test runner, which the session does not reach
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour is in browser.css, which the session does not reach
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is in the browser about CSS files, which the session does not reach
@answer n=11 entity=color-palette#2.3 v=yes mode=stale
why: examples/seven_guis/src/tasks/cells.rs is reached whole by the use_graph_signal registrations, so the cited lines 600-603 for the stand accent and focus outline can have moved
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default stylesheet link, input focus and button colours are in default.css, which the session does not reach
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair is in default.css and the select measurement, which the session does not reach, and no colour moved
@answer n=14 entity=color-palette#2.6 v=no
why: the SelectListView highlight constants are in select_list.rs, which the session does not reach
@answer n=15 entity=color-palette#2.7 v=no
why: the SelectListView disabled-text constant is in select_list.rs, which the session does not reach
@answer n=16 entity=color-palette#2.8 v=no
why: the SelectListView border constant is in select_list.rs, which the session does not reach
@answer n=17 entity=color-palette#2.9 v=no
why: the open select list plain pair is measured in select_popup_painted.rs, which the session does not reach, and no colour moved
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight is in blitz-paint lib.rs, which the session does not reach
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent button colours are in examples/counter and examples/transparent, which the session does not reach
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip and tab surface values are in browser.css, which the session does not reach
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar values are in browser.css, which the session does not reach
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page backgrounds are in the browser about CSS files, which the session does not reach
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background is in apps/readme blitz-markdown-overrides.css, which the session does not reach
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is in examples/todomvc todomvc.css, which the session does not reach
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette is in examples/wasm_hello lib.rs, which the session does not reach
