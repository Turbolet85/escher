# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · no · the script fetch bytes rule lives in blitz-vibey-script, which the session did not reach and the report leaves untouched
input-validation#1.27 · no · the file: URL rule lives in blitz-vibey-script, not a reached file
input-validation#1.28 · no · the script type rule lives in blitz-vibey-script, not a reached file
input-validation#1.29 · no · the scripting flag lives in blitz-html, not a reached file
input-validation#1.30 · no · stylesheet encoding lives in blitz-dom net.rs, not reached; the report leaves the net rows untouched
input-validation#1.31 · no · iframe decoding lives in blitz-dom net.rs, not reached; the net rows are untouched
input-validation#1.32 · no · image format handling lives in blitz-dom net.rs, not reached; the net rows are untouched
input-validation#1.33 · no · favicon decoding lives in apps/browser, not a reached file
input-validation#1.34 · no · font sniffing lives in blitz-dom net.rs, not reached; the list fetches nothing
input-validation#1.35 · no · font length and WOFF fallback live in blitz-dom util.rs, not reached
input-validation#1.36 · no · @font-face URL handling lives in blitz-dom net.rs, not reached; the list fetches nothing
input-validation#1.37 · no · SVG encoding and DTD live in node/svg.rs, which is not a reached file
input-validation#1.38 · no · SVGZ detection lives in node/svg.rs, not a reached file
input-validation#1.39 · no · viewBox checks live in node/svg.rs, not a reached file
input-validation#1.40 · no · inline SVG parsing lives in layout/construct.rs, not a reached file
input-validation#1.41 · no · CSS declaration validation lives in resolved_style.rs and cssom.rs, neither a reached file
input-validation#1.42 · no · set_style_property lives in node/element.rs, not a reached file
input-validation#1.43 · no · selectorText setter lives in cssom.rs, not a reached file
input-validation#1.44 · no · query-string parsing lives in query_selector.rs, not a reached file
input-validation#1.45 · yes · stale · document.rs is reached whole and its additions at 761-1089 and later moved the cited lines 1663-1668 for the style text decoding
input-validation#1.46 · yes · stale · stylo.rs is reached whole and its additions sit below :446, so the cited :898-919 and :1202-1209 may have moved and the report does not show them holding
input-validation#1.47 · yes · stale · stylo.rs is reached whole and the cited :921-950 and :1060-1066 sit below its additions at :446, so they may have moved and are not shown to hold
input-validation#1.48 · yes · stale · stylo.rs is reached whole and the cited :952-982 sit below its additions at :446, so they may have moved and are not shown to hold
input-validation#1.49 · yes · stale · stylo.rs is reached whole and the cited :1149 and :1183 sit below its additions at :446, so they may have moved and are not shown to hold
input-validation#1.50 · yes · add-to · F8: a multiple select's size is read through parse_unsigned_integer, capped at 65534, and joins the markup numeric attributes row
