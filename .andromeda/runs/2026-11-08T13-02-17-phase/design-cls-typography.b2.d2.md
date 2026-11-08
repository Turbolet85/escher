@answer n=1 entity=typography#3.4 v=no
why: line-height normal mapping is blitz layout's own guarantee; the read consumes boxes through layout and sets no line height.
@answer n=2 entity=typography#3.5 v=yes
why: states an open select list row's height and that a larger authored font clips it; the read answers and reports off-screen and covered causes for owner-served select rows (SelectListView), so the minimal fixture's row geometry rests on it.
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing resolve against font size; the read reports ids, verbs and causes, not text spacing.
@answer n=4 entity=typography#3.7 v=no
why: text locale from the lang attribute; the read does not consult locale.
@answer n=5 entity=typography#3.8 v=no
why: text-align and wrap mapping; paint and wrapping are not read by the chunk.
@answer n=6 entity=typography#3.9 v=no
why: ch and ic metric measurement; the chunk reads no unit metrics.
@answer n=7 entity=typography#3.10 v=no
why: root font-size seeding for rem and rlh units; blitz-dom's own stylist behaviour, which the chunk does not act on.
@answer n=8 entity=typography#3.11 v=no
why: list marker glyphs; list markers are not actionable nodes and the read does not reach them.
@answer n=9 entity=typography#3.12 v=no
why: font emboldening strength and features; the read reports no paint.
@answer n=10 entity=typography#3.13 v=no
why: a select group label's heavier paint through RunPaint; the read lists groups under their select owner but reports ids, verbs and causes, not paint.
@answer n=11 entity=typography#3.14 v=no
why: auto text-decoration thickness and overline position; paint only, not read by the chunk.
@answer n=12 entity=typography#4.1 v=no
why: records existing example fixture font families; the chunk adds no font and no example that this line names.
@answer n=13 entity=typography#4.2 v=no
why: records the graphite example's fonts and breakpoints; the chunk adds no font to that example.
@answer n=14 entity=typography#4.3 v=no
why: records the gosub, servo-new-reduced-1 and pseudo example fonts; the chunk touches none of them.
@answer n=15 entity=typography#4.4 v=no
why: records the servo.css and servo.html font stacks; the chunk touches none of them.
@answer n=16 entity=typography#4.5 v=no
why: records the custom-widget and Preact example font stacks; the chunk touches none of them.
@answer n=17 entity=typography#4.6 v=no
why: records the text-decoration fixture's properties; the chunk adds no decoration test.
@answer n=18 entity=typography#5.1 v=no
why: records a search absence of font-family declarations over listed files; the chunk adds no font declaration, so the absence holds unchanged.
@answer n=19 entity=typography#5.2 v=no
why: records a search absence of font-family declarations over listed s09 files; the chunk adds no font declaration, so the absence holds unchanged.
@answer n=20 entity=typography#5.3 v=no
why: records a search absence of font-family declarations outside test fixtures; the chunk adds no font declaration, so the absence holds unchanged.
@answer n=21 entity=typography#5.4 v=no
why: records the absence of a project type scale or font tokens; the chunk adds no type scale or font token.
