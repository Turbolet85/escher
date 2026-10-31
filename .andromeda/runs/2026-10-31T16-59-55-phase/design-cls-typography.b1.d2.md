# Classified — design · typography.b1

## Answers
typography#1.1 · no · browser chrome text in apps/browser, which the chunk does not touch and the task lists as not needed
typography#1.2 · no · browser suggestion section header in apps/browser, outside the list's reach
typography#1.3 · no · browser FPS overlay monospace, browser chrome outside the list's reach
typography#1.4 · no · about:history headings in an app stylesheet, not an engine default the list reads
typography#1.5 · no · apps/readme GitHub markdown body, an app's own stylesheet the task lists as not needed
typography#1.6 · yes · the default select's `system-ui, sans-serif` family is the closed select's label font, which the scope names for the row text and a closed label
typography#1.7 · no · default heading sizes and weights, which the chunk neither reads nor sets
typography#1.8 · no · default code and pre family, not a select or list value
typography#1.9 · no · todomvc example fixture's own values, which the task lists as not needed
typography#1.10 · no · wgpu_texture example's own stylesheet, a demonstration the task lists as not needed
typography#1.11 · no · text input editor default size, a text-input component the list does not use
typography#1.12 · yes · the engine's generic base sizes, 16px for sans-serif, are the text mapping the row text size is named through
typography#1.13 · no · list bullet markers for `li`, which the open list paints none of
typography#2.1 · yes · WASM registers bundled DejaVu Sans for sans-serif, the family the list's rows paint in on the WASM canvas surface the scope names
typography#2.2 · no · a locator naming where the stand's font registration lives, stating no value or rule the chunk must meet
typography#2.3 · yes · on WASM the list's text needs a context with bundled fonts via build_single_font_ctx, the surface the scope names
typography#2.4 · yes · the system-fonts feature decides whether a row's and a closed label's system family resolves, the loading and absent families the scope names
typography#2.5 · no · the bullet font is registered for list markers, which the open list does not paint
typography#2.6 · no · standing S2 · a new pixel check through the driver's screenshot owes a font context that resolves sans-serif
typography#2.7 · no · the page-script FontFaceSet stub, which the list's paint neither calls nor reads
typography#2.8 · no · standing S1 · a new test that measures or asserts on text runs in blitz-tests with blitz-dom's system-fonts feature
typography#3.1 · yes · generic family mapping, with None as sans-serif, is the engine's text mapping the row family resolves through
typography#3.2 · yes · font weight conversion is the engine's text mapping the row weight is named through
typography#3.3 · no · font-variant feature mapping, which the list's rows do not set and the scope does not name
typography#3.4 · yes · line-height normal resolves as 1.2 × font size, which the list's row height, not yet set, is read against
