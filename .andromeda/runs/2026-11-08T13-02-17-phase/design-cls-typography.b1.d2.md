@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes belong to the browser app, which the chunk does not touch
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion section header type belongs to the browser app, outside the read
@answer n=3 entity=typography#1.3 v=no
why: browser FPS overlay monospace belongs to the browser app, outside the read
@answer n=4 entity=typography#1.4 v=no
why: about:history heading type is a browser page style, outside the driver read
@answer n=5 entity=typography#1.5 v=no
why: rdme body stack belongs to the readme app, which the chunk does not paint
@answer n=6 entity=typography#1.6 v=yes
why: the default family of inputs, selects and buttons is the family the stand proof's select rows and buttons are measured at
@answer n=7 entity=typography#1.7 v=no
why: default heading sizes bind no node the read reports or the stand proof measures
@answer n=8 entity=typography#1.8 v=no
why: default code and pre family bind no element in the read's scope
@answer n=9 entity=typography#1.9 v=no
why: select multiple size rows are a different element from the open select list's rows the read answers
@answer n=10 entity=typography#1.10 v=no
why: the todomvc app stack is an example app outside the chunk
@answer n=11 entity=typography#1.11 v=no
why: the wgpu_texture example stack is an example app outside the chunk
@answer n=12 entity=typography#1.12 v=no
why: the text editor size is the editing surface's, which the read neither measures nor dispatches on
@answer n=13 entity=typography#1.13 v=yes
why: the default generic size is the size the stand proof's unstyled fixture text is painted and measured at
@answer n=14 entity=typography#1.14 v=no
why: list bullet font binds no node the read reports and no marker the stand proof measures
@answer n=15 entity=typography#2.1 v=no
why: WASM bundled font registration is outside the chunk, which builds no WASM target
@answer n=16 entity=typography#2.2 v=no
why: it only locates the stand's font registration, and the font behaviour it leads to is carried by typography#2.6 and typography#2.8
@answer n=17 entity=typography#2.3 v=yes
why: the minimal fixture's font context is a call the stand proof's text measuring depends on, and build_single_font_ctx disables system fonts, a fork the scope leaves open
@answer n=18 entity=typography#2.4 v=yes
why: it names the system-fonts feature, which the scope names as the build feature the stand proof's text measuring depends on
@answer n=19 entity=typography#2.5 v=no
why: the bullet font is registered for list bullets, which neither the read nor the stand proof measures
@answer n=20 entity=typography#2.6 v=yes
why: the scope names the screenshot fonts-unavailable refusal, and the stand's bundled font passing that check is what the proof's text measuring rests on
@answer n=21 entity=typography#2.7 v=no
why: the document.fonts stub belongs to the JS runtime, which the chunk's read does not use
@answer n=22 entity=typography#2.8 v=yes
why: the scope names the system-fonts build feature, and without it text measures 0x0 so the off-screen and covered checks pass vacuously
@answer n=23 entity=typography#3.1 v=yes
why: it maps the generic None to sans-serif, the family the stand proof's text measurement and the fonts-unavailable check resolve through
@answer n=24 entity=typography#3.2 v=no
why: weight, width and variation conversion sets no value the read or the stand proof uses
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature tag mapping sets no feature the read or the stand proof uses
