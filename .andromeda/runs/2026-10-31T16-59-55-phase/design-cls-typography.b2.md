# Classified — design · typography.b2

## Answers
typography#3.5 · no · letter-spacing resolves against font size, but the scope names only the row text's font family, size and weight, not letter spacing
typography#3.6 · no · text locale from `lang`; the scope names no locale or `lang` reading for a row or a label
typography#3.7 · no · text-align and word, line and wrap mapping; the scope names no alignment or break setting for a row or a label
typography#3.8 · no · `ch` and `ic` metrics; the list's width is taken from the select and the scope names no `ch` or `ic` unit
typography#3.9 · yes · rem resolves against the root font-size seed; the row text's size is named and measured, and a rem-sized row or label resolves against this seed
typography#3.10 · no · list markers apply to list items, which the chunk does not paint; the scope names no list item
typography#3.11 · yes · synthetic emboldening in blitz-paint/src, reached whole, carries the row's named weight when the face lacks bold
typography#3.12 · no · text-decoration thickness and positions; the scope names no decoration on a row, a label or a state
typography#4.1 · no · `sans-serif` and `monospace` in example fixtures; examples are demonstrations the scope excludes
typography#4.2 · no · graphite example's Inter and Bona Nova values; a demonstration fixture the scope excludes
typography#4.3 · no · gosub and servo-new-reduced example values; demonstration fixtures the scope excludes
typography#4.4 · no · servo.css Fira Sans and Fira Mono values; a demonstration fixture the scope excludes
typography#4.5 · no · custom-widget and Preact system stacks; demonstration fixtures the scope excludes
typography#4.6 · no · text-decoration fixture; a demonstration the scope excludes
typography#5.1 · yes · the plan's record of no font-family declarations over s08; the scope asks how absent families are recorded for a row and a closed select's label
typography#5.2 · yes · the same record over the s09 files; the scope asks how absent families are recorded for a row and a closed select's label
typography#5.3 · yes · the same record outside fixtures, with the one vibey-script hit; the scope asks how absent families are recorded for a row and a label
typography#5.4 · yes · no project type scale or font tokens; the scope asks how the plan records a row's font family and size with no engine tokens
