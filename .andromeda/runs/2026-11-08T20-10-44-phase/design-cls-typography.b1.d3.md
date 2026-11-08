@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes and families are painted by the browser app, which the coverage read and the owed replay test do not paint or measure
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion header type is a browser CSS value; the chunk paints and measures no text
@answer n=3 entity=typography#1.3 v=no
why: browser FPS overlay monospace font is a browser CSS value outside the handler count and replay test
@answer n=4 entity=typography#1.4 v=no
why: about:history heading type is a page stylesheet value; the chunk takes no screenshot and reads no text
@answer n=5 entity=typography#1.5 v=no
why: readme body font stack and line-height are the readme app's CSS; the chunk renders and measures no text
@answer n=6 entity=typography#1.6 v=no
why: blitz-dom default form control fonts are engine defaults the chunk neither sets nor reads
@answer n=7 entity=typography#1.7 v=no
why: blitz-dom default heading sizes are engine defaults the chunk neither sets nor reads
@answer n=8 entity=typography#1.8 v=no
why: blitz-dom default code and pre font is an engine default the chunk neither sets nor reads
@answer n=9 entity=typography#1.9 v=no
why: blitz-dom select multiple row line-height is layout of a painted control the coverage read does not paint
@answer n=10 entity=typography#1.10 v=no
why: todomvc example font values belong to that example app, which the chunk does not build or replay
@answer n=11 entity=typography#1.11 v=no
why: wgpu_texture example font stack belongs to that example app, which the chunk does not build or replay
@answer n=12 entity=typography#1.12 v=no
why: text input editor size is a paint-time parley value the coverage count and owed test never reach
@answer n=13 entity=typography#1.13 v=no
why: engine generic base sizes affect text layout only, and the chunk measures no text
@answer n=14 entity=typography#1.14 v=no
why: list bullet font is a layout detail of painted lists, outside the handler count and replay test
@answer n=15 entity=typography#2.1 v=no
why: WASM bundled DejaVu registration is a web build concern; the chunk touches no WASM build
@answer n=16 entity=typography#2.2 v=no
why: the seven_guis stand's font registration is held in architecture, not a value the coverage read or owed test must honour
@answer n=17 entity=typography#2.3 v=no
why: custom FontContext setup for WASM and native configs is not set or read by the chunk's code or tests
@answer n=18 entity=typography#2.4 v=no
why: font feature flags in dioxus-native Cargo.toml are build features the chunk does not change or depend on
@answer n=19 entity=typography#2.5 v=no
why: bullet font registration in the default font context is engine behaviour the chunk does not act on
@answer n=20 entity=typography#2.6 v=no
why: the screenshot fonts-unavailable refusal binds only a screenshot, and the chunk takes no screenshot
@answer n=21 entity=typography#2.7 v=no
why: document.fonts stub in the bootstrap script is a web runtime detail the chunk does not touch
@answer n=22 entity=typography#2.8 v=no
why: the vacuous-pass rule covers blitz-tests font assertions, while the owed test lives in escher-driver and measures no text
@answer n=23 entity=typography#3.1 v=no
why: generic font family mapping to Parley generics is text conversion the chunk does not exercise
@answer n=24 entity=typography#3.2 v=no
why: font weight, width and style conversion is text conversion the chunk does not exercise
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature tag mapping for font-variant properties is text conversion the chunk does not exercise
