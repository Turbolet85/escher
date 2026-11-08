@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes and families; the chunk does not touch the browser app.
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion section header type; the browser app is not reached by the read.
@answer n=3 entity=typography#1.3 v=no
why: browser FPS overlay monospace font; the browser app is not reached by the read.
@answer n=4 entity=typography#1.4 v=no
why: about:history page styling; the driver read and the stand fixture do not reach it.
@answer n=5 entity=typography#1.5 v=no
why: rdme body font stack and size; the readme app is not reached by the chunk.
@answer n=6 entity=typography#1.6 v=no
why: blitz-dom default control font family; the read paints nothing and the stand proof checks non-zero text, not the control family.
@answer n=7 entity=typography#1.7 v=no
why: blitz-dom default heading sizes and weights; no heading is read or measured by the chunk.
@answer n=8 entity=typography#1.8 v=no
why: blitz-dom default code and pre font; not reached by the read or the stand fixture.
@answer n=9 entity=typography#1.9 v=no
why: select[multiple][size] row line-height; the open select list is SelectListView, not a multi-select listbox.
@answer n=10 entity=typography#1.10 v=no
why: todomvc example font and sizes; the example is not reached by the chunk.
@answer n=11 entity=typography#1.11 v=no
why: wgpu_texture example font stack; the example is not reached by the chunk.
@answer n=12 entity=typography#1.12 v=no
why: text input editor size of 16px; it sizes the editor and does not decide a type call's acceptance or the non-zero text measure.
@answer n=13 entity=typography#1.13 v=no
why: engine generic base sizes; the stand proof needs non-zero text, not a particular default size, and the scope names no size.
@answer n=14 entity=typography#1.14 v=no
why: list bullet font stack; not reached by the read or the stand proof.
@answer n=15 entity=typography#2.1 v=no
why: WASM bundled DejaVu Sans registration; the chunk is native and headless, with no WASM build.
@answer n=16 entity=typography#2.2 v=no
why: pointer locating the headless stand's font registration in architecture; it states no behaviour or value the chunk must honour.
@answer n=17 entity=typography#2.3 v=no
why: build_single_font_ctx with system fonts disabled; the scope names the cargo system-fonts build feature, not this runtime font context, and the chunk does not call it.
@answer n=18 entity=typography#2.4 v=yes
why: dioxus-native lists the system-fonts feature, which the scope names and the stand proof's non-zero text depends on.
@answer n=19 entity=typography#2.5 v=no
why: bullet font always registered in the default context; not reached by the read or the stand's text measure.
@answer n=20 entity=typography#2.6 v=yes
why: the screenshot fonts-unavailable refusal is named by the scope, and the stand proof must pass its sans-serif check.
@answer n=21 entity=typography#2.7 v=no
why: document.fonts stub in the JS runtime; the chunk is native and runs no settle.
@answer n=22 entity=typography#2.8 v=yes
why: the scope names the system-fonts build feature, and without it text measures 0x0 so the stand's off-screen and covered checks pass vacuously.
@answer n=23 entity=typography#3.1 v=yes
why: None maps to sans-serif in Parley, the family the scope's fonts-unavailable check rules over.
@answer n=24 entity=typography#3.2 v=no
why: font weight, width, style and variation conversion; the read and the stand proof set no variation settings.
@answer n=25 entity=typography#3.3 v=no
why: font-variant to OpenType feature tag mapping; not reached by the read or the stand proof.
