# Classified — design · typography.b1

## Answers
typography#1.1 · no · browser chrome text in apps/browser, which the chunk does not reach
typography#1.2 · no · browser suggestion section header in apps/browser, which the chunk does not reach
typography#1.3 · no · browser FPS overlay in apps/browser, which the chunk does not reach
typography#1.4 · no · about:history page styling in apps/browser, which the chunk does not reach
typography#1.5 · no · rdme body stack is an app's own stylesheet, Not needed
typography#1.6 · yes · the select's default `system-ui, sans-serif` is the family the closed select's label and the open list's row text read through the engine's text mapping
typography#1.7 · no · default heading sizes sit in a default.css unit the chunk does not edit, so only a re-check every change owes
typography#1.8 · no · default code/pre `-moz-fixed` sits in a default.css unit the chunk does not edit, so only a re-check every change owes
typography#1.9 · no · todomvc page font is host page styling and an example fixture, Not needed
typography#1.10 · no · wgpu_texture example font is an example fixture's own value, Not needed
typography#1.11 · no · text input editor size in node/text.rs is not reached, and the list's rows are not editors
typography#1.12 · yes · the engine's generic base sizes (monospace 13px, other generics 16px) are the size a row's text takes through the engine's text mapping, which the scope names
typography#1.13 · no · list bullet font of list-item markers is not the select's list
typography#2.1 · no · the wasm_hello example's bundled-font registration is an example fixture's own value, Not needed
typography#2.2 · no · standing S2 · a new pixel check through the driver's `screenshot` owes a font context that resolves `sans-serif`
typography#2.3 · yes · on the WASM canvas the list's text needs a font context with bundled fonts, built with `build_single_font_ctx`, and the chunk's new paint must pass one
typography#2.4 · no · dioxus-native cargo feature list is a build configuration the scope does not name
typography#2.5 · no · the bullet font is registered for list-marker text, which the chunk does not paint through
typography#2.6 · no · standing S2 · a new pixel check through the driver's `screenshot` owes a font context that resolves `sans-serif`, or it is refused `fonts-unavailable` with nothing painted
typography#2.7 · no · the `document.fonts` stub in blitz-vibey-script is a JS API the list does not read
typography#2.8 · no · standing S1 · a new test that measures or asserts on text runs in the blitz-tests crate with `blitz-dom`'s `system-fonts` feature
typography#3.1 · yes · generic families map to Parley generics with `None` as sans-serif, which the row's font family through the engine's text mapping reads
typography#3.2 · yes · font weight, width, style and variation settings convert to Parley values, which the row's weight through the engine's text mapping reads
typography#3.3 · no · `font-variant-*` feature tags are not read by the row, since the scope names family, size and weight only
typography#3.4 · yes · `line-height: normal` resolves to 1.2 × font size, which bears on the list's row height the scope names
