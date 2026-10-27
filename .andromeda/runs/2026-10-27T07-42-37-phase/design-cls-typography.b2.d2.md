# Classified — design · typography.b2

## Answers
typography#3.3 · no · the font-variant mapping sits in stylo_to_parley.rs, which the take does not reach, and no Need or reached test names a font-variant value
typography#3.4 · yes · line-height normal resolves to 1.2 times the font size, the default that the re-measured text bounds rows read under a moved Taffy, Stylo or winit (text layout and measured bounds)
typography#3.5 · no · letter and word spacing resolve in stylo_to_parley.rs, which the take does not reach, and no Need or reached test names letter or word spacing
typography#3.6 · yes · the lang attribute sets the _x_lang value at stylo.rs:1201-1203, and stylo.rs is reached whole (the details :open pseudo-class lands there), so that rule must keep holding
typography#3.7 · yes · the white-space collapse mapping, which the re-measured text rows read at the normal value, and the reached target pre_overflow_scroll.rs runs on white-space behaviour
typography#3.8 · no · the ch and ic metrics live in font_metrics.rs, which the take does not reach, and no Need or reached test names a ch or ic unit
typography#3.9 · yes · document.rs:2322-2343 seeds the stylist device with the root font size, document.rs is reached whole, and the 16px initial root size is the default the re-measured rows read
typography#3.10 · yes · the disclosure-open and disclosure-closed markers paint the details open state that the take's :open change (53e9d81c6) reaches, so they belong to the details styling the take re-reads
typography#3.11 · no · emboldening needs the font-embolden feature or macOS and iOS, the take reaches no blitz-paint file, and no Need or reached test names emboldening or hinting
typography#3.12 · no · text-decoration thickness and position are blitz-paint rules the take does not reach, and no Need or reached test names a text decoration
typography#4.1 · no · the font families sit in the examples/assets pages (cursor, border-styles, bottom_only, google), which the take does not reach, and the stand is examples/seven_guis
typography#4.2 · no · the graphite font rules sit in the examples/assets/graphite.html page, which the take does not reach
typography#4.3 · no · the gosub, servo-new-reduced-1 and pseudo font rules sit in examples/assets pages, which the take does not reach
typography#4.4 · no · the servo.css and servo.html font rules sit in examples/assets, which the take does not reach
typography#4.5 · no · custom_widget.rs and the Preact pages are examples the take does not reach, and the desktop-native custom widget pattern is not in the delta
typography#4.6 · no · the text-decoration fixture is an examples/assets page the take does not reach
typography#5.1 · no · the take adds no font-family declaration to any product file, so this search over the s08 slice holds
typography#5.2 · no · the take adds no font-family declaration to any product file, so this search over the s09 slice holds
typography#5.3 · no · its one match is in packages/blitz-vibey-script/tests/dom.rs, a test file the claim excludes, and the take adds no font-family to product code
typography#5.4 · no · the take adds no type scale or font tokens, and it keeps the bundled stand font unchanged rather than declaring a family
