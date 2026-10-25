# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · script fetch bytes in blitz-vibey-script; the chunk touches no script path and the statement describes none
input-validation#1.27 · no · script type gating in blitz-vibey-script; script execution is not reached
input-validation#1.28 · no · the HTML parser's scripting flag; the chunk changes no parse path and the statement states no script behaviour
input-validation#1.29 · no · fetched stylesheet UTF-8 decoding; the remote-content fetch and parse paths are not reached
input-validation#1.30 · no · fetched iframe HTML decoding; the remote-content iframe path is not reached
input-validation#1.31 · no · fetched image format decoding; the remote-content image path is not reached
input-validation#1.32 · no · favicon bytes in apps/browser; the browser app is not touched
input-validation#1.33 · no · fetched font format sniffing; the statement's fonts-unavailable limit reads the font context's sans-serif family, not the fetch sniff
input-validation#1.34 · no · fetched font length and WOFF fallback; the remote-content font path is not reached
input-validation#1.35 · no · the @font-face fetch of a hint-less data: source; the chunk routes no URL and changes no fetch path, and the statement restates no URL rule
input-validation#1.36 · no · SVG encoding and DTD parsing in svg.rs; the SVG parse path is not reached and the statement describes no SVG
input-validation#1.37 · no · SVG gzip detection and decompression; the SVG parse path is not reached
input-validation#1.38 · no · SVG viewBox numbers; the SVG parse path is not reached
input-validation#1.39 · no · inline SVG parse in layout construction; layout construction is not reached and the statement describes no inline SVG
input-validation#1.40 · no · CSS declaration validation and CSSOM dropping; the CSS parse path is not reached and the statement states no declaration rule
input-validation#1.41 · no · set_style_property and remove_style_property rejections; no driver verb sets a style and the chunk touches no element setter
input-validation#1.42 · no · the selectorText setter in cssom.rs; CSSOM is not reached and no driver verb takes a selector
input-validation#1.43 · no · query-string parse errors; no driver verb takes a selector, every target being addressed by stable id
input-validation#1.44 · no · `<style>` text entity decoding; the CSS parse path is not reached
input-validation#1.45 · no · bgcolor legacy attribute parsing; legacy attributes are not reached and the statement names no bgcolor read
input-validation#1.46 · no · legacy dimension attributes through Stylo's parser; the statement states no table-width or table-cell rule behind its zero-size defect
input-validation#1.47 · no · SVG width and height legacy attributes; the SVG parse path is not reached
input-validation#1.48 · no · border and body margin parsed as unsigned integers; legacy-attribute parsing is not reached
input-validation#1.49 · no · markup numeric attributes with parse fallbacks in layout; layout is not reached and the statement names none of these attributes
input-validation#1.50 · no · rowspan clamping in table layout; table layout is not reached
