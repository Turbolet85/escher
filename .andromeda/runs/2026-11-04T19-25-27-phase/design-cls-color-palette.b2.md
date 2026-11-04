@answer n=1 entity=color-palette#3.7 v=no
why: the main background of the custom-widget, static-HTML and wgpu_texture examples; the probe paints and styles nothing and runs on seven_guis
@answer n=2 entity=color-palette#4.1 v=no
why: browser tooltip and status-bar text in apps/browser; the chunk does not reach the browser app
@answer n=3 entity=color-palette#4.2 v=no
why: browser about-page muted text in apps/browser; the chunk does not reach those pages
@answer n=4 entity=color-palette#4.3 v=no
why: browser error page paragraph colour in apps/browser; the chunk does not reach it
@answer n=5 entity=color-palette#4.4 v=no
why: todomvc body and heading colours; the probe runs on seven_guis, not todomvc, and styles nothing
@answer n=6 entity=color-palette#4.5 v=no
why: wasm_hello heading and code colours; the web-spa surface is not_needed and the probe reads no WASM page
@answer n=7 entity=color-palette#5.1 v=no
why: disabled checkbox and radio accent paint in blitz-paint form controls; the probe paints nothing and seven_guis Counter or CRUD has no checkbox or radio
@answer n=8 entity=color-palette#6.1 v=no
why: WPT runner terminal colour for PASS; CLI terminal output colours are not_needed and the probe adds no terminal output
@answer n=9 entity=color-palette#6.2 v=no
why: WPT runner terminal colour for FAIL with some passing subtests; CLI terminal output colours are not_needed and the probe adds no terminal output
@answer n=10 entity=color-palette#6.3 v=no
why: WPT runner terminal colour for FAIL; CLI terminal output colours are not_needed and the probe adds no terminal output
@answer n=11 entity=color-palette#6.4 v=no
why: WPT runner terminal colour for TIMEOUT; CLI terminal output colours are not_needed and the probe adds no terminal output
@answer n=12 entity=color-palette#6.5 v=no
why: WPT runner terminal colour for SKIP; CLI terminal output colours are not_needed and the probe adds no terminal output
@answer n=13 entity=color-palette#6.6 v=no
why: WPT runner terminal colour for CRASH; CLI terminal output colours are not_needed and the probe adds no terminal output
@answer n=14 entity=color-palette#7.1 v=no
why: browser urlbar border colour in apps/browser; the chunk does not reach the browser chrome
@answer n=15 entity=color-palette#7.2 v=no
why: browser chrome focus colour in apps/browser; the chunk does not reach the browser chrome
@answer n=16 entity=color-palette#7.3 v=no
why: blitz-dom default input focus outline paint; the chunk names no focus or outline behaviour and the probe paints nothing
@answer n=17 entity=color-palette#8.1 v=no
why: default scrollbar thumb and stroke paint values in blitz-paint; the probe does no scrolling and paints nothing
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution and author thumb blending; the probe does no scrolling and the scope names no scrollbar behaviour
@answer n=19 entity=color-palette#8.3 v=no
why: unchecked radio ring and checkbox tick paint in blitz-paint form controls; the stand has no radio or checkbox and the probe paints nothing
@answer n=20 entity=color-palette#8.4 v=no
why: debug overlay content, padding, border and margin colours in blitz-paint; the devtools overlay pattern is not_needed and the probe paints nothing
@answer n=21 entity=color-palette#8.5 v=no
why: devtools layout stroke colours for block, flex and grid; the devtools overlay pattern is not_needed and the probe paints nothing
@answer n=22 entity=color-palette#8.6 v=no
why: currentColor replacement in SVG serialization; the chunk reads no SVG and the probe paints nothing
@answer n=23 entity=color-palette#9.1 v=no
why: rdme GitHub markdown colour tokens in apps/readme; the chunk does not reach the readme app
@answer n=24 entity=color-palette#10.1 v=no
why: records the absence of colour tokens or custom properties and the fixed SelectListView colour constants; the task names those constants as not_needed and the chunk reads no token
@answer n=25 entity=color-palette#11.1 v=no
why: the clip-path fixture's 25-colour block palette in examples/assets; the chunk does not reach that fixture
