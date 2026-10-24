# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · no · the script fetch rule lives in blitz-vibey-script's fetch.rs, which the session did not reach
input-validation#1.27 · no · the script type gate lives in blitz-vibey-script's document.rs, which the session did not reach
input-validation#1.28 · no · the scripting-disabled parse lives in blitz-html's html_sink.rs, which the session did not reach
input-validation#1.29 · no · the stylesheet UTF-8 check lives in blitz-dom's net.rs, which the session did not reach
input-validation#1.30 · no · the iframe lossy decode lives in blitz-dom's net.rs, which the session did not reach
input-validation#1.31 · no · the image format path lives in blitz-dom's net.rs, which the session did not reach
input-validation#1.32 · no · the favicon check lives in apps/browser's favicon.rs, which the session did not reach
input-validation#1.33 · no · the font sniff lives in blitz-dom's net.rs and its witness in font_face_sniffed_source.rs, neither reached
input-validation#1.34 · no · the font length and WOFF fallback live in blitz-dom's util.rs, which the session did not reach
input-validation#1.35 · no · the @font-face URL path (net.rs) and its witness (font_face_sniffed_source.rs) are unreached, and the session adds no verb, so no driver verb takes a URL
input-validation#1.36 · no · the SVG encoding and DTD rule lives in blitz-dom's node/svg.rs, which the session did not reach
input-validation#1.37 · no · the SVGZ detection lives in blitz-dom's node/svg.rs, which the session did not reach
input-validation#1.38 · no · the viewBox check lives in blitz-dom's node/svg.rs, which the session did not reach
input-validation#1.39 · no · the inline SVG parse lives in blitz-dom's layout/construct.rs, which the session did not reach
input-validation#1.40 · no · the CSS declaration rule lives in blitz-dom's resolved_style.rs and cssom.rs, neither reached
input-validation#1.41 · no · the style setter rule lives in blitz-dom's node/element.rs, which the session did not reach
input-validation#1.42 · no · the selectorText rule lives in blitz-dom's cssom.rs, which the session did not reach
input-validation#1.43 · no · the query-string parse lives in blitz-dom's query_selector.rs, which the session did not reach
input-validation#1.44 · no · the style-element entity decode lives in blitz-dom's document.rs, which the session did not reach
input-validation#1.45 · no · the bgcolor check lives in blitz-dom's stylo.rs, which the session did not reach
input-validation#1.46 · no · the dimension parse lives in blitz-dom's stylo.rs, which the session did not reach
input-validation#1.47 · no · the SVG width and height rule lives in blitz-dom's stylo.rs, which the session did not reach
input-validation#1.48 · no · the border and body-margin parse lives in blitz-dom's stylo.rs, which the session did not reach
input-validation#1.49 · no · the markup numeric fallbacks live in blitz-dom's layout/mod.rs and layout/table.rs, neither reached
input-validation#1.50 · no · the rowspan clamp lives in blitz-dom's layout/table.rs, which the session did not reach
