@answer n=1 entity=spacing#1.1 v=no
why: the rdme spacing tokens live in apps/readme/assets/github-markdown.css, a stylesheet the session did not touch, and the pulldown-cmark bump changes parsing, not CSS values
@answer n=2 entity=spacing#2.1 v=no
why: apps/browser/assets/browser.css is unchanged, and the zip 2.4.2 dependency pulled by anyrender_serialize moves no padding or gap value
@answer n=3 entity=spacing#2.2 v=no
why: apps/browser/assets/about-history.css is unchanged by the session
@answer n=4 entity=spacing#2.3 v=no
why: apps/readme/assets/blitz-markdown-overrides.css is unchanged, and the pulldown-cmark 0.13.6 bump does not alter its max-width or padding
@answer n=5 entity=spacing#2.4 v=no
why: the card and Home padding is held in layout-templates, which the session did not touch
@answer n=6 entity=spacing#2.5 v=no
why: packages/blitz-dom/assets/default.css is unchanged by the session
@answer n=7 entity=spacing#2.6 v=no
why: the blitz-dom multiple-select padding rule in packages/blitz-dom/assets/default.css is unchanged by the session
@answer n=8 entity=spacing#2.7 v=no
why: packages/blitz-dom/src/node/select_list.rs, stylo.rs and default.css are unchanged, and no Cargo.lock change reaches the size_rows hint
@answer n=9 entity=spacing#3.1 v=no
why: the hspace and vspace mapping in packages/blitz-dom/src/stylo.rs is unchanged by the session
@answer n=10 entity=spacing#3.2 v=no
why: the body margin mapping in packages/blitz-dom/src/stylo.rs is unchanged by the session
@answer n=11 entity=spacing#3.3 v=no
why: the overlay scrollbar thickness and margin constants in packages/blitz-dom/src/node/scrollbar.rs are unchanged by the session
@answer n=12 entity=spacing#3.4 v=no
why: the select popup constants and MAX_ROWS are in select_popup.rs and select_list.rs, neither changed, and the new dioxus_runtime_probe.rs target does not touch them
@answer n=13 entity=spacing#3.5 v=no
why: the place logic in packages/blitz-dom/src/node/select_list.rs is unchanged by the session
@answer n=14 entity=spacing#3.6 v=no
why: the single-line input centering in packages/blitz-dom/src/node/node.rs is unchanged by the session
@answer n=15 entity=spacing#3.7 v=no
why: the list marker padding in packages/blitz-paint/src/render.rs is unchanged by the session
@answer n=16 entity=spacing#3.8 v=no
why: the double text decoration offset in packages/blitz-paint/src/text.rs is unchanged by the session
@answer n=17 entity=spacing#4.1 v=no
why: the example gap and padding values in examples/assets/border-styles.html and cursor.html are unchanged by the session
@answer n=18 entity=spacing#4.2 v=no
why: the graphite.html root variables and their media queries are unchanged by the session
@answer n=19 entity=spacing#4.3 v=no
why: the hr margin and body padding in examples/assets/hr.html are unchanged by the session
@answer n=20 entity=spacing#4.4 v=no
why: the servo.css column gap, container and button padding are unchanged by the session
@answer n=21 entity=spacing#4.5 v=no
why: the Preact TodoMVC and reference page spacing values in examples/preact are unchanged by the session
