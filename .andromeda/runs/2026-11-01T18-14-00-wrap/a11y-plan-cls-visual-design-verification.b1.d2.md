# Classified — a11y-plan · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the WCAG helper sits in color.rs and border.rs, neither of which the session reaches
visual-design-verification#1.2 · yes · stale · render.rs is a reached merge surface with hunks from line 14 on, so the cited 799-801 and 876-888 digits move
visual-design-verification#1.3 · yes · stale · render.rs, a reached merge surface, carries the thumb's hover and drag paint, and the report shows no scrollbar_drag re-run
visual-design-verification#1.4 · yes · add-to · F1, text_contrast now also reads None past a non-normal mix-blend-mode, joining its list of None conditions, and contrast.rs citations past 68 move
visual-design-verification#1.5 · no · the load page and Cells ratio tests hold no edit and every row stands, per the report
visual-design-verification#1.6 · yes · add-to · F2, the lint's contrast reads None past a blend through the one reader, so a blended text's pair is unread by the lint
visual-design-verification#1.7 · no · escher-driver holds no edit, and the sub-pixel size change reaches only a line reading whole pixels, which this line does not
visual-design-verification#1.8 · no · the lint's overlap and highlighted-row limits live in escher-driver and stand_act_popup, neither reached
visual-design-verification#1.9 · yes · stale · default.css is a reached merge surface whose upstream picture rule left, so digits past it move, the cited select rules among them if they sit below it
visual-design-verification#1.10 · no · select_popup_painted reads the open list's pair as recorded on both builds, per the report, and the list painting holds no hunk
visual-design-verification#1.11 · yes · stale · default.css is a reached merge surface whose upstream picture rule left, so the cited 161-199 digits may move while the chosen-row rules stay
visual-design-verification#1.12 · no · the SelectListView constants sit in select_list.rs, which the session does not reach
visual-design-verification#1.13 · no · the one-height row limit sits in docs/capabilities.md and the open-list constants, neither changed
visual-design-verification#2.1 · no · the absence is a search over the 15 s05 files, and the session's blend check is a harness unit test outside that set
visual-design-verification#2.2 · no · the session adds no prefers- or contrast media rule, and default.css changes only drop the upstream picture rule
visual-design-verification#3.1 · no · browser.css is not reached
visual-design-verification#3.2 · no · about-newtab.css is not reached
visual-design-verification#3.3 · no · todomvc.css is not reached
visual-design-verification#3.4 · no · examples/seven_guis has no diff
visual-design-verification#3.5 · no · cells.rs has no diff and stand_keyboard holds
visual-design-verification#3.6 · no · the counter and transparent apps are not reached
visual-design-verification#3.7 · no · github-markdown.css is not reached
visual-design-verification#3.8 · yes · stale · default.css is a reached merge surface whose picture rule left, so the cited 878-884 digits past it move while the outline rules stay
visual-design-verification#3.9 · no · google.html is not reached
visual-design-verification#3.10 · yes · stale · render.rs is a reached merge surface with 13 hunks from line 14 on, so the cited caret and selection spans 972-1091 move
