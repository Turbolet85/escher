# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · no · the byte check lives in packages/blitz-vibey-script/src/fetch.rs, which is not reached
input-validation#1.27 · yes · stale · packages/blitz-vibey-script/tests/dom.rs gained 40 added lines reaching :900-909, so its witness citations :944-968 and :970-990 can have moved
input-validation#1.28 · no · the type rule lives in packages/blitz-vibey-script/src/document.rs, which is not reached
input-validation#1.29 · no · the scripting flag lives in packages/blitz-html/src/html_sink.rs, which is not reached
input-validation#1.30 · no · the stylesheet UTF-8 rule lives in packages/blitz-dom/src/net.rs, which is not reached
input-validation#1.31 · no · the iframe decode lives in packages/blitz-dom/src/net.rs, which is not reached
input-validation#1.32 · no · the image decode lives in packages/blitz-dom/src/net.rs, which is not reached
input-validation#1.33 · no · the favicon check lives in apps/browser/src/favicon.rs, which is not reached
input-validation#1.34 · no · net.rs and its witness font_face_sniffed_source.rs are not among the changed files
input-validation#1.35 · no · the length and decompression fallback lives in packages/blitz-dom/src/util.rs, which is not reached
input-validation#1.36 · yes · stale · the edited check stand_load_page, missing from the reached list, covers the load page this row cites, and the session report does not show its fixed request holds
input-validation#1.37 · no · the encoding and DTD rule lives in packages/blitz-dom/src/node/svg.rs, which is not reached
input-validation#1.38 · no · the SVGZ rule lives in packages/blitz-dom/src/node/svg.rs, which is not reached
input-validation#1.39 · no · the viewBox rule lives in packages/blitz-dom/src/node/svg.rs, which is not reached
input-validation#1.40 · no · the inline SVG parse lives in packages/blitz-dom/src/layout/construct.rs, which is not reached
input-validation#1.41 · no · the declaration parse lives in packages/blitz-dom/src/resolved_style.rs and cssom.rs, which are not reached
input-validation#1.42 · no · the setter lives in packages/blitz-dom/src/node/element.rs, which is not reached
input-validation#1.43 · no · the selectorText rule lives in packages/blitz-dom/src/cssom.rs, which is not reached
input-validation#1.44 · no · the query parse lives in packages/blitz-dom/src/query_selector.rs, which is not reached
input-validation#1.45 · yes · stale · packages/blitz-dom/src/document.rs was edited and is reached whole, so the style-text citation :1327-1332 can have moved and the session report does not show it holds
input-validation#1.46 · no · the bgcolor rule lives in packages/blitz-dom/src/stylo.rs, which is not reached
input-validation#1.47 · no · the dimension parse lives in packages/blitz-dom/src/stylo.rs, which is not reached
input-validation#1.48 · no · the SVG width and height parse lives in packages/blitz-dom/src/stylo.rs, which is not reached
input-validation#1.49 · no · the border and body margin parse lives in packages/blitz-dom/src/stylo.rs, which is not reached
input-validation#1.50 · no · the numeric attribute fallbacks live in packages/blitz-dom/src/layout/mod.rs and table.rs, which are not reached
