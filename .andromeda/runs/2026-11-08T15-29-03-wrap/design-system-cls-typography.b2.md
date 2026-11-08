@answer n=1 entity=typography#3.4 v=no
why: the line-height mapping lives in blitz-dom files that the session's change does not reach
@answer n=2 entity=typography#3.5 v=no
why: the select row-height reading sits in blitz-dom document.rs, which the change does not reach, and no popup paint changed
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing resolve in blitz-dom stylo_to_parley, which the change does not reach
@answer n=4 entity=typography#3.7 v=no
why: the text locale mapping in blitz-dom is outside the files the change reaches
@answer n=5 entity=typography#3.8 v=no
why: the text-align and wrap mappings in blitz-dom are outside the files the change reaches
@answer n=6 entity=typography#3.9 v=no
why: the ch and ic metrics live in blitz-dom font_metrics.rs, which the change does not reach
@answer n=7 entity=typography#3.10 v=no
why: the stylist root font-size seeding and its rem test sit outside the files the change reaches
@answer n=8 entity=typography#3.11 v=no
why: list markers in blitz-dom layout list.rs are outside the files the change reaches
@answer n=9 entity=typography#3.12 v=no
why: font emboldening in blitz-paint is untouched, and the session adds no paint change
@answer n=10 entity=typography#3.13 v=no
why: the select group-label bold in blitz-paint and select_popup_painted.rs is outside the files the change reaches
@answer n=11 entity=typography#3.14 v=no
why: text-decoration thickness and position in blitz-paint are outside the files the change reaches
@answer n=12 entity=typography#4.1 v=no
why: the fixture font stacks in examples/assets are outside the files the change reaches
@answer n=13 entity=typography#4.2 v=no
why: the graphite fixture font rules in examples/assets are outside the files the change reaches
@answer n=14 entity=typography#4.3 v=no
why: the gosub and servo fixture fonts in examples/assets are outside the files the change reaches
@answer n=15 entity=typography#4.4 v=no
why: the servo.css and servo.html fixture fonts are outside the files the change reaches
@answer n=16 entity=typography#4.5 v=no
why: custom_widget.rs and the preact fixtures are outside the files the change reaches
@answer n=17 entity=typography#4.6 v=no
why: the text-decoration fixture is outside the files the change reaches
@answer n=18 entity=typography#5.1 v=no
why: the session brings no font-family declaration, so the no-match reading over the s08 files holds
@answer n=19 entity=typography#5.2 v=no
why: the session brings no font-family declaration, so the no-match reading over the s09 files holds
@answer n=20 entity=typography#5.3 v=no
why: the session brings no font-family declaration outside fixtures, so the s10 reading holds
@answer n=21 entity=typography#5.4 v=no
why: the session brings no type scale or font token, so the s12 reading holds
