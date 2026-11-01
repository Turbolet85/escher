# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the WCAG helper lives in blitz-paint's color.rs and render/border.rs, neither reached nor a predicted conflict surface; the chunk restates the helper rather than calling it
visual-design-verification#1.2 · no · the default scrollbar thumb stroke lives in render.rs and scrollbars.rs, neither reached nor a conflict surface, and no scope item names scrollbars
visual-design-verification#1.3 · no · the near-white author thumb's hover and drag reading lives in scrollbar_drag.rs, not reached and outside the scope
visual-design-verification#1.4 · yes · stand_contrast's three tables and 17 rows, the recorded ratios and accent colour, and Harness::text_contrast's None conditions (opacity, filter, background-image), which the scope names as re-read and re-measured
visual-design-verification#1.5 · yes · the load page and Cells ratio tests in stand_contrast, which the scope names as the load page's and Cells' ratios re-measured in both layout modes
visual-design-verification#1.6 · yes · the visual lint's contrast reading goes through the same text_contrast call the scope names, and the at-boot zero reading and the a11y leg's test count over its targets are re-measured
visual-design-verification#1.7 · yes · the open list's rows and closed select options are read through select_list_bounds, closed_select_of and visible_region, which the scope names, and the flight booker's no-overlap reading at boot is a stand check re-run in both modes
visual-design-verification#1.8 · yes · an open list's highlighted row and its contrast reading sit on the select model's list rows that select_popup and stand_act_popup assert, and the at-boot zero overlap reading is re-measured
visual-design-verification#1.9 · yes · the user-agent select's default.css values (inline-block box, input border, padding, background, multiple-select rows) are in a conflict surface, and the stand's select#flight-type pair at 17.40 is read by stand_contrast
visual-design-verification#1.10 · yes · select_popup_painted reads the highlight and disabled row against SelectListView's constants and the open list's plain pair at the default sheet and the stand, and it is the font-embolden witness the scope names
visual-design-verification#1.11 · yes · the default sheet's option:checked chosen-row pair in default.css, a conflict surface, is one of the pairs the select list's default-sheet reading covers
visual-design-verification#1.12 · yes · SelectListView's colour constants, which the scope names explicitly
visual-design-verification#1.13 · no · the one-height row clipping of a larger authored font is a stated limit in docs/capabilities.md that no scoped check reads and the chunk does not need to meet; the scope names the rows' bounds, not their clipping
visual-design-verification#2.1 · no · an observed-absent record of a past search over s05 files, excluded by the Not needed list
visual-design-verification#2.2 · no · an observed-absent record of a past search over s11 files and the recorded absence of contrast and preference media rules, excluded by the Not needed list
visual-design-verification#3.1 · no · the browser urlbar focus style in apps/browser/assets/browser.css is not reached and no scope item names it
visual-design-verification#3.2 · no · the new-tab search input's focus style in apps/browser/assets/about-newtab.css is not reached and no scope item names it
visual-design-verification#3.3 · no · the todomvc `:focus` outline in examples/todomvc/src/todomvc.css is not reached and no scope item names it
visual-design-verification#3.4 · no · the seven_guis input focus outlines in temp_converter.rs and flight_booker.rs are not reached; focus visibility is not a target and no scoped check reads the outline
visual-design-verification#3.5 · yes · Cells' focused-cell style `.cells-grid:focus .cell.current`, which the scope names through the cells.rs conflict
visual-design-verification#3.6 · no · the counter and transparent button focus outlines in examples/counter and examples/transparent are not reached and no scope item names them
visual-design-verification#3.7 · no · the rdme stylesheet focus outline in apps/readme/assets/github-markdown.css is not reached and no scope item names it
visual-design-verification#3.8 · yes · blitz-dom's default sheet input focus outline and focus-visible suppression in default.css, a conflict surface, so the rule moves with the merge
visual-design-verification#3.9 · no · the google fixture's forced-colors outline in examples/assets/google.html is not reached, and forced colours are outside the scope
visual-design-verification#3.10 · yes · a password input's painted bullets, in the input's text colour, and its caret at the bullet index are the painted masked text the scope names through the text.rs and dioxus_document.rs conflicts, measured by password_painted_masked
