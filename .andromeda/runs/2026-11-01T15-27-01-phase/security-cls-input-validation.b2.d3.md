# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · DataUriNetProvider in blitz-shell's net.rs is outside the reached files and the scope names no shell data-URL behaviour
input-validation#1.27 · yes · the scope names the default fetcher's `data:` narrowing, and this entity states its data: decoding and non-UTF-8 InvalidData rule
input-validation#1.28 · yes · the scope names the default fetcher's `file:` narrowing kept in bootstrap.rs and events.rs, witnessed by remote_document_cannot_load_file_urls and file_document_can_load_sibling_file_urls
input-validation#1.29 · no · the `type` attribute filter in document.rs is outside the reached files and the scope names no script-type rule
input-validation#1.30 · no · the scripting-disabled HTML parse in html_sink.rs is outside the reached files and the scope names no parser flag
input-validation#1.31 · yes · the scope names f6edb9701's fetched-stylesheet UTF-8 rule, which applies to the new path
input-validation#1.32 · no · iframe HTML lossy decoding in blitz-dom net.rs is not named by the scope
input-validation#1.33 · no · image format decoding in blitz-dom net.rs is not named by the scope
input-validation#1.34 · no · favicon byte validation in the browser app is not named by the scope, and no commit it names touches it
input-validation#1.35 · yes · the sniffed font-format rule is witnessed by font_face_sniffed_source, a proof the chunk re-runs under fork CI green
input-validation#1.36 · no · font length and WOFF fallback in util.rs is not named by the scope and has no witness it names
input-validation#1.37 · yes · the ratified hint-less `data:` @font-face widening and its witness font_face_sniffed_source, re-run under fork CI and the stand checks
input-validation#1.38 · no · SVG DTD and encoding parsing in svg.rs is not named by the scope
input-validation#1.39 · no · SVGZ decompression in svg.rs is not named by the scope
input-validation#1.40 · yes · the scope names the SVG attribute readers 8b1a0a3ce moves under the clamped-numeric rule, and the viewBox reader is one
input-validation#1.41 · yes · its rule lives in construct.rs, a predicted conflict surface
input-validation#1.42 · yes · the scope names declaration validation by parsing for page-reached CSS under the remote-content boundary
input-validation#1.43 · yes · the setter rejects invalid page-reached declarations, the declaration validation the remote-content boundary names
input-validation#1.44 · no · the selectorText setter in cssom.rs is not named by the scope, which names declarations only
input-validation#1.45 · no · query-string parsing in query_selector.rs is not named by the scope
input-validation#1.46 · no · `<style>` entity decoding in document.rs is not named by the scope, which names declaration validation only
input-validation#1.47 · yes · its rule lives in stylo.rs, a predicted conflict surface
input-validation#1.48 · yes · its rule lives in stylo.rs, a predicted conflict surface
input-validation#1.49 · yes · its rule lives in stylo.rs, a predicted conflict surface
input-validation#1.50 · yes · its rule lives in stylo.rs, a predicted conflict surface
