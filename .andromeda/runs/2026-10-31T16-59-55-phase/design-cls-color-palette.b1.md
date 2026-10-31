# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · yes · the list's new engine colours are `Color` values converted from Stylo colours through sRGB, and the colour-model rule governs them
color-palette#1.2 · yes · the list's colours are read against the Light/Dark scheme that feeds `prefers-color-scheme`
color-palette#1.3 · yes · a scheme change recascades `light-dark()` and system colours, so the list's scheme-dependent defaults in `default.css` resolve on that path
color-palette#1.4 · yes · the list paints in every document and an iframe sub-document takes the parent's scheme, so its list colours follow it
color-palette#1.5 · no · the legacy `bgcolor` mapping sets author background on page elements; the list's surface is an engine default the chunk does not read `bgcolor` for
color-palette#1.6 · no · the frame clear colour sits under the page, and the list's contrast is taken against its own surface, not the clear colour
color-palette#1.7 · no · the canvas background sits under the page, and the list's contrast is taken against its own surface
color-palette#1.8 · no · the WPT runner's white fill belongs to the runner, which the chunk neither edits nor gives a WPT check
color-palette#2.1 · no · the browser chrome focus colour belongs to the browser app's stylesheet, which the list does not touch
color-palette#2.2 · no · the browser about:history link colour is an app stylesheet value, not an engine default
color-palette#2.3 · yes · the stand's flight-type select border is the value the stand's popup and its `stand_contrast` rows read
color-palette#2.4 · yes · the focus indication the select and its open rows show reads the engine's input focus outline `#4D90FE` in `default.css`, which the chunk edits
color-palette#2.5 · yes · the `multiple` select's chosen-row pair in `default.css` is a row state the list's chosen row sits beside and must not move
color-palette#2.6 · no · the text-selection highlight paints document text selection, not the list's rows, whose states are chosen, pointer, key and disabled
color-palette#2.7 · no · the counter and transparent buttons are example fixtures, which a new engine default does not read
color-palette#3.1 · no · the browser tabstrip surfaces are browser chrome the list does not touch
color-palette#3.2 · no · the browser tooltip, FPS overlay and status bar are browser chrome, not engine defaults
color-palette#3.3 · no · the browser about-page backgrounds are app stylesheet values
color-palette#3.4 · no · the rdme page background is the apps/readme stylesheet's own value
color-palette#3.5 · no · the todomvc page background is host page styling, outside what the engine paints
color-palette#3.6 · no · the wasm_hello dark palette is an example fixture's own value
color-palette#3.7 · no · the example backgrounds are demonstrations a new engine default does not read
color-palette#4.1 · no · the browser tooltip and status bar text is browser chrome
color-palette#4.2 · no · the about-page muted text is an app stylesheet value
color-palette#4.3 · no · the browser error page paragraph is an app page's own value
