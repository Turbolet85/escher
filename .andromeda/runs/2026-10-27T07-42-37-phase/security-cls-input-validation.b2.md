# Classified — security · input-validation.b2

## Answers
input-validation#1.26 · yes · the task's Needs read the ScriptFetcher narrowing (`cee6b8731`) against the recorded fetcher rule, and this row is that fetcher's `data:` URL and byte rejection, which must hold on the take
input-validation#1.27 · no · the script-type gate lives in blitz-vibey-script's document.rs, which the chunk does not reach, and the task names no script type
input-validation#1.28 · no · the scripting flag lives in blitz-html's html_sink.rs, which the chunk does not reach, and the task names no parse change
input-validation#1.29 · no · the stylesheet UTF-8 rule lives in blitz-dom's net.rs, which the chunk does not reach, and the task names no stylesheet change
input-validation#1.30 · no · the iframe decode lives in blitz-dom's net.rs, which the chunk does not reach, and the task names no iframe change
input-validation#1.31 · no · the image decode lives in blitz-dom's net.rs, which the chunk does not reach, and the task names no image path
input-validation#1.32 · yes · names the favicon tests in apps/browser/src/favicon.rs as its witness, which the re-assertion of the fast leg's workspace tests covers
input-validation#1.33 · yes · names font_face_sniffed_source.rs as its witness, which the re-assertion of the fast leg's workspace tests covers
input-validation#1.34 · no · the WOFF length and fallback rule lives in blitz-dom's util.rs, which the chunk does not reach, and the task names no font-byte change
input-validation#1.35 · yes · names font_face_sniffed_source.rs as its witness for the hint-less `data:` source, which the re-assertion of the fast leg's workspace tests covers
input-validation#1.36 · no · the SVG encoding and DTD rule lives in blitz-dom's svg.rs, which the chunk does not reach, and the task names no SVG parse change
input-validation#1.37 · no · SVGZ detection lives in blitz-dom's svg.rs, which the chunk does not reach, and the task names no compressed-SVG path
input-validation#1.38 · yes · the task names degenerate aspect ratios discarded, and this row's degenerate flag for a zero-width or zero-height viewBox in svg.rs is a candidate input, a fork the task leaves open
input-validation#1.39 · yes · the inline SVG parse call in construct.rs, a predicted conflict surface, keeps its parse-error fallback and log line on the take
input-validation#1.40 · yes · the task names the CSS property accessors and the cssText assignment, which carry property and value pairs that must still be ignored when invalid
input-validation#1.41 · yes · blitz-dom's node/element.rs is reached whole, and the CSS property accessors the task adds must keep this rejection of unsupported names and invalid values by returning false
input-validation#1.42 · yes · the task names selectorText assignment never throwing, so an invalid selector list must still leave the rule unchanged in cssom.rs
input-validation#1.43 · yes · the task's native API argument checks carry query strings into this parse, which must keep returning its parse errors
input-validation#1.44 · yes · blitz-dom's document.rs is reached whole, and its `<style>` text entity decoding must hold on the take
input-validation#1.45 · yes · blitz-dom's stylo.rs is reached whole, and its bgcolor hex-only acceptance must hold on the take
input-validation#1.46 · yes · blitz-dom's stylo.rs is reached whole, and its HTML dimension parse rides Stylo's parser, which the take may move
input-validation#1.47 · yes · blitz-dom's stylo.rs is reached whole, and its SVG width and height rejections must hold on the take
input-validation#1.48 · yes · blitz-dom's stylo.rs is reached whole, and its border and body-margin integer parses must hold on the take
input-validation#1.49 · yes · the replaced-element width and height parse in layout/mod.rs may feed the task's degenerate-aspect-ratio discard, a fork the task leaves open, and its table parts are not named
input-validation#1.50 · no · the rowspan clamp lives in table.rs, which the chunk does not reach, and the task names no table span path
