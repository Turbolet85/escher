@answer n=1 entity=typography#1.1 v=no
why: the session changed no browser chrome CSS or apps/browser file, and the new test target and lock bump do not reach apps/browser/assets/browser.css
@answer n=2 entity=typography#1.2 v=no
why: the session changed no apps/browser suggestion styling, and browser.css:329-336 is not in any file the change reached
@answer n=3 entity=typography#1.3 v=no
why: the FPS overlay rule at browser.css:269-270 sits in an apps/browser file the session did not touch
@answer n=4 entity=typography#1.4 v=no
why: about-history.css is an apps/browser asset the session did not change, and the session moved no font stack there
@answer n=5 entity=typography#1.5 v=no
why: the rdme body font stack and sizes in github-markdown.css are unchanged, and the pulldown-cmark 0.13.6 bump changes the markdown parser, not the typography rule
@answer n=6 entity=typography#1.6 v=no
why: the blitz-dom default.css form, heading and code rules are unchanged, since no blitz-dom file moved in the session
@answer n=7 entity=typography#1.7 v=no
why: the blitz-dom default heading rules in default.css are unchanged, and the session touched no blitz-dom file
@answer n=8 entity=typography#1.8 v=no
why: the blitz-dom default code and pre family in default.css is unchanged, and the session moved no font rule
@answer n=9 entity=typography#1.9 v=no
why: the select multiple line-height rule at default.css:183-193 is unchanged, since no blitz-dom asset moved
@answer n=10 entity=typography#1.10 v=no
why: the todomvc stylesheet is an example the session did not change, and no example file moved
@answer n=11 entity=typography#1.11 v=no
why: the wgpu_texture styles.css is an example the session did not change
@answer n=12 entity=typography#1.12 v=no
why: the PlainEditor base size at blitz-dom text.rs:86 is unchanged, since the session touched no text-measuring code
@answer n=13 entity=typography#1.13 v=no
why: the engine generic base sizes in font_metrics.rs:167-176 are unchanged, since no font or text-measuring code changed
@answer n=14 entity=typography#1.14 v=no
why: the list bullet font rules in blitz-dom list.rs are unchanged, since no layout or font file moved in the session
@answer n=15 entity=typography#2.1 v=no
why: the WASM DejaVu Sans registration in examples/wasm_hello/src/lib.rs:75-100 is unchanged, and the session touched no example or font code
@answer n=16 entity=typography#2.2 v=no
why: the stand's font registration lives in the architecture document, and the run stand stays unmoved at 237 over 51 files
@answer n=17 entity=typography#2.3 v=no
why: build_single_font_ctx and the config.rs font context code are unchanged, since no font or text-measuring code moved
@answer n=18 entity=typography#2.4 v=no
why: the font features in packages/dioxus-native/Cargo.toml:20-25 and :47-48 are unchanged, and the session changed no dioxus-native manifest
@answer n=19 entity=typography#2.5 v=no
why: the bullet font registration at blitz-dom document.rs:469-471 and lib.rs:32 is unchanged, since no blitz-dom file moved
@answer n=20 entity=typography#2.6 v=no
why: the fonts-unavailable refusal at blitz-dom document.rs:1522-1535 is unchanged, and the session changed no screenshot or font-check code
@answer n=21 entity=typography#2.7 v=no
why: the stub FontFaceSet in packages/blitz-vibey-script fonts.js is unchanged, since the session touched no script runtime file
@answer n=22 entity=typography#2.8 v=no
why: the new probe target leaves the blitz-tests system-fonts feature unchanged, and tests/blitz-tests/Cargo.toml:17 is not in any file the change reached
@answer n=23 entity=typography#3.1 v=no
why: the generic family mapping in stylo_to_parley.rs:50-60 is unchanged, since no font or text-measuring code moved
@answer n=24 entity=typography#3.2 v=no
why: the weight, width and style conversion in stylo_to_parley.rs:87-112 is unchanged, since no text-measuring code moved
@answer n=25 entity=typography#3.3 v=no
why: the OpenType feature tag mapping in stylo_to_parley.rs:134-262 is unchanged, since no text-measuring code moved
