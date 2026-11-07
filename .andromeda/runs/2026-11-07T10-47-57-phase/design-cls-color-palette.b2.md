@answer n=1 entity=color-palette#3.7 v=no
why: the main background colour of the custom-widget, static-HTML and wgpu_texture examples; the chunk's Dioxus document, MountedData and probe tests do not reach those examples.
@answer n=2 entity=color-palette#4.1 v=no
why: the browser tooltip and status bar text colour is browser chrome stylesheet paint; the chunk re-checks documents, element ids, mounted events and focus, not paint colours.
@answer n=3 entity=color-palette#4.2 v=no
why: the browser about-page muted text colour is a stylesheet value the buffered render neither reads nor changes.
@answer n=4 entity=color-palette#4.3 v=no
why: the browser error-page paragraph colour is a stylesheet value outside every check the chunk names.
@answer n=5 entity=color-palette#4.4 v=no
why: the todomvc body and heading colours belong to an example app the chunk does not touch.
@answer n=6 entity=color-palette#4.5 v=no
why: the wasm_hello heading and code colours belong to an example app the chunk does not touch.
@answer n=7 entity=color-palette#5.1 v=no
why: the disabled checkbox and radio accent is a form-control paint value; the chunk changes how mutations are applied, not what a form control paints.
@answer n=8 entity=color-palette#6.1 v=no
why: the WPT runner's PASS terminal colour lives in the wpt runner crate, which the chunk does not reach.
@answer n=9 entity=color-palette#6.2 v=no
why: the WPT runner's FAIL-with-passing-subtests terminal colour lives in the wpt runner crate, which the chunk does not reach.
@answer n=10 entity=color-palette#6.3 v=no
why: the WPT runner's FAIL terminal colour lives in the wpt runner crate, which the chunk does not reach.
@answer n=11 entity=color-palette#6.4 v=no
why: the WPT runner's TIMEOUT terminal colour lives in the wpt runner crate, which the chunk does not reach.
@answer n=12 entity=color-palette#6.5 v=no
why: the WPT runner's SKIP terminal colour lives in the wpt runner crate, which the chunk does not reach.
@answer n=13 entity=color-palette#6.6 v=no
why: the WPT runner's CRASH terminal colour lives in the wpt runner crate, which the chunk does not reach.
@answer n=14 entity=color-palette#7.1 v=no
why: the browser urlbar border colour is browser chrome stylesheet paint that the chunk neither changes nor checks.
@answer n=15 entity=color-palette#7.2 v=no
why: the browser chrome focus border colour is stylesheet paint; the chunk's focus checks read focus state, not border colour.
@answer n=16 entity=color-palette#7.3 v=no
why: the blitz-dom default input focus outline colour is paint; set_focus and the inverted pins assert that focus moves to the named input, not the colour of its outline.
@answer n=17 entity=color-palette#8.1 v=no
why: the default scrollbar thumb and stroke palette is blitz-paint's scrollbar painting; the chunk's programmatic scroll rule reads which boxes scroll and their offsets, not thumb colour.
@answer n=18 entity=color-palette#8.2 v=no
why: scrollbar-color resolution and author thumb blending are scrollbar paint the chunk neither calls nor changes; its scroll rule is about which boxes scroll, not their colour.
@answer n=19 entity=color-palette#8.3 v=no
why: unchecked radio ring and checkbox tick colours are form-control paint the chunk does not reach.
@answer n=20 entity=color-palette#8.4 v=no
why: the debug overlay content, padding, border and margin colours are a paint feature the chunk neither calls nor changes.
@answer n=21 entity=color-palette#8.5 v=no
why: the devtools layout stroke colours are a devtools paint feature outside the chunk's reach.
@answer n=22 entity=color-palette#8.6 v=no
why: the currentColor substitution applies to SVG attribute values in serialization; the chunk's snapshot re-check reads document structure and element ids, and no SVG is named in its scope.
@answer n=23 entity=color-palette#9.1 v=no
why: the rdme GitHub markdown colour tokens belong to the readme app, which the chunk does not touch.
@answer n=24 entity=color-palette#10.1 v=no
why: the recorded absence of colour tokens or custom properties in the engine is a fact about colour; the chunk adds no token or custom property.
@answer n=25 entity=color-palette#11.1 v=no
why: the clip-path fixture's 25-colour block palette is an example asset the chunk does not reach.
