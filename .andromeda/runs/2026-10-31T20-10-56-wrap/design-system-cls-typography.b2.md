# Classified — design-system · typography.b2

## Answers
typography#3.5 · no · the letter and word spacing mapping lives in stylo_to_parley.rs, which the session did not reach
typography#3.6 · yes · stale · stylo.rs gained lines past 1124, so the cited lines 1217-1219 may have moved; the lang reading is not reported as changed
typography#3.7 · no · the text-align and wrap mappings live in stylo_to_parley.rs, which the session did not reach
typography#3.8 · no · the ch and ic metrics live in font_metrics.rs, which the session did not reach
typography#3.9 · yes · stale · document.rs gained 340 lines at 761-1089, so the stylist-seed citation at 2691-2712 moved
typography#3.10 · no · the list markers live in layout/list.rs, which the session did not reach
typography#3.11 · yes · contradict · a synthesized group header (F3) on a plain build keeps hint on and takes the bold, and on an emboldening build takes twice the strength, so hinting off when emboldening no longer holds
typography#3.12 · no · the decoration thickness and position lines sit before text.rs line 573, where the report moved nothing, and the change touches only stroke and hinting
typography#4.1 · no · the examples/assets fixtures it cites are not reached by the session
typography#4.2 · no · the graphite fixture in examples/assets is not reached by the session
typography#4.3 · no · the gosub, servo-new-reduced and pseudo fixtures in examples/assets are not reached by the session
typography#4.4 · no · the servo fixtures in examples/assets are not reached by the session
typography#4.5 · no · examples/custom_widget.rs and the preact fixtures are not reached by the session
typography#4.6 · no · the text-decoration fixture in examples/assets is not reached by the session
typography#5.1 · no · the session adds no font-family declaration; default.css gains only the multiple select's overflow and row line-height, and list rows take the select's own font
typography#5.2 · no · the session adds no font-family or FontFamily code; list rows take the select's own font
typography#5.3 · no · the session adds no font-family declaration outside fixtures; the list's values are engine constants
typography#5.4 · no · the list's values are engine constants that establish no type scale, and no font token was added
