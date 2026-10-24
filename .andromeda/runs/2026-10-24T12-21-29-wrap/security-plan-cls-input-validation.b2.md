# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · no · blitz-vibey-script is untouched by the take, and script execution is in the Untouched list
input-validation#1.27 · no · blitz-vibey-script is untouched by the take, so its type gate and cite hold
input-validation#1.28 · no · blitz-html's html_sink.rs is untouched by the take, so the scripting-flag cites hold
input-validation#1.29 · no · net.rs is untouched by the take, so the stylesheet UTF-8 cites hold
input-validation#1.30 · no · net.rs is untouched by the take, so the iframe decode cites hold
input-validation#1.31 · no · net.rs is untouched by the take, so the image decode cites hold
input-validation#1.32 · no · favicon.rs is untouched, and the apps/browser changes (mimalloc, Parallel) reach neither it nor its tests
input-validation#1.33 · no · net.rs is untouched, and the sniffed font source is in the Untouched list
input-validation#1.34 · no · util.rs is untouched by the take, so the font length and compression cites hold
input-validation#1.35 · no · net.rs is untouched, and the @font-face sniffed source is in the Untouched list
input-validation#1.36 · no · svg.rs is untouched by the take, so the SVG encoding and DTD cites hold
input-validation#1.37 · no · svg.rs is untouched by the take, so the SVGZ cites hold
input-validation#1.38 · no · svg.rs is untouched, and SVG viewBox handling is in the Untouched list
input-validation#1.39 · yes · stale · construct.rs gained lines at 6, 23 and 29, above line 470, so the parse_svg_image cite at 470-490 moves
input-validation#1.40 · no · resolved_style.rs and cssom.rs are untouched by the take
input-validation#1.41 · yes · stale · ElementData gained its classes field, with new and Clone keeping it, so the set_style_property and remove_style_property cites at 780-796 and 846-850 move
input-validation#1.42 · no · cssom.rs is untouched by the take, so the selectorText cites hold
input-validation#1.43 · no · query_selector.rs is untouched by the take, and the query-string parse reads no changed code
input-validation#1.44 · no · document.rs is untouched by the take, so the style-text decode cite holds
input-validation#1.45 · yes · stale · stylo.rs gained lines at 29-35 and 157-163, above these cites, so the bgcolor cites at 896-917 and 1184-1191 move
input-validation#1.46 · yes · stale · stylo.rs gained lines at 29-35 and 157-163, above these cites, so the dimension cites at 919-948 and 1058-1064 move
input-validation#1.47 · yes · stale · stylo.rs gained lines at 29-35 and 157-163, above these cites, so the SVG width and height cites at 950-980 move
input-validation#1.48 · yes · stale · stylo.rs gained lines at 29-35 and 157-163, above these cites, so the border and body margin cites at 1131 and 1165 move
input-validation#1.49 · no · layout/mod.rs gained lines only at 856-858, after the cites at 280-391, and table.rs is untouched
input-validation#1.50 · no · table.rs is untouched by the take, so the rowspan clamp cite holds
