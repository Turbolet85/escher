@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and sRGB conversion sit in blitz-dom util.rs, which the session did not reach
@answer n=2 entity=color-palette#1.2 v=no
why: the colour-scheme enum lives in blitz-traits and blitz-dom, neither reached by the session
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on scheme change is in blitz-dom document.rs, which the session did not reach
@answer n=4 entity=color-palette#1.4 v=no
why: iframe scheme copying lives in blitz-dom resolve.rs, which the session did not reach
@answer n=5 entity=color-palette#1.5 v=no
why: the bgcolor mapping lives in blitz-dom stylo.rs, which the session did not reach
@answer n=6 entity=color-palette#1.6 v=no
why: the base_color clear colour lives in dioxus-native config and renderer, not in the dioxus-native-dom files the session changed
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background rule is in blitz-paint render.rs, which the session did not reach
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white fill lives in the wpt runner, which the session did not reach
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour lives in browser.css, which the session did not reach
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour lives in the about css files, which the session did not reach
@answer n=11 entity=color-palette#2.3 v=yes mode=stale
why: the session's use_graph_signal registrations changed cells.rs, so the cited cells.rs:600-603 focus-outline lines may have moved, though no colour value moved
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default sheet rules for link, focus and button colours are in default.css, which the session did not reach
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair lives in blitz-dom default.css, which the session did not reach
@answer n=14 entity=color-palette#2.6 v=no
why: the SelectListView highlight constants live in blitz-dom select_list.rs, which the session did not reach
@answer n=15 entity=color-palette#2.7 v=no
why: the SelectListView disabled-text constant lives in blitz-dom select_list.rs, which the session did not reach
@answer n=16 entity=color-palette#2.8 v=no
why: the SelectListView border constant lives in blitz-dom select_list.rs, which the session did not reach
@answer n=17 entity=color-palette#2.9 v=no
why: the plain select-list pair is measured by the select_popup_painted test, which the session did not reach
@answer n=18 entity=color-palette#2.10 v=no
why: the engine default selection highlight lives in blitz-paint lib.rs, which the session did not reach
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent button colours live in their example app.rs files, which the session did not reach
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surfaces live in browser.css, which the session did not reach
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, overlay and status bar values live in browser.css, which the session did not reach
@answer n=22 entity=color-palette#3.3 v=no
why: the about-page backgrounds live in the about css files, which the session did not reach
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background lives in the readme overrides css, which the session did not reach
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background lives in todomvc.css, which the session did not reach
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello palette lives in examples/wasm_hello lib.rs, which the session did not reach
