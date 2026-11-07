@answer n=1 entity=typography#3.4 v=no
why: the session reaches only dioxus-native-dom bridge files, and the line-height mapping in stylo_to_parley.rs and layout/mod.rs did not change.
@answer n=2 entity=typography#3.5 v=no
why: the open select list row layout in blitz-dom document.rs is not a file the session reached, and the select font path is untouched.
@answer n=3 entity=typography#3.6 v=no
why: letter and word spacing resolution in stylo_to_parley.rs is untouched by the session.
@answer n=4 entity=typography#3.7 v=no
why: text locale from the lang attribute runs through stylo.rs and stylo_to_parley.rs, neither of which the session reached.
@answer n=5 entity=typography#3.8 v=no
why: text-align and wrap mappings in stylo_to_parley.rs are untouched, and the session changed no text layout setting.
@answer n=6 entity=typography#3.9 v=no
why: ch and ic metric measurement in font_metrics.rs is untouched; the session touched no text measuring path.
@answer n=7 entity=typography#3.10 v=no
why: the stylist root font-size seeding in blitz-dom document.rs and the rem test are not reached; the session's reached document file is dioxus_document.rs in a different crate.
@answer n=8 entity=typography#3.11 v=no
why: list marker mapping in layout/list.rs is untouched by the session.
@answer n=9 entity=typography#3.12 v=no
why: font emboldening in blitz-paint text.rs and Cargo features is untouched; paint is unchanged and no dependency moved.
@answer n=10 entity=typography#3.13 v=no
why: the select group label RunPaint path in blitz-paint text.rs is untouched by the session.
@answer n=11 entity=typography#3.14 v=no
why: text-decoration thickness and offset in blitz-paint text.rs is untouched by the session.
@answer n=12 entity=typography#4.1 v=no
why: the example font-family declarations in the cited asset files are not reached and no CSS or stylesheet changed.
@answer n=13 entity=typography#4.2 v=no
why: the graphite example's font stack and @font-face rules are in an asset file the session did not reach, and no stylesheet changed.
@answer n=14 entity=typography#4.3 v=no
why: the gosub and servo-new-reduced example font declarations are untouched assets, and no stylesheet changed.
@answer n=15 entity=typography#4.4 v=no
why: the servo.css font stacks and Google Fonts load are untouched assets, and no stylesheet changed.
@answer n=16 entity=typography#4.5 v=no
why: the custom-widget and Preact system font stacks are in example files the session did not reach, and no font stack changed.
@answer n=17 entity=typography#4.6 v=no
why: the text-decoration fixture is an untouched asset, and the session changed no text decoration rule.
@answer n=18 entity=typography#5.1 v=no
why: the absence of font-family declarations still holds, since the session added no font-family, token or stylesheet.
@answer n=19 entity=typography#5.2 v=no
why: the absence of font family declarations over the s09 files still holds, since the session added no font declaration.
@answer n=20 entity=typography#5.3 v=no
why: the absence of font-family declarations outside test fixtures still holds, since the session added no font declaration.
@answer n=21 entity=typography#5.4 v=no
why: the absence of a project type scale or font tokens still holds, since the session added no token or font declaration.
