@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes belong to the browser app, which the chunk does not touch
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion header belongs to the browser app, which the chunk does not touch
@answer n=3 entity=typography#1.3 v=no
why: browser FPS overlay belongs to the browser app, which the chunk does not touch
@answer n=4 entity=typography#1.4 v=no
why: about:history styling belongs to the browser app, which the chunk does not touch
@answer n=5 entity=typography#1.5 v=no
why: rdme body typography belongs to the readme app, which the chunk does not touch
@answer n=6 entity=typography#1.6 v=yes
why: the default family of inputs, selects and buttons is the family the read's actionable controls are measured at in the stand proof
@answer n=7 entity=typography#1.7 v=no
why: default headings are not stated to appear in the minimal stand fixture or any read target
@answer n=8 entity=typography#1.8 v=no
why: default code and pre styling is not a read target and not in the stand fixture
@answer n=9 entity=typography#1.9 v=no
why: the blitz-dom select[multiple][size] row line-height is not the SelectListView row the read lists
@answer n=10 entity=typography#1.10 v=no
why: the todomvc example is outside the driver read and the stand
@answer n=11 entity=typography#1.11 v=no
why: the wgpu_texture example is outside the driver read and the stand
@answer n=12 entity=typography#1.12 v=yes
why: the 16px text input editor size is the size at which the read's text inputs are measured for their off-screen and covered checks
@answer n=13 entity=typography#1.13 v=yes
why: the generic base size is the size the stand's text with no set font is measured at, which the proof's non-zero text depends on
@answer n=14 entity=typography#1.14 v=no
why: list bullets are not actionable nodes and the chunk does not touch list layout
@answer n=15 entity=typography#2.1 v=no
why: WASM bundled font registration is outside the native headless stand proof
@answer n=16 entity=typography#2.2 v=no
why: the entity only locates the stand's font registration in architecture and states no value the chunk must honour
@answer n=17 entity=typography#2.3 v=yes
why: the scope leaves open whether the stand's font context disables system fonts through build_single_font_ctx, a fork over the stand proof's non-zero text
@answer n=18 entity=typography#2.4 v=yes
why: the system-fonts build feature that the scope names is listed among the dioxus-native features
@answer n=19 entity=typography#2.5 v=no
why: the always-registered bullet font concerns list markers the chunk does not paint or read
@answer n=20 entity=typography#2.6 v=yes
why: the scope names the screenshot fonts-unavailable refusal and the stand proof depends on the same non-zero text check
@answer n=21 entity=typography#2.7 v=no
why: the document.fonts stub is a web runtime shim outside the native driver read
@answer n=22 entity=typography#2.8 v=yes
why: the scope names the system-fonts build feature, and the stand proof's non-zero text depends on it, so the S2 convention binds here
@answer n=23 entity=typography#3.1 v=yes
why: the generic family mapping with None as sans-serif sets the family that unset-font text in the stand proof is measured at
@answer n=24 entity=typography#3.2 v=no
why: weight, width and variation conversion is outside the family and size the proof measures at, and the chunk sets no type
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature tags for ligatures and numerals are engine text mapping the chunk does not touch
