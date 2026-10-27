# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · yes · the scope names the ScriptFetcher's file: narrowing, and this rule is the same fetcher's typed InvalidData for invalid data: URLs and non-UTF-8 script bytes, which the narrowing must not break
input-validation#1.27 · no · the script type gate lives in blitz-vibey-script document.rs, which is not reached; the runtime split moves runtime.rs and the scope names module eval logging, not this gate
input-validation#1.28 · no · the scripting flag lives in blitz-html html_sink.rs, which is not reached and not named by the scope
input-validation#1.29 · no · the stylesheet UTF-8 check lives in blitz-dom net.rs, which is not reached and not named by the scope
input-validation#1.30 · no · the iframe lossy decode lives in blitz-dom net.rs, which is not reached and not named by the scope
input-validation#1.31 · no · the image format decode lives in blitz-dom net.rs, which is not reached and not named by the scope
input-validation#1.32 · no · the favicon check lives in apps/browser favicon.rs, which is not on the reached list and not named by the scope
input-validation#1.33 · no · the font format sniff lives in blitz-dom net.rs, which is not reached; its named check runs only in the generic fast leg, not as a stated re-assertion
input-validation#1.34 · no · the font length and WOFF fallback live in blitz-dom util.rs, which is not reached and not named by the scope
input-validation#1.35 · no · the @font-face URL rule lives in blitz-dom net.rs, which is not reached; its named check runs only in the generic fast leg, and the stand boots on bundled fonts
input-validation#1.36 · no · the SVG encoding and DTD rule lives in blitz-dom svg.rs, which is not reached and not named by the scope
input-validation#1.37 · no · SVGZ detection lives in blitz-dom svg.rs, which is not reached and not named by the scope
input-validation#1.38 · yes · svg.rs is not reached, but the Needs' degenerate aspect-ratio discard acts on the zero-width-or-height flag this rule sets
input-validation#1.39 · yes · the rule lives in blitz-dom layout/construct.rs, a predicted conflict surface
input-validation#1.40 · yes · the scope names the CSS property accessors and cssText's never-throw, which this rule carries as invalid declarations ignored per CSSOM
input-validation#1.41 · yes · the rule lives in blitz-dom node/element.rs, which is reached whole
input-validation#1.42 · yes · the scope names selectorText assignment never throwing, which this setter's invalid-selector rule must keep
input-validation#1.43 · yes · the scope's native-API argument checks reach query strings, whose parse errors must stay returned
input-validation#1.44 · yes · the rule lives in blitz-dom document.rs, which is reached whole
input-validation#1.45 · yes · the rule lives in blitz-dom stylo.rs, which is reached whole
input-validation#1.46 · yes · the rule lives in blitz-dom stylo.rs, which is reached whole
input-validation#1.47 · yes · the rule lives in blitz-dom stylo.rs, which is reached whole
input-validation#1.48 · yes · the rule lives in blitz-dom stylo.rs, which is reached whole
input-validation#1.49 · no · the markup numeric parses live in layout/mod.rs and layout/table.rs, neither reached, and the layout bullet names aspect ratios and panic paths, not these parses
input-validation#1.50 · no · the rowspan clamp lives in layout/table.rs, which is not reached and not named by the scope
