# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · script fetching in blitz-vibey-script is Not needed (script execution and its ScriptFetcher), and the chunk runs no script
input-validation#1.27 · no · the script type gate is script execution in blitz-vibey-script, Not needed; the chunk executes no script
input-validation#1.28 · no · the parse-time scripting flag is script-execution territory, Not needed; the chunk changes no HTML parse
input-validation#1.29 · no · fetched-stylesheet UTF-8 decoding is stylesheet decoding, Not needed; the chunk changes no decode and reads no rejected body
input-validation#1.30 · no · iframe HTML decoding is remote web content handling, Not needed (iframe depth caps named); the chunk fetches no iframe
input-validation#1.31 · no · fetched-image decoding is image decoding, Not needed; the chunk decodes no image
input-validation#1.32 · no · the favicon check is apps/browser code, which the chunk does not touch
input-validation#1.33 · no · @font-face format sniffing is Not needed, its witness runs only in the standing fast gate, and the stand's bundled font is registered without a fetch
input-validation#1.34 · no · fetched-font length and decompression fallback is remote font handling, Not needed; the bundled font is not fetched, so the fallback is not read
input-validation#1.35 · yes · the stand's boot on its bundled font with no @font-face source binds, since stand_lint's rows and the fonts-unavailable precheck read that font; the sniffed-source widening and its witness are Not needed here
input-validation#1.36 · no · SVG encoding and DTD handling is SVG, Not needed; the chunk parses no SVG
input-validation#1.37 · no · SVGZ decompression is SVG, Not needed; the chunk decodes no SVG
input-validation#1.38 · no · the SVG viewBox check is SVG, Not needed; the lint's zero-size defect reads layout bounds, not a viewBox
input-validation#1.39 · no · inline SVG parsing and its logged parse error are SVG and engine error-logging, Not needed; the chunk adds no log site
input-validation#1.40 · no · CSS declaration validation is the engine cascade; the chunk reads computed styles and changes no cascade path
input-validation#1.41 · no · the DOM style setters are not called by the chunk, which reads styles and writes none
input-validation#1.42 · no · the CSSOM selectorText setter edits a stylesheet the chunk does not touch
input-validation#1.43 · no · query-string parsing is query_selector's, and the lint takes no query string
input-validation#1.44 · no · HTML-entity decoding of style text is stylesheet decoding, Not needed; the chunk changes no style text
input-validation#1.45 · no · the legacy bgcolor parse sits outside the lint's computed-style colour reads, and the task names no legacy colour attribute
input-validation#1.46 · no · the legacy dimension parse is outside the chunk, and the task names no legacy size attribute on a re-measured row
input-validation#1.47 · no · SVG width and height attributes are SVG, Not needed; the chunk parses no SVG
input-validation#1.48 · no · the legacy border and body-margin parse is layout the chunk does not change, and the task names no such attribute on a re-measured row
input-validation#1.49 · no · the default-fallback parses for textarea, replaced elements, col span and colspan are layout the chunk does not change, and the task names no such element on a re-measured row
input-validation#1.50 · no · the rowspan clamp is table layout the chunk does not change, and the task names no table among the re-measured rows
