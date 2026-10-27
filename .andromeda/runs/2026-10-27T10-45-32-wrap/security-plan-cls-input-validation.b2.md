# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.26 · yes · add-to · F1 joins beside InvalidData: a non-file: document's file: load is refused with a FetchError before any read, and blitz-vibey-script's fetch.rs gained lines 10-12 and 23 above the cited 60-67
input-validation#1.27 · yes · stale · blitz-vibey-script's document.rs gained lines 16, 163-169, 210-211 and 222 above the cited 307-316, so the span moved
input-validation#1.28 · no · blitz-html's html_sink.rs is not reached, and no scripting-flag rule changed
input-validation#1.29 · no · blitz-dom's net.rs is not reached, and the stylesheet UTF-8 check is unchanged
input-validation#1.30 · no · blitz-dom's net.rs is not reached, and the iframe decode is unchanged
input-validation#1.31 · no · blitz-dom's net.rs is not reached, and the image decode path is unchanged
input-validation#1.32 · no · apps/browser's favicon.rs is not reached, and tab.rs skips favicon probing only for error pages, leaving the byte check as it was
input-validation#1.33 · no · net.rs and the font_face_sniffed_source witness are not reached, and the sniff is unchanged
input-validation#1.34 · no · blitz-dom's util.rs is not reached, and the length and decompression fallback are unchanged
input-validation#1.35 · no · net.rs and its witness are not reached, and the unresolvable-URL skip is unchanged
input-validation#1.36 · no · blitz-dom's node/svg.rs is not reached, and the encoding and DTD rule is unchanged
input-validation#1.37 · no · blitz-dom's node/svg.rs is not reached, and the SVGZ detection is unchanged
input-validation#1.38 · no · blitz-dom's node/svg.rs is not reached, and the viewBox check is unchanged
input-validation#1.39 · yes · stale · blitz-dom's layout/construct.rs (a merge surface) gained lines 313-317 above the cited 451-471, so the span moved
input-validation#1.40 · no · blitz-dom's resolved_style.rs and cssom.rs are not reached, and the accessors change no parse or CSSOM value rule
input-validation#1.41 · yes · add-to · F4 joins beside the set and remove name check: script's CSS accessors take their names from Stylo's property list, and node/element.rs (a merge surface) is reached too
input-validation#1.42 · no · blitz-dom's cssom.rs is not reached, and the selectorText rule is unchanged
input-validation#1.43 · no · query_selector.rs is not reached, and :open changes selector matching, not the query parse
input-validation#1.44 · no · blitz-dom's document.rs is not reached, and the style-text entity decode is unchanged
input-validation#1.45 · yes · stale · stylo.rs is a merge surface, so the cited lines 898-919 and 1186-1193 may have moved under the sync
input-validation#1.46 · yes · stale · stylo.rs is a merge surface, so the cited lines 921-950 and 1060-1066 may have moved under the sync
input-validation#1.47 · yes · stale · stylo.rs is a merge surface, so the cited lines 952-982 may have moved under the sync
input-validation#1.48 · yes · stale · stylo.rs is a merge surface, so the cited lines 1133 and 1167 may have moved under the sync
input-validation#1.49 · no · blitz-dom's layout/mod.rs and layout/table.rs are not in the reached files, and the Untouched list names them as not reached
input-validation#1.50 · no · blitz-dom's layout/table.rs is not in the reached files, and the rowspan clamp is unchanged
