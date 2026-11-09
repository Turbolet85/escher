@answer n=1 entity=typography#3.4 v=no
why: line-height mapping lives in blitz-dom stylo_to_parley and layout, none of which the session reaches
@answer n=2 entity=typography#3.5 v=no
why: select-list row height rests on blitz-dom document.rs and a 2026-10-31 report, neither reached by the session
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing resolve in blitz-dom stylo_to_parley, which the session does not touch
@answer n=4 entity=typography#3.7 v=no
why: text locale mapping in blitz-dom stylo and stylo_to_parley is outside every reached file
@answer n=5 entity=typography#3.8 v=no
why: text-align and wrap mapping in blitz-dom stylo_to_parley is outside every reached file
@answer n=6 entity=typography#3.9 v=no
why: ch and ic metrics in blitz-dom font_metrics.rs are outside every reached file
@answer n=7 entity=typography#3.10 v=no
why: stylist root font-size seeding in blitz-dom document.rs and its rem test are not reached, and the session moves no rem or root font rule
@answer n=8 entity=typography#3.11 v=no
why: list marker glyphs in blitz-dom layout/list.rs are outside every reached file
@answer n=9 entity=typography#3.12 v=no
why: font emboldening in blitz-paint Cargo.toml, lib.rs and text.rs is outside every reached file, and the session moves no paint rule
@answer n=10 entity=typography#3.13 v=no
why: the RunPaint group-header bold lives in blitz-paint text.rs and select_popup_painted.rs, neither reached; the reached common/mod.rs is replay harness, and the session moves no paint behaviour
@answer n=11 entity=typography#3.14 v=no
why: text-decoration thickness and OS/2 position code in blitz-paint text.rs is outside every reached file
@answer n=12 entity=typography#4.1 v=no
why: font-family values in examples/assets and cursor fixtures are not reached, and the session adds no font declaration
@answer n=13 entity=typography#4.2 v=no
why: graphite fixture font rules in examples/assets/graphite.html are not reached
@answer n=14 entity=typography#4.3 v=no
why: gosub, servo-new-reduced-1 and pseudo fixture font rules are not reached
@answer n=15 entity=typography#4.4 v=no
why: servo.css and servo.html font stacks are not reached
@answer n=16 entity=typography#4.5 v=no
why: custom_widget.rs and preact font stacks are not reached
@answer n=17 entity=typography#4.6 v=no
why: text-decoration fixture examples/assets/text-decoration.html is not reached
@answer n=18 entity=typography#5.1 v=no
why: the session's reached files add no font-family declaration, and the task moves no font behaviour
@answer n=19 entity=typography#5.2 v=no
why: the session's reached files add no font-family or FontFamily declaration, and the task moves no font behaviour
@answer n=20 entity=typography#5.3 v=no
why: the session's reached files add no font-family declaration outside fixtures, and the task moves no font behaviour
@answer n=21 entity=typography#5.4 v=no
why: the session adds no project type scale or font token, and the task moves no font behaviour
