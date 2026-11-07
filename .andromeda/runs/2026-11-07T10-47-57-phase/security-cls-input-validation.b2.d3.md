@answer n=1 entity=input-validation#1.26 v=no
why: data URL decoding in the blitz-shell net provider is not edited or reached, and the buffered render fetches nothing
@answer n=2 entity=input-validation#1.27 v=no
why: script fetch byte checks in blitz-vibey-script are not reached, since the Dioxus bridge runs no document scripts
@answer n=3 entity=input-validation#1.28 v=no
why: file URL gating for script fetch in blitz-vibey-script is not reached by the buffered render
@answer n=4 entity=input-validation#1.29 v=no
why: script type gating in blitz-vibey-script is not reached, since the bridge runs no document scripts
@answer n=5 entity=input-validation#1.30 v=no
why: HTML parsing with scripting disabled in blitz-html is not edited or called, because the replay creates nodes through MutationWriter and not the HTML parser
@answer n=6 entity=input-validation#1.31 v=no
why: stylesheet UTF-8 and render-blocking rules belong to the net path pinned by render_blocking_stylesheet, which the chunk does not re-run and does not fetch through
@answer n=7 entity=input-validation#1.32 v=no
why: stylesheet href resolution in the load path is not touched, and the chunk resolves no URL and adds no panic path
@answer n=8 entity=input-validation#1.33 v=no
why: iframe HTML decoding is a net-side fetch path that the buffered render does not reach
@answer n=9 entity=input-validation#1.34 v=no
why: image fetch decoding in blitz-dom net is not reached, since the buffered render fetches nothing
@answer n=10 entity=input-validation#1.35 v=no
why: favicon validation is in apps/browser, which the chunk does not reach
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing is a net-side fetch path that the buffered render does not reach
@answer n=12 entity=input-validation#1.37 v=no
why: font byte length and WOFF fallback in blitz-dom util are not reached, because the replay carries no font bytes
@answer n=13 entity=input-validation#1.38 v=no
why: the @font-face source URL widening is a separate engine change, no scope names font-face sources, and the stand_* witnesses do not re-run font_face_sniffed_source
@answer n=14 entity=input-validation#1.39 v=no
why: SVG encoding and DTD parsing in blitz-dom node/svg.rs is not edited or reached by the buffered render
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ detection and decompression in blitz-dom node/svg.rs is not edited or reached by the buffered render
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox number checks in blitz-dom node/svg.rs are not edited or reached by the buffered render
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parsing in blitz-dom construct.rs is not edited, and the replay passes element and attribute values through unchanged without naming an SVG path
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation is blitz-dom behaviour on a value the replay passes through unchanged, and the scope names no style-property call that the chunk makes or adds
@answer n=19 entity=input-validation#1.44 v=no
why: mix-blend-mode and isolation painting in blitz-paint is not edited or re-measured by the chunk, and the scope names no blend path
@answer n=20 entity=input-validation#1.45 v=no
why: the chunk calls no set_style_property or remove_style_property, the scope names none, and the replay keeps the existing MutationWriter path
@answer n=21 entity=input-validation#1.46 v=no
why: the CSSOM selectorText setter in blitz-dom cssom is not reached by the replay or by the probe tests
@answer n=22 entity=input-validation#1.47 v=no
why: query selector string parsing is not reached by the buffered render or by its tests
@answer n=23 entity=input-validation#1.48 v=no
why: entity decoding of style element text is a blitz-dom parse step on an element the replay creates, the chunk passes text unchanged and decodes nothing, and no scope names it
@answer n=24 entity=input-validation#1.49 v=no
why: legacy bgcolor parsing in blitz-dom stylo.rs sits downstream of the replayed set_attribute, a component the chunk uses and leaves unchanged, and the scope names no bgcolor or legacy attribute
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension parsing in blitz-dom stylo.rs sits downstream of the replayed set_attribute, a component the chunk uses and leaves unchanged, and the scope names no legacy dimension attribute
