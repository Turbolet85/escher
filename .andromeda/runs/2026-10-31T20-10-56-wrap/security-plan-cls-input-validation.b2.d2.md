# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · no · script-fetch bytes check lives in blitz-vibey-script, not a reached file, and the script rows are untouched
input-validation#1.27 · no · file: URL script fetch lives in blitz-vibey-script, not a reached file, and the list adds no script path
input-validation#1.28 · no · script type check lives in blitz-vibey-script/src/document.rs, not a reached file
input-validation#1.29 · no · scripting flag lives in blitz-html html_sink.rs, not a reached file
input-validation#1.30 · no · fetched-stylesheet UTF-8 check lives in blitz-dom net.rs, not a reached file, and the list fetches nothing
input-validation#1.31 · no · iframe decoding lives in blitz-dom net.rs, not a reached file, and the list fetches nothing
input-validation#1.32 · no · image format handling lives in blitz-dom net.rs, not a reached file, and no fetch is added
input-validation#1.33 · no · favicon check lives in apps/browser, not a reached file
input-validation#1.34 · no · font sniffing lives in blitz-dom net.rs, not a reached file, and blitz-net is not reached
input-validation#1.35 · no · font length and compression live in blitz-dom util.rs, not a reached file
input-validation#1.36 · no · the font-face URL path lives in net.rs, not a reached file; the driver schema keeps twelve verbs and seven argument kinds with none taking a URL, so the claim holds on the changed schema.rs
input-validation#1.37 · no · SVG encoding and DTD parsing live in blitz-dom svg.rs, not a reached file
input-validation#1.38 · no · SVGZ handling lives in blitz-dom svg.rs, not a reached file
input-validation#1.39 · no · viewBox checks live in blitz-dom svg.rs, not a reached file
input-validation#1.40 · no · inline SVG parsing lives in layout/construct.rs, not a reached file
input-validation#1.41 · no · CSS declaration parsing lives in resolved_style.rs and cssom.rs, neither a reached file
input-validation#1.42 · no · set_style_property lives in blitz-dom element.rs, not a reached file
input-validation#1.43 · no · selectorText setter lives in blitz-dom cssom.rs, not a reached file
input-validation#1.44 · no · query-string parsing lives in blitz-dom query_selector.rs, not a reached file
input-validation#1.45 · yes · stale · document.rs is a reached file, and lines added at 761-1089 and later move the `<style>` decoding citation at :1663-1668, which the report does not list as holding
input-validation#1.46 · yes · stale · stylo.rs is a reached file with additions below line 446, so the bgcolor citations at :898-919 and :1202-1209 may have moved and the report does not show them holding
input-validation#1.47 · yes · stale · stylo.rs is a reached file with additions below line 446, so the dimension citations at :921-950 and :1060-1066 may have moved and the report does not show them holding
input-validation#1.48 · yes · stale · stylo.rs is a reached file with additions below line 446, so the SVG width/height citation at :952-982 may have moved and the report does not show it holding
input-validation#1.49 · yes · stale · stylo.rs is a reached file with additions below line 446, so the border and body-margin citations at :1149 and :1183 may have moved and the report does not show them holding
input-validation#1.50 · yes · add-to · F8: `size` on a `multiple` select, parsed by parse_unsigned_integer and capped at 65534, joins the Markup numeric attributes row
