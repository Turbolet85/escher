# Classified — a11y · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the bevel and scrollbar-thumb contrast helper lives in blitz-paint's color.rs and border.rs, neither reached; the harness restates it and leaves blitz-paint untouched
visual-design-verification#1.2 · no · the scrollbar thumb contrast stroke is painted in render.rs and tested in scrollbars.rs, none of them reached by the take
visual-design-verification#1.3 · no · the near-white thumb hover and drag test in scrollbar_drag.rs is not reached and the scope changes no scrollbar
visual-design-verification#1.4 · yes · stand_contrast's three tables, 17 rows and required ratios, and Harness::text_contrast's None conditions (opacity, filter, background-image) that the take's contrast-touching commits may reach are re-read and re-measured, the accent pin shift recorded
visual-design-verification#1.5 · yes · stand_contrast's the_load_page_meets_its_ratios and cells_meets_its_ratios assert Cells' ratios, which the cells.rs conflict and the stand re-measure
visual-design-verification#1.6 · yes · the visual lint's contrast reading goes through the same Harness::text_contrast call the chunk re-reads, and the a11y leg's 114 tests over 14 targets is a count the chunk re-measures
visual-design-verification#1.7 · yes · the overlap skip reads the snapshot's client rects, select-list row bounds and visible_region through the scrolling.rs conflict, and the boot reading [0, 0, 0, 0] is a value the chunk re-measures
visual-design-verification#1.8 · yes · the lint's no-overlap reading on an open list's row and its plain-pair contrast limit are witnessed by stand_act_popup, which asserts the select model the mutator and node.rs conflicts move
visual-design-verification#1.9 · yes · the user-agent select rules in default.css (the blitz-dom default sheet, a conflict surface) and select_interaction's multiple-select rows keep their meaning under the take, and stand_contrast reads the author-styled flight-type pair
visual-design-verification#1.10 · yes · select_popup_painted reads the highlight and disabled row against SelectListView's constants and the open list's plain pair at the default sheet, all in the select list's merge surface
visual-design-verification#1.11 · yes · the default sheet's select[multiple] > option:checked engine pair lies in default.css, a conflict surface, and select_interaction measures it in both layout modes
visual-design-verification#1.12 · yes · SelectListView's HIGHLIGHT_TEXT, HIGHLIGHT_BACKGROUND, DISABLED_TEXT and BORDER constants sit in select_list.rs, part of the select list's sixteen-file merge surface re-read row by row
visual-design-verification#1.13 · yes · the open list's one-height rows on the select's font are select-list layout that the list's merge surface re-reads, and ae774d7f4 (unrounded layout) can move the row bounds they read
visual-design-verification#2.1 · no · a past search record of contrast checks over s05 files, which the chunk's named files do not include and which states no behaviour the chunk must honour
visual-design-verification#2.2 · no · a past search record of contrast and prefers- media handling over s11 files, the recorded absence the chunk adds no handling against
visual-design-verification#3.1 · no · the browser urlbar focus border in apps/browser/assets/browser.css is not reached and sits outside the stand
visual-design-verification#3.2 · no · the new-tab search input outline in apps/browser/assets/about-newtab.css is not reached and sits outside the stand
visual-design-verification#3.3 · no · the todomvc :focus outline in examples/todomvc is not reached and sits outside the stand
visual-design-verification#3.4 · no · the seven_guis focus replacement lives in temp_converter.rs and flight_booker.rs, neither reached, and the keyboard checks read engine focus, not the outline paint
visual-design-verification#3.5 · yes · the Cells focused-cell style `.cells-grid:focus .cell.current` in cells.rs, a conflict surface, is named by the chunk's Cells scope and its focus witnesses
visual-design-verification#3.6 · no · the counter and transparent button outlines are authored in examples/counter and examples/transparent, not reached, and the keyboard checks read engine focus, not outline paint
visual-design-verification#3.7 · no · the rdme stylesheet focus-visible outline in apps/readme is not reached and sits outside the stand
visual-design-verification#3.8 · yes · the default sheet's input focus outline and focus-visible suppression (default.css:98-101, 882-888) sit in default.css, a conflict surface whose control values a check reads
visual-design-verification#3.9 · no · the google fixture forced-colors outlines in examples/assets/google.html are not reached and sit outside the stand's checks
visual-design-verification#3.10 · yes · the password input's bullets, painted in its text colour and measured through the text.rs conflict and the password masking the take holds, bind; the caret paint in render.rs does not
