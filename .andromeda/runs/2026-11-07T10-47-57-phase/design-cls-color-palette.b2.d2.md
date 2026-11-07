@answer n=1 entity=color-palette#3.7 v=no
why: the row is an example-page surface background colour; the chunk changes no colour and no reached file holds this value
@answer n=2 entity=color-palette#4.1 v=no
why: a browser tooltip and status text colour in browser.css; the chunk renders no browser UI and edits no CSS
@answer n=3 entity=color-palette#4.2 v=no
why: browser about-page muted text colours; no about page or its CSS is reached by the buffered render
@answer n=4 entity=color-palette#4.3 v=no
why: browser error page paragraph colour in error.html; the chunk does not touch that page or its text colour
@answer n=5 entity=color-palette#4.4 v=no
why: todomvc body and heading colours; the example is not reached and the probe pins no colour value
@answer n=6 entity=color-palette#4.5 v=no
why: wasm_hello heading and code colours; the web-spa paint claim is that the buffer leaves paint unchanged, and no test of the chunk re-asserts or pins these values
@answer n=7 entity=color-palette#5.1 v=no
why: disabled checkbox and radio accent value in form_controls.rs; form controls are not touched and no probe test reads paint
@answer n=8 entity=color-palette#6.1 v=no
why: WPT runner terminal colour for PASS; the runner is a separate tool the chunk does not reach
@answer n=9 entity=color-palette#6.2 v=no
why: WPT runner terminal colour for FAIL with passing subtests; the runner is a separate tool the chunk does not reach
@answer n=10 entity=color-palette#6.3 v=no
why: WPT runner terminal colour for FAIL; the runner is a separate tool the chunk does not reach
@answer n=11 entity=color-palette#6.4 v=no
why: WPT runner terminal colour for TIMEOUT; the runner is a separate tool the chunk does not reach
@answer n=12 entity=color-palette#6.5 v=no
why: WPT runner terminal colour for SKIP; the runner is a separate tool the chunk does not reach
@answer n=13 entity=color-palette#6.6 v=no
why: WPT runner terminal colour for CRASH; the runner is a separate tool the chunk does not reach
@answer n=14 entity=color-palette#7.1 v=no
why: browser urlbar border colour in browser.css; browser chrome is not touched by the chunk
@answer n=15 entity=color-palette#7.2 v=no
why: browser chrome focus border colour in browser.css; browser chrome is not touched by the chunk
@answer n=16 entity=color-palette#7.3 v=no
why: blitz-dom default input focus outline colour; the chunk changes no default.css rule and the focus assertions read focus state, not outline paint
@answer n=17 entity=color-palette#8.1 v=no
why: default scrollbar thumb paint values; the chunk's scroll rule concerns which boxes an into-view scroll writes, and no probe test pins thumb colour
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution and author thumb blending; the chunk reads no scrollbar paint and its scroll rule is about which boxes are written
@answer n=19 entity=color-palette#8.3 v=no
why: unchecked radio ring and checkbox tick colours in form_controls.rs; form controls are not touched
@answer n=20 entity=color-palette#8.4 v=no
why: debug overlay content, padding, border and margin colours; the chunk does not touch or enable the debug overlay
@answer n=21 entity=color-palette#8.5 v=no
why: devtools layout stroke colours in render.rs; devtools are not touched and no probe test reads these strokes
@answer n=22 entity=color-palette#8.6 v=yes
why: the snapshot re-assertion serialises document output and the scope leaves open whether SVG currentColor attributes reach it, so the row binds as a fork
@answer n=23 entity=color-palette#9.1 v=no
why: rdme GitHub markdown colour tokens in apps/readme; the chunk does not reach that app
@answer n=24 entity=color-palette#10.1 v=no
why: the absence of colour tokens in the engine holds trivially since the chunk adds none and no test reads one
@answer n=25 entity=color-palette#11.1 v=no
why: the clip-path fixture's 25-colour palette in examples/assets; the example is not reached by the chunk
