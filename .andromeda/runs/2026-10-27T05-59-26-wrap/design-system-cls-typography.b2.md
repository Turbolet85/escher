# Classified — design-system · typography.b2

## Answers
typography#3.3 · no · the feature-tag mapping lives in blitz-dom's stylo_to_parley.rs, which the session neither reaches nor changes
typography#3.4 · no · the line-height mapping lives in blitz-dom's stylo_to_parley.rs and layout/mod.rs, neither reached nor changed
typography#3.5 · no · letter and word spacing live in blitz-dom's stylo_to_parley.rs, which the session neither reaches nor changes
typography#3.6 · no · the text-locale rule lives in blitz-dom's stylo_to_parley.rs and stylo.rs, neither reached nor changed
typography#3.7 · no · the text-align and wrap mappings live in blitz-dom's stylo_to_parley.rs, which the session neither reaches nor changes
typography#3.8 · no · the ch and ic metrics live in blitz-dom's font_metrics.rs, which the session neither reaches nor changes
typography#3.9 · no · the stylist seed lives in blitz-dom's document.rs and its rem check in blitz-tests, neither reached nor changed
typography#3.10 · no · list markers live in blitz-dom's layout/list.rs, which the session neither reaches nor changes
typography#3.11 · no · the emboldening rule lives in blitz-paint's source and Cargo.toml, and the session changes no manifest
typography#3.12 · no · text-decoration thickness lives in blitz-paint's text.rs, which the session neither reaches nor changes
typography#4.1 · no · the font stacks live in examples/assets pages the session does not reach, and no example value moved
typography#4.2 · no · the graphite font stacks live in examples/assets/graphite.html, which the session does not reach
typography#4.3 · no · the gosub, servo-new-reduced-1 and pseudo stacks live in example pages the session does not reach
typography#4.4 · no · the servo.css and servo.html stacks live in example assets the session does not reach
typography#4.5 · no · the custom-widget and Preact stacks live in examples the session does not reach, and no example value moved
typography#4.6 · no · the text-decoration fixture is a test page the session does not reach
typography#5.1 · no · the session moves no font declaration in the 16 listed s08 files, so the no-match search still holds
typography#5.2 · no · the session moves no font declaration in the 32 listed s09 files, so the no-match search still holds
typography#5.3 · no · the session changes no file in the 32 s10 slice files, and its one match, packages/blitz-vibey-script/tests/dom.rs, is unchanged
typography#5.4 · no · the session adds no type scale or font token, and the stand's DejaVu registration (F2) is a code call, not a token
