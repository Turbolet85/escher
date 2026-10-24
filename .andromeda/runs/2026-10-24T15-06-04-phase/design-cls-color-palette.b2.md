# Classified — design · color-palette.b2

## Answers
color-palette#4.5 · no · todomvc body and heading colours; examples/todomvc is not a reached file
color-palette#4.6 · no · wasm_hello heading and code colours; examples/wasm_hello is not a reached file
color-palette#5.1 · no · invalid colours in flight_booker.rs, not reached; the sweep reads invalid only as the aria-invalid snapshot reading, and the stand colour rows are the later A11y re-read's
color-palette#5.2 · no · success colours in flight_booker.rs, not reached; the sweep reads state only as snapshot readings, and the stand colour rows are the later A11y re-read's
color-palette#5.3 · no · disabled checkbox and radio paint in form_controls.rs, not reached; the sweep reads disabled only as a snapshot reading and changes no engine painting
color-palette#6.1 · no · WPT runner's PASS terminal colour in wpt/runner/src/main.rs, not reached; the WPT runner's cli tokens are Not needed
color-palette#6.2 · no · WPT runner's FAIL-with-passing-subtests terminal colour, not reached; the WPT runner's cli tokens are Not needed
color-palette#6.3 · no · WPT runner's FAIL terminal colour, not reached; the WPT runner's cli tokens are Not needed
color-palette#6.4 · no · WPT runner's TIMEOUT terminal colour, not reached; the WPT runner's cli tokens are Not needed
color-palette#6.5 · no · WPT runner's SKIP terminal colour, not reached; the WPT runner's cli tokens are Not needed
color-palette#6.6 · no · WPT runner's CRASH terminal colour, not reached; the WPT runner's cli tokens are Not needed
color-palette#7.1 · no · browser urlbar border in apps/browser/assets/browser.css, not reached; no browser screen is touched
color-palette#7.2 · no · browser chrome focus colour in apps/browser/assets/browser.css, not reached; no browser screen is touched
color-palette#7.3 · no · blitz-dom default input outline in default.css, not reached; no named witness reads focus colour
color-palette#8.1 · no · engine default scrollbar thumb palette in render.rs and the scrollbar tests, not reached; the screenshot witnesses compare shots with the harness raster and across processes, pinning no palette value
color-palette#8.2 · no · scrollbar-color resolution in blitz-dom scrollbar.rs and render.rs, not reached; the scroll witnesses read position, not colour
color-palette#8.3 · no · unchecked radio ring and checkbox tick colours in form_controls.rs, not reached; no named witness reads checked as paint
color-palette#8.4 · no · debug overlay colours in debug_overlay.rs, not reached; the task names no debug overlay
color-palette#8.5 · no · devtools layout strokes in render.rs, not reached; the devtools overlay is out of the chunk with no windowed run
color-palette#8.6 · no · currentColor replacement in blitz-dom serialize.rs, not reached; the task names no SVG serialisation
color-palette#9.1 · no · rdme's GitHub markdown colour tokens in apps/readme/assets/github-markdown.css, not reached; no markdown viewer screen is touched
color-palette#10.1 · no · colour tokens absent from the engine and integration crates; the chunk defines none (Not needed), so this is a re-check every change owes on its reached engine files, not a need
color-palette#11.1 · no · clip-path fixture palette in examples/assets/clip-path.html, not reached; the chunk's fixtures are new minimal pages with no palette need
color-palette#11.2 · no · border-styles fixture colours in examples/assets/border-styles.html, not reached; the chunk's fixtures are new minimal pages with no palette need
color-palette#11.3 · no · reduced GitHub fixture's custom property in examples/assets/github_profile_reduced.html, not reached; the chunk defines no colour token set (Not needed)
