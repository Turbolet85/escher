# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · script fetch is script execution in blitz-vibey-script, which the task's Not needed list excludes and no reached file holds
input-validation#1.27 · no · the script `type` gate is blitz-vibey-script's, and script execution is in the Not needed list
input-validation#1.28 · no · the scripting flag is set in blitz-html's sink, outside the reached set, and the Needs name no script or noscript witness
input-validation#1.29 · no · stylesheet UTF-8 validation is a CSS input check on fetched content in net.rs, which the Not needed list excludes for offline fixtures
input-validation#1.30 · no · iframe HTML decoding is remote web content in net.rs, which the Not needed list excludes, and the Needs name no iframe witness
input-validation#1.31 · no · image format sniffing is an image input check on fetched content in net.rs, which the Not needed list excludes
input-validation#1.32 · no · the favicon check lives in the browser app, which the Not needed list excludes
input-validation#1.33 · no · font format sniffing is @font-face sniffing, which the Not needed list excludes, and the Needs name no font body
input-validation#1.34 · no · font length and WOFF fallback are font input checks in util.rs, which the Not needed list excludes
input-validation#1.35 · no · @font-face URL skipping and its ratified data-URL widening are remote-font behaviour the Not needed list excludes, and no witness the Needs name routes a URL through it
input-validation#1.36 · no · SVG encoding and DTD handling are SVG input checks, which the Not needed list excludes
input-validation#1.37 · no · SVGZ detection is an SVG input check, which the Not needed list excludes
input-validation#1.38 · no · SVG viewBox parsing is an SVG input check, which the Not needed list excludes
input-validation#1.39 · no · inline SVG parsing is an SVG input check, which the Not needed list excludes, and the Needs name no inline svg witness
input-validation#1.40 · no · CSS declaration validation in resolved_style.rs and cssom.rs is a CSS input check, and the RTL, vertical and transformed-box witnesses are the chunk's own valid CSS, which no invalid declaration reaches
input-validation#1.41 · no · set_style_property's rejection of invalid input is a CSS input check the Needs do not name, and the Needs name no invalid-value write
input-validation#1.42 · no · the selectorText setter is a CSSOM input check in cssom.rs, and the Needs name no selector-text write
input-validation#1.43 · no · query-string parsing is a selector input check in query_selector.rs, and the Needs name no query-string input the chunk supplies
input-validation#1.44 · no · style-text entity decoding in document.rs is a CSS text check, and the Needs name no entity-bearing style witness
input-validation#1.45 · no · bgcolor parsing in stylo.rs is a legacy-attribute read the Needs do not name, and the chunk adds tests, not engine code paths
input-validation#1.46 · no · legacy dimension parsing in stylo.rs is a legacy-attribute read the Needs do not name, and the Needs name no width or height attribute witness
input-validation#1.47 · no · SVG width and height parsing is an SVG input check, which the Not needed list excludes
input-validation#1.48 · no · border and body-margin integer parsing is a legacy-attribute read the Needs do not name, and the chunk adds tests, not engine code paths
input-validation#1.49 · no · markup numeric fallbacks in layout/mod.rs and layout/table.rs sit outside the reached set, and the Needs name no textarea, col or colspan witness
input-validation#1.50 · no · rowspan clamping is table layout in layout/table.rs, outside the reached set and an engine code path the chunk does not add
