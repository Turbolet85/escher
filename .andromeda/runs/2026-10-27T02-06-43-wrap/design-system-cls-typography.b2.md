# Classified — design-system · typography.b2

## Answers
typography#3.3 · no · its OpenType feature-tag mapping sits in packages/blitz-dom/src/stylo_to_parley.rs, under packages/blitz-dom/ where the guard confirmed nothing changed, and the report keeps the typography mapping untouched
typography#3.4 · no · its line-height mapping sits in packages/blitz-dom/src/stylo_to_parley.rs and packages/blitz-dom/src/layout/mod.rs, under packages/blitz-dom/ where the guard confirmed nothing changed, and the report keeps the typography mapping untouched
typography#3.5 · no · its letter and word spacing mapping sits in packages/blitz-dom/src/stylo_to_parley.rs, under packages/blitz-dom/ where the guard confirmed nothing changed, and the report keeps the typography mapping untouched
typography#3.6 · no · its locale read sits in packages/blitz-dom/src/stylo_to_parley.rs and packages/blitz-dom/src/stylo.rs, under packages/blitz-dom/ where the guard confirmed nothing changed, and the report keeps the typography mapping untouched
typography#3.7 · no · its text-align and wrapping mappings sit in packages/blitz-dom/src/stylo_to_parley.rs, under packages/blitz-dom/ where the guard confirmed nothing changed, and the report keeps the typography mapping untouched
typography#3.8 · no · its ch and ic metrics sit in packages/blitz-dom/src/font_metrics.rs, under packages/blitz-dom/ where the guard confirmed nothing changed, and the report keeps the typography mapping untouched
typography#3.9 · no · its root font-size seeding sits in packages/blitz-dom/src/document.rs and its rem test in tests/blitz-tests/tests/rem_after_viewport_change.rs, neither reached, and the guard confirmed nothing under packages/blitz-dom/ changed
typography#3.10 · no · its list markers sit in packages/blitz-dom/src/layout/list.rs, under packages/blitz-dom/ where the guard confirmed nothing changed, and the report keeps the typography mapping untouched
typography#3.11 · no · its emboldening feature and strength sit in packages/blitz-paint/Cargo.toml, packages/blitz-paint/src/lib.rs and packages/blitz-paint/src/text.rs, none reached, and the report records no dependency or paint code changed
typography#3.12 · no · its decoration thickness and OS/2 metric reads sit in packages/blitz-paint/src/text.rs, not reached, and the report records no paint code changed
typography#4.1 · no · its body and code font-family values sit in examples/assets/cursor.html, examples/assets/border-styles.html, examples/assets/bottom_only.html and examples/assets/google.html, none reached, and no fixture stylesheet changed
typography#4.2 · no · its graphite.html font rules sit in examples/assets/graphite.html, not reached, and no fixture stylesheet changed
typography#4.3 · no · its gosub, servo-new-reduced-1 and pseudo fixture values sit in examples/assets/gosub.html, examples/assets/servo-new-reduced-1.html and examples/assets/pseudo.html, none reached, and no fixture stylesheet changed
typography#4.4 · no · its servo.css and servo.html font rules sit in examples/assets/servo.css and examples/assets/servo.html, none reached, and no stylesheet changed
typography#4.5 · no · its custom_widget and Preact page values sit in examples/custom_widget.rs, examples/preact/index.html and examples/preact/core_dom_apis.html, none reached, and no example UI or stylesheet changed
typography#4.6 · no · its text-decoration fixture sits in examples/assets/text-decoration.html, not reached, and no stylesheet changed
typography#5.1 · no · the 16 s08 files still hold no font-family declaration, since the refactor adds no font-family or font_family text to packages/escher-driver/src/lint.rs, packages/escher-driver/src/mcp_tests.rs or tests/blitz-tests/tests/stand_requirement_sweep.rs, and the report records nothing added to any font
typography#5.2 · no · the 32 s09 files still hold no font-family declaration, since the refactor adds no font-family, font_family or FontFamily text to those three reached files, and the report records nothing added to any font
typography#5.3 · no · the 32 s10 files still match only packages/blitz-vibey-script/tests/dom.rs, since the refactor adds no font-family or font_family text to the reached files, and the report records nothing added to any font
typography#5.4 · no · the 61 s12 files still hold no project type scale or font token, since the report records nothing added to any font or token and no stylesheet changed
