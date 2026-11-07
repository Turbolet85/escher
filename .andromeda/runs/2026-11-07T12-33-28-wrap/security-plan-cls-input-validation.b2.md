@answer n=1 entity=input-validation#1.26 v=no
why: DataUriNetProvider lives in packages/blitz-shell/src/net.rs, a file the change does not reach
@answer n=2 entity=input-validation#1.27 v=no
why: the script fetch's data: and UTF-8 checks live in packages/blitz-vibey-script/src/fetch.rs, which the change does not reach
@answer n=3 entity=input-validation#1.28 v=no
why: the file: URL gate (document_may_load and its three call sites) lives in blitz-vibey-script, which the change does not reach
@answer n=4 entity=input-validation#1.29 v=no
why: the script type allow-list lives in packages/blitz-vibey-script/src/document.rs, which the change does not reach
@answer n=5 entity=input-validation#1.30 v=no
why: the scripting-disabled parse lives in packages/blitz-html/src/html_sink.rs, which the change does not reach
@answer n=6 entity=input-validation#1.31 v=no
why: the stylesheet UTF-8 and render-blocking rules live in packages/blitz-dom/src/net.rs, which the change does not reach
@answer n=7 entity=input-validation#1.32 v=no
why: the stylesheet href resolution lives in packages/blitz-dom/src/mutator.rs, which the change does not reach
@answer n=8 entity=input-validation#1.33 v=no
why: the iframe from_utf8_lossy decode lives in packages/blitz-dom/src/net.rs, which the change does not reach
@answer n=9 entity=input-validation#1.34 v=no
why: the image format decode lives in packages/blitz-dom/src/net.rs, which the change does not reach
@answer n=10 entity=input-validation#1.35 v=no
why: the favicon byte check lives in apps/browser/src/favicon.rs, which the change does not reach
@answer n=11 entity=input-validation#1.36 v=no
why: the font format sniff lives in packages/blitz-dom/src/net.rs, which the change does not reach
@answer n=12 entity=input-validation#1.37 v=no
why: the font length and WOFF fallback live in packages/blitz-dom/src/util.rs, which the change does not reach
@answer n=13 entity=input-validation#1.38 v=no
why: the @font-face source URL skip lives in packages/blitz-dom/src/net.rs, which the change does not reach
@answer n=14 entity=input-validation#1.39 v=no
why: the SVG UTF-8 and DTD handling lives in packages/blitz-dom/src/node/svg.rs, which the change does not reach
@answer n=15 entity=input-validation#1.40 v=no
why: the SVGZ detection lives in packages/blitz-dom/src/node/svg.rs, which the change does not reach
@answer n=16 entity=input-validation#1.41 v=no
why: the viewBox number checks live in packages/blitz-dom/src/node/svg.rs, which the change does not reach
@answer n=17 entity=input-validation#1.42 v=no
why: the inline SVG parse lives in packages/blitz-dom/src/layout/construct.rs, which the change does not reach
@answer n=18 entity=input-validation#1.43 v=no
why: the CSS declaration validation lives in blitz-dom's resolved_style.rs and cssom.rs, which the change does not reach
@answer n=19 entity=input-validation#1.44 v=no
why: the mix-blend-mode and isolation handling lives in packages/blitz-paint, which the change does not reach
@answer n=20 entity=input-validation#1.45 v=no
why: set_style_property and remove_style_property live in packages/blitz-dom/src/node/element.rs, which the change does not reach
@answer n=21 entity=input-validation#1.46 v=no
why: the selectorText setter lives in packages/blitz-dom/src/cssom.rs, which the change does not reach
@answer n=22 entity=input-validation#1.47 v=no
why: the query-string parse lives in packages/blitz-dom/src/query_selector.rs, which the change does not reach
@answer n=23 entity=input-validation#1.48 v=no
why: the style element entity decode lives in packages/blitz-dom/src/document.rs, which the change does not reach
@answer n=24 entity=input-validation#1.49 v=no
why: the bgcolor parse lives in packages/blitz-dom/src/stylo.rs, which the change does not reach
@answer n=25 entity=input-validation#1.50 v=no
why: the legacy dimension parse lives in packages/blitz-dom/src/stylo.rs, which the change does not reach
