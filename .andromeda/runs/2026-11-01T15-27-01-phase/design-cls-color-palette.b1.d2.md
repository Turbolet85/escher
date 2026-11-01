# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · yes · the Stylo-to-Color sRGB conversion paints the default-sheet colours that the chunk re-reads and re-measures (util.rs is unreached, but the re-read goes through it)
color-palette#1.2 · yes · the viewport's default Light scheme is what the stand contrast rows and default-sheet re-measures paint with
color-palette#1.3 · yes · the narrowed restyle invalidation (fe33b778b) leaves open whether a colour-scheme change still fully recascades light-dark() values
color-palette#1.4 · no · the iframe sub-document scheme copy in resolve.rs is not named by the scope, and no iframe document is in scope
color-palette#1.5 · yes · the bgcolor mapping lives in stylo.rs, a predicted conflict surface reached whole
color-palette#1.6 · yes · the frame clear colour, unset by default, is the backdrop the re-measured screenshot pixels paint over
color-palette#1.7 · yes · the canvas fallback from root to body background is what the re-measured screenshot pixels paint
color-palette#1.8 · no · the WPT runner's white fill sits in the WPT runner, which the scope puts out of scope
color-palette#2.1 · no · browser chrome focus colour is app chrome the scope does not restyle
color-palette#2.2 · no · the about:history link colour is in the browser app's stylesheet, which the scope does not name
color-palette#2.3 · yes · the cells.rs:551-554 accent and focus outline rule lives in examples/seven_guis/src/tasks/cells.rs, a conflict surface resolved to keep the fixture's values accurate
color-palette#2.4 · yes · the default.css link, input focus and button rules live in default.css, a conflict surface that dee854c76 edits
color-palette#2.5 · yes · the multiple select's chosen-row pair in default.css:161-199 is the highlight pair the scope names and the 12.56:1 re-measure reads
color-palette#2.6 · yes · SelectListView::HIGHLIGHT_BACKGROUND and HIGHLIGHT_TEXT are engine-constant colours of the open list the scope names
color-palette#2.7 · yes · SelectListView::DISABLED_TEXT is an engine-constant colour of the open list the scope names
color-palette#2.8 · yes · SelectListView::BORDER is an engine-constant colour of the open list the scope names
color-palette#2.9 · yes · the open list's plain pair is measured by select_popup_painted, which the select model and open list needs re-measure
color-palette#2.10 · no · the text-selection highlight is set in blitz-paint lib.rs, which the scope does not name, and it is not in the default sheet
color-palette#2.11 · no · the counter and transparent button colours are fixtures of examples the scope names no change to
color-palette#3.1 · no · the browser tabstrip colours are app chrome the scope does not restyle
color-palette#3.2 · no · the browser tooltip and overlay colours are app chrome the scope does not restyle
color-palette#3.3 · no · the browser about-page backgrounds are browser app styles the scope does not name
color-palette#3.4 · no · the rdme page background is the readme app's stylesheet, which the scope does not name
color-palette#3.5 · no · the todomvc page background is a fixture the scope names no change to
color-palette#3.6 · no · the wasm_hello palette is an example the scope names no change to, and the wasm surface is out of scope
