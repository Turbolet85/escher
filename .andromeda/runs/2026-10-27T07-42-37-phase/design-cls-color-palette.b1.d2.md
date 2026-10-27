# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · yes · the Stylo-to-sRGB conversion carries the computed colours that stand_contrast re-asserts and the re-measured shots read, through a possibly moved Stylo (Needs)
color-palette#1.2 · yes · the colour-scheme enum sits in blitz-traits/src/shell.rs, which is reached whole
color-palette#1.3 · yes · the recascade on a scheme change sits in document.rs:2312-2319, and document.rs is reached whole
color-palette#1.4 · no · the iframe scheme copy sits in resolve.rs, which no reached file names and the take does not touch
color-palette#1.5 · yes · the bgcolor mapping sits in stylo.rs:1186-1193, and stylo.rs is reached whole for the :open edit
color-palette#1.6 · no · the frame clear base_color sits in dioxus-native config.rs and renderer, neither reached, and the take's window change names no clear colour
color-palette#1.7 · yes · the canvas background fallback paints the backdrop of every re-measured screenshot, which the take re-measures when Taffy, Stylo or winit moves
color-palette#1.8 · no · the WPT white fill sits in wpt/runner ref_test.rs, which the delta does not touch, and WPT_COMMIT pins only the suite revision
color-palette#2.1 · yes · the chrome focus and icon-button hover and active colours sit in apps/browser's browser.css, reached whole, and the take moves its toolbar icons (d63ed2c8c, Needs)
color-palette#2.2 · yes · the about:history link colour sits in apps/browser's about css, reached whole
color-palette#2.3 · yes · it records the stand's accent and its success-message and select colours, the accent stand_contrast re-asserts row for row (Needs)
color-palette#2.4 · yes · the user-agent link, focus-outline and button colours in blitz-dom default.css are the engine-painted defaults the Needs re-read through Stylo
color-palette#2.5 · no · the default selection highlight is painted in blitz-paint lib.rs, which no reached file names, and no re-measured shot or named check holds a selection
color-palette#2.6 · no · the counter and transparent example buttons sit in examples/counter and examples/transparent, neither reached nor a stand page
color-palette#3.1 · yes · the tabstrip, active-tab and URL bar colours sit in apps/browser's browser.css, reached whole, and the take moves the URL bar (d63ed2c8c, Needs)
color-palette#3.2 · yes · the tooltip, FPS overlay and status bar colours sit in apps/browser's browser.css, reached whole
color-palette#3.3 · yes · the about-page background sits in apps/browser's about css, reached whole
color-palette#3.4 · no · the rdme page colours sit in apps/readme, which no reached file names and the take does not touch
color-palette#3.5 · yes · the seven_guis page background is the backdrop stand_contrast reads under text outside the controls (the todomvc part is not reached)
color-palette#3.6 · no · the wasm_hello dark palette sits in examples/wasm_hello, which is neither reached nor a stand page
color-palette#3.7 · no · the custom-widget, static-HTML and wgpu_texture backgrounds sit in examples no reached file names (the custom widget example is not in the delta)
color-palette#4.1 · yes · the browser tooltip and status bar text colour sits in apps/browser's browser.css, reached whole
color-palette#4.2 · yes · the about-page muted text colours sit in apps/browser's about css, reached whole
color-palette#4.3 · yes · the error page paragraph colour sits in apps/browser/assets/error.html, reached whole
color-palette#4.4 · yes · the seven_guis text colour is the text stand_contrast re-asserts outside the controls
