@answer n=1 entity=screen-reader-support#8.2 v=yes
why: the graph keys on the accesskit node and author_id lands in dioxus-native-dom, so the new accesskit code must sit behind the accessibility feature this entity names
@answer n=2 entity=screen-reader-support#9.1 v=no
why: language and direction are a not_needed scope item, and the chunk reads and changes no lang attribute
@answer n=3 entity=screen-reader-support#9.2 v=no
why: the dir attribute's default stylesheet mapping is language and direction, a not_needed scope item the chunk does not touch
@answer n=4 entity=screen-reader-support#9.3 v=no
why: html lang values in fixtures are language and direction, a not_needed scope item, and the chunk edits no fixture
@answer n=5 entity=screen-reader-support#9.4 v=no
why: page lang declarations are language and direction, a not_needed scope item the chunk does not touch
@answer n=6 entity=screen-reader-support#10.1 v=no
why: an observed absence of html lang in s02 fixtures is language and direction, a not_needed scope item the chunk does not touch
@answer n=7 entity=screen-reader-support#11.1 v=no
why: an sr-only tooltip in a captured fixture is a text alternative in fixtures, a not_needed scope item the chunk does not touch
@answer n=8 entity=screen-reader-support#11.2 v=no
why: google fixture aria-labels are text alternatives in fixtures, a not_needed scope item the chunk does not touch
@answer n=9 entity=screen-reader-support#11.3 v=no
why: aria-hidden and aria-atomic in the google fixture are hidden-content and fixture text alternatives, both not_needed scope items
@answer n=10 entity=screen-reader-support#11.4 v=no
why: aria-hidden bar spans in the servo-new fixture are hidden-content, a not_needed scope item the chunk does not touch
@answer n=11 entity=screen-reader-support#11.5 v=no
why: image alt text in the servo fixture is a text alternative in fixtures, a not_needed scope item the chunk does not touch
@answer n=12 entity=screen-reader-support#11.6 v=no
why: a missing alt attribute in a fixture is a text alternative in fixtures, a not_needed scope item the chunk does not touch
@answer n=13 entity=screen-reader-support#12.1 v=no
why: IME enabling and cursor area on text focus are IME and focus behaviour, both not_needed scope items the chunk does not change
@answer n=14 entity=screen-reader-support#12.2 v=no
why: IME enabled by create_text_editor on a pre-focused input is IME behaviour, a not_needed scope item the chunk does not change
@answer n=15 entity=screen-reader-support#12.3 v=no
why: IME commit, preedit and disable events applied to the editor are IME behaviour, a not_needed scope item the chunk does not change
@answer n=16 entity=screen-reader-support#13.1 v=no
why: an absence of accessibility integration over 17 s06 files the chunk does not name, so no rule over those files moves
