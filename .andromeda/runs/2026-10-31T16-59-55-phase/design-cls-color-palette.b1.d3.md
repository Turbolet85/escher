# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · yes · the list's new colours are `Color` values, sRGB-converted, which the list's engine default is read against
color-palette#1.2 · yes · the system scheme enum feeds `prefers-color-scheme`, and the list's colours are read against the scheme
color-palette#1.3 · yes · a scheme change recascades, so the open list's scheme-dependent defaults resolve on change
color-palette#1.4 · yes · an iframe's open list reads the scheme copied from its parent, and the list paints in every document
color-palette#1.5 · no · the `bgcolor` attribute maps an author's background onto the select itself, which the chunk does not change and the list's defaults do not read
color-palette#1.6 · no · the frame clear colour sits behind the whole frame, and the list neither sets nor reads it
color-palette#1.7 · no · the canvas background sits beneath the page, and the list's own opaque surface and contrast do not read it
color-palette#1.8 · no · the WPT runner's white fill is a test-harness background that no scope item names
color-palette#2.1 · no · browser chrome focus, which the list does not touch
color-palette#2.2 · no · browser about-page link colour, browser chrome the list does not touch
color-palette#2.3 · yes · names the flight booker's `select#flight-type` border, which the scope names and the stand rows read, held in layout-templates
color-palette#2.4 · yes · the input focus outline `#4D90FE` is a focus colour the select-and-rows focus indication may read, in default.css where the open list's defaults are edited; the link and button values are re-checks only
color-palette#2.5 · yes · the `multiple` chosen-row pair must not move and sits beside the list's chosen-option state
color-palette#2.6 · yes · the engine-painted selection highlight in blitz-paint/src, a reached file with no units named, is an engine-painted default the list's colours are read against
color-palette#2.7 · no · example fixtures' own named colours, which a new engine default does not read
color-palette#3.1 · no · browser tabstrip chrome, which the list does not touch
color-palette#3.2 · no · browser tooltip, FPS overlay and status bar chrome, which the list does not touch
color-palette#3.3 · no · browser about-page backgrounds, which the list does not touch
color-palette#3.4 · no · apps/readme's own stylesheet, an app's values and not an engine default
color-palette#3.5 · no · todomvc host page styling, outside what the engine paints
color-palette#3.6 · no · wasm_hello example fixture's demonstration palette
color-palette#3.7 · no · example fixtures' demonstration backgrounds
color-palette#4.1 · no · browser tooltip and status bar text, browser chrome the list does not touch
color-palette#4.2 · no · browser about-page muted text, browser chrome the list does not touch
color-palette#4.3 · no · browser error page paragraph, browser chrome the list does not touch
