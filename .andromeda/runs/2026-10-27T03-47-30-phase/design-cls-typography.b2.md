# Classified — design · typography.b2

## Answers
typography#3.3 · no · the engine's font-variant feature-tag mapping; the chunk edits no engine code and names no font-variant feature
typography#3.4 · no · the engine's line-height mapping and its normal rule; the chunk re-measures no layout value and re-reads no test that asserts a line height
typography#3.5 · no · letter and word spacing resolved against font size in the engine; the chunk names no spacing and changes no engine code
typography#3.6 · no · the text locale from the lang attribute in the engine; no chunk scope names a locale
typography#3.7 · no · the text-align and wrapping mappings in the engine; the chunk names none of them and changes no engine code
typography#3.8 · no · ch and ic metric measurement in the engine; the chunk names no unit and re-measures no metric
typography#3.9 · no · the rem root-size seeding and rem_after_viewport_change.rs; the chunk re-reads neither that test nor any rem value
typography#3.10 · no · list-marker glyphs in layout/list.rs; the chunk changes no list rendering and names no marker
typography#3.11 · no · the font-embolden feature gates in blitz-paint; the chunk changes no build feature (Not needed: no build feature changes)
typography#3.12 · no · text-decoration thickness and the OS/2 ascent positions in blitz-paint; the chunk names no decoration and re-measures none
typography#4.1 · no · the example pages' font families (cursor, border-styles, Google fixtures); the chunk edits no example or fixture
typography#4.2 · no · the graphite fixture's typography; the chunk reaches no fixture and names none
typography#4.3 · no · the gosub, servo-new-reduced and pseudo fixtures' families; not the seven_guis stand and not reached
typography#4.4 · no · the servo.css fixture's Fira families; not the seven_guis stand and not reached
typography#4.5 · no · the custom-widget and Preact stacks; not the seven_guis stand and not reached
typography#4.6 · no · the text-decoration fixture's coverage; the chunk reaches no fixture
typography#5.1 · yes · the Needs keep a section's absence statement as it stands when the move leaves the section with no stand row; this batch holds no stand row, so this s08 font-family absence is kept, not extended
typography#5.2 · yes · the same Needs carve-out, for the s09 font-family absence; kept as it stands, not extended
typography#5.3 · yes · the same Needs carve-out, for the s10 font-family absence; kept as it stands, not extended
typography#5.4 · yes · the same Needs carve-out, for the section's own absence of a type scale or font tokens; the chunk adds no font token, so the statement stays as it stands
