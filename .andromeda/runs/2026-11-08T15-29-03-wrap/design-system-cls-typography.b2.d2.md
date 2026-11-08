@answer n=1 entity=typography#3.4 v=no
why: its line-height mapping cites blitz-dom and layout files the session does not reach.
@answer n=2 entity=typography#3.5 v=no
why: its select row-height rule cites blitz-dom document.rs, which the session does not reach, and the actions read does not change row height.
@answer n=3 entity=typography#3.6 v=no
why: its letter and word spacing rule cites stylo_to_parley.rs, which the session does not reach.
@answer n=4 entity=typography#3.7 v=no
why: its text locale rule cites stylo files the session does not reach.
@answer n=5 entity=typography#3.8 v=no
why: its text-align and wrap mapping cites stylo_to_parley.rs, which the session does not reach.
@answer n=6 entity=typography#3.9 v=no
why: its ch and ic metric rule cites font_metrics.rs, which the session does not reach.
@answer n=7 entity=typography#3.10 v=no
why: its stylist seeding and rem rule cites document.rs and rem_after_viewport_change.rs, neither of which the session reaches.
@answer n=8 entity=typography#3.11 v=no
why: its list marker rule cites layout/list.rs, which the session does not reach.
@answer n=9 entity=typography#3.12 v=no
why: its emboldening strength rule cites blitz-paint text.rs and Cargo files the session does not reach, and the refusal moved elsewhere.
@answer n=10 entity=typography#3.13 v=no
why: its select group header bold rule cites blitz-paint text.rs and select_popup_painted.rs, neither of which the session reaches.
@answer n=11 entity=typography#3.14 v=no
why: its text-decoration thickness and position rule cites blitz-paint text.rs, which the session does not reach.
@answer n=12 entity=typography#4.1 v=no
why: its font-family examples cite example asset and custom_widget files the session does not reach.
@answer n=13 entity=typography#4.2 v=no
why: its graphite.html font and size values cite an example asset the session does not reach.
@answer n=14 entity=typography#4.3 v=no
why: its gosub, servo-new-reduced and pseudo font examples cite example assets the session does not reach.
@answer n=15 entity=typography#4.4 v=no
why: its servo.css font stacks cite an example asset the session does not reach.
@answer n=16 entity=typography#4.5 v=no
why: its custom-widget and Preact font values cite example files the session does not reach.
@answer n=17 entity=typography#4.6 v=no
why: its text-decoration fixture cites an example asset the session does not reach.
@answer n=18 entity=typography#5.1 v=yes mode=stale
why: the fonts-unavailable screenshot refusal test files are reached whole, so the font-family search over the s08 files is not re-run and may now match.
@answer n=19 entity=typography#5.2 v=yes mode=stale
why: the fonts-unavailable screenshot refusal test files are reached whole, so the font family search over the s09 files is not re-run and may now match.
@answer n=20 entity=typography#5.3 v=yes mode=stale
why: the fonts-unavailable screenshot refusal test files are reached whole, so the font-family search over the s10 slice files is not re-run and its one match may move.
@answer n=21 entity=typography#5.4 v=yes mode=stale
why: the fonts-unavailable screenshot refusal test files are reached whole, so the font-family search over the s12 slice files is not re-run and its one fixture hit may move.
