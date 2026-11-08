@answer n=1 entity=input-validation#1.26 v=no
why: data URL handling in the blitz-shell net provider is the not_needed net provider scope, and the graph fetches nothing
@answer n=2 entity=input-validation#1.27 v=no
why: script fetch bytes in blitz-vibey-script are the not_needed script execution scope, and the stand runs no document scripts
@answer n=3 entity=input-validation#1.28 v=no
why: file URL gating for script loads is the not_needed blitz-vibey-script scope, which the chunk does not reach
@answer n=4 entity=input-validation#1.29 v=no
why: script type filtering is the not_needed script types scope, and the chunk executes no script
@answer n=5 entity=input-validation#1.30 v=no
why: the HTML scripting flag is an engine parse behaviour the chunk only uses, and no scope clause names it or needs new code to meet it
@answer n=6 entity=input-validation#1.31 v=no
why: stylesheet encoding and render-blocking loads are the not_needed stylesheet fetch and engine rendering scope, and the chunk changes no engine parsing or rendering
@answer n=7 entity=input-validation#1.32 v=no
why: stylesheet href resolution in blitz-dom is engine fetch validation the not_needed scope excludes, and the graph reads no href
@answer n=8 entity=input-validation#1.33 v=no
why: iframe HTML decoding is an engine fetch path the chunk does not reach
@answer n=9 entity=input-validation#1.34 v=no
why: image format decoding is the not_needed image parsing scope, and the chunk reads no image
@answer n=10 entity=input-validation#1.35 v=no
why: favicon bytes belong to the browser app, which the not_needed browser scope excludes
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing is the not_needed font parsing scope, and the chunk fetches no font
@answer n=12 entity=input-validation#1.37 v=no
why: font length and WOFF decompression are the not_needed font parsing scope, and the chunk changes no font handling
@answer n=13 entity=input-validation#1.38 v=no
why: the font-face source URL path is engine font fetch the not_needed scope excludes, and no driver verb or agent-supplied URL reaches it
@answer n=14 entity=input-validation#1.39 v=no
why: SVG encoding and DTD parsing are the not_needed SVG parsing scope, and the chunk parses no SVG
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ decompression is the not_needed SVG parsing scope, and the chunk parses no SVG
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox numbers are the not_needed SVG parsing scope, and the chunk parses no SVG
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parsing in blitz-dom layout construction is the not_needed SVG and layout scope, which the chunk does not reach
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation is the not_needed CSS parsing scope, and the graph reads no style property
@answer n=19 entity=input-validation#1.44 v=no
why: mix-blend-mode and isolation paint is the not_needed engine rendering scope, and the chunk changes no paint
@answer n=20 entity=input-validation#1.45 v=no
why: set_style_property and remove_style_property are blitz-dom mutators the graph never calls, and the chunk writes no style
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter is a CSSOM mutator the graph never calls
@answer n=22 entity=input-validation#1.47 v=no
why: query string selector parsing is a blitz-dom query path the graph does not use, since it reads the vdom walk and the signals
@answer n=23 entity=input-validation#1.48 v=no
why: style element entity decoding is the not_needed CSS parsing scope, and the chunk changes no stylesheet parse
@answer n=24 entity=input-validation#1.49 v=no
why: bgcolor legacy attribute parsing is the not_needed CSS parsing scope and holds no graph-carried state or control value
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension attribute parsing is the not_needed CSS and layout scope, and the chunk changes no layout value
