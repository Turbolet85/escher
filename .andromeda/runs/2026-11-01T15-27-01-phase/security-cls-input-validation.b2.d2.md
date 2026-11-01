# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · blitz-shell's DataUriNetProvider is not the default fetcher, is not reached, and the scope names no shell data: behaviour
input-validation#1.27 · yes · the default fetcher's data: handling is named as recorded in the script runtime files, so its invalid-data: and non-UTF-8 script errors bind
input-validation#1.28 · yes · the default fetcher's file: narrowing is named as recorded for bootstrap.rs and events.rs; the file: load check and its witnesses bind
input-validation#1.29 · no · script type gating in document.rs is not reached and not named by the scope
input-validation#1.30 · no · the scripting flag in html_sink.rs is not reached and not named by the scope
input-validation#1.31 · yes · the fetched-stylesheet UTF-8 rule is named from f6edb9701 as applying to the new path
input-validation#1.32 · no · iframe decoding in blitz-dom net.rs is not reached and not named by the scope
input-validation#1.33 · no · image fetch decoding is not reached and not named; f6edb9701 covers stylesheets only
input-validation#1.34 · no · favicon decoding in the browser app is not reached, and the browser app is not needed
input-validation#1.35 · no · font format sniffing in net.rs is not reached and not named by the scope
input-validation#1.36 · no · font length and WOFF fallback in util.rs is not reached and not named by the scope
input-validation#1.37 · no · @font-face URL skipping in net.rs is not reached and not named; the ratified font-face widening is not one the scope names
input-validation#1.38 · no · SVG data encoding and DTD handling in svg.rs is not reached; the scope names SVG attributes, not the SVG data parser
input-validation#1.39 · no · SVGZ decompression in svg.rs is not reached and not named by the scope
input-validation#1.40 · yes · viewBox is an SVG attribute, and the scope names 8b1a0a3ce's SVG attributes under the clamped-numeric rule
input-validation#1.41 · yes · the inline SVG parse-error log and empty image data sit in construct.rs, a predicted conflict surface
input-validation#1.42 · yes · the remote-content boundary names declaration validation by parsing for page-reached paint properties, so invalid declarations ignored per CSSOM bind
input-validation#1.43 · yes · the setter validates declarations by parsing, the path the scope's declaration-validation rule covers for page-reached paint properties
input-validation#1.44 · no · the selectorText setter in cssom.rs is not reached and not named by the scope
input-validation#1.45 · no · query-selector parsing in query_selector.rs is not reached and not named by the scope
input-validation#1.46 · no · style-element entity decoding in document.rs is not reached and not named by the scope
input-validation#1.47 · yes · bgcolor parsing lives in stylo.rs, a predicted conflict surface
input-validation#1.48 · yes · dimension parsing lives in stylo.rs, a predicted conflict surface
input-validation#1.49 · yes · SVG width and height rejection lives in stylo.rs, a predicted conflict surface
input-validation#1.50 · yes · border and body margin parsing lives in stylo.rs, a predicted conflict surface
