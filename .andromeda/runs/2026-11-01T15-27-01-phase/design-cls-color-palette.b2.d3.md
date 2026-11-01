# Classified — design · color-palette.b2

## Answers
color-palette#3.7 · no · the custom-widget, static-HTML and wgpu_texture example backgrounds are fixture values of examples the scope does not name
color-palette#4.1 · no · browser tooltip text is a browser app value; apps/browser assets are Not needed
color-palette#4.2 · no · browser about-page muted text is a browser app value; apps/browser assets are Not needed
color-palette#4.3 · no · browser error page paragraph is a browser app value, not reached
color-palette#4.4 · no · todomvc body and heading values are fixture values of an example other than seven_guis, Not needed
color-palette#4.5 · no · wasm_hello heading and code values are an example's fixture values the scope names no change to
color-palette#5.1 · no · the disabled checkbox/radio accent is painted in blitz-paint form_controls.rs, which the chunk does not reach, and no contrast row names it
color-palette#6.1 · no · WPT runner PASS terminal colour; the WPT runner's output colours are out of scope
color-palette#6.2 · no · WPT runner FAIL-with-passing-subtests terminal colour; the WPT runner's output colours are out of scope
color-palette#6.3 · no · WPT runner FAIL terminal colour; the WPT runner's output colours are out of scope
color-palette#6.4 · no · WPT runner TIMEOUT terminal colour; the WPT runner's output colours are out of scope
color-palette#6.5 · no · WPT runner SKIP terminal colour; the WPT runner's output colours are out of scope
color-palette#6.6 · no · WPT runner CRASH terminal colour; the WPT runner's output colours are out of scope
color-palette#7.1 · no · browser urlbar border is a browser app value; apps/browser assets are Not needed
color-palette#7.2 · no · browser chrome focus colour is a browser app value; apps/browser assets are Not needed
color-palette#7.3 · yes · default.css is a conflict surface and holds this blitz-dom default input focus outline (lines 43-46, 98-101, 111); the resolution keeps it, and the engine default values are read against the resolved sheet
color-palette#8.1 · no · the default scrollbar thumb palette is painted in blitz-paint render.rs, which the chunk does not reach; no re-read check names scrollbar colours
color-palette#8.2 · no · scrollbar-color resolution lives in node/scrollbar.rs, which the chunk does not reach; the scope names no scrollbar behaviour
color-palette#8.3 · no · radio ring and checkbox tick colours are painted in form_controls.rs, which the chunk does not reach
color-palette#8.4 · no · debug overlay colours are in debug_overlay.rs, which the chunk does not reach; the scope names no change to the overlay
color-palette#8.5 · no · devtools layout stroke colours are in render.rs and devtools.rs, which the chunk does not reach; the scope names no devtools change
color-palette#8.6 · no · the currentColor replacement lives in node/serialize.rs, which the chunk does not reach; the scope names no serialization behaviour
color-palette#9.1 · no · rdme GitHub markdown tokens are apps/readme values, Not needed
color-palette#10.1 · yes · the absence of colour tokens and custom properties binds over the taken delta, and the SelectListView engine-constant colours and highlight pair it records are named in the Needs
color-palette#11.1 · no · the clip-path fixture palette is an example fixture the scope does not name
