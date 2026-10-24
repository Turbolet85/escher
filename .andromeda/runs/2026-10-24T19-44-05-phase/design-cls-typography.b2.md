# Classified — design · typography.b2

## Answers
typography#3.4 · no · line-height sets line boxes only; no chunk check reads a line box and the chunk changes no layout
typography#3.5 · no · letter and word spacing scale inline advances only; no tab-order, colour or activation check reads them
typography#3.6 · no · the text locale sets shaping and line breaking only, which no chunk check reads
typography#3.7 · no · alignment and wrapping place text inside its box; the checks read focus order, activation and colour pairs
typography#3.8 · no · ch and ic are length units from glyph advances that no chunk check reads; the font they measure comes in through the bundled-font need
typography#3.9 · yes · rem resolves against the root font-size (16px initially, re-resolved after a viewport change), the computed size the large-text threshold reads for rem-sized stand text, which the pinned-viewport boot reaches
typography#3.10 · no · list markers are glyphs no contrast row or key check reads
typography#3.11 · no · emboldening is paint-time synthesis behind the font-embolden feature; the contrast rows read the computed weight and the chunk takes no screenshot
typography#3.12 · no · text-decoration thickness is paint-only; the contrast rows read the text colour, not any decoration
typography#4.1 · no · the sans-serif and monospace families belong to example fixture pages; the stand boots its own bundled font
typography#4.2 · no · graphite is an example page whose sizes and weights are not the stand's; no check boots it
typography#4.3 · no · gosub and servo-new-reduced are example pages whose families and sizes no stand check reads
typography#4.4 · no · the servo page's Fira families load from Google Fonts; the offline stand boots no such page
typography#4.5 · no · the custom-widget stack is excluded by the task; the Preact TodoMVC page is not the stand
typography#4.6 · no · the text-decoration fixture exercises paint-level decoration that no chunk check reads
typography#5.1 · no · the absence of font-family declarations in the 16 s08 files; the chunk declares none in its reached files, a re-check every change owes
typography#5.2 · no · the absence of font-family declarations in the 32 s09 files; the chunk declares none in its reached files, a re-check every change owes
typography#5.3 · no · font-family declarations outside fixtures match only a blitz-vibey-script test; the chunk declares none, a re-check every change owes
typography#5.4 · no · no project type scale or font token exists; the chunk adds no token and reads none, a re-check every change owes
