@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes are not changed by the buffered render and no chrome text is measured by the probe
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion header style is browser UI the chunk does not touch
@answer n=3 entity=typography#1.3 v=no
why: browser FPS overlay font is browser UI the chunk does not touch
@answer n=4 entity=typography#1.4 v=no
why: about:history heading style is a page the chunk does not render or measure
@answer n=5 entity=typography#1.5 v=no
why: readme body font belongs to the readme app, which the chunk does not touch
@answer n=6 entity=typography#1.6 v=yes
why: the default system-ui family of text inputs is the family the chunk's get_client_rect and focus assertions over text inputs are measured at
@answer n=7 entity=typography#1.7 v=no
why: default heading styles are not focused, measured or rendered differently by the chunk
@answer n=8 entity=typography#1.8 v=no
why: default code and pre family is not measured or focused by the chunk
@answer n=9 entity=typography#1.9 v=no
why: select multiple row line-height belongs to a select list, which the chunk does not touch
@answer n=10 entity=typography#1.10 v=no
why: todomvc styles belong to an example app the chunk does not touch or measure
@answer n=11 entity=typography#1.11 v=no
why: wgpu_texture styles belong to an example app the chunk does not touch or measure
@answer n=12 entity=typography#1.12 v=yes
why: the text input editor is created at 16.0, the size at which the text inputs the chunk measures are laid out
@answer n=13 entity=typography#1.13 v=no
why: the generic default size is overridden for text inputs by the explicit editor size in row 1.12, so the chunk does not measure at it
@answer n=14 entity=typography#1.14 v=no
why: list bullet font is a list the chunk does not measure or change
@answer n=15 entity=typography#2.1 v=no
why: wasm bundled font registration is set by the app, and the chunk's code neither registers nor reads fonts on wasm
@answer n=16 entity=typography#2.2 v=no
why: a pointer to the stand's font registration in architecture, and the chunk does not touch font_ctx
@answer n=17 entity=typography#2.3 v=no
why: wasm FontContext configuration is app-side, and the chunk's code does not configure fonts
@answer n=18 entity=typography#2.4 v=yes
why: the system-fonts feature named here is the one the blitz-tests text measurement the scope names depends on
@answer n=19 entity=typography#2.5 v=no
why: the bullet font registered in the default context is untouched by the chunk
@answer n=20 entity=typography#2.6 v=no
why: the screenshot fonts-unavailable refusal is a path the scope names as unchanged
@answer n=21 entity=typography#2.7 v=no
why: the document.fonts stub lives in the JS runtime bootstrap, which the chunk does not reach
@answer n=22 entity=typography#2.8 v=yes
why: the scope names the blitz-tests system-fonts dependency of get_client_rect and focus assertions over text inputs, so the ported test and inverted pins must run with the feature
@answer n=23 entity=typography#3.1 v=no
why: the generic-to-Parley mapping is engine-internal and the measured family is carried by row 1.6, so the chunk's code reads no mapping
@answer n=24 entity=typography#3.2 v=no
why: weight, width and style conversion changes no text style the chunk measures or asserts
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature tag mapping changes no text style the chunk measures or asserts
