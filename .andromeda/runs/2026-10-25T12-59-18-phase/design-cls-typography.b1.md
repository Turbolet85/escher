# Classified — design · typography.b1

## Answers
typography#1.1 · no · the browser chrome text sizes are apps/browser CSS, and the chunk touches no app under apps/
typography#1.2 · no · the browser suggestion section header is apps/browser CSS, and the chunk touches no app under apps/
typography#1.3 · no · the browser FPS overlay font is apps/browser CSS, and the chunk touches no app under apps/
typography#1.4 · no · the about:history font stack is apps/browser CSS, and the chunk touches no app under apps/
typography#1.5 · no · the rdme body stack is apps/readme CSS, and the chunk touches no app under apps/
typography#1.6 · yes · the blitz-dom default stack for inputs, selects and buttons paints the flight booker's select and Cells' controls, the engine-painted form-control defaults the needs name
typography#1.7 · no · the h1–h6 defaults paint only where a task authors a heading, and no need or task the chunk names has one
typography#1.8 · no · the code and pre defaults paint only authored code, which no stand task or need reaches
typography#1.9 · yes · seven_guis' base 14px is the stand's own text size, which the select's label inherits where the engine's default leaves size unstated (the home title rule alone does not bind)
typography#1.10 · no · the todomvc styles sit in examples/todomvc, which the chunk does not reach
typography#1.11 · no · the wgpu_texture styles sit in examples/wgpu_texture, which the chunk does not reach
typography#1.12 · yes · the text input editor's 16.0 starting size lays out Cells' cell inputs, which are text inputs whose text extents the lint reads for truncated and clipped
typography#1.13 · yes · the engine's generic base sizes (16px, monospace 13px) set the size an unstyled select and Cells' controls take, which the form-control spacing need names
typography#1.14 · no · list bullet fonts paint only list markers, and the select's options and the Cells grid are no list items
typography#2.1 · no · the WASM bundled-font registration is for WASM builds, and the chunk builds no WASM page
typography#2.2 · yes · stand.rs, a reached file, registers the same bundled DejaVu Sans for every generic on the headless native boot the stand's text paints through
typography#2.3 · yes · build_single_font_ctx is the function the headless stand's font context is built with, one font for every generic with system fonts off, which the served select and Cells paint in
typography#2.4 · yes · the woff feature decodes the stand's bundled DejaVu through seven_guis' native woff, which the headless build keeps on
typography#2.5 · no · the bullet font serves list markers, which the chunk paints none of, and the stand builds its font context with build_single_font_ctx rather than the default one
typography#2.6 · yes · the stand's bundled font passes the fonts-unavailable check, so a shot or lint of the served Cells and select must hold to that refusal before painting
typography#2.7 · no · document.fonts is a JS stub in blitz-vibey-script, and the Dioxus stand carries no page script, so no held load reaches it
typography#2.8 · yes · blitz-tests needs blitz-dom's system-fonts feature or text measures 0x0 and the re-pinned stand checks pass vacuously
typography#3.1 · yes · the generic-family mapping, None as sans-serif, is the text mapping the needs name for Cells' cells and the select's chosen label
typography#3.2 · yes · weight, width, style and variation conversion is part of the same text mapping the needs name for Cells' cells and the select's label
typography#3.3 · yes · the OpenType feature-tag mapping is part of the same stylo-to-parley mapping the needs name as Cells' cells and the select's label lay out
