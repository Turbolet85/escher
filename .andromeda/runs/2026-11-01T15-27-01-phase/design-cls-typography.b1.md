# Classified — design · typography.b1

## Answers
typography#1.1 · no · browser chrome sizes in apps/browser/assets/browser.css; the browser app is neither reached nor named by the scope
typography#1.2 · no · browser suggestion header values in browser.css; app chrome the scope does not restyle
typography#1.3 · no · browser FPS overlay monospace in browser.css; app chrome outside the scope
typography#1.4 · no · about:history heading values in about-history.css; no file of the browser app is named
typography#1.5 · no · readme body stack in github-markdown.css; the readme app's CSS is not named by the scope
typography#1.6 · yes · default.css form-control font stacks sit in a conflict surface, and the recorded engine default type is read against the resolved sheet
typography#1.7 · yes · default.css heading sizes and weights sit in a conflict surface, and the recorded engine default type is read against the resolved sheet
typography#1.8 · yes · default.css code/pre font sits in a conflict surface, and the recorded engine default type is read against the resolved sheet
typography#1.9 · yes · default.css select[multiple][size] line-height sits in a conflict surface, and the open select list's row type is re-read
typography#1.10 · no · todomvc fixture values; the scope names no change to examples other than seven_guis
typography#1.11 · no · wgpu_texture fixture font stack; examples other than seven_guis are out of scope
typography#1.12 · yes · recorded engine default type (text input editor size 16.0) that the chunk re-reads as its own work, wherever its file lives
typography#1.13 · yes · recorded engine generic base sizes are engine default type values the chunk reads against the resolved default sheet
typography#1.14 · yes · recorded engine list-bullet font stack is an engine default type value the chunk reads as its own work
typography#2.1 · no · WASM bundled DejaVu registration in examples/wasm_hello; the scope names no wasm surface
typography#2.2 · yes · names font_ctx() of the seven_guis headless stand, which the seven_guis fixture and the re-run stand text checks rely on
typography#2.3 · no · WASM custom FontContext and build_single_font_ctx in dioxus-native and blitz-dom lib.rs lines 137-168, which are neither edited nor named
typography#2.4 · yes · feature list includes font-embolden, which the new local gate builds through select_popup_painted
typography#2.5 · no · bullet font registration in document.rs and lib.rs line 32, outside the edited lines 96-97 and not named by the scope
typography#2.6 · no · standing S2 · a test that takes a driver screenshot needs a font context resolving sans-serif, since the shot is refused otherwise
typography#2.7 · no · document.fonts stub in bootstrap/fonts.js, a file the take does not reach and the scope does not name
typography#2.8 · no · standing S1 · a blitz-tests target builds blitz-dom with system-fonts, so font-dependent assertions do not pass on text measuring 0x0
typography#3.1 · yes · engine generic-family mapping (None as sans-serif) is a recorded engine text mapping value read against the resolved construct and inline build surfaces
typography#3.2 · yes · weight, width, style and variation mapping is a recorded engine text mapping value read against construct.rs and inline/build.rs
typography#3.3 · yes · OpenType feature-tag mapping is a recorded engine text mapping value read against construct.rs and inline/build.rs
