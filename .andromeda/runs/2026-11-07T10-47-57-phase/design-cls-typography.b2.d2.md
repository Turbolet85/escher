@answer n=1 entity=typography#3.4 v=yes
why: line-height normal resolves to 1.2 × font size, which sets the box height that the chunk's get_client_rect measurement of text inputs reads (need 9)
@answer n=2 entity=typography#3.5 v=no
why: open select list row height; no select list is touched by the chunk
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing alter inline text width, and the chunk's geometry asserts name no content-sized text box
@answer n=4 entity=typography#3.7 v=no
why: text locale affects shaping only, which the chunk's geometry and focus assertions do not read
@answer n=5 entity=typography#3.8 v=no
why: text-align and wrapping move text inside a box, not the box rect or focus the chunk asserts
@answer n=6 entity=typography#3.9 v=no
why: ch and ic unit metrics apply to boxes sized in those units, and the chunk's scope names no such box
@answer n=7 entity=typography#3.10 v=no
why: the stylist device seeding in document.rs is not edited by the chunk and the scope names no rem-sized measurement
@answer n=8 entity=typography#3.11 v=no
why: list marker glyphs are paint-side content that the chunk neither measures nor focuses
@answer n=9 entity=typography#3.12 v=no
why: font emboldening is paint-only and the chunk paints nothing
@answer n=10 entity=typography#3.13 v=no
why: select list group header paint; no select list is touched
@answer n=11 entity=typography#3.14 v=no
why: text-decoration thickness and position are paint-only
@answer n=12 entity=typography#4.1 v=no
why: example fixture font families; the chunk runs no example fixture
@answer n=13 entity=typography#4.2 v=no
why: graphite example page fonts and sizes; the chunk runs no example fixture
@answer n=14 entity=typography#4.3 v=no
why: gosub and servo-new-reduced example fonts; the chunk runs no example fixture
@answer n=15 entity=typography#4.4 v=no
why: servo example stylesheet fonts; the chunk runs no example fixture
@answer n=16 entity=typography#4.5 v=no
why: custom widget and Preact example fonts; the chunk runs no example fixture
@answer n=17 entity=typography#4.6 v=no
why: text-decoration fixture exercises paint properties only, which the chunk does not re-measure
@answer n=18 entity=typography#5.1 v=no
why: absence record of font-family declarations in the s08 files; the chunk adds no font-family declaration
@answer n=19 entity=typography#5.2 v=no
why: absence record of font family declarations in the s09 files; the chunk adds no font-family declaration
@answer n=20 entity=typography#5.3 v=no
why: absence record of font-family declarations outside fixtures; the chunk adds no font-family declaration
@answer n=21 entity=typography#5.4 v=no
why: absence record of a project type scale or font tokens; the chunk adds no type token
