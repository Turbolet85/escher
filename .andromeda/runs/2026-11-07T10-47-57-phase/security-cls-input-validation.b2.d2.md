@answer n=1 entity=input-validation#1.26 v=no
why: data: URL handling in blitz-shell's net provider; the buffered render fetches nothing and the chunk does not reach it.
@answer n=2 entity=input-validation#1.27 v=no
why: script fetch byte decoding in blitz-vibey-script; the Dioxus bridge runs no document scripts.
@answer n=3 entity=input-validation#1.28 v=no
why: file: URL gating for script fetch in blitz-vibey-script; the bridge's render path does not load scripts.
@answer n=4 entity=input-validation#1.29 v=no
why: script type filter in blitz-vibey-script; the chunk executes no document scripts.
@answer n=5 entity=input-validation#1.30 v=no
why: HTML parsing scripting flag in blitz-html; the chunk parses no HTML.
@answer n=6 entity=input-validation#1.31 v=no
why: stylesheet UTF-8 and render-blocking rules in blitz-dom's net path; the chunk fetches nothing and changes no load.
@answer n=7 entity=input-validation#1.32 v=no
why: stylesheet href resolution in blitz-dom; the replay passes href through MutationWriter unchanged and the scope does not name this rule.
@answer n=8 entity=input-validation#1.33 v=no
why: iframe HTML lossy decoding in blitz-dom's net path; the chunk fetches no iframe.
@answer n=9 entity=input-validation#1.34 v=no
why: fetched image format handling in blitz-dom's net path; the chunk fetches no images.
@answer n=10 entity=input-validation#1.35 v=no
why: favicon validation in apps/browser; the scope excludes apps/browser.
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing in blitz-dom's net path; the chunk loads no fonts.
@answer n=12 entity=input-validation#1.37 v=no
why: font length and WOFF fallback in blitz-dom's util; the chunk touches no font bytes.
@answer n=13 entity=input-validation#1.38 v=no
why: @font-face source URL skipping in blitz-dom's net path; no URL the chunk handles reaches this path.
@answer n=14 entity=input-validation#1.39 v=no
why: SVG data UTF-8 and DTD parsing in blitz-dom; the chunk parses no SVG.
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ decompression in blitz-dom; the chunk parses no SVG data.
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox validation in blitz-dom; the chunk sets no viewBox and parses no SVG.
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parse in blitz-dom's layout construct; the replay carries no SVG image data the scope names.
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation in blitz-dom; the scope names only the four Dioxus attribute rules and the replay passes style values unchanged.
@answer n=19 entity=input-validation#1.44 v=no
why: mix-blend-mode and isolation painting in blitz-paint; the chunk paints nothing new and changes no paint path.
@answer n=20 entity=input-validation#1.45 v=no
why: set_style_property and remove_style_property return values in blitz-dom; the scope does not name them.
@answer n=21 entity=input-validation#1.46 v=no
why: selectorText setter in blitz-dom's cssom; the chunk uses no CSSOM.
@answer n=22 entity=input-validation#1.47 v=no
why: query-string parsing in blitz-dom's query_selector; the chunk runs no selector query.
@answer n=23 entity=input-validation#1.48 v=no
why: style element text entity decoding in blitz-dom; the replayed text passes through unchanged and the scope does not name this path.
@answer n=24 entity=input-validation#1.49 v=no
why: legacy bgcolor parsing in blitz-dom's stylo; the replay passes the value unchanged and the guarantee stays in blitz-dom's own code.
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension parsing in blitz-dom's stylo; the replay passes the value unchanged and the guarantee stays in blitz-dom's own code.
