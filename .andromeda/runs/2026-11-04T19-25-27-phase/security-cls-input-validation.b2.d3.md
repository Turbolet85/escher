@answer n=1 entity=input-validation#1.26 v=no
why: the data: URL net provider is the windowed shell's fetch path, and the probe fetches nothing remote (not-needed item 2)
@answer n=2 entity=input-validation#1.27 v=no
why: invalid data: URLs and non-UTF-8 script bytes belong to the script fetch, which the probe does not run (not-needed item 3)
@answer n=3 entity=input-validation#1.28 v=no
why: file: URL loading by script fetch, fetch(), module import and script src is document-script behaviour the probe does not run (not-needed item 3)
@answer n=4 entity=input-validation#1.29 v=no
why: the script type attribute gates only document scripts, which the probe does not run (not-needed item 3)
@answer n=5 entity=input-validation#1.30 v=no
why: the HTML parser's scripting flag affects document scripts, and the probe reads no parse verdict from it
@answer n=6 entity=input-validation#1.31 v=no
why: fetched stylesheet encoding and render blocking are named not needed (not-needed item 2), and the stand's one stylesheet is a fixed LoadNet answer the settle never waits on
@answer n=7 entity=input-validation#1.32 v=no
why: stylesheet href resolution and fetched stylesheets are named not needed (not-needed items 2 and 8), and the probe resolves no href
@answer n=8 entity=input-validation#1.33 v=no
why: fetched iframe HTML decoding is named not needed (not-needed item 2)
@answer n=9 entity=input-validation#1.34 v=no
why: fetched image format decoding is named not needed (not-needed item 2), and the probe loads no image
@answer n=10 entity=input-validation#1.35 v=no
why: favicon image validation belongs to the browser app, named not needed (not-needed item 2)
@answer n=11 entity=input-validation#1.36 v=no
why: fetched font format sniffing is named not needed (not-needed item 2), and the stand draws its bundled font without fetching one
@answer n=12 entity=input-validation#1.37 v=no
why: font length and failed-WOFF fallback apply to fetched font bytes, named not needed (not-needed item 2)
@answer n=13 entity=input-validation#1.38 v=yes
why: the row states the stand boots on its bundled font, the family the probe measures at, and holds the widening only while no agent- or user-supplied URL reaches font loading, a bound the probe's stand boot (stand::boot_load) and its no-URL-verb nothing-crosses rule must stay inside
@answer n=14 entity=input-validation#1.39 v=no
why: SVG data encoding and DTD handling are named not needed (not-needed item 2)
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ decompression is SVG data, named not needed (not-needed item 2)
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox number checks are SVG data, named not needed (not-needed item 2)
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parse and its engine log line are SVG data named not needed (not-needed item 2), and the probe adds no log field
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration parsing is named not needed and not read as a verdict (not-needed item 10)
@answer n=19 entity=input-validation#1.44 v=no
why: mix-blend-mode and isolation are a paint and CSS value path the probe neither changes nor reads (not-needed items 9 and 10)
@answer n=20 entity=input-validation#1.45 v=no
why: set_style_property and remove_style_property are style mutations the read-only probe never calls (needed item 1), and CSS parsing is named not needed (not-needed item 10)
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter is CSSOM rule mutation the probe never calls, and selector parsing for it is named not needed (not-needed item 10)
@answer n=22 entity=input-validation#1.47 v=yes
why: the stand harness's query helpers carry this parser and panic on an invalid selector (needed item 8), so the probe must pass only valid fixed selectors, and this row states the engine returns the parse error rather than raising it
@answer n=23 entity=input-validation#1.48 v=no
why: style element text entity decoding is named not needed and not read as a verdict (not-needed item 10)
@answer n=24 entity=input-validation#1.49 v=no
why: the bgcolor legacy attribute is legacy attribute parsing, named not needed (not-needed item 10)
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension attributes through Stylo's parser are markup numeric attributes and legacy parsing, named not needed (not-needed item 10)
