# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the bevel and scrollbar-thumb contrast helper in blitz-paint's color.rs and render/border.rs is not reached or named; the harness restates the ratio and blitz-paint stays untouched
visual-design-verification#1.2 · no · the default scrollbar thumb stroke in render.rs and its scrollbars.rs test are neither reached nor named
visual-design-verification#1.3 · no · the near-white author thumb hover and drag test in scrollbar_drag.rs is neither reached nor named
visual-design-verification#1.4 · yes · Harness::text_contrast's None conditions (opacity, filter, background-image) and stand_contrast's three tables, 17 rows, and the accent re-colour record, all named in Needs and re-measured
visual-design-verification#1.5 · yes · the_load_page_meets_its_ratios and cells_meets_its_ratios, the load page's and Cells' ratios named in Needs, in both layout modes
visual-design-verification#1.6 · yes · the visual lint's contrast reading goes through the named Harness::text_contrast call, and the a11y leg's 114 tests over 14 targets is a count the chunk re-measures
visual-design-verification#1.7 · yes · narrowly: clip_region takes an open list row's region from visible_region, a call the scope names through the scrolling.rs conflict; the overlap skip and in_select_box rules in escher-driver are not reached
visual-design-verification#1.8 · yes · the lint's contrast reading of a highlighted row goes through the named text_contrast call, and the highlight pair is SelectListView's constant on the select list's merge surface
visual-design-verification#1.9 · yes · default.css is a conflict surface holding the single and multiple select values (lines 77-86, 155-199) that select_interaction measures, and the flight-type pair stand_contrast reads
visual-design-verification#1.10 · yes · select_popup_painted reads the highlight and disabled rows against SelectListView's constants and measures the open list's plain pair at the default sheet and the stand, all on the select list's merge surface
visual-design-verification#1.11 · yes · the engine-default chosen-row pair in default.css lines 161-199 (a conflict surface) is measured by select_interaction in both layout modes
visual-design-verification#1.12 · yes · SelectListView's four colour constants sit on the select list's merge surface, re-read row by row
visual-design-verification#1.13 · yes · the one-height row limit lives in the select list's rows, on the merge surface re-read row by row, where a moved pin can change the clipping
visual-design-verification#2.1 · no · observed-absent record of a past contrast search over slice files the chunk's named files do not include
visual-design-verification#2.2 · no · observed-absent record of contrast or user-preference media handling over s11 files, which the chunk does not name
visual-design-verification#3.1 · no · the browser urlbar focus border in apps/browser/assets/browser.css is neither reached nor named
visual-design-verification#3.2 · no · the new-tab search input outline in apps/browser/assets/about-newtab.css is neither reached nor named
visual-design-verification#3.3 · no · todomvc's focus outline in examples/todomvc is neither reached nor named, and focus visibility is outside SC 2.1.1, 1.4.3 and 2.4.3
visual-design-verification#3.4 · no · the seven_guis input and flight-type focus borders in temp_converter.rs and flight_booker.rs are neither reached nor named as a value the scope reads
visual-design-verification#3.5 · yes · Cells' focused-cell style (the grid's current-cell outline in cells.rs) is named in the cells.rs conflict scope
visual-design-verification#3.6 · no · the counter and transparent button focus outlines are in examples not reached or named
visual-design-verification#3.7 · no · the rdme focus-visible outline in apps/readme is neither reached nor named
visual-design-verification#3.8 · yes · default.css is a conflict surface and the input focus outline and focus-visible suppression rules (lines 98-101, 882-888) live in it
visual-design-verification#3.9 · no · the google fixture's forced-colors focus rule is neither reached nor named, and no media handling is in scope
visual-design-verification#3.10 · yes · the password input's bullets are painted in the input's text colour through the text.rs conflict (stroke_text_as), a named painted-text surface; the caret and scroll lines in render.rs are not reached
