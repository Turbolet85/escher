@answer n=1 entity=screen-reader-support#8.2 v=yes
why: the graph keys on the accesskit node and author_id lands in dioxus-native-dom, so that code must sit behind the accessibility feature this entity names
@answer n=2 entity=screen-reader-support#9.1 v=no
why: lang and xml:lang element language is outside the chunk (language and direction is not_needed)
@answer n=3 entity=screen-reader-support#9.2 v=no
why: the dir bidi-isolation stylesheet rule is outside the chunk (language and direction is not_needed)
@answer n=4 entity=screen-reader-support#9.3 v=no
why: html lang attributes in the s02 fixtures are outside the chunk (fixture language is not_needed)
@answer n=5 entity=screen-reader-support#9.4 v=no
why: page lang declarations are outside the chunk (language is not_needed)
@answer n=6 entity=screen-reader-support#10.1 v=no
why: the absence of lang in the other s02 fixtures is not re-measured by the chunk
@answer n=7 entity=screen-reader-support#11.1 v=no
why: an sr-only tooltip in a GitHub fixture is a fixture text alternative (not_needed)
@answer n=8 entity=screen-reader-support#11.2 v=no
why: descriptive aria-labels in the google fixture are fixture text alternatives (not_needed)
@answer n=9 entity=screen-reader-support#11.3 v=no
why: aria-hidden and aria-atomic in the google fixture are hidden-content rules (not_needed)
@answer n=10 entity=screen-reader-support#11.4 v=no
why: aria-hidden bar spans in the servo-new fixture are hidden-content exclusion (not_needed)
@answer n=11 entity=screen-reader-support#11.5 v=no
why: servo.html image alt text is a fixture text alternative (not_needed)
@answer n=12 entity=screen-reader-support#11.6 v=no
why: a fixture image with no alt attribute is a fixture text alternative (not_needed)
@answer n=13 entity=screen-reader-support#12.1 v=no
why: IME enabling and cursor area on text-input focus is IME behaviour the chunk does not touch (not_needed; no focus rule changes)
@answer n=14 entity=screen-reader-support#12.2 v=no
why: IME enabled by create_text_editor for a pre-focused input is IME behaviour the chunk does not touch (not_needed)
@answer n=15 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable events on the text editor are IME behaviour the chunk does not touch (not_needed)
@answer n=16 entity=screen-reader-support#13.1 v=no
why: the absence of accessibility integration in the s06 files is a current-state observation the chunk does not re-measure
