# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · script fetch bytes are script execution, which the chunk leaves untouched (Not needed, script execution on Boa and the default ScriptFetcher); no Need names a data URL or script bytes
input-validation#1.27 · no · script type gating is script execution, untouched by the chunk (Not needed, script execution on Boa); no Need names a script element
input-validation#1.28 · no · the parser's scripting flag is HTML parsing, which the chunk does not change (Not needed, remote web content handling, no engine change)
input-validation#1.29 · no · stylesheet UTF-8 decoding is remote web content handling (Not needed, no engine change); the stand_lint re-read takes its test name alone, not a stylesheet load
input-validation#1.30 · no · iframe HTML decoding is remote web content handling (Not needed, subdocument caps, no engine change); no Need names an iframe
input-validation#1.31 · no · image format sniffing is remote web content handling (Not needed, no engine change); no Need decodes an image
input-validation#1.32 · no · favicon acceptance is apps/browser code, which no reached file touches and no Need names
input-validation#1.33 · no · fetched-font sniffing is remote web content handling (Not needed, fetched fonts); the bundled DejaVu Sans is not a fetched font, since the stand has no @font-face source (per input-validation#1.35), and the design-system row moves as prose only
input-validation#1.34 · no · font length and WOFF fallback are fetched-font handling (Not needed, fetched fonts, no engine change), and the chunk re-measures no font
input-validation#1.35 · no · the @font-face URL path is engine font fetching (Not needed, remote web content handling, no engine change); no Need names a font fetch or a URL argument
input-validation#1.36 · no · SVG encoding and DTD parsing is remote web content handling (Not needed, SVG, no engine change); no Need or reached unit names SVG
input-validation#1.37 · no · SVGZ decompression is remote web content handling (Not needed, SVG, no engine change); no Need or reached unit names SVG
input-validation#1.38 · no · the SVG viewBox check is engine SVG parsing (Not needed, SVG, no engine change); no Need or reached unit names SVG
input-validation#1.39 · no · inline SVG parsing is engine SVG handling (Not needed, SVG, no engine change); no reached unit or Need names an inline svg element
input-validation#1.40 · no · CSS declaration validation is CSS parsing (Not needed, remote web content handling, no engine change); no Need names a style property
input-validation#1.41 · no · the style setters are engine API that no Need or reached unit names, and the chunk changes no engine code
input-validation#1.42 · no · the selectorText setter is CSSOM engine code (Not needed, remote web content handling, no engine change); no Need names a rule
input-validation#1.43 · no · query-string parsing is engine selector handling, untouched; no Need or reached check names a query string
input-validation#1.44 · no · style element entity decoding is HTML parsing (Not needed, remote web content handling, no engine change)
input-validation#1.45 · no · bgcolor parsing is legacy styling in the engine (Not needed, remote web content handling, no engine change), and the chunk moves colour rows as prose and re-measures none
input-validation#1.46 · no · legacy dimension parsing is engine styling (Not needed, remote web content handling, no engine change); the stand_lint re-read takes its test name, not a layout
input-validation#1.47 · no · SVG width and height parsing is remote web content handling (Not needed, SVG, no engine change); no Need or reached unit names SVG
input-validation#1.48 · no · border and body margin parsing is engine styling (Not needed, remote web content handling, no engine change); no Need names one
input-validation#1.49 · no · markup numeric fallbacks are engine table and textarea layout (Not needed, remote web content handling, no engine change); no reached unit states a table or textarea value
input-validation#1.50 · no · rowspan clamping is engine table layout (Not needed, remote web content handling, no engine change); no reached unit states a rowspan
