@answer n=1 entity=input-validation#1.26 v=no
why: the data URL handling of the blitz-shell net provider is not reached, since the chunk fetches nothing
@answer n=2 entity=input-validation#1.27 v=no
why: script fetch of data URLs belongs to blitz-vibey-script, and the stand runs no document scripts
@answer n=3 entity=input-validation#1.28 v=no
why: the file URL gate of the script fetcher is script execution the stand never runs
@answer n=4 entity=input-validation#1.29 v=no
why: script type gating in blitz-vibey-script is script execution the stand never runs
@answer n=5 entity=input-validation#1.30 v=no
why: HTML parsing with scripting disabled is not on the chunk path, since the Dioxus stand builds a vdom and parses no HTML
@answer n=6 entity=input-validation#1.31 v=no
why: fetched stylesheet UTF-8 and render-blocking behaviour is engine stylesheet fetch validation that the chunk does not change or reach
@answer n=7 entity=input-validation#1.32 v=no
why: stylesheet href resolution in blitz-dom starts no load in the chunk, which fetches nothing and edits no engine code
@answer n=8 entity=input-validation#1.33 v=no
why: iframe HTML decoding in blitz-dom runs on a fetch the chunk never makes
@answer n=9 entity=input-validation#1.34 v=no
why: fetched image format decoding is image parsing that the chunk does not change or reach
@answer n=10 entity=input-validation#1.35 v=no
why: favicon image validation lives in apps/browser, which the chunk does not touch
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing of fetched fonts is font parsing that the chunk does not change or reach
@answer n=12 entity=input-validation#1.37 v=no
why: font length and WOFF decompression fallback is font parsing that the chunk does not change or reach
@answer n=13 entity=input-validation#1.38 v=no
why: the ratified font-face sniffing path is an upstream font fetch, and the scope gives the new verb bounded argument kinds and no URL, so no chunk path reaches it
@answer n=14 entity=input-validation#1.39 v=no
why: SVG encoding and DTD parsing in blitz-dom is SVG parsing that the chunk does not change or reach
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ decompression is SVG parsing that the chunk does not change or reach
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox number validation is SVG parsing that the chunk does not change or reach
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parsing in blitz-dom layout is engine parsing that the chunk does not change or reach
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation in blitz-dom is CSS parsing that the chunk does not change, and the chunk adds no style write
@answer n=19 entity=input-validation#1.44 v=no
why: mix-blend-mode and isolation painting is engine rendering that the chunk does not change or reach
@answer n=20 entity=input-validation#1.45 v=no
why: the style write rejection path is engine CSS parsing that the chunk does not change, and the chunk adds no style write
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter in blitz-dom cssom is CSS selector handling that the chunk does not call or change
@answer n=22 entity=input-validation#1.47 v=no
why: query string selector parsing in blitz-dom is selector handling that the chunk graph read does not use
@answer n=23 entity=input-validation#1.48 v=no
why: style element entity decoding is CSS parsing that the chunk does not change or reach
@answer n=24 entity=input-validation#1.49 v=no
why: legacy bgcolor parsing is CSS value parsing that the chunk does not change or reach, and bgcolor carries no masked value
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension parsing in blitz-dom stylo is CSS value parsing that the chunk does not change or reach
