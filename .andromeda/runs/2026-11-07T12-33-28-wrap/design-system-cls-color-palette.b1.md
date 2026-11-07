@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and sRGB conversion in blitz-dom util.rs are not among the session's reached files and no colour value moved
@answer n=2 entity=color-palette#1.2 v=no
why: the system colour scheme enum and its prefers-color-scheme feed are untouched by the mutation buffer change
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on scheme change lives in blitz-dom document.rs, which the session did not reach, and the rule still holds
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe scheme copy in blitz-dom resolve.rs is not reached and the rule still holds
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor mapping in blitz-dom stylo.rs is not reached and the rule still holds
@answer n=6 entity=color-palette#1.6 v=no
why: the base_color frame clear colour in dioxus-native config.rs and dioxus_renderer.rs is not among the four reached files and the clear colour is unchanged
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background in blitz-paint render.rs is unchanged because paint is unchanged and only the replay order moved
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill in the ref_test runner is not reached by the session
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour in browser.css was not touched and no colour or stylesheet changed
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour in the browser CSS files was not touched and no colour changed
@answer n=11 entity=color-palette#2.3 v=no
why: the seven_guis stand accent and Cells focus outline colour were not changed, and the session kept Cells synchronous scroll_to handles
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default stylesheet link, focus outline and button colours were not changed and default.css is not reached
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple select chosen-row pair in default.css was not changed and no colour moved
@answer n=14 entity=color-palette#2.6 v=no
why: the SelectListView highlight constants in select_list.rs are unchanged and that file is not reached
@answer n=15 entity=color-palette#2.7 v=no
why: the SelectListView disabled-text constant is unchanged and that file is not reached
@answer n=16 entity=color-palette#2.8 v=no
why: the SelectListView border constant is unchanged and that file is not reached
@answer n=17 entity=color-palette#2.9 v=no
why: the open select plain row pair measured by select_popup_painted is unchanged and no colour moved
@answer n=18 entity=color-palette#2.10 v=no
why: the engine default text-selection highlight in blitz-paint lib.rs is unchanged and that file is not reached
@answer n=19 entity=color-palette#2.11 v=no
why: the named button colours in the counter and transparent examples were not changed
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip and tab surface values in browser.css were not changed
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar values in browser.css were not changed
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page background values were not changed
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background values in the readme overrides were not changed
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background values were not changed
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette values were not changed
