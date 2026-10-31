# Classified — design · typography.b1

## Answers
typography#1.1 · no · browser chrome text in apps/browser, chrome the list does not touch
typography#1.2 · no · browser suggestion section header, browser chrome the list does not touch
typography#1.3 · no · browser FPS overlay, browser chrome the list does not touch
typography#1.4 · no · about:history page styling, outside the reached files
typography#1.5 · no · rdme body styling, an app's own stylesheet the chunk does not reach
typography#1.6 · yes · the default select's `system-ui, sans-serif` family is the closed select's label font and the open list's row font, which the scope names
typography#1.7 · no · default heading sizes are other units of default.css that the change does not edit
typography#1.8 · no · code/pre defaults are other units of default.css that the change does not edit
typography#1.9 · no · todomvc's own values, an example fixture the chunk does not reach
typography#1.10 · no · wgpu_texture's own values, an example fixture's demonstration
typography#1.11 · no · text input editor size, not a select or a list row
typography#1.12 · yes · the generic base sizes (monospace 13px, other generics 16px) set the row text size and a closed select's label size through the engine's text mapping
typography#1.13 · no · list bullet fonts for `li` markers, which the open list does not draw
typography#2.1 · yes · the WASM canvas list paints its row text in the bundled DejaVu Sans registered for sans-serif
typography#2.2 · yes · the stand's popup and the stand_* rows paint through the stand's font registration, which the scope names
typography#2.3 · yes · the list paints through the font context set, so the WASM and dioxus-native contexts must carry the fonts the list's rows use
typography#2.4 · no · standing S1 · a new test that measures or asserts on text runs in blitz-tests with blitz-dom's system-fonts feature
typography#2.5 · no · the bullet font registration serves list markers the open list does not draw
typography#2.6 · no · standing S2 · a new pixel check through the driver's screenshot owes a font context that resolves sans-serif
typography#2.7 · yes · the scope names text loading for a row and a closed select's label, and this states how the plan's font API reports loaded
typography#2.8 · no · standing S1 · a new test that measures or asserts on text runs in blitz-tests with blitz-dom's system-fonts feature, or it passes vacuously on text of 0x0
typography#3.1 · yes · the generic family mapping (None as sans-serif) sets the row and closed select label family through the engine's text mapping
typography#3.2 · yes · the weight conversion sets the row and closed select label weight through the engine's text mapping
typography#3.3 · no · OpenType feature tags for font-variant properties, which the scope does not name for rows
typography#3.4 · yes · line-height normal resolves to 1.2 times the font size, which sets the row height the scope names
