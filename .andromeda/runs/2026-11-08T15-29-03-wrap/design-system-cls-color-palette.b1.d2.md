@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and Stylo sRGB conversion live in blitz-dom util.rs, which the session's driver and bridge changes do not reach, and no colour changed
@answer n=2 entity=color-palette#1.2 v=no
why: the colour-scheme enum and its viewport feed live in blitz-traits and blitz-dom stylo_device.rs, neither reached by the session
@answer n=3 entity=color-palette#1.3 v=no
why: the colour-scheme recascade in blitz-dom document.rs is not touched, and no colour or token changed
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe colour-scheme copy in blitz-dom resolve.rs is not reached by the session
@answer n=5 entity=color-palette#1.5 v=no
why: the legacy bgcolor mapping in blitz-dom stylo.rs is not reached by the session
@answer n=6 entity=color-palette#1.6 v=no
why: the optional base_color lives in the dioxus-native crate, a different crate from the dioxus-native-dom bridge change, and it is unchanged
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background rule in blitz-paint render.rs is not reached by the session
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white background fill is in the WPT runner, which the session left unchanged
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour is in the browser app stylesheet, which the session left unchanged
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is in the browser about-page stylesheets, which the session left unchanged
@answer n=11 entity=color-palette#2.3 v=no
why: the stand accent is held in layout-templates and its cells.rs file is not in the session's file list, and no colour changed
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default stylesheet link, focus and button colours are not reached, and no colour changed
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair in the default stylesheet is not reached, and no select colour changed
@answer n=14 entity=color-palette#2.6 v=no
why: the SelectListView highlight constants are unchanged, as the session left the open select list's engine colour constants alone
@answer n=15 entity=color-palette#2.7 v=no
why: the SelectListView disabled-text constant is unchanged, as the session left the open select list's engine colour constants alone
@answer n=16 entity=color-palette#2.8 v=no
why: the SelectListView border constant is unchanged, as the session left the open select list's engine colour constants alone
@answer n=17 entity=color-palette#2.9 v=no
why: the plain select-row pair and its select_popup_painted test are not in the session's file list, and the select pair is unchanged
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight in blitz-paint lib.rs is not reached by the session
@answer n=19 entity=color-palette#2.11 v=no
why: the named counter and transparent button colours are in example apps the session did not touch
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface colours are in the browser app stylesheet, which the session left unchanged
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar colours are in the browser app stylesheet, which the session left unchanged
@answer n=22 entity=color-palette#3.3 v=no
why: the about-page background is in the browser about-page stylesheets, which the session left unchanged
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background is in the readme app stylesheet, which the session did not touch
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is in an example app the session did not touch
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette is in an example app the session did not touch
