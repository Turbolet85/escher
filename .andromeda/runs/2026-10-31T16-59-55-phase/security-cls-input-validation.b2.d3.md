# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · script fetch's invalid-data refusal; the list fetches nothing and adds no script path
input-validation#1.27 · no · file: URL script-load check; the list adds no script path and no fetch
input-validation#1.28 · no · script type attribute; not read by the list or its code
input-validation#1.29 · no · HTML scripting flag at parse; the list paints from the document already parsed and the scope names no parse behaviour
input-validation#1.30 · no · fetched stylesheet encoding; the chunk fetches no stylesheet
input-validation#1.31 · no · fetched iframe HTML decoding; the list fetches nothing
input-validation#1.32 · no · fetched image format; excluded by the scope's fetched-images exclusion
input-validation#1.33 · no · favicon image bytes; not part of the select
input-validation#1.34 · no · fetched font format sniffing; excluded by the scope's fetched-fonts exclusion, the list paints from fonts already loaded
input-validation#1.35 · no · font length and compression fallback; excluded by the scope's fetched-fonts exclusion
input-validation#1.36 · no · @font-face source URL; the chunk adds no URL and routes no agent- or user-supplied URL through this path
input-validation#1.37 · no · SVG encoding and DTD; no SVG is reached by the chunk
input-validation#1.38 · no · SVGZ decompression; no SVG is reached by the chunk
input-validation#1.39 · no · SVG viewBox numbers; no SVG is reached by the chunk
input-validation#1.40 · no · inline SVG parse; no SVG is reached by the chunk
input-validation#1.41 · yes · CSS property and value validation by parsing, invalid declarations ignored; the new engine colour declarations in assets/default.css must pass this rule like any other
input-validation#1.42 · no · set_style_property and remove_style_property return values; the scope names no style-setter call
input-validation#1.43 · no · selectorText setter; not reached by the chunk
input-validation#1.44 · no · query-string parsing and parse errors; the scope names no query call, and the stand's select#flight-type selector is unchanged
input-validation#1.45 · no · style element text entity decoding; not reached by the chunk
input-validation#1.46 · no · bgcolor legacy attribute; not reached by the chunk
input-validation#1.47 · no · legacy dimension attributes; not reached by the chunk
input-validation#1.48 · no · SVG width and height attributes; not reached by the chunk
input-validation#1.49 · no · border and body margin unsigned integers; not reached by the chunk
input-validation#1.50 · yes · markup numeric rule with a default fallback; the select's size is read by this chunk and falls under this parse with a default fallback
