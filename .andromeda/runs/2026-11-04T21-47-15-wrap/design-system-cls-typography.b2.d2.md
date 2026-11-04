@answer n=1 entity=typography#3.4 v=no
why: the line-height mapping lives in packages/blitz-dom stylo_to_parley.rs and layout/mod.rs, which the session did not change
@answer n=2 entity=typography#3.5 v=no
why: the select-list row height lives in packages/blitz-dom document.rs and the select-popup report, and neither file is reached
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing resolve in packages/blitz-dom stylo_to_parley.rs, which the session did not change
@answer n=4 entity=typography#3.7 v=no
why: text locale resolves in packages/blitz-dom stylo_to_parley.rs and stylo.rs, which the session did not change
@answer n=5 entity=typography#3.8 v=no
why: text-align and wrap mapping lives in packages/blitz-dom stylo_to_parley.rs, which the session did not change
@answer n=6 entity=typography#3.9 v=no
why: ch and ic metrics live in packages/blitz-dom font_metrics.rs, which the session did not change
@answer n=7 entity=typography#3.10 v=no
why: rem seeding lives in packages/blitz-dom document.rs and rem_after_viewport_change.rs, and the session reaches neither
@answer n=8 entity=typography#3.11 v=no
why: list markers live in packages/blitz-dom layout/list.rs, which the session did not change
@answer n=9 entity=typography#3.12 v=no
why: emboldening lives in packages/blitz-paint Cargo.toml and text.rs, no Cargo.toml changed, and the Cargo.lock bump moves only pulldown-cmark
@answer n=10 entity=typography#3.13 v=no
why: the synthesized bold header lives in packages/blitz-paint text.rs and select_popup_painted.rs, and the session reaches neither
@answer n=11 entity=typography#3.14 v=no
why: text-decoration thickness and position live in packages/blitz-paint text.rs, which the session did not change
@answer n=12 entity=typography#4.1 v=no
why: the font-family fixtures under examples/assets and examples/ are untouched, since no example changed
@answer n=13 entity=typography#4.2 v=no
why: the graphite fixture under examples/assets is untouched, since no example changed
@answer n=14 entity=typography#4.3 v=no
why: the gosub, servo-new-reduced and pseudo fixtures under examples are untouched, since no example changed
@answer n=15 entity=typography#4.4 v=no
why: the servo.css and servo.html fixtures under examples/assets are untouched, since no example changed
@answer n=16 entity=typography#4.5 v=no
why: the custom_widget.rs and preact fixtures under examples are untouched, since no example changed
@answer n=17 entity=typography#4.6 v=no
why: the text-decoration fixture under examples/assets is untouched, since no example changed
@answer n=18 entity=typography#5.1 v=no
why: the new probe file reads Dioxus node state and sets no font, and the report keeps the font feature unchanged
@answer n=19 entity=typography#5.2 v=no
why: the new probe file reads Dioxus node state and sets no font, and the report keeps the font feature unchanged
@answer n=20 entity=typography#5.3 v=no
why: the new probe file reads Dioxus node state and sets no font, and the report keeps the font feature unchanged
@answer n=21 entity=typography#5.4 v=no
why: the new probe file reads Dioxus node state and sets no font, and the report keeps the font feature unchanged
