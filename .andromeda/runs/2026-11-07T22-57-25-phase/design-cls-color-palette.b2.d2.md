@answer n=1 entity=color-palette#3.7 v=no
why: the #f4e8d2 main background sits on the custom-widget, static-HTML and wgpu_texture examples, which the chunk does not edit and which paint nothing it reads
@answer n=2 entity=color-palette#4.1 v=no
why: the browser tooltip and status bar text colours are on apps/browser CSS, outside every file the chunk reaches
@answer n=3 entity=color-palette#4.2 v=no
why: the browser about-page muted text colours are on apps/browser CSS, outside every file the chunk reaches
@answer n=4 entity=color-palette#4.3 v=no
why: the browser error page paragraph colour is on apps/browser/assets/error.html, outside every file the chunk reaches
@answer n=5 entity=color-palette#4.4 v=no
why: the todomvc body and heading colours are on examples/todomvc CSS, outside every file the chunk reaches
@answer n=6 entity=color-palette#4.5 v=no
why: the wasm_hello heading and code colours are on examples/wasm_hello/src/lib.rs, outside every file the chunk reaches
@answer n=7 entity=color-palette#5.1 v=no
why: the disabled checkbox and radio accent is painted by blitz-paint form_controls, which the chunk does not touch and whose paint the chunk does not change
@answer n=8 entity=color-palette#6.1 v=no
why: the WPT runner PASS terminal colour is on wpt/runner/src/main.rs, a tool the chunk does not reach; the driver's JSON-only output rule is not a colour rule over this runner
@answer n=9 entity=color-palette#6.2 v=no
why: the WPT runner FAIL-with-passing-subtests terminal colour is on wpt/runner/src/main.rs, which the chunk does not reach
@answer n=10 entity=color-palette#6.3 v=no
why: the WPT runner FAIL terminal colour is on wpt/runner/src/main.rs, which the chunk does not reach
@answer n=11 entity=color-palette#6.4 v=no
why: the WPT runner TIMEOUT terminal colour is on wpt/runner/src/main.rs, which the chunk does not reach
@answer n=12 entity=color-palette#6.5 v=no
why: the WPT runner SKIP terminal colour is on wpt/runner/src/main.rs, which the chunk does not reach
@answer n=13 entity=color-palette#6.6 v=no
why: the WPT runner CRASH terminal colour is on wpt/runner/src/main.rs, which the chunk does not reach
@answer n=14 entity=color-palette#7.1 v=no
why: the browser urlbar border colour is on apps/browser CSS, outside every file the chunk reaches
@answer n=15 entity=color-palette#7.2 v=no
why: the browser chrome focus border colour is on apps/browser CSS, outside every file the chunk reaches
@answer n=16 entity=color-palette#7.3 v=no
why: the blitz-dom default input focus outline is painted by blitz-dom default.css, which the chunk does not touch and the graph read does no hit-testing over
@answer n=17 entity=color-palette#8.1 v=no
why: the default scrollbar thumb palette is painted by blitz-paint render.rs, which the chunk does not touch; the chunk paints nothing
@answer n=18 entity=color-palette#8.2 v=no
why: the scrollbar-color resolution and thumb blend are in blitz-dom scrollbar.rs and blitz-paint render.rs, which the chunk does not touch
@answer n=19 entity=color-palette#8.3 v=no
why: the unchecked radio ring and checkbox tick colours are painted by blitz-paint form_controls, which the chunk does not touch
@answer n=20 entity=color-palette#8.4 v=no
why: the debug overlay content, padding, border and margin colours are on blitz-paint debug_overlay.rs, which the chunk does not touch
@answer n=21 entity=color-palette#8.5 v=no
why: the devtools layout stroke colours are on blitz-paint render.rs, which the chunk does not touch
@answer n=22 entity=color-palette#8.6 v=no
why: the SVG currentColor replacement is in blitz-dom serialize.rs, which the chunk does not touch; the snapshot carrier serializes no SVG colour
@answer n=23 entity=color-palette#9.1 v=no
why: the rdme GitHub markdown colour tokens are on apps/readme CSS, outside every file the chunk reaches
@answer n=24 entity=color-palette#10.1 v=no
why: the absence of colour tokens or custom properties over the engine and integration crates is a fact the chunk keeps, not a rule its code acts on, since the chunk paints nothing and adds no colour
@answer n=25 entity=color-palette#11.1 v=no
why: the 25-colour clip-path palette is on examples/assets/clip-path.html, outside every file the chunk reaches
