# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · the script fetcher is a Not-needed surface; no delta commit targets script execution, and the Stylo manifest move reaches only the manifest
input-validation#1.27 · no · the script type gate is a Not-needed surface; no delta commit targets script execution
input-validation#1.28 · no · html_sink.rs is in no take file; the coupled html5ever pin is a manifest version, not the parser's scripting setting
input-validation#1.29 · no · net.rs is in no take file; no Need or delta commit names stylesheet fetching or its UTF-8 check
input-validation#1.30 · no · net.rs's lossy iframe decode is in no take file; the iframe Needs are the recursion caps, the stale-response discard and mutator.rs's src skip
input-validation#1.31 · no · net.rs's image decode is in no take file; no Need or delta commit names image decoding
input-validation#1.32 · no · favicon.rs is in no take file; mimalloc changes apps/browser's allocator, not the bytes it accepts
input-validation#1.33 · no · net.rs's font sniff is in no take file; no Need or delta commit adds or changes a font input path
input-validation#1.34 · no · util.rs's font fallback is in no take file; no Need or delta commit names font decompression
input-validation#1.35 · no · net.rs is in no take file; the boundary-widening Need reaches input paths this merge adds, and the task names none
input-validation#1.36 · no · node/svg.rs is in no take file; no Need or delta commit names SVG decoding (its inline caller is 1.39)
input-validation#1.37 · no · node/svg.rs is in no take file; no Need or delta commit names SVGZ decompression
input-validation#1.38 · no · node/svg.rs is in no take file; no Need or delta commit names viewBox parsing
input-validation#1.39 · yes · layout/construct.rs is named under the remote-content and graceful-degradation Needs; its inline SVG parse failure stays a typed fallback, logged within the sink's allowlist, with no new panic path
input-validation#1.40 · no · resolved_style.rs and cssom.rs are in no take file; no Need or delta commit names CSS declaration validation
input-validation#1.41 · no · element.rs is taken for class atoms, but the style setter's rejection is not among the element.rs readers the Needs keep
input-validation#1.42 · no · cssom.rs is in no take file; no Need or delta commit names the selectorText setter
input-validation#1.43 · no · query_selector.rs is in no take file; no Need or delta commit names selector-string parsing
input-validation#1.44 · no · document.rs is taken for the clock and its markup-attribute readers, but the <style> text decode is neither
input-validation#1.45 · no · stylo.rs is named in neither the delta nor the Needs, so this legacy bgcolor reader is not reached
input-validation#1.46 · no · stylo.rs is named in neither the delta nor the Needs, so this legacy dimension reader is not reached
input-validation#1.47 · no · stylo.rs is named in neither the delta nor the Needs, so these legacy SVG size readers are not reached
input-validation#1.48 · no · stylo.rs is named in neither the delta nor the Needs, so these legacy border and body margin readers are not reached
input-validation#1.49 · yes · layout/mod.rs is reached by the aspect-ratio natural-height change and the Stylo calc-pointer item, so its replaced-element width/height reads must keep their .parse().ok() fallbacks
input-validation#1.50 · no · layout/table.rs is reached only through the Stylo calc-pointer blocks, and the rowspan clamp is not among them
