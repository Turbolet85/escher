@answer n=1 entity=typography#3.4 v=no
why: the line-height rule lives in blitz-dom stylo_to_parley.rs and layout/mod.rs, neither reached by the session, and the task records blitz-dom and layout unchanged
@answer n=2 entity=typography#3.5 v=no
why: the select row-height rule lives in blitz-dom document.rs, not a reached file, and the task records layout unchanged
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing resolve in blitz-dom stylo_to_parley.rs, not a reached file, and no styling moved
@answer n=4 entity=typography#3.7 v=no
why: text locale resolves in blitz-dom stylo_to_parley.rs and stylo.rs, neither reached by the session
@answer n=5 entity=typography#3.8 v=no
why: text-align and wrap mapping live in blitz-dom stylo_to_parley.rs, not a reached file
@answer n=6 entity=typography#3.9 v=no
why: ch and ic metrics live in blitz-dom font_metrics.rs, not a reached file
@answer n=7 entity=typography#3.10 v=no
why: the stylist seeding lives in blitz-dom document.rs and its test in tests/blitz-tests/tests/rem_after_viewport_change.rs, neither reached, and blitz-dom is unchanged
@answer n=8 entity=typography#3.11 v=no
why: list markers live in blitz-dom layout/list.rs, not a reached file
@answer n=9 entity=typography#3.12 v=no
why: font emboldening lives in blitz-paint Cargo.toml, lib.rs and text.rs, none reached, and no dependency or feature changed
@answer n=10 entity=typography#3.13 v=no
why: the group header paint lives in blitz-paint text.rs and tests/blitz-tests/tests/select_popup_painted.rs, neither reached
@answer n=11 entity=typography#3.14 v=no
why: text-decoration thickness and position live in blitz-paint text.rs, not a reached file
@answer n=12 entity=typography#4.1 v=no
why: the body and code font-family rules live in examples/assets fixtures, none reached, since the session touches seven_guis and dioxus-native-dom only
@answer n=13 entity=typography#4.2 v=no
why: the graphite font rules live in examples/assets/graphite.html, not a reached file
@answer n=14 entity=typography#4.3 v=no
why: the gosub, servo-new-reduced and pseudo font rules live in examples/assets fixtures, not reached
@answer n=15 entity=typography#4.4 v=no
why: the servo font stylesheet lives in examples/assets, not a reached file
@answer n=16 entity=typography#4.5 v=no
why: examples/custom_widget.rs and examples/preact are not reached by the session
@answer n=17 entity=typography#4.6 v=no
why: the text-decoration fixture lives in examples/assets, not a reached file
@answer n=18 entity=typography#5.1 v=no
why: the session adds no font-family declaration and the task records font unchanged, so the no-match over the s08 files still holds
@answer n=19 entity=typography#5.2 v=no
why: the session adds no font declaration and the task records font unchanged, so the no-match over the s09 files still holds
@answer n=20 entity=typography#5.3 v=no
why: the only match stays in packages/blitz-vibey-script/tests/dom.rs, which the session does not reach, and it adds no font-family declaration
@answer n=21 entity=typography#5.4 v=no
why: the session adds no font token and the task records font unchanged, so the no-match over the s12 files still holds
