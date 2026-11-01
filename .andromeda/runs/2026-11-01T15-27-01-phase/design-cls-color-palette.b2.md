# Classified — design · color-palette.b2

## Answers
color-palette#3.7 · no · main background of the custom-widget, static-HTML and wgpu_texture examples; none is reached, and examples other than seven_guis are outside the scope
color-palette#4.1 · no · browser tooltip and status text colour; apps/browser chrome is not restyled by the scope
color-palette#4.2 · no · browser about-page muted text colours; apps/browser assets are not named
color-palette#4.3 · no · browser error page paragraph colour; apps/browser assets are not named
color-palette#4.4 · no · todomvc body and heading colours; examples other than seven_guis are outside the scope
color-palette#4.5 · no · wasm_hello heading and code colours; the scope names no wasm surface
color-palette#5.1 · no · disabled checkbox and radio accent in form_controls.rs, a file the chunk does not reach
color-palette#6.1 · no · WPT runner PASS terminal colour; upstream WPT runner output is out of scope
color-palette#6.2 · no · WPT runner FAIL-with-passing-subtests terminal colour; upstream WPT runner output is out of scope
color-palette#6.3 · no · WPT runner FAIL terminal colour; upstream WPT runner output is out of scope
color-palette#6.4 · no · WPT runner TIMEOUT terminal colour; upstream WPT runner output is out of scope
color-palette#6.5 · no · WPT runner SKIP terminal colour; upstream WPT runner output is out of scope
color-palette#6.6 · no · WPT runner CRASH terminal colour; upstream WPT runner output is out of scope
color-palette#7.1 · no · browser urlbar border colour; apps/browser chrome is not restyled by the scope
color-palette#7.2 · no · browser chrome focus colour; apps/browser chrome is not restyled by the scope
color-palette#7.3 · yes · blitz-dom default input focus outline #4D90FE sits in default.css, a conflict surface reached whole, which the take edits by its picture UA rule and whose recorded engine default colour is read against the resolved sheet
color-palette#8.1 · no · default scrollbar thumb and track palette in blitz-paint render.rs, not reached; scrolling.rs carries no paint colour
color-palette#8.2 · no · scrollbar-color resolution in node/scrollbar.rs and render.rs, not reached; scrolling.rs does not resolve scrollbar colours
color-palette#8.3 · no · unchecked radio ring and checkbox tick colours in form_controls.rs, not reached
color-palette#8.4 · no · debug overlay content, padding, border and margin colours in debug_overlay.rs, not reached
color-palette#8.5 · no · devtools layout stroke colours in render.rs, not reached
color-palette#8.6 · no · currentColor replacement in SVG serialization in serialize.rs, not reached; the SVG attribute damage in mutator.rs is a different rule
color-palette#9.1 · no · rdme GitHub markdown colour tokens in apps/readme, not named by the scope
color-palette#10.1 · yes · the open select list's engine-constant colours on SelectListView are named by the scope and read row by row, and the no-tokens absence rules over engine conflict-surface files a take may move
color-palette#11.1 · no · clip-path example block palette in examples/assets, outside the examples/seven_guis fixture the scope names
