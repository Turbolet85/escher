@answer n=1 entity=typography#3.4 v=no
why: the session changed no line-height or layout code in packages/blitz-dom, and its Cargo.lock change moves only pulldown-cmark
@answer n=2 entity=typography#3.5 v=no
why: the select list row-height rule lives in packages/blitz-dom/src/document.rs, which the session did not change
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing resolution in packages/blitz-dom/src/stylo_to_parley.rs is untouched by the session
@answer n=4 entity=typography#3.7 v=no
why: the locale mapping from the lang attribute in stylo_to_parley.rs and stylo.rs is untouched by the session
@answer n=5 entity=typography#3.8 v=no
why: the text-align and wrap mappings in stylo_to_parley.rs are untouched, and the session reaches no paint or text file
@answer n=6 entity=typography#3.9 v=no
why: the ch and ic metric code in packages/blitz-dom/src/font_metrics.rs is untouched by the session
@answer n=7 entity=typography#3.10 v=no
why: the stylist device seeding in document.rs and the rem test file are not among the session's reached files, and the report says no font or text code changed
@answer n=8 entity=typography#3.11 v=no
why: the list marker mapping in packages/blitz-dom/src/layout/list.rs is untouched by the session
@answer n=9 entity=typography#3.12 v=no
why: the emboldening rules in packages/blitz-paint are untouched, and the session changed no blitz-paint feature or source
@answer n=10 entity=typography#3.13 v=no
why: the group header bold rule in packages/blitz-paint/src/text.rs and its select_popup_painted.rs test are untouched by the session
@answer n=11 entity=typography#3.14 v=no
why: the text-decoration thickness and overline position code in packages/blitz-paint/src/text.rs is untouched by the session
@answer n=12 entity=typography#4.1 v=no
why: the session moved no example asset or fixture, and the Cargo.lock change does not reach font declarations
@answer n=13 entity=typography#4.2 v=no
why: the graphite example stylesheet and its font rules are untouched by the session
@answer n=14 entity=typography#4.3 v=no
why: the gosub and servo-new-reduced-1 example fixtures are untouched by the session
@answer n=15 entity=typography#4.4 v=no
why: the servo.css and servo.html example assets are untouched by the session
@answer n=16 entity=typography#4.5 v=no
why: the custom widget, Preact and reference example files are untouched by the session
@answer n=17 entity=typography#4.6 v=no
why: the text-decoration fixture is untouched by the session
@answer n=18 entity=typography#5.1 v=no
why: the new probe target reads listeners and state and the report says the blitz-tests font feature is unchanged, so the font-family search still finds no declaration
@answer n=19 entity=typography#5.2 v=no
why: the new probe target carries no font declaration, as the report's untouched font item states, so the font-family search still holds
@answer n=20 entity=typography#5.3 v=no
why: the probe target is a Dioxus listener and state probe with no font-family declaration, so the single packages/blitz-vibey-script match is unchanged
@answer n=21 entity=typography#5.4 v=no
why: the session records no type scale or font token, and the new probe target adds no font-family string to the searched slice
