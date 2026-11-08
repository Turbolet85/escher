@answer n=1 entity=input-validation#1.26 v=no
why: blitz-dom fragment percent-decoding in scrolling is not reached by the read, its fixtures or any named verb; the scope names no fragment path.
@answer n=2 entity=input-validation#1.27 v=no
why: file: URL path handling in the net provider; the read loads nothing (task not_needed item 1).
@answer n=3 entity=input-validation#1.28 v=no
why: data: URL decoding in the net provider; the read loads nothing (task not_needed item 1).
@answer n=4 entity=input-validation#1.29 v=no
why: DataUriNetProvider in blitz-shell; the read loads nothing and the chunk adds no data: source (task not_needed item 1).
@answer n=5 entity=input-validation#1.30 v=no
why: script fetch bytes in blitz-vibey-script; script execution is outside the chunk (task not_needed item 2).
@answer n=6 entity=input-validation#1.31 v=no
why: script fetch of file: URLs by document; script loading is outside the chunk (task not_needed item 2).
@answer n=7 entity=input-validation#1.32 v=no
why: script type attribute gating; script execution is outside the chunk and the read runs no script (task not_needed item 2).
@answer n=8 entity=input-validation#1.33 v=no
why: HTML parsed with scripting disabled is a parser behaviour the scope does not name and the read's refusal and fixtures do not depend on.
@answer n=9 entity=input-validation#1.34 v=no
why: fetched stylesheet UTF-8 and render-blocking behaviour is an engine rule for every document; the read loads no stylesheet and the chunk makes no engine change (task item 13).
@answer n=10 entity=input-validation#1.35 v=no
why: stylesheet href resolution and non-blocking load is an engine rule; the chunk loads no stylesheet and makes no engine change (task item 13).
@answer n=11 entity=input-validation#1.36 v=no
why: fetched iframe HTML decoding; the read loads nothing and iframes are not reached (task not_needed item 1).
@answer n=12 entity=input-validation#1.37 v=no
why: fetched image format decoding; the read loads nothing (task not_needed item 1).
@answer n=13 entity=input-validation#1.38 v=no
why: favicon image validation in apps/browser; the browser app is outside the chunk (task not_needed item 3).
@answer n=14 entity=input-validation#1.39 v=no
why: font format sniffing for fetched fonts; the read loads no font and the fixtures name no @font-face source.
@answer n=15 entity=input-validation#1.40 v=no
why: font byte length and WOFF fallback in blitz-dom util; the read loads no font.
@answer n=16 entity=input-validation#1.41 v=no
why: @font-face source URL resolution and the ratified sniffing widening; no driver verb takes a URL and the scope names no font source.
@answer n=17 entity=input-validation#1.42 v=no
why: inline and fetched SVG encoding and DTD parsing; the read parses no SVG and the scope names none.
@answer n=18 entity=input-validation#1.43 v=no
why: SVGZ magic-byte decompression; the read parses no SVG and the scope names none.
@answer n=19 entity=input-validation#1.44 v=no
why: SVG viewBox number validation; the read parses no SVG and the scope names none.
@answer n=20 entity=input-validation#1.45 v=no
why: inline SVG parse-error logging in construct.rs; the chunk adds no SVG and no log line of that site.
@answer n=21 entity=input-validation#1.46 v=no
why: CSS declaration ignoring per CSSOM is a Stylo and blitz-dom behaviour the scope does not name; the covered and off-screen fixtures need only ordinary valid CSS.
@answer n=22 entity=input-validation#1.47 v=no
why: mix-blend-mode and isolation painting is an engine rule for every document; the read paints nothing and no fixture blends an element (task item 13).
@answer n=23 entity=input-validation#1.48 v=no
why: set_style_property and remove_style_property rejection is a DOM mutator behaviour; the read changes no style and runs no step.
@answer n=24 entity=input-validation#1.49 v=no
why: selectorText setter rejection is CSSOM rule editing the read never reaches.
@answer n=25 entity=input-validation#1.50 v=no
why: query selector parsing in query_selector.rs; the read and the stable element id rule take no selector string and the scope names none.
