# Classified — a11y-plan · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the helper and its border-bevel use sit in color.rs and border.rs, neither reached, and the cited sites are not in the change
visual-design-verification#1.2 · yes · stale · render.rs is reached by the select-list paint, so the scrollbar-stroke citations 760-762 and 837-849 may have moved
visual-design-verification#1.3 · no · the thumb hover and drag claim cites scrollbar_drag.rs, which is not reached, and hit_with_scrollbar takes the list's hit only with a list open, so the page thumb path holds with none open
visual-design-verification#1.4 · yes · stale · STATE_ROWS 18 → 20 with tally (19, 0, 1, 0) and stand_contrast citations past line 12 move; F21 (the open list's plain pair measured by select_popup_painted) also belongs beside this record
visual-design-verification#1.5 · yes · stale · stand_contrast lines added from 12 on move the citations 670-700 and 702-727; the two tests and five rows stand
visual-design-verification#1.6 · yes · stale · the a11y leg tally moves 14 · 112 · 0 → 14 · 114 · 0; the stand-at-boot [0, 0, 0, 0] reading holds
visual-design-verification#1.7 · yes · stale · Entry.in_closed_select is renamed in_select_box (true also for an open list's row) and lint.rs citations past the insert move; the closed-select exclusion holds; F22 (no overlap read on an open list's row) also belongs beside it
visual-design-verification#1.8 · yes · add-to · F26 (an open list's row height is the select's font on one line, so a larger authored font clips) belongs beside the select's rendering; default.css and select_interaction.rs citations may have moved
visual-design-verification#1.9 · yes · add-to · F20 (the list's colours are engine constants on SelectListView, HIGHLIGHT_TEXT #000000 and HIGHLIGHT_BACKGROUND #c8c8c8, not tokens) belongs beside this hardcoded chosen-row pair
visual-design-verification#2.1 · no · the absence is a search over 15 s05 files outside the reached set; the session's new contrast reads sit in select_popup_painted.rs
visual-design-verification#2.2 · no · the contrast|prefers- search covers 21 s11 files outside the reached set
visual-design-verification#3.1 · no · the urlbar CSS lives in apps/browser, which is not reached
visual-design-verification#3.2 · no · about-newtab.css lives in apps/browser, which is not reached
visual-design-verification#3.3 · no · todomvc.css is not reached
visual-design-verification#3.4 · no · the flight booker's and temp converter's focus styles are unreached author CSS, and focus stays on the select with no style change
visual-design-verification#3.5 · no · cells.rs is not reached, and stand_keyboard and stand_contrast only gained tests, so the Cells outline's coverage holds
visual-design-verification#3.6 · no · counter and transparent app CSS are not reached
visual-design-verification#3.7 · no · github-markdown.css is not reached
visual-design-verification#3.8 · yes · stale · default.css is reached and its citations 98-101 and 882-888 may have moved; the 2px #4D90FE input outline holds
visual-design-verification#3.9 · no · examples/assets was not edited
visual-design-verification#3.10 · yes · stale · render.rs is reached and its caret and selection citations 933-953, 969-976 and 978-1052 may have moved; the caret behaviour holds
visual-design-verification#4.1 · no · examples/assets/bbc_reduced.html was not edited
visual-design-verification#5.1 · yes · add-to · F24 (the open list has no motion and needs no advance) belongs beside the motion absence
visual-design-verification#6.1 · no · browser.css is not reached
visual-design-verification#6.2 · yes · stale · stylo.rs is reached and its citation 510 may have moved; :open and flush_open_state are unmoved, so the claim holds
