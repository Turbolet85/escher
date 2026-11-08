@answer n=1 entity=input-validation#1.26 v=no
why: the shell's data: URL net provider is not reached, and the graph fetches nothing (not_needed 2)
@answer n=2 entity=input-validation#1.27 v=no
why: script fetching belongs to blitz-vibey-script, and the stand runs no document scripts (not_needed 3)
@answer n=3 entity=input-validation#1.28 v=no
why: the file: URL script fetcher in blitz-vibey-script is not reached (not_needed 3)
@answer n=4 entity=input-validation#1.29 v=no
why: script type gating in blitz-vibey-script is not reached, and the stand runs no document scripts (not_needed 3)
@answer n=5 entity=input-validation#1.30 v=no
why: blitz-html's scripting flag is not reached, and the chunk changes no HTML parsing (not_needed 3, 7)
@answer n=6 entity=input-validation#1.31 v=no
why: blitz-dom stylesheet encoding and render-blocking is engine behaviour the chunk neither changes nor reads (carve-out 17, not_needed 7)
@answer n=7 entity=input-validation#1.32 v=no
why: blitz-dom stylesheet href resolution is not reached, and the chunk makes no engine change (carve-out 17)
@answer n=8 entity=input-validation#1.33 v=no
why: iframe HTML decoding in blitz-dom net is not reached, and the chunk changes no engine fetching (not_needed 2, 7)
@answer n=9 entity=input-validation#1.34 v=no
why: image format decoding in blitz-dom net is not reached, and the chunk changes no image parsing (not_needed 7)
@answer n=10 entity=input-validation#1.35 v=no
why: favicon validation is the browser app's, which the graph does not touch (not_needed 1)
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing in blitz-dom is not reached, and the chunk changes no font parsing (not_needed 7)
@answer n=12 entity=input-validation#1.37 v=no
why: font length and WOFF fallback in blitz-dom util is not reached (not_needed 7)
@answer n=13 entity=input-validation#1.38 v=yes
why: the new verb's arguments must stay URL-free, because the ratified @font-face widening is safe only while no driver verb takes a URL (needed 1)
@answer n=14 entity=input-validation#1.39 v=no
why: SVG DTD parsing in blitz-dom is not reached, and the chunk changes no SVG parsing (not_needed 7)
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ decompression in blitz-dom is not reached (not_needed 7)
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox number checks in blitz-dom are not reached (not_needed 7)
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parsing in blitz-dom layout construct is not reached (not_needed 7)
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation in blitz-dom is not reached, and the chunk changes no CSS parsing (not_needed 7)
@answer n=19 entity=input-validation#1.44 v=no
why: blend-mode painting is blitz-paint's and would be an engine change for every document, which the scope expects none of (carve-out 17)
@answer n=20 entity=input-validation#1.45 v=no
why: the style property setters in blitz-dom are not changed by the chunk, which touches no CSS setting path (not_needed 7)
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter in blitz-dom cssom is not reached (not_needed 7)
@answer n=22 entity=input-validation#1.47 v=no
why: query-string selector parsing is not reached, since the graph walks the vdom through dioxus-core and not through selectors (needed 16)
@answer n=23 entity=input-validation#1.48 v=no
why: style element text decoding in blitz-dom is not reached (not_needed 7)
@answer n=24 entity=input-validation#1.49 v=no
why: legacy bgcolor parsing in blitz-dom stylo is not reached (not_needed 7)
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension parsing in blitz-dom stylo is not reached (not_needed 7)
