# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the WCAG helper serves border bevels and scrollbar thumbs in blitz-paint's color.rs and border.rs, which the chunk does not change; the list's contrast is read through `Harness::text_contrast` (#1.4), not this helper
visual-design-verification#1.2 · no · the default scrollbar thumb stroke (render.rs 757-759, 834-846) and its test in scrollbars.rs; the chunk changes no scrollbar
visual-design-verification#1.3 · no · the near-white author thumb's hover and drag test in scrollbar_drag.rs; scrollbar behaviour the chunk does not touch
visual-design-verification#1.4 · yes · `Harness::text_contrast` reads None under opacity, filter or background-image, so the list's new default pair must be read through it; the three stand_contrast tables and counts (17, 14, 18) move with the flight booker popup's new row
visual-design-verification#1.5 · no · the load-page and Cells tests in stand_contrast.rs are other units of that file that only keep holding; the chunk's new row sits in the select#flight-type tables
visual-design-verification#1.6 · yes · the lint's contrast reading over an open list's options keeps `Harness::text_contrast` and stand_contrast's exempt rule, and the boot reading of no finding on the four lean tasks is re-measured; the finding format is S7's and does not bind this entity
visual-design-verification#1.7 · yes · the `overlaps` exclusion for a closed single select's options, optgroup included, is the lint.rs unit the chunk edits, and the flight booker's boot reading of no `overlap` is re-measured
visual-design-verification#1.8 · yes · default.css is reached whole, and the closed single select's inline-block box, border, padding and background sit beside the new list defaults; select#flight-type's author pair at 17.40 is a stand_contrast row
visual-design-verification#1.9 · yes · default.css is reached whole, and the multiple select's chosen-row pair is a default the new list pair sits beside; select_interaction measures it in both layout modes and the multiple select's rows stay as built
visual-design-verification#2.1 · no · an audit search for `contrast` over 15 s05 files; it states no behaviour of the list and no check the chunk's code or tests must honour
visual-design-verification#2.2 · yes · the engine holds no contrast or user-preference media handling, and the scope names reduced motion and the colours as P4 forks over that absence
visual-design-verification#3.1 · no · the browser urlbar focus border in apps/browser, not reached and outside the stand
visual-design-verification#3.2 · no · the new-tab search input's outline in apps/browser, not reached
visual-design-verification#3.3 · no · todomvc's `:focus` outline in examples/todomvc, not reached
visual-design-verification#3.4 · no · the flight booker's select#flight-type focus border lives in examples/seven_guis, not reached; the chunk changes no select focus style, and the option focus ring is the P4 fork
visual-design-verification#3.5 · no · Cells' grid focus outline in examples/seven_guis, not reached
visual-design-verification#3.6 · no · counter and transparent button focus outlines in examples, not reached
visual-design-verification#3.7 · no · the rdme stylesheet's focus-visible outline in apps/readme, not reached
visual-design-verification#3.8 · yes · default.css is reached whole, and its input focus outline (98-101) and focus-visible suppression (871-877) are the engine's default focus indicator that the option focus ring must answer in the focus contract fork
visual-design-verification#3.9 · no · the google fixture's forced-colors focus rule under examples/assets; the chunk edits no fixture markup
visual-design-verification#3.10 · no · caret and selection paint for focused text inputs in blitz-paint render.rs; the list is not a text input and the rule rules over no list unit
visual-design-verification#4.1 · no · the BBC fixture's reduced-motion queries under examples/assets; the chunk edits no fixture markup
visual-design-verification#5.1 · yes · the absence of prefers-reduced-motion handling over s02 files is the state the reduced-motion P4 fork rules over, and the scope names reduced motion
visual-design-verification#6.1 · no · the disabled icon button opacity of 0.35 in apps/browser, not reached and outside the stand's controls
visual-design-verification#6.2 · no · author `:open` styles reach details and dialog's OPEN state, which the chunk does not touch; the list's open state reaches the tree through aria-expanded, not `:open`
