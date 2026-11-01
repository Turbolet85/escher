# Classified — a11y-plan · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · the helper's definition in color.rs and its border.rs bevel call are unreached, and the render.rs blend layer and inline-box skip leave its use for bevels and thumbs in place
visual-design-verification#1.2 · yes · stale · render.rs is a reached file with 13 ranges from line 14 on, so the citations at render.rs:799-801 and 876-888 move; the thumb contrast stroke claim holds
visual-design-verification#1.3 · no · scrollbar_drag.rs is unreached and the hover-and-drag thumb change it asserts is not a blend, inline-box or list-painting change
visual-design-verification#1.4 · yes · add-to · F1: text_contrast now reads None past an element with a mix-blend-mode other than normal, a fourth condition beside opacity, filter and background-image
visual-design-verification#1.5 · no · stand_contrast.rs is unreached, its load-page and Cells ratio rows all stand, and the shared read it names is unchanged
visual-design-verification#1.6 · yes · add-to · F2: Session::lint's contrast reads the same None past a blend, so a blended text's pair is unread by the lint, beside the existing not-measured clause
visual-design-verification#1.7 · no · the overlap reading in lint.rs and its stand_lint test are unreached, and escher-driver holds no edit
visual-design-verification#1.8 · no · its stated limit is the lint's plain-pair read of highlighted rows, in unreached stand_lint and stand_act_popup; the blend fact is placed on #1.6
visual-design-verification#1.9 · yes · stale · default.css is a merge surface and upstream's picture rule left the sheet, so the citations at default.css:77-78, 84-86, 155-159 and 161-199 may move; every select rule stays and the pair holds
visual-design-verification#1.10 · no · select_popup_painted.rs and select_list.rs are unreached, and the list painting holds no hunk
visual-design-verification#1.11 · yes · stale · default.css is a merge surface and upstream's picture rule left the sheet, so the citation at default.css:161-199 may move; every select rule stays and the #c8c8c8 pair holds
visual-design-verification#1.12 · no · the SelectListView constants in select_list.rs are unreached and unedited
visual-design-verification#1.13 · no · docs/capabilities.md is unreached and the list painting holds no hunk, so the one-height row clip stands
visual-design-verification#2.1 · no · the absence is a sweep over the 15 s05 files, none of them reached; the new blend test lands in the harness crate and is placed under F1 beside #1.4
visual-design-verification#2.2 · no · the sweep for contrast or prefers- media handling covers 21 s11 files the session does not reach, and it adds no media handling
visual-design-verification#3.1 · no · apps/browser/assets/browser.css is unreached
visual-design-verification#3.2 · no · apps/browser/assets/about-newtab.css is unreached
visual-design-verification#3.3 · no · examples/todomvc/src/todomvc.css is unreached
visual-design-verification#3.4 · no · examples/seven_guis has no diff, and its focus rules are unedited
visual-design-verification#3.5 · no · examples/seven_guis has no diff, so cells.rs and its grid outline stand
visual-design-verification#3.6 · no · examples/counter and examples/transparent are unreached, and their 4px outlines are unedited
visual-design-verification#3.7 · no · apps/readme/assets/github-markdown.css is unreached
visual-design-verification#3.8 · yes · stale · default.css is a merge surface and upstream's picture rule left the sheet, so the citations at default.css:98-101 and 878-884 may move; the 2px #4D90FE input outline and its suppression stand
visual-design-verification#3.9 · no · examples/assets/google.html is unreached
visual-design-verification#3.10 · yes · stale · render.rs is a reached file with 13 ranges from line 14 on, so the citations at render.rs:972-992, 1008-1015 and 1017-1091 move; the caret and bullet behaviour stands
