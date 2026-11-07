@answer n=1 entity=typography#3.4 v=yes
why: the default line-height normal sets the text-input box height that the probe's get_client_rect geometry reads.
@answer n=2 entity=typography#3.5 v=no
why: the open select list's row height; no select list is touched by the chunk.
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing act only when authored, and the chunk authors none.
@answer n=4 entity=typography#3.7 v=no
why: text locale only shapes text; the chunk neither reads nor sets a locale.
@answer n=5 entity=typography#3.8 v=no
why: text-align and wrap settings are not authored or asserted by the chunk's measures.
@answer n=6 entity=typography#3.9 v=no
why: ch and ic metrics feed ch-unit lengths, which the chunk neither uses nor asserts.
@answer n=7 entity=typography#3.10 v=no
why: root font-size seeding is blitz-dom document setup the chunk does not change, and no rem value is asserted.
@answer n=8 entity=typography#3.11 v=no
why: list markers are not in the chunk's mutation path or its assertions.
@answer n=9 entity=typography#3.12 v=no
why: emboldening is a paint-stage value the chunk neither changes nor measures.
@answer n=10 entity=typography#3.13 v=no
why: the select group header's paint; no select list is touched.
@answer n=11 entity=typography#3.14 v=no
why: text-decoration thickness and position are paint-stage values the chunk does not touch.
@answer n=12 entity=typography#4.1 v=no
why: example fixture font families; the chunk touches no example or fixture page.
@answer n=13 entity=typography#4.2 v=no
why: the graphite example fixture's type rules; the chunk touches no example page.
@answer n=14 entity=typography#4.3 v=no
why: the gosub and pseudo example fixture rules; the chunk touches no example page.
@answer n=15 entity=typography#4.4 v=no
why: the servo example fixture font stack; the chunk touches no example page.
@answer n=16 entity=typography#4.5 v=no
why: the custom-widget and Preact example font values; the chunk touches no example.
@answer n=17 entity=typography#4.6 v=no
why: the text-decoration example fixture; the chunk touches no example.
@answer n=18 entity=typography#5.1 v=no
why: a font-family absence search; the chunk adds no font declaration.
@answer n=19 entity=typography#5.2 v=no
why: a font-family absence search; the chunk adds no font declaration.
@answer n=20 entity=typography#5.3 v=no
why: a font-family absence search outside fixtures; the chunk adds no font declaration.
@answer n=21 entity=typography#5.4 v=no
why: an absence of font tokens in the project; the chunk adds none.
