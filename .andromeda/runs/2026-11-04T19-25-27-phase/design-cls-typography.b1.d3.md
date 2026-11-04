@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes belong to apps/browser CSS, which the probe does not read or paint
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion header style in apps/browser is outside the seven_guis probe
@answer n=3 entity=typography#1.3 v=no
why: browser FPS overlay monospace is an apps/browser value the probe does not read
@answer n=4 entity=typography#1.4 v=no
why: about:history heading style is an apps/browser page the stand does not run
@answer n=5 entity=typography#1.5 v=no
why: readme body stack in apps/readme is a value of an app the probe does not run
@answer n=6 entity=typography#1.6 v=no
why: blitz-dom default form-control families are painted defaults; the probe reads nodes, handlers and state, not input styling
@answer n=7 entity=typography#1.7 v=no
why: blitz-dom default heading sizes are painted defaults the probe does not read
@answer n=8 entity=typography#1.8 v=no
why: blitz-dom default code and pre family is a painted default the probe does not read
@answer n=9 entity=typography#1.9 v=no
why: blitz-dom select multiple line-height is a layout default; the probe's link verdicts do not measure listbox row height
@answer n=10 entity=typography#1.10 v=no
why: todomvc body and heading sizes belong to examples/todomvc, which is not the seven_guis stand
@answer n=11 entity=typography#1.11 v=no
why: wgpu_texture system-ui family belongs to the wgpu example, which the probe does not run
@answer n=12 entity=typography#1.12 v=no
why: text input editor default size is a text-metric default; the probe's link verdicts do not measure text input geometry
@answer n=13 entity=typography#1.13 v=no
why: engine generic base sizes are text metrics the probe neither reads nor re-measures; its layout modes touch link readings, not font sizes
@answer n=14 entity=typography#1.14 v=no
why: list bullet font is a marker value the probe does not read
@answer n=15 entity=typography#2.1 v=no
why: WASM DejaVu registration belongs to the web-spa surface, which is outside the probe
@answer n=16 entity=typography#2.2 v=no
why: a pointer to where the stand's font registration is held; it states no value the probe reads, and the scope does not name font_ctx or its registration
@answer n=17 entity=typography#2.3 v=no
why: FontContext setting and the WASM bundled-font requirement are a launch and WASM rule the scope does not name; the windowed and desktop probe does not set a context
@answer n=18 entity=typography#2.4 v=no
why: dioxus-native Cargo feature list is a dependency fact the scope does not name; the blitz-tests build it could affect is carried by S1 as its own convention
@answer n=19 entity=typography#2.5 v=no
why: bullet font registration in the default font context concerns list markers the probe does not read
@answer n=20 entity=typography#2.6 v=no
why: fonts-unavailable refusal governs driver screenshots; the probe adds no screenshot or driver verb and runs through poll and VirtualClock
@answer n=21 entity=typography#2.7 v=no
why: document.fonts is a JS bootstrap stub; the probe is native Rust and reads no JS font API
@answer n=22 entity=typography#2.8 v=no standing=S1
why: states the blitz-tests system-fonts build convention S1, carried by that convention rather than this entity
@answer n=23 entity=typography#3.1 v=no
why: generic family mapping to Parley concerns computed font families the probe does not read
@answer n=24 entity=typography#3.2 v=no
why: weight, width and style conversion to Parley concerns computed font values the probe does not read
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature tag mapping concerns computed font features the probe does not read
