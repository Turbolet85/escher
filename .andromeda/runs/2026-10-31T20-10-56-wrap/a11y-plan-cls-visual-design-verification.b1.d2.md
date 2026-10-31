# Classified — a11y-plan · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the cited files color.rs and render/border.rs are not in the reached set, and the bevel and thumb use they describe is unchanged
visual-design-verification#1.2 · yes · stale · render.rs is reached whole and its cited lines 760-762 and 837-849 can have moved with the select-list paint; the thumb-stroke claim itself holds
visual-design-verification#1.3 · no · scrollbar_drag.rs is not reached and the thumb hover and drag behaviour is unchanged
visual-design-verification#1.4 · yes · stale · stand_contrast STATE_ROWS 18 → 20 with tally 19 pass and 1 exempt, so the STATE_ROWS count and "pass 17" stand moved; citations below line 12 moved
visual-design-verification#1.5 · yes · stale · stand_contrast gained rows from line 12 on, so the citations 670-700 and 702-727 moved; the test names and the five-row count hold
visual-design-verification#1.6 · yes · stale · the a11y leg tally moves 14 · 112 · 0 → 14 · 114 · 0, so "112 tests" is stale; F22's unread highlight contrast also belongs here
visual-design-verification#1.7 · yes · contradict · names the retired Entry.in_closed_select, now in_select_box and true also for an open list's row (F22), read through a closed_select_of that answers None for an open select
visual-design-verification#1.8 · yes · stale · default.css and select_interaction.rs citations past their inserts moved; the closed select's pair stays unread while the open list's plain pair is measured (F21)
visual-design-verification#1.9 · yes · add-to · F20: the open list's highlight pair #000000 on #c8c8c8 is an engine constant on SelectListView, beside this hardcoded sheet pair; default.css and select_interaction citations moved too
visual-design-verification#2.1 · no · the s05 search set names no reached file, so the absence of contrast checks in it is unchanged
visual-design-verification#2.2 · no · the s11 search set names no reached file, so the absence of contrast or media handling in it is unchanged
visual-design-verification#3.1 · no · browser.css is not reached and its urlbar focus border and outline are unchanged
visual-design-verification#3.2 · no · about-newtab.css is not reached and its input outline is unchanged
visual-design-verification#3.3 · no · todomvc.css is not reached and its focus outline is unchanged
visual-design-verification#3.4 · no · temp_converter.rs and flight_booker.rs are not reached, and the select's focus border is unchanged
visual-design-verification#3.5 · no · cells.rs is not reached, and stand_keyboard and stand_contrast still cover the Cells outline (only rows and tests were added to them)
visual-design-verification#3.6 · no · counter and transparent app.rs are not reached and their outlines are unchanged
visual-design-verification#3.7 · no · github-markdown.css is not reached and its focus outline is unchanged
visual-design-verification#3.8 · yes · stale · default.css is reached and its citations 98-101 and 882-888 may sit past an insert whose citations moved; the 2px #4D90FE claim holds
visual-design-verification#3.9 · no · google.html is an unreached fixture and its forced-colors rule is unchanged
visual-design-verification#3.10 · yes · stale · render.rs is reached whole and the caret citations 933-953, 969-976 and 978-1052 can have moved; the caret behaviour holds
visual-design-verification#4.1 · no · examples/assets is unedited and the BBC fixture's reduced-motion queries are unchanged
visual-design-verification#5.1 · no · the s02 search set names no reached file, and the session adds no reduced-motion or colour-scheme rule
visual-design-verification#6.1 · no · browser.css is not reached and the disabled icon opacity is unchanged
visual-design-verification#6.2 · yes · stale · stylo.rs is reached and its citations past an insert moved, so the citation 510 can have moved; the :open reading and flush_open_state are unmoved
