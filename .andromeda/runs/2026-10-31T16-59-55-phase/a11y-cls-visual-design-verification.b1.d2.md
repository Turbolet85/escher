# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the WCAG ratio helper in blitz-paint's color.rs is cited for border bevels and scrollbar thumbs, which the chunk does not edit; the chunk's own reader is the harness's restated `contrast_ratio`
visual-design-verification#1.2 · no · the scrollbar thumb contrast stroke in blitz-paint render.rs and scrollbars.rs is a scrollbar rule; the list adds no scrollbar
visual-design-verification#1.3 · no · the near-white author scrollbar thumb hover and drag rule in scrollbar_drag.rs is unrelated to the list
visual-design-verification#1.4 · yes · the `text_contrast` reader rules (opacity, filter, background-image returning `None`, the 3.0 and 4.5 thresholds) and stand_contrast's 17, 14 and 18 row tables, which move with any new `select#flight-type` or popup row
visual-design-verification#1.5 · no · the load page and Cells ratio tests are another job of stand_contrast, not one of the three tables the chunk moves, and the list never opens on either page
visual-design-verification#1.6 · yes · the lint's `contrast` reading through `Harness::text_contrast` with the `enabled == Some(false)` exemption, which the chunk keeps, and the boot reading `[0, 0, 0, 0]` with no finding of any kind
visual-design-verification#1.7 · yes · the driver lint's `overlaps` exclusion for a closed single select's options (`closed_select_of`, `in_closed_select`, the optgroup case) that the chunk restates, with the snapshot's client-rect bounds it edits
visual-design-verification#1.8 · yes · the closed single select's display rules (chosen option inline, others hidden) that the open list must paint over, and the multiple select's block rows that stay as built, with the stand's author CSS on `select#flight-type` read at 17.40
visual-design-verification#1.9 · yes · the engine holds no tokens, so the list's new default colour pair is hardcoded beside the multiple select's chosen-row pair, which must stay at 12.56 as built
visual-design-verification#2.1 · no · an absent-contrast-checks observation over older s05 files; the chunk's checks sit in stand_contrast and stand_lint, not those files
visual-design-verification#2.2 · yes · states the engine holds no reduced-motion or contrast-preference handling, the basis of the scope's named P4 forks for list motion and list colours under SC 1.4.3
visual-design-verification#3.1 · no · the browser urlbar focus border in apps/browser, which the chunk does not touch
visual-design-verification#3.2 · no · the browser new-tab search input outline in apps/browser, which the chunk does not touch
visual-design-verification#3.3 · no · todomvc's `:focus` outline rule in examples/todomvc, which the chunk does not touch
visual-design-verification#3.4 · no · the flight booker's author focus border on `select#flight-type` in examples/seven_guis, a stand rule the chunk neither edits nor names; the list's focus ring is for options
visual-design-verification#3.5 · no · Cells' grid focus outline, not reached by the chunk
visual-design-verification#3.6 · no · counter and transparent button focus outlines in examples, not reached by the chunk
visual-design-verification#3.7 · no · the rdme stylesheet focus outline in apps/readme, not reached by the chunk
visual-design-verification#3.8 · no · default.css's input focus outline and the iframe, body and html suppression, which govern inputs and not a select or option
visual-design-verification#3.9 · no · the google fixture's forced-colours outline in examples/assets, a fixture the chunk edits none of
visual-design-verification#3.10 · no · text-input caret and selection painting in blitz-paint render.rs, which the list's overlay paint does not change
visual-design-verification#4.1 · no · the BBC fixture's reduced-motion queries in examples/assets, a fixture markup inventory the chunk does not edit
visual-design-verification#5.1 · yes · records the absence of prefers-reduced-motion rules in the engine, the basis of the scope's named P4 fork on list motion
visual-design-verification#6.1 · no · disabled icon button opacity in apps/browser, not reached by the chunk
visual-design-verification#6.2 · no · `:open` matching for details and dialog through stylo.rs, which the chunk neither uses nor edits for the select's open state
