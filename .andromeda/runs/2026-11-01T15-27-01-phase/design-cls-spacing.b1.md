# Classified — design · spacing.b1

## Answers
spacing#1.1 · no · the rdme spacing tokens live in apps/readme/assets/github-markdown.css, a file of an app the scope does not name (Not needed)
spacing#2.1 · no · browser urlbar, tab and menu values live in apps/browser/assets/browser.css, which the scope does not name (Not needed)
spacing#2.2 · no · about:history values live in apps/browser/assets/about-history.css, which the scope does not name (Not needed)
spacing#2.3 · no · readme markdown body values live in apps/readme/assets/blitz-markdown-overrides.css, which the scope does not name (Not needed)
spacing#2.4 · no · a pointer to layout-templates for the seven_guis card and Home padding, with no value of its own; the cells.rs conflict edit does not reach those values
spacing#2.5 · yes · the blitz-dom body margin sits in default.css, a conflict surface the Needs read against the resolved sheet
spacing#2.6 · yes · the multiple-select row padding sits in default.css, a conflict surface the Needs read against the resolved sheet
spacing#2.7 · yes · the multiple-select size hint sits in stylo.rs, which the Needs name, and its overflow rule sits in default.css, a conflict surface
spacing#3.1 · yes · the hspace and vspace margin mapping sits in stylo.rs, a predicted conflict surface
spacing#3.2 · yes · the body margin attribute mapping sits in stylo.rs, a predicted conflict surface
spacing#3.3 · no · overlay scrollbar thumb geometry sits in scrollbar.rs, which the chunk does not reach or name, and the scrolling.rs scope does not cover it
spacing#3.4 · yes · MAX_ROWS and the engine box and row constants are the open select list's measured geometry, on the select list's row-by-row re-read, and select_popup.rs witnesses them under a take
spacing#3.5 · yes · the place rules for an open select list (its width bounds, finite edges, side choice) sit in select_list.rs, the select list's merge surface re-read row by row
spacing#3.6 · yes · the vertical centring of single-line inputs sits in node/node.rs, a predicted conflict surface
spacing#3.7 · no · the outside list marker offset sits in blitz-paint render.rs, which the chunk does not reach or name; the named text-paint needs cover text.rs only
spacing#3.8 · yes · the double text decoration offset sits in blitz-paint text.rs, a predicted conflict surface, which f76777766 also changes
spacing#4.1 · no · fixture values of examples/assets, an example other than examples/seven_guis that the scope names no change to (Not needed)
spacing#4.2 · no · fixture values of examples/assets/graphite.html, an example other than examples/seven_guis that the scope names no change to (Not needed)
spacing#4.3 · no · fixture values of examples/assets/hr.html, an example other than examples/seven_guis that the scope names no change to (Not needed)
spacing#4.4 · no · fixture values of examples/assets/servo.css, an example other than examples/seven_guis that the scope names no change to (Not needed)
spacing#4.5 · no · fixture values of examples/preact, an example other than examples/seven_guis that the scope names no change to (Not needed)
