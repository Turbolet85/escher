# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · yes · the Stylo-to-Color sRGB conversion carries the engine colour values the scope reads against the resolved sheet and the SelectListView constants
color-palette#1.2 · no · the viewport colour-scheme enum is not named by the scope and feeds prefers-color-scheme outside the reached files
color-palette#1.3 · no · the scheme-change recascade is a viewport path; fe33b778b's narrowing concerns DOM mutation, not scheme changes
color-palette#1.4 · no · iframe scheme copying in resolve.rs, a file not reached or named by the scope
color-palette#1.5 · yes · the bgcolor mapping lives in stylo.rs, a predicted conflict surface
color-palette#1.6 · no · the frame clear colour is dioxus-native window config, outside the reached files and the scope
color-palette#1.7 · yes · the canvas fallback is the backdrop the text contrast rows and Harness::text_contrast read when the root is transparent
color-palette#1.8 · no · WPT runner white fill; upstream WPT conformance is out of scope
color-palette#2.1 · no · browser chrome focus colour; apps/browser is named by no file of the scope
color-palette#2.2 · no · browser about:history link colour; apps/browser is named by no file of the scope
color-palette#2.3 · yes · cells.rs:551-554 current-cell focus outline is a recorded value the cells.rs conflict resolution must keep accurate
color-palette#2.4 · yes · the default link, input focus and button colours live in default.css, a predicted conflict surface
color-palette#2.5 · yes · the multiple-select chosen row pair lives in default.css, a conflict surface, and is the highlight pair the scope names
color-palette#2.6 · yes · SelectListView::HIGHLIGHT_BACKGROUND and HIGHLIGHT_TEXT are engine-constant colours the scope names
color-palette#2.7 · yes · SelectListView::DISABLED_TEXT is an engine-constant colour the scope names
color-palette#2.8 · yes · SelectListView::BORDER is an engine-constant colour the scope names
color-palette#2.9 · yes · the plain pair is measured by select_popup_painted, the witness the font-embolden gate runs and the list's contrast re-proof reads
color-palette#2.10 · no · the default text-selection highlight in blitz-paint lib.rs is not reached, and no scope names selection painting
color-palette#2.11 · no · counter and transparent example buttons; fixture values of examples other than seven_guis are not changed by the scope
color-palette#3.1 · no · browser tabstrip surfaces; apps/browser is named by no file of the scope
color-palette#3.2 · no · browser tooltip, overlay and status bar colours; apps/browser is named by no file of the scope
color-palette#3.3 · no · browser about-page backgrounds; apps/browser is named by no file of the scope
color-palette#3.4 · no · readme page backgrounds; apps/readme is named by no file of the scope
color-palette#3.5 · no · todomvc page background; the scope names no change to todomvc values
color-palette#3.6 · no · wasm_hello dark palette; the scope names no wasm surface
