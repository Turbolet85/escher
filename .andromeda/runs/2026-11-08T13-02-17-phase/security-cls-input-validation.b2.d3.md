@answer n=1 entity=input-validation#1.26 v=no
why: URL fragment matching in scrolling.rs; the chunk takes and matches no URL fragment.
@answer n=2 entity=input-validation#1.27 v=no
why: file: URL intake in the net provider is not_needed item 1; the read loads nothing.
@answer n=3 entity=input-validation#1.28 v=no
why: data: URL decoding in the net provider is not_needed item 1; the read loads nothing.
@answer n=4 entity=input-validation#1.29 v=no
why: data: URL handling in the shell net provider is not_needed item 1; the read loads nothing.
@answer n=5 entity=input-validation#1.30 v=no
why: script fetch byte checks are script execution, not_needed item 2; the read runs no script.
@answer n=6 entity=input-validation#1.31 v=no
why: file: URL loading by script is script execution, not_needed item 2; the chunk loads no script.
@answer n=7 entity=input-validation#1.32 v=no
why: script type attribute filtering is script execution, not_needed item 2.
@answer n=8 entity=input-validation#1.33 v=no
why: the HTML parser's scripting flag is an engine behaviour the scope does not name, and the chunk's fixtures run no script.
@answer n=9 entity=input-validation#1.34 v=no
why: fetched stylesheet UTF-8 handling is net provider intake, not_needed item 1; the read loads no stylesheet.
@answer n=10 entity=input-validation#1.35 v=no
why: stylesheet href resolution and render blocking is an engine rule the scope does not name; the read starts no load.
@answer n=11 entity=input-validation#1.36 v=no
why: iframe HTML decoding is remote content intake, not_needed item 1.
@answer n=12 entity=input-validation#1.37 v=no
why: fetched image decoding is remote content intake, not_needed item 1.
@answer n=13 entity=input-validation#1.38 v=no
why: favicon acceptance belongs to the browser app, not_needed item 3.
@answer n=14 entity=input-validation#1.39 v=no
why: font format sniffing of fetched fonts is net provider intake, not_needed item 1; the read loads no font.
@answer n=15 entity=input-validation#1.40 v=no
why: font length and WOFF fallback is net provider intake, not_needed item 1.
@answer n=16 entity=input-validation#1.41 v=no
why: @font-face source URL handling is net provider intake, and the chunk's verbs take no URL as the entity itself records.
@answer n=17 entity=input-validation#1.42 v=no
why: SVG encoding and DTD parsing is remote content intake and engine parsing, not_needed items 1 and 9.
@answer n=18 entity=input-validation#1.43 v=no
why: SVGZ decompression is remote content intake, not_needed item 1.
@answer n=19 entity=input-validation#1.44 v=no
why: SVG viewBox numbers are engine layout and paint, not_needed item 9.
@answer n=20 entity=input-validation#1.45 v=no
why: inline SVG parsing is engine markup handling the scope does not name, not_needed items 1 and 9.
@answer n=21 entity=input-validation#1.46 v=no
why: CSS declaration validation is engine CSS validation, not_needed item 9.
@answer n=22 entity=input-validation#1.47 v=no
why: mix-blend-mode and isolation painting is engine paint, not_needed item 9.
@answer n=23 entity=input-validation#1.48 v=no
why: set_style_property rejection is a DOM mutator the chunk neither calls nor names.
@answer n=24 entity=input-validation#1.49 v=no
why: the selectorText setter is CSSOM mutation the chunk neither calls nor names.
@answer n=25 entity=input-validation#1.50 v=no
why: query-string parsing in query_selector is an engine lookup the scope does not name, so it is a re-check every change owes.
