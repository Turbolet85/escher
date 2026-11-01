# Classified — design · typography.b1

## Answers
typography#1.1 · no · browser chrome text in apps/browser/assets/browser.css; no app chrome is restyled and that file is not named
typography#1.2 · no · browser suggestion header in browser.css; app chrome is not in scope
typography#1.3 · no · browser FPS overlay monospace in browser.css; app chrome is not in scope
typography#1.4 · no · about:history stylesheet of the browser app; no file of that app is named
typography#1.5 · no · rdme body stack in github-markdown.css; the readme app is not named
typography#1.6 · yes · default.css is a conflict surface, and its form-control font stacks are engine type defaults read against the resolved sheet
typography#1.7 · yes · default.css heading sizes and bold weight are engine type defaults in a conflict surface, read against the resolved sheet
typography#1.8 · yes · default.css code and pre family is an engine type default in a conflict surface, read against the resolved sheet
typography#1.9 · yes · default.css select[multiple][size] line-height is the row value the open list and a multiple select's rows are read against
typography#1.10 · no · todomvc stylesheet; the scope names no change to todomvc fixture values
typography#1.11 · no · wgpu_texture stylesheet; not reached and not named
typography#1.12 · yes · the 16.0 editor size is the default text inputs are painted at, which the re-measured seven_guis cells fixture values read
typography#1.13 · yes · generic base sizes are the family and size the re-measured bounds, snapshot text and font-embolden witness are painted at
typography#1.14 · no · list bullet font stack in list.rs; the chunk re-measures no list bullets and names no list file
typography#2.1 · no · WASM bundled DejaVu registration in wasm_hello; the scope names no wasm surface
typography#2.2 · no · pointer only: it names where the stand's font registration lives and states no value or behaviour
typography#2.3 · no · build_single_font_ctx sits in lib.rs lines 137-168, a unit the change does not edit, and the scope names no call to it
typography#2.4 · yes · the font-embolden and apple-font-embolden features in dioxus-native's Cargo.toml are the feature names the new local font-embolden gate builds with
typography#2.5 · no · bullet font registration in document.rs; that file is not reached and the chunk makes no bullet call
typography#2.6 · no · standing S2 · a driver screenshot needs a font context resolving sans-serif, refused fonts-unavailable otherwise
typography#2.7 · yes · the document.fonts stub lives in bootstrap/fonts.js, loaded by the bootstrap.rs conflict surface, so a resolution there can move it
typography#2.8 · no · standing S1 · a blitz-tests target builds blitz-dom with system-fonts
typography#3.1 · yes · the generic family mapping to Parley generics is a recorded text mapping value the scope reads against the resolved construct.rs and inline/build.rs
typography#3.2 · yes · the weight, width, style and variation mapping is a recorded text mapping value the scope reads against the resolved construct.rs and inline/build.rs
typography#3.3 · yes · the OpenType feature-tag mapping is a recorded text mapping value the scope reads against the resolved construct.rs and inline/build.rs
