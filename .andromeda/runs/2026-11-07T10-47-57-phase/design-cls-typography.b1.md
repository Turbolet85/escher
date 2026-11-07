@answer n=1 entity=typography#1.1 v=no
why: browser chrome text sizes and families; the chunk changes mutation application, and bridge documents and ids do not depend on font values
@answer n=2 entity=typography#1.2 v=no
why: browser suggestion header style; no chrome text style is touched or measured by the chunk
@answer n=3 entity=typography#1.3 v=no
why: FPS overlay monospace font; the overlay is not reached by the buffered render
@answer n=4 entity=typography#1.4 v=no
why: about:history heading font; the bridge's documents are compared by identity, not by font
@answer n=5 entity=typography#1.5 v=no
why: rdme body font stack; the readme app is not reached by the mutation buffer change
@answer n=6 entity=typography#1.6 v=yes
why: the default system-ui and sans-serif family of text inputs is the family the chunk's text-input geometry and focus assertions are measured at (scope item 9)
@answer n=7 entity=typography#1.7 v=no
why: blitz-dom heading defaults; no heading geometry is asserted or re-pinned by the chunk
@answer n=8 entity=typography#1.8 v=no
why: code and pre default family; no code text is measured or re-asserted by the chunk
@answer n=9 entity=typography#1.9 v=no
why: select multiple row line-height; no select list is touched by the chunk
@answer n=10 entity=typography#1.10 v=no
why: todomvc example font stack; example CSS, and the chunk's documents are compared by identity not by font
@answer n=11 entity=typography#1.11 v=no
why: wgpu_texture example font stack; the example is not reached by the chunk
@answer n=12 entity=typography#1.12 v=yes
why: the 16px size text-input editors lay out at is the size the chunk's text-input geometry is measured at (scope item 9)
@answer n=13 entity=typography#1.13 v=no
why: generic base sizes; the text-input size is set by the editor row 1.12, so the generic mapping does not govern the measured inputs
@answer n=14 entity=typography#1.14 v=no
why: list bullet font stack; no list bullets are measured or re-asserted by the chunk
@answer n=15 entity=typography#2.1 v=yes
why: the WASM bundled DejaVu Sans registration is the default family the web-spa surface paints with, which scope item 5 re-asserts as unchanged in what it paints
@answer n=16 entity=typography#2.2 v=no
why: a pointer to the architecture section that holds the headless stand's font registration; it states no value or behaviour
@answer n=17 entity=typography#2.3 v=yes
why: on WASM a font context with bundled fonts must be provided, the mechanism that gives the web-spa surface the default family scope item 5 re-asserts
@answer n=18 entity=typography#2.4 v=no
why: lists dioxus-native's Cargo feature set; the chunk changes no feature, and the blitz-tests system-fonts need is row 2.8
@answer n=19 entity=typography#2.5 v=no
why: bullet font registration in the default font context; no bullet text is measured or re-asserted by the chunk
@answer n=20 entity=typography#2.6 v=no
why: the screenshot fonts-unavailable refusal; the screenshot path is not touched by the chunk (not-needed scope)
@answer n=21 entity=typography#2.7 v=no
why: the document.fonts stub in blitz-vibey-script; that file is not reached by the chunk
@answer n=22 entity=typography#2.8 v=yes
why: scope item 9 names the blitz-dom system-fonts dependency of blitz-tests text measurement and focus assertions, which the ported test and the inverted pins run under
@answer n=23 entity=typography#3.1 v=no
why: generic-family mapping to Parley generics; engine text mapping is excluded by the not-needed scope and the chunk does not change it
@answer n=24 entity=typography#3.2 v=no
why: font weight, width and style conversion; text style is excluded by the not-needed scope and the chunk does not change it
@answer n=25 entity=typography#3.3 v=no
why: OpenType feature tag mapping for font-variant properties; text style is excluded by the not-needed scope and the chunk does not change it
