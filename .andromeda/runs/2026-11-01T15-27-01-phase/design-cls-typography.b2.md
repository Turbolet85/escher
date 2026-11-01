# Classified — design · typography.b2

## Answers
typography#3.4 · yes · line-height normal mapping in stylo_to_parley.rs is a recorded engine text mapping value that Needs reads against construct.rs and inline build.rs, and the re-measured bounds read it
typography#3.5 · yes · the open select list's row height through select_row_layout is what the list's row-by-row re-read and the select_popup witnesses measure
typography#3.6 · yes · letter and word spacing mapping is a recorded engine text mapping value that Needs reads against construct.rs and inline build.rs
typography#3.7 · yes · the locale rule's stylo.rs lines 1217-1219 sit in the stylo.rs conflict surface
typography#3.8 · yes · text-align and wrap mappings are recorded engine text mapping values that Needs reads against construct.rs and inline build.rs
typography#3.9 · no · ch and ic measurement in font_metrics.rs is a component the chunk only uses, and no reached file or named behaviour reads it
typography#3.10 · no · stylist device seeding in document.rs and the rem test are neither reached nor named by the scope
typography#3.11 · no · list marker glyphs in layout/list.rs are neither reached nor named by the scope
typography#3.12 · yes · the emboldening strength and hinting rule lives in blitz-paint text.rs, a conflict surface, and the font-embolden gate builds it
typography#3.13 · yes · the group-header RunPaint rule lives in text.rs, a conflict surface, and the font-embolden witness select_popup_painted is re-read
typography#3.14 · yes · decoration thickness and overline and line-through positions live in text.rs, a conflict surface, and Needs names decoration painting changes
typography#4.1 · no · sans-serif and Google fixture font stacks in examples/assets files other than seven_guis, which the scope names no change to
typography#4.2 · no · graphite fixture font values in examples/assets other than seven_guis, which the scope does not change
typography#4.3 · no · gosub, servo-new-reduced and pseudo fixture font values in examples/assets other than seven_guis, which the scope does not change
typography#4.4 · no · servo.css fixture font stacks in examples/assets other than seven_guis, which the scope does not change
typography#4.5 · no · custom_widget and preact font stacks in examples other than seven_guis, which the scope does not change
typography#4.6 · no · the text-decoration fixture is not named by the scope and no re-measure reads it, while decoration painting itself binds through typography#3.14
typography#5.1 · yes · a no-match record over the s08 files that the chunk must keep true, since its merge into conflict surfaces can bring a font-family match in
typography#5.2 · yes · a no-match record over the s09 files that the chunk must keep true, since its merge into conflict surfaces can bring a font-family or FontFamily match in
typography#5.3 · yes · a no-match record over the s10 files that the chunk must keep true, since the blitz-vibey-script runtime conflict surfaces it resolves can bring a font-family match in
typography#5.4 · no · the chunk defines no project type scale or font tokens, a Not needed item, and the one fixture hit is a sans-serif value in a fixture
