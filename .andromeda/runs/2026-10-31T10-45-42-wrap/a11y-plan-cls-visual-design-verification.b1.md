# Classified — a11y-plan · visual-design-verification.b1

## Answers
visual-design-verification#1.1 · no · blitz-paint shows no diff and its color.rs and border.rs helper lines are unreached, so the helper still holds
visual-design-verification#1.2 · no · the thin default thumb stroke is painted in blitz-paint, which shows no diff, and scrollbars.rs is unreached
visual-design-verification#1.3 · no · the cited scrollbar_drag.rs is unreached, the thumb paint sits in blitz-paint, which shows no diff, and the session names no scrollbar change
visual-design-verification#1.4 · yes · stale · stand_contrast grows from 5 to 7 tests with Cells and load page coverage, so its row-table counts and cited stand_contrast.rs ranges can have moved
visual-design-verification#1.5 · yes · contradict · overlaps now skips a closed select's options, so the stand at boot reads no finding, and the a11y leg reads 112 tests, not 85
visual-design-verification#1.6 · yes · contradict · overlaps now skips a closed select's options, so stand_lint's at_boot reads [0, 0, 0, 0] and the flight booker's overlap is no longer read
visual-design-verification#1.7 · yes · add-to · F24, the engine default chosen-row pair of a multiple select (#c8c8c8 on #000000, 12.56:1 by select_interaction), joins beside this closed select's user-agent pair
visual-design-verification#2.1 · no · the session adds its contrast checks to test files and the reached cells.rs gains none, so the absence holds
visual-design-verification#2.2 · no · the session adds no media-query or user-preference handling and its contrast rows are tests
visual-design-verification#3.1 · no · browser.css is not among the session's changed files
visual-design-verification#3.2 · no · about-newtab.css is not among the session's changed files
visual-design-verification#3.3 · no · todomvc.css is not among the session's changed files
visual-design-verification#3.4 · yes · add-to · F25, Cells' grid focus cue (2px solid #3a5ce5 outline on .cell.current at -2px offset), joins the seven_guis focus cues beside the input border cues
visual-design-verification#3.5 · no · the counter and transparent app.rs files are not among the session's changed files
visual-design-verification#3.6 · no · github-markdown.css is not among the session's changed files
visual-design-verification#3.7 · yes · stale · default.css is reached whole and gains group and row display rules, so its cited lines 98-101 and 871-877 can have moved
visual-design-verification#3.8 · no · google.html is not among the session's changed files
visual-design-verification#3.9 · no · the caret paint is in blitz-paint, which shows no diff, and password_painted_masked.rs is unreached, the focus fixup moves focus but not the caret paint
visual-design-verification#4.1 · no · bbc_reduced.html is not among the session's changed files and the session adds no motion
visual-design-verification#5.1 · no · the session adds no motion and reduced motion is the popup chunk's fork
visual-design-verification#6.1 · no · browser.css is not among the session's changed files and disabled still reads as before
visual-design-verification#6.2 · yes · contradict · open_state now asserts :open matching in its four asserts, so the no-test-asserts reading no longer holds
visual-design-verification#6.3 · no · the WPT runner main.rs is not among the session's changed files
visual-design-verification#6.4 · no · examples/form.rs is not among the session's changed files and the session changes no input colour
visual-design-verification#7.1 · no · the viewport scheme and zoom are carried in blitz-traits shell.rs, which is not among the session's changed files
