# Classified — design · typography.b2

## Answers
typography#3.3 · no · stylo_to_parley.rs is not reached, and normal selects no non-default feature while the scope names no font-variant value
typography#3.4 · yes · the default line-height normal sets every text box's height, which the exact-bounds re-measurement reads under a moved Taffy or Stylo
typography#3.5 · no · stylo_to_parley.rs is not reached, and normal letter-spacing and word-spacing add nothing while the scope names no spacing value
typography#3.6 · yes · stylo.rs is reached whole for the :open pseudo-class, so its lang-to-_x_lang mapping at lines 1201-1203, which feeds Parley's text locale, binds
typography#3.7 · yes · the default white-space, wrap and text-align settings lay out every measured text row, and a moved Stylo reaches these Parley mappings
typography#3.8 · no · font_metrics.rs is not reached, and the ch and ic units bind only where a style uses them, which the scope does not name
typography#3.9 · yes · document.rs is reached whole, and its stylist-device seeding at lines 2322-2343 sets the root font size that rem and rlh read, which the take's document edits must keep
typography#3.10 · no · layout/list.rs is not reached, and the scope names no list or marker glyph
typography#3.11 · no · the emboldening is feature-gated with its Apple path only on macOS and iOS, blitz-paint is not reached, and the scope names no emboldened text
typography#3.12 · no · text-decoration adds nothing by default, blitz-paint's text.rs is not reached, and the scope names no decorated text
typography#4.1 · no · examples/assets is not reached, and no stand row loads these example pages
typography#4.2 · no · the graphite example page is not reached, and no stand row or check reads its fonts or sizes
typography#4.3 · no · the example pages are not reached, and the pseudo page's 32px Font Awesome glyph size is not the flow classification that 2963b65a2 changes
typography#4.4 · no · servo.css and servo.html are not reached, and the stand loads no Google Font
typography#4.5 · no · the custom widget example is not in the delta, the task's Not needed names that pattern, and the Preact pages are not reached
typography#4.6 · no · the text-decoration fixture page is not reached, and the scope names no decorated text
typography#5.1 · no · the s08 font-family search records no declaration, and the take's scope names no font-family change
typography#5.2 · no · the s09 font-family search records no declaration for this take to honour, and the take's scope names no font-family change
typography#5.3 · no · its one match is the test file packages/blitz-vibey-script/tests/dom.rs, which the absence's test-fixture exclusion covers, and the scope names no font-family change
typography#5.4 · no · the record of no project type scale or font tokens stands, since the take's scope names none, and its one sans-serif fixture hit is a page's font-family, not a token
