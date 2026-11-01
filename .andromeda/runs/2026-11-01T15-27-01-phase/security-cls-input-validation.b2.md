# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · no · DataUriNetProvider sits in blitz-shell's net.rs, which no reached file names and the scope does not name
input-validation#1.27 · yes · the default fetcher's data: and non-UTF-8 script byte handling, which the scope keeps as recorded under the script runtime's file: and data: narrowing
input-validation#1.28 · yes · the default fetcher's file: narrowing and the file-document rule the scope keeps as recorded, with the fetch paths a new script or fetch path must not widen
input-validation#1.29 · no · the type-attribute gate sits in vibey-script's document.rs, which no reached file names and the scope does not change
input-validation#1.30 · no · scripting-disabled HTML parse in blitz-html's html_sink.rs, not reached and not named by the scope
input-validation#1.31 · yes · f6edb9701 moves the fetched-stylesheet UTF-8 rule onto the new path, which the scope names
input-validation#1.32 · no · iframe HTML decoding in net.rs, not reached and not named by the scope
input-validation#1.33 · no · image format decoding in net.rs, not reached and not named by the scope
input-validation#1.34 · no · favicon decoding in apps/browser, which no commit the scope names touches
input-validation#1.35 · yes · names font_face_sniffed_source as its witness, which the re-run proofs and fork CI carry
input-validation#1.36 · no · font length and WOFF fallback in util.rs, not reached and not named by the scope
input-validation#1.37 · yes · the ratified hint-less data: @font-face widening, witnessed by font_face_sniffed_source that the re-run proofs and fork CI carry
input-validation#1.38 · no · SVG UTF-8 and DTD rules in svg.rs, which the scope does not name
input-validation#1.39 · no · SVGZ decompression in svg.rs, which the scope does not name
input-validation#1.40 · yes · viewBox is an SVG attribute of the kind 8b1a0a3ce moves, under the closed-parser and clamped-numeric rules the scope names
input-validation#1.41 · yes · inline SVG parse in construct.rs, a predicted conflict surface, with the logged parse error under the no-log rule the scope names
input-validation#1.42 · yes · declaration validation by parsing, the rule the scope names for the new paint properties reached by page CSS
input-validation#1.43 · yes · rejection of unsupported names and invalid values is the validation a page script's style setter meets the new paint properties under, which the scope names
input-validation#1.44 · no · selectorText setter in cssom.rs, not reached and not named by the scope
input-validation#1.45 · no · query-string parse errors in query_selector.rs, not reached and not named by the scope
input-validation#1.46 · no · <style> entity decoding in document.rs, not reached; the scope names declaration validation, not text decoding
input-validation#1.47 · yes · bgcolor parsing in stylo.rs, a predicted conflict surface
input-validation#1.48 · yes · dimension parsing in stylo.rs, a predicted conflict surface, with the nonzero table variant
input-validation#1.49 · yes · SVG width and height parsing in stylo.rs, a predicted conflict surface, under the SVG attribute rules the scope names
input-validation#1.50 · yes · border and body margin parsing in stylo.rs, a predicted conflict surface
