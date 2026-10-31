# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · yes · the list's new engine colours are read against the Color type and its sRGB conversion, the colour model the Needs name
color-palette#1.2 · yes · the list's colours are read against the system colour scheme, which feeds prefers-color-scheme, so the Light/Dark rule binds
color-palette#1.3 · yes · a scheme change recascades light-dark() and system colours, which the list's new default colours in default.css may use
color-palette#1.4 · yes · the list paints in every document, and an iframe sub-document takes the parent's colour scheme, so its list colours follow it
color-palette#1.5 · no · the legacy bgcolor attribute is a presentational rule on any element, and the chunk's select and list do not read it
color-palette#1.6 · no · the frame clear colour is the renderer's frame background, outside the list's colours and the closed select, which the scope does not name
color-palette#1.7 · no · the canvas background is the root element's background, not the list's colours or the closed select's paint
color-palette#1.8 · no · the white fill is the WPT runner's ref-test surface, and the WPT runner is outside the chunk's scope
color-palette#2.1 · no · browser chrome focus colour; the browser chrome is not touched (Not needed)
color-palette#2.2 · no · browser about:history link colour; browser chrome, not touched (Not needed)
color-palette#2.3 · yes · the flight booker's select#flight-type border, named in Needs, is held in layout-templates and not as a palette value, so the stand row reads it there; the stand's accent and success colours are not reached
color-palette#2.4 · yes · the input focus outline #4D90FE is the engine focus default that the select's focus indication and the open rows' focus indication (Needs) may read; the link colour and button background do not bind
color-palette#2.5 · yes · the chosen-row pair is the state the open list's chosen-option row needs (Needs), and the multiple select rows built by engine-debts must not move what they paint
color-palette#2.6 · no · the text-selection highlight is a paint default for selected text, which neither the list's scope nor its Needs name; a re-check every change owes
color-palette#2.7 · no · named button colours in the counter and transparent examples are demonstration fixtures, which a new engine default does not read
color-palette#3.1 · no · browser tabstrip colours; the browser chrome is not touched (Not needed)
color-palette#3.2 · no · browser tooltip, FPS overlay and status bar backgrounds; browser chrome, not touched (Not needed)
color-palette#3.3 · no · browser about-page backgrounds; browser chrome, not touched (Not needed)
color-palette#3.4 · no · the apps/readme page background is an app's own stylesheet, no engine default (Not needed)
color-palette#3.5 · no · the todomvc page background is host page styling, outside what the engine paints (Not needed)
color-palette#3.6 · no · the wasm_hello dark palette is an example fixture's own value, a demonstration the new default does not read
color-palette#3.7 · no · the custom-widget, static-HTML and wgpu_texture main background is an example fixture's own value, a demonstration the new default does not read
color-palette#4.1 · no · browser tooltip and status bar text; browser chrome, not touched (Not needed)
color-palette#4.2 · no · browser about-page muted text; browser chrome, not touched (Not needed)
color-palette#4.3 · no · browser error page paragraph colour; browser chrome, not touched (Not needed)
