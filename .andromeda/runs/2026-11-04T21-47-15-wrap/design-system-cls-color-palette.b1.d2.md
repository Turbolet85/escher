@answer n=1 entity=color-palette#1.1 v=no
why: the Color type and its sRGB conversion sit in packages/blitz-dom/src/util.rs, which the session did not change and the untouched colour claim covers
@answer n=2 entity=color-palette#1.2 v=no
why: the colour-scheme enum and its prefers-color-scheme feed sit in blitz-traits, blitz-dom stylo_device.rs and device_coalescing.rs, none of which the session changed
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on colour-scheme change lives in packages/blitz-dom/src/document.rs, which the session did not change
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe colour-scheme copy lives in packages/blitz-dom/src/resolve.rs, which the session did not change
@answer n=5 entity=color-palette#1.5 v=no
why: the bgcolor mapping lives in packages/blitz-dom/src/stylo.rs, which the session did not change
@answer n=6 entity=color-palette#1.6 v=no
why: the optional base_color lives in packages/dioxus-native config and renderer files the session did not change, and Dioxus stays at its 0.7.3 requirement
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background rule lives in packages/blitz-paint/src/render.rs, which the session did not change
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white fill lives in wpt/runner/src/test_runners/ref_test.rs, which the session did not change
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus colour lives in apps/browser/assets/browser.css, and the untouched colour and browser-icon claims hold with no apps/browser file changed
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour lives in apps/browser/assets/about-history.css and about-newtab.css, which the session did not change
@answer n=11 entity=color-palette#2.3 v=no
why: the seven_guis stand accent and its cited sources are unchanged, and the stand's count moved without any colour value moving
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default link, focus and button colours live in packages/blitz-dom/assets/default.css, which the session did not change
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen-row pair lives in packages/blitz-dom/assets/default.css, which the session did not change
@answer n=14 entity=color-palette#2.6 v=no
why: the select list highlighted-row constants live in packages/blitz-dom/src/node/select_list.rs, which the session did not change, and the untouched SelectListView claim holds
@answer n=15 entity=color-palette#2.7 v=no
why: the select list disabled-text constant lives in packages/blitz-dom/src/node/select_list.rs, which the session did not change, and the untouched SelectListView claim holds
@answer n=16 entity=color-palette#2.8 v=no
why: the select list border constant lives in packages/blitz-dom/src/node/select_list.rs, which the session did not change, and the untouched SelectListView claim holds
@answer n=17 entity=color-palette#2.9 v=no
why: the select list plain-row pair is measured by select_popup_painted.rs, which the session did not change, and no SelectListView constant moved
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight lives in packages/blitz-paint/src/lib.rs, which the session did not change
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent button colours live in examples/counter and examples/transparent sources, which the session did not change
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface colours live in apps/browser/assets/browser.css, which the session did not change, and the untouched surface-scale claim holds
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar colours live in apps/browser/assets/browser.css, which the session did not change
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page background lives in apps/browser/assets/about-*.css, which the session did not change
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background lives in apps/readme/assets/blitz-markdown-overrides.css, which the session did not change, and a pulldown-cmark parser bump sets no page colour
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background lives in examples/todomvc/src/todomvc.css, which the session did not change
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette lives in examples/wasm_hello/src/lib.rs, which the session did not change
