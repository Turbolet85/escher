@answer n=1 entity=typography#1.1 v=no
why: browser chrome font sizes live in apps/browser CSS, which the handler count and the owed replay test never set or read
@answer n=2 entity=typography#1.2 v=no
why: suggestion header typography lives in apps/browser CSS; the chunk paints and measures no text
@answer n=3 entity=typography#1.3 v=no
why: FPS overlay font lives in apps/browser CSS; the chunk adds no overlay and measures no text
@answer n=4 entity=typography#1.4 v=no
why: about:history heading font lives in apps/browser CSS; the chunk touches no about page
@answer n=5 entity=typography#1.5 v=no
why: rdme body font stack lives in apps/readme CSS; the chunk touches no readme app
@answer n=6 entity=typography#1.6 v=no
why: blitz-dom default form control fonts are in a stylesheet none of the chunk's reached files set or read
@answer n=7 entity=typography#1.7 v=no
why: blitz-dom default heading sizes; the chunk measures no text and reads no default stylesheet
@answer n=8 entity=typography#1.8 v=no
why: blitz-dom default code and pre family; the chunk paints no code text
@answer n=9 entity=typography#1.9 v=no
why: blitz-dom select row line-height; the chunk paints and measures no select list
@answer n=10 entity=typography#1.10 v=no
why: todomvc font stack in its example CSS; the chunk does not build or run todomvc
@answer n=11 entity=typography#1.11 v=no
why: wgpu_texture example font stack; the chunk does not build or run that example
@answer n=12 entity=typography#1.12 v=no
why: text editor base size in blitz-dom text.rs, a file the chunk does not reach
@answer n=13 entity=typography#1.13 v=no
why: engine generic base sizes in font_metrics.rs; the chunk measures no text
@answer n=14 entity=typography#1.14 v=no
why: list bullet font stack in blitz-dom list layout; the chunk renders no list
@answer n=15 entity=typography#2.1 v=no
why: WASM font registration in wasm_hello; the chunk builds no WASM target
@answer n=16 entity=typography#2.2 v=no
why: architecture holds the stand's font registration; the chunk leaves font_ctx() unchanged and the rule states no behaviour the coverage read or owed test must honour
@answer n=17 entity=typography#2.3 v=no
why: custom FontContext and build_single_font_ctx; the chunk sets no font context
@answer n=18 entity=typography#2.4 v=no
why: font feature flags in dioxus-native Cargo.toml; the chunk changes no feature
@answer n=19 entity=typography#2.5 v=no
why: default bullet font registration in blitz-dom; the chunk lays out no bullets
@answer n=20 entity=typography#2.6 v=no
why: the screenshot fonts-unavailable refusal; the chunk takes no screenshot and its replay names no screenshot step
@answer n=21 entity=typography#2.7 v=no
why: document.fonts stub in blitz-vibey-script; the chunk runs no page font loading
@answer n=22 entity=typography#2.8 v=no
why: blitz-tests system-fonts feature; the chunk adds no blitz-tests and its tests live in escher-driver
@answer n=23 entity=typography#3.1 v=no
why: generic family mapping in stylo_to_parley.rs; the chunk converts no font family
@answer n=24 entity=typography#3.2 v=no
why: weight, width and style conversion in stylo_to_parley.rs; the chunk converts no font value
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature mapping in stylo_to_parley.rs; the chunk sets no font-variant
