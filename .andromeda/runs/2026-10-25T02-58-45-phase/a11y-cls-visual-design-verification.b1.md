# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · blitz-paint's helper for border bevels and scrollbar thumbs, where the statement claims only the harness's restated WCAG ratio (#1.4) and blitz-paint stays untouched
visual-design-verification#1.2 · no · default scrollbar thumb contrast stroke, a non-text paint outside the stand and outside the tier-0 reach the statement states
visual-design-verification#1.3 · no · near-white scrollbar thumb change on hover and drag, a scrollbar behaviour outside the statement's reach
visual-design-verification#1.4 · yes · the stand's SC 1.4.3 record (stand_contrast's 18 control rows, 14 TEXT_ROWS and 19 STATE_ROWS, the exempt rule enabled Some(false) and the 5.47 accent) is the contrast reach the statement states
visual-design-verification#1.5 · yes · the lint's contrast defect through Harness::text_contrast under the same exempt rule, and the fourteen-file a11y leg's 84 tests, are both named in the statement's Needs
visual-design-verification#1.6 · no · a select's default border, padding and background and its unmeasured text pair, where the statement's select limits are multiple, optgroup and no popup and the stand holds no select
visual-design-verification#2.1 · no · a surface search of 15 s05 files for contrast, not a claim the statement makes, since its contrast reach is the stand's measured rows (#1.4)
visual-design-verification#2.2 · no · absent contrast or prefers-* media handling, in the user-preference sections the statement says it makes no claim about
visual-design-verification#3.1 · no · browser urlbar focus border and outline, in the focus-ring section the statement says it makes no claim about
visual-design-verification#3.2 · no · new-tab search input outline removed on focus, in the focus-ring section the statement makes no claim about
visual-design-verification#3.3 · no · todomvc focus outline set to 0, in the focus-ring section the statement makes no claim about
visual-design-verification#3.4 · no · seven_guis inputs signal focus by border colour or box-shadow, in the focus-ring section the statement makes no claim about
visual-design-verification#3.5 · no · counter and transparent buttons' 4px focus outline, in the focus-ring section the statement makes no claim about
visual-design-verification#3.6 · no · rdme focus-visible outline in apps/readme, in the focus-ring section the statement makes no claim about
visual-design-verification#3.7 · no · blitz-dom default input focus outline and its focus-visible suppression, in the focus-ring section the statement makes no claim about
visual-design-verification#3.8 · no · google fixture forced-colors focus outlines, a user-preference section the statement makes no claim about
visual-design-verification#3.9 · yes · a focused password input paints its bullets in the input's text colour, witnessed by password_painted_masked.rs:117-131, which is the password-painted-as-bullets claim the statement carries for a shot (the caret clause rides with it)
visual-design-verification#4.1 · no · the BBC fixture's prefers-reduced-motion queries, in the motion section the statement says it makes no claim about
visual-design-verification#5.1 · no · absent prefers-reduced-motion and prefers-color-scheme rules, in the motion and colour-scheme sections the statement makes no claim about
visual-design-verification#6.1 · no · browser disabled icon buttons at 0.35 opacity in apps/browser, outside the stand (the statement states the opacity None case in general, #1.4)
visual-design-verification#6.2 · no · WPT runner prints its status word beside its colour, a status cue the statement excludes and a surface it does not reach
visual-design-verification#6.3 · no · a comment in the form example on accent-color, in examples/form.rs outside the chunk's reach
visual-design-verification#7.1 · no · the viewport's colour scheme and zoom fields, in the colour-scheme and zoom sections the statement makes no claim about (its Light and scale 1 already read in #1.4)
visual-design-verification#7.2 · no · colour-scheme changes tracked as a device change, in a colour-scheme section the statement makes no claim about
visual-design-verification#7.3 · no · scale and zoom changes invalidating text shaping, in a zoom section the statement makes no claim about
