# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · the script runtime's fetcher is not reached, the list adds no script path
input-validation#1.27 · no · the file: URL check of the script fetch, which the list does not reach since it fetches nothing
input-validation#1.28 · no · the script type attribute governs script execution, outside the list
input-validation#1.29 · no · the HTML parser's scripting flag, outside the list's code
input-validation#1.30 · no · fetched stylesheet encoding, the list fetches nothing and default.css is the engine's own, not fetched
input-validation#1.31 · no · fetched iframe HTML decoding, outside the list
input-validation#1.32 · no · fetched image format, the list paints option text and no images
input-validation#1.33 · no · favicon bytes in apps/browser, outside the select
input-validation#1.34 · no · fetched font format sniffing in net.rs, which the chunk does not change and whose witness is a re-check
input-validation#1.35 · no · fetched font length and compression, which the chunk does not change
input-validation#1.36 · no · the @font-face URL path stays as ratified, the chunk adds no URL route and the list fetches nothing
input-validation#1.37 · no · SVG data encoding and DTD, outside the list
input-validation#1.38 · no · SVGZ decompression, outside the list
input-validation#1.39 · no · SVG viewBox numbers, outside the list
input-validation#1.40 · no · inline SVG parse, outside the list
input-validation#1.41 · yes · the new default-stylesheet colour declarations in default.css must pass CSS parsing, with invalid declarations ignored per CSSOM
input-validation#1.42 · no · set_style_property and remove_style_property are not named by the scope and the chunk's code does not call them
input-validation#1.43 · no · the selectorText setter, the chunk edits no selector
input-validation#1.44 · no · query-string parsing, not named by the scope
input-validation#1.45 · no · style element text entity decoding, outside the list
input-validation#1.46 · no · the bgcolor legacy attribute, not read by the list
input-validation#1.47 · no · the legacy HTML dimension attributes, not read by the list
input-validation#1.48 · no · the SVG width and height legacy attributes, outside the list
input-validation#1.49 · no · border and body margin integers, outside the list
input-validation#1.50 · yes · the select's size read, which the chunk adds, must parse with a default fallback and never panic
