# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · blitz-paint's own helper for border bevels and scrollbar thumbs, which the chunk does not call or edit; the lint's formula comes from the harness (#1.4)
visual-design-verification#1.2 · no · a default scrollbar thumb's contrast stroke in blitz-paint and its scrollbars.rs test, outside the findings walk and the two touched tests
visual-design-verification#1.3 · no · a near-white author scrollbar thumb's hover and drag change in scrollbar_drag.rs, which the chunk does not reach
visual-design-verification#1.4 · yes · the lint's contrast defect reads Harness::text_contrast under the exempt rule (enabled reads Some(false)) and the 4.5 and 3.0 thresholds the Needs keep, and stand_lint's boot re-run reads the control pairs this records
visual-design-verification#1.5 · yes · the lint's own contrast defect (direct text or a text control, read through text_contrast, exempt where enabled reads Some(false), a None reading not measured, findings written id, defect, bounds, ratio, required) is the walk the chunk refactors
visual-design-verification#1.6 · yes · the flight booker's one boot overlap (flight-type-one-way and flight-type-return, each option on its select's client rect) is the reading the Needs keep unchanged through the refactored walk
visual-design-verification#1.7 · yes · the flight booker's author-styled select pair (#1a1a1a on #ffffff, 15px, 17.40 against 4.5) bears on the zero-contrast boot reading stand_lint re-runs; the user-agent pair it replaces is not measured
visual-design-verification#2.1 · no · an observed absence of contrast checks in the s05 files, which the chunk neither adds nor re-searches; the contrast rules it needs are carried by #1.4 and #1.5
visual-design-verification#2.2 · no · an observed absence of contrast and user-preference media handling in the s11 files, which the chunk adds no media query to and the lint never reads
visual-design-verification#3.1 · no · the browser app's urlbar focus border and outline, outside the chunk; the lint and the two touched tests read no outline
visual-design-verification#3.2 · no · the browser's new-tab search input, which drops its outline on focus, an app the chunk does not reach
visual-design-verification#3.3 · no · todomvc's :focus outline of 0, an example the chunk does not reach
visual-design-verification#3.4 · no · seven_guis focus borders, the flight booker's select included; the lint reads no focus style and the focus-ring section is Not needed
visual-design-verification#3.5 · no · counter and transparent buttons' 4px focus outline, a focus-ring style the lint does not read
visual-design-verification#3.6 · no · rdme's 2px focus-visible outline, a viewer stylesheet the chunk does not reach
visual-design-verification#3.7 · no · blitz-dom's default input focus outline and its iframe, body and html suppression, an engine style the chunk does not change
visual-design-verification#3.8 · no · the google fixture's forced-colors focus outlines, an example page the chunk does not reach
visual-design-verification#3.9 · no · focused text-input caret and selection painting in blitz-paint, the password caret included, which the chunk does not reach
visual-design-verification#4.1 · no · the BBC fixture's prefers-reduced-motion queries, a motion fixture outside the chunk
visual-design-verification#5.1 · no · an observed absence of prefers-reduced-motion and prefers-color-scheme rules in the s02 files, a motion and colour-scheme record the chunk does not touch
visual-design-verification#6.1 · no · the browser app's disabled icon buttons at 0.35 opacity, outside the stand; disabled nodes are exempt from the lint's contrast read
visual-design-verification#6.2 · no · the WPT runner's printed status words, an app the chunk does not reach
visual-design-verification#6.3 · no · a comment in the form example about input colour, an example the chunk does not reach
visual-design-verification#7.1 · no · the viewport's colour scheme and zoom fields, which the chunk neither sets nor reads; the colour scheme and zoom sections are Not needed
visual-design-verification#7.2 · no · the viewport's colour-scheme change tracked as a stylist device rebuild in blitz-dom, which the chunk does not reach
