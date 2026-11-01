# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · no · the data: URL scheme handling lives in packages/blitz-shell/src/net.rs, which the session did not reach
input-validation#1.27 · no · script fetch byte checks live in packages/blitz-vibey-script/src/fetch.rs, which the session did not reach
input-validation#1.28 · no · the file: URL gate sits in blitz-vibey-script fetch, modules, document and lib files, none of them reached
input-validation#1.29 · no · the script type check sits in packages/blitz-vibey-script/src/document.rs, which the session did not reach
input-validation#1.30 · no · the scripting flag sits in packages/blitz-html/src/html_sink.rs, which the session did not reach
input-validation#1.31 · yes · stale · packages/blitz-dom/src/net.rs is a merge surface reached whole, and the cited lines sit past the moved :144-147 region
input-validation#1.32 · yes · stale · packages/blitz-dom/src/net.rs is a merge surface reached whole, and the cited iframe decode at :552-555 can shift with the added lines
input-validation#1.33 · yes · stale · packages/blitz-dom/src/net.rs is a merge surface reached whole, and the image decode range :575-610 contains the moved :576-579 citation
input-validation#1.34 · no · the favicon check sits in apps/browser/src/favicon.rs, which the session did not reach
input-validation#1.35 · yes · stale · packages/blitz-dom/src/net.rs is a merge surface reached whole, and the cited sniff at :314-335 sits past the moved :144-147 region
input-validation#1.36 · no · the font length and compression check sits in packages/blitz-dom/src/util.rs, which the session did not reach
input-validation#1.37 · yes · stale · packages/blitz-dom/src/net.rs is a merge surface reached whole, and the cited font-face URL lines :462-466 and :498-504 can shift with the added lines
input-validation#1.38 · no · the SVG encoding and DTD check sits in packages/blitz-dom/src/node/svg.rs, which the session did not reach
input-validation#1.39 · no · the SVGZ check sits in packages/blitz-dom/src/node/svg.rs, which the session did not reach
input-validation#1.40 · no · the viewBox check sits in packages/blitz-dom/src/node/svg.rs, which the session did not reach
input-validation#1.41 · yes · stale · packages/blitz-dom/src/layout/construct.rs is a merge surface reached whole, and the cited inline SVG parse at :456-476 can shift with the moved lines
input-validation#1.42 · yes · add-to · F7: mix-blend-mode and isolation from page CSS now paint after Stylo's parse, which this validated-by-parsing row must carry, and resolved_style.rs is a merge surface
input-validation#1.43 · yes · stale · packages/blitz-dom/src/node/element.rs is a merge surface reached whole, and the cited set and remove style lines at :807-823 and :873-877 can shift
input-validation#1.44 · no · the selectorText check sits in packages/blitz-dom/src/cssom.rs, which the session did not reach
input-validation#1.45 · no · the query-string parse sits in packages/blitz-dom/src/query_selector.rs, which the session did not reach
input-validation#1.46 · yes · stale · packages/blitz-dom/src/document.rs is a merge surface reached whole, and the cited style text decode at :1685-1690 can shift with the added lines
input-validation#1.47 · yes · stale · packages/blitz-dom/src/stylo.rs is a merge surface reached whole, and the cited bgcolor lines at :906-927 and :1210-1217 can shift
input-validation#1.48 · yes · stale · packages/blitz-dom/src/stylo.rs is a merge surface reached whole, and the cited dimension lines at :929-958 and :1068-1074 can shift
input-validation#1.49 · yes · stale · packages/blitz-dom/src/stylo.rs is a merge surface reached whole, and the cited SVG width and height lines at :960-990 can shift
input-validation#1.50 · yes · stale · packages/blitz-dom/src/stylo.rs is a merge surface reached whole, and the cited border and margin lines at :1157 and :1191 can shift
