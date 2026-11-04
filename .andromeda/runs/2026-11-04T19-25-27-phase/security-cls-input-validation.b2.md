@answer n=1 entity=input-validation#1.26 v=no
why: the shell's data: URL net provider is not reached, since the probe fetches nothing and the windowed shell is read only for its poll path
@answer n=2 entity=input-validation#1.27 v=no
why: invalid data: URL and script-byte handling is blitz-vibey-script fetch, excluded by the chunk's not-needed script execution scope
@answer n=3 entity=input-validation#1.28 v=no
why: the file: URL gate is a blitz-vibey-script fetch rule, excluded by the chunk's not-needed script execution scope, and the stand loads no file: document
@answer n=4 entity=input-validation#1.29 v=no
why: script type filtering is a blitz-vibey-script rule, excluded by the chunk's not-needed script execution scope
@answer n=5 entity=input-validation#1.30 v=no
why: the HTML scripting flag is a document-script behaviour the probe neither reads nor relies on
@answer n=6 entity=input-validation#1.31 v=no
why: failed-stylesheet render blocking is engine load accounting that the probe's settles never read, since they never wait on a load in flight, and the stand's one stylesheet is a fixed in-process UTF-8 answer
@answer n=7 entity=input-validation#1.32 v=no
why: an unresolvable stylesheet href is a page-CSS load path the probe never sets, and the chunk writes no href
@answer n=8 entity=input-validation#1.33 v=no
why: fetched iframe HTML decoding is remote-content boundary work, excluded by the chunk's not-needed remote content scope, and the probe loads no iframe
@answer n=9 entity=input-validation#1.34 v=no
why: fetched image decoding is remote-content boundary work, excluded by the chunk's not-needed remote content scope, and the probe fetches nothing
@answer n=10 entity=input-validation#1.35 v=no
why: favicon decoding is a browser-app path under apps/browser that the Dioxus probe on the stand never reaches
@answer n=11 entity=input-validation#1.36 v=no
why: font format sniffing applies to fetched fonts, the stand uses its bundled font, and the chunk neither reads nor re-asserts font_face_sniffed_source
@answer n=12 entity=input-validation#1.37 v=no
why: font length and WOFF fallback apply to fetched fonts only, which the stand's bundled font never passes through
@answer n=13 entity=input-validation#1.38 v=no
why: the ratified @font-face URL widening is reached by no agent- or user-supplied URL, and the chunk adds no verb and no @font-face source
@answer n=14 entity=input-validation#1.39 v=no
why: SVG data DTD parsing is not reached, since the probe's apps and stand render no SVG data
@answer n=15 entity=input-validation#1.40 v=no
why: SVGZ decompression is an SVG data path the probe does not reach
@answer n=16 entity=input-validation#1.41 v=no
why: SVG viewBox number checks are not reached, since the probe paints and reads no SVG
@answer n=17 entity=input-validation#1.42 v=no
why: inline SVG parsing is not reached, since the probe's apps contain no inline svg
@answer n=18 entity=input-validation#1.43 v=no
why: CSS declaration validation is excluded by the chunk's not-needed CSS parsing scope, and the probe reads no style as a verdict
@answer n=19 entity=input-validation#1.44 v=no
why: mix-blend-mode and isolation are paint-layer behaviour the probe does not measure, excluded by the chunk's not-needed engine layout and paint scope
@answer n=20 entity=input-validation#1.45 v=no
why: set_style_property and remove_style_property are writes the read-only probe never makes
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter is a CSSOM write the read-only probe never makes
@answer n=22 entity=input-validation#1.47 v=yes
why: the probe's query helpers pass fixed selectors through this parser, so the valid-selector bound the chunk must stay inside is what this entity states
@answer n=23 entity=input-validation#1.48 v=no
why: style element entity decoding is CSS and style-element parsing, excluded by the chunk's not-needed scope and not read by the probe as a verdict
@answer n=24 entity=input-validation#1.49 v=no
why: bgcolor legacy-attribute parsing is excluded by the chunk's not-needed legacy-attribute scope
@answer n=25 entity=input-validation#1.50 v=no
why: legacy dimension attributes are excluded by the chunk's not-needed markup numeric scope
