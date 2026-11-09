@answer n=1 entity=color-palette#1.1 v=no
why: the Color type is defined in packages/blitz-dom/src/util.rs, a file the session does not reach and the session adds no colour
@answer n=2 entity=color-palette#1.2 v=no
why: the colour-scheme enum lives in blitz-traits and blitz-dom files the session does not reach, and it adds no colour
@answer n=3 entity=color-palette#1.3 v=no
why: the recascade on scheme change is in packages/blitz-dom/src/document.rs, which the session does not reach
@answer n=4 entity=color-palette#1.4 v=no
why: the iframe scheme copy is in packages/blitz-dom/src/resolve.rs, which the session does not reach
@answer n=5 entity=color-palette#1.5 v=no
why: the bgcolor mapping is in packages/blitz-dom/src/stylo.rs, which the session does not reach
@answer n=6 entity=color-palette#1.6 v=no
why: the frame clear colour is in packages/dioxus-native, a different crate from the dioxus-native-dom files the session reaches, and the session adds no colour
@answer n=7 entity=color-palette#1.7 v=no
why: the canvas background fallback is in packages/blitz-paint/src/render.rs, which the session does not reach
@answer n=8 entity=color-palette#1.8 v=no
why: the WPT white fill is in wpt/runner/src/test_runners/ref_test.rs, which the session does not reach
@answer n=9 entity=color-palette#2.1 v=no
why: the browser chrome focus values are in apps/browser/assets/browser.css, which the session does not reach and no colour is added
@answer n=10 entity=color-palette#2.2 v=no
why: the about:history link colour is in apps/browser/assets/about-*.css, which the session does not reach
@answer n=11 entity=color-palette#2.3 v=no
why: the seven_guis stand accent is cited at examples/seven_guis/src/tasks/cells.rs, which the session does not reach, and its session_host.rs change carries no colour
@answer n=12 entity=color-palette#2.4 v=no
why: the blitz-dom default link, input and button colours are in packages/blitz-dom/assets/default.css, which the session does not reach
@answer n=13 entity=color-palette#2.5 v=no
why: the multiple-select chosen row is in packages/blitz-dom/assets/default.css, which the session does not reach and no colour is added
@answer n=14 entity=color-palette#2.6 v=no
why: the select list highlight constants are in packages/blitz-dom/src/node/select_list.rs, which the session does not reach
@answer n=15 entity=color-palette#2.7 v=no
why: the select list disabled-row constant is in packages/blitz-dom/src/node/select_list.rs, which the session does not reach
@answer n=16 entity=color-palette#2.8 v=no
why: the select list border constant is in packages/blitz-dom/src/node/select_list.rs, which the session does not reach
@answer n=17 entity=color-palette#2.9 v=no
why: the select list plain-row pair is measured in tests/blitz-tests/tests/select_popup_painted.rs, a test file the session does not reach
@answer n=18 entity=color-palette#2.10 v=no
why: the default text-selection highlight is in packages/blitz-paint/src/lib.rs, which the session does not reach
@answer n=19 entity=color-palette#2.11 v=no
why: the counter and transparent button colours are in examples/counter and examples/transparent app.rs files, which the session does not reach
@answer n=20 entity=color-palette#3.1 v=no
why: the browser tabstrip surface values are in apps/browser/assets/browser.css, which the session does not reach and no colour is added
@answer n=21 entity=color-palette#3.2 v=no
why: the browser tooltip, FPS overlay and status bar values are in apps/browser/assets/browser.css, which the session does not reach
@answer n=22 entity=color-palette#3.3 v=no
why: the browser about-page background is in apps/browser/assets/about-*.css, which the session does not reach
@answer n=23 entity=color-palette#3.4 v=no
why: the rdme page background is in apps/readme/assets/blitz-markdown-overrides.css, which the session does not reach
@answer n=24 entity=color-palette#3.5 v=no
why: the todomvc page background is in examples/todomvc/src/todomvc.css, which the session does not reach
@answer n=25 entity=color-palette#3.6 v=no
why: the wasm_hello dark palette is in examples/wasm_hello/src/lib.rs, which the session does not reach
