@answer n=1 entity=input-validation#1.26 v=no
why: the data: URL net provider in blitz-shell is not reached; the chunk fetches nothing and the shell is untouched
@answer n=2 entity=input-validation#1.27 v=no
why: script fetch byte validation in blitz-vibey-script is not reached; the Dioxus bridge runs no document scripts
@answer n=3 entity=input-validation#1.28 v=no
why: the script-fetch file: URL gate is not reached; the bridge runs no document scripts and fetches nothing
@answer n=4 entity=input-validation#1.29 v=no
why: script type gating in blitz-vibey-script is not reached; the chunk names no script element
@answer n=5 entity=input-validation#1.30 v=yes
why: the scope names dangerous_inner_html applied with set_inner_html, a call whose HTML parse runs under this scripting-disabled sink, and the replayed call must still run in its original order
@answer n=6 entity=input-validation#1.31 v=no
why: fetched-stylesheet UTF-8 and failed-load handling is not reached; the chunk fetches nothing and names no stylesheet load
@answer n=7 entity=input-validation#1.32 v=no
why: stylesheet href resolution in blitz-dom's mutator is a component the chunk only uses; the scope names no href and the replay passes attribute values unchanged
@answer n=8 entity=input-validation#1.33 v=no
why: iframe HTML decoding is not reached; the chunk fetches no iframe content
@answer n=9 entity=input-validation#1.34 v=no
why: image decoding is not reached; the chunk fetches no images and names no image element
@answer n=10 entity=input-validation#1.35 v=no
why: favicon validation lives in apps/browser, which the chunk does not reach
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing is not reached; the chunk loads no fonts
@answer n=12 entity=input-validation#1.37 v=no
why: font length checks and the WOFF fallback are not reached; the chunk loads no fonts
@answer n=13 entity=input-validation#1.38 v=no
why: the @font-face source fetch path is not reached; the stand's boot-time stylesheet request is outside this rule and the chunk changes no font loading
@answer n=14 entity=input-validation#1.39 v=no
why: SVG encoding and DTD handling in blitz-dom's svg parser is not named by the scope and the replay passes no SVG data
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ detection and decompression is not named by the scope and no SVGZ data is replayed
@answer n=16 entity=input-validation#1.41 v=no
why: viewBox number validation is a blitz-dom parse the scope does not name; the replay passes attribute values unchanged
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parsing in layout construction is not named by the scope and the chunk changes no construction path
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation is a blitz-dom component the chunk only uses; the scope names no style call and the replay keeps attribute calls in order
@answer n=19 entity=input-validation#1.44 v=no
why: blend mode and isolation painting is not reached; the chunk changes no paint path and names no blend value
@answer n=20 entity=input-validation#1.45 v=no
why: set_style_property and remove_style_property are not named by the scope; the replay keeps attribute calls and passes no declarations
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter is CSSOM code the chunk does not reach
@answer n=22 entity=input-validation#1.47 v=no
why: query-string selector parsing is not named by the scope and the chunk adds no selector query
@answer n=23 entity=input-validation#1.48 v=no
why: style element entity decoding is an HTML parse path the scope does not name; the replay passes text values unchanged
@answer n=24 entity=input-validation#1.49 v=no
why: bgcolor parsing is a legacy-attribute mapping in blitz-dom the scope does not name; the replay passes attribute values unchanged
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension parsing is a Stylo mapping in blitz-dom the scope does not name; the replay passes attribute values unchanged
