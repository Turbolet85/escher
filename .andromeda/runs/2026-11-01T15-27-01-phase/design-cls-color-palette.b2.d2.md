# Classified — design · color-palette.b2

## Answers
color-palette#3.7 · no · the custom-widget, static-HTML and wgpu_texture example backgrounds; no reached file or named unit holds them, and other examples' fixture values are out of scope
color-palette#4.1 · no · browser tooltip text in apps/browser/assets/browser.css; the browser app is not named by the scope
color-palette#4.2 · no · browser about-page muted text; the browser app is not named by the scope
color-palette#4.3 · no · browser error page paragraph; the browser app is not named by the scope
color-palette#4.4 · no · todomvc body and heading values; fixture values of todomvc are out of scope
color-palette#4.5 · no · wasm_hello heading and code values; the scope names no wasm surface or wasm_hello change
color-palette#5.1 · no · disabled checkbox and radio accent in form_controls.rs; that file is not reached and no named check reads this value
color-palette#6.1 · no · WPT runner PASS terminal colour; the WPT runner's command line output and owo-colors values are out of scope
color-palette#6.2 · no · WPT runner FAIL-with-passes terminal colour; out of scope as the WPT runner's output
color-palette#6.3 · no · WPT runner FAIL terminal colour; out of scope as the WPT runner's output
color-palette#6.4 · no · WPT runner TIMEOUT terminal colour; out of scope as the WPT runner's output
color-palette#6.5 · no · WPT runner SKIP terminal colour; out of scope as the WPT runner's output
color-palette#6.6 · no · WPT runner CRASH terminal colour; out of scope as the WPT runner's output
color-palette#7.1 · no · browser urlbar border colour; the browser app is not named by the scope
color-palette#7.2 · no · browser chrome focus colour; the browser app is not named by the scope
color-palette#7.3 · yes · the blitz-dom default input outline values sit in packages/blitz-dom/assets/default.css, a predicted conflict surface, so a resolution can move the rule
color-palette#8.1 · no · default scrollbar thumb values in blitz-paint render.rs; the file is not reached and no named check reads these values
color-palette#8.2 · no · scrollbar-color resolution in scrollbar.rs and render.rs; neither file is reached or named by the scope
color-palette#8.3 · no · unchecked radio ring and checkbox tick colours in form_controls.rs; the file is not reached
color-palette#8.4 · no · debug overlay box colours in debug_overlay.rs; the overlay is not named by the scope
color-palette#8.5 · no · devtools layout stroke colours in render.rs; the devtools overlay is not named by the scope
color-palette#8.6 · no · currentColor replacement in serialize.rs; the file is not reached and the SVG damage change is layout, not serialization
color-palette#9.1 · no · rdme GitHub markdown tokens in apps/readme; the readme app is not named by the scope
color-palette#10.1 · yes · records that the open select list's colours are fixed constants on SelectListView with no colour token or custom property; the scope names SelectListView's engine-constant colours, and a take touching default.css could add custom properties
color-palette#11.1 · no · clip-path fixture block palette in examples/assets/clip-path.html; that example is not named by the scope
