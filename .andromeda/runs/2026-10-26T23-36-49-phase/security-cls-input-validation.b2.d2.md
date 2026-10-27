# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · the script fetch is blitz-vibey-script's ScriptFetcher, which the scope lists as not needed
input-validation#1.27 · no · the script type gate is blitz-vibey-script's script execution, which the scope lists as not needed
input-validation#1.28 · no · the scripting flag governs script execution, which the scope excludes, and the scope names no script
input-validation#1.29 · no · stylesheet decoding is listed not needed; the load page's one stylesheet is a fixed request LoadNet answers, which the chunk does not change
input-validation#1.30 · no · iframe HTML decoding is remote web content the scope lists as not needed, and the chunk fetches no iframe
input-validation#1.31 · no · image decoding is remote web content the scope lists as not needed, and the stand fetches no image
input-validation#1.32 · no · the favicon check lives in apps/browser, which the chunk does not reach
input-validation#1.33 · no · fetched-font sniffing is listed not needed; the chunk reads no fetched font, and the stand's bundled font takes no @font-face path
input-validation#1.34 · no · fetched-font length and WOFF fallback are listed not needed, and the stand's bundled font takes no fetch path
input-validation#1.35 · no · @font-face source handling is listed not needed, and no driver verb or stand page routes a URL through it
input-validation#1.36 · no · SVG data encoding and DTD parsing are listed not needed, and the chunk parses no SVG
input-validation#1.37 · no · SVGZ decompression is listed not needed (SVG), and the chunk reads no SVG data
input-validation#1.38 · no · SVG viewBox numbers are listed not needed (SVG), and the chunk changes no bounds the zero-size defect reads
input-validation#1.39 · no · inline SVG parsing and its logged parse error are listed not needed (SVG and engine error logging)
input-validation#1.40 · no · CSS declaration validation is the engine's cascade, which the chunk neither parses nor changes; the lint reads only the computed styles it yields
input-validation#1.41 · no · the scope names no style mutation, and the lint is read-only
input-validation#1.42 · no · the selectorText setter is a CSSOM mutation the scope does not name, and the chunk changes no engine code
input-validation#1.43 · no · the query-string parser is an engine selector API the scope does not name; the driver addresses targets by stable id
input-validation#1.44 · no · the style element's text entity decoding is the engine's CSS parse, which the scope does not name
input-validation#1.45 · no · the legacy bgcolor mapping is an engine style rule the scope does not name; the lint reads its result only through text_contrast
input-validation#1.46 · no · legacy width and height dimension parsing is an engine style rule the scope does not name
input-validation#1.47 · no · SVG width and height parsing is listed not needed (SVG)
input-validation#1.48 · no · legacy border and body-margin parsing is an engine style rule the scope does not name
input-validation#1.49 · no · table and replaced-element numeric attributes are engine layout fallbacks the scope does not name
input-validation#1.50 · no · rowspan clamping is a table-layout rule the scope does not name
