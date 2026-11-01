# Classified — design · color-palette.b1

## Answers
color-palette#1.1 · yes · the Stylo-to-Color sRGB conversion is what the re-measured stand contrast rows and the default.css colours read; stylo.rs and default.css are conflict surfaces, util.rs itself is not reached
color-palette#1.2 · yes · the Light default scheme is the scheme the re-measured stand contrast rows paint with; the enum lives in unreached shell.rs and stylo_device.rs
color-palette#1.3 · no · the full recascade on scheme change lives in document.rs, which is not reached; the scope names no scheme change, and the fe33b778b narrowing is for DOM mutation
color-palette#1.4 · no · the iframe sub-document scheme copy in resolve.rs is unreached and the scope names no iframe change
color-palette#1.5 · yes · the legacy bgcolor mapping sits in stylo.rs, a conflict surface, so a resolution can move it
color-palette#1.6 · no · the frame clear colour lives in dioxus-native config.rs and the renderer, neither reached, and it is unset by default
color-palette#1.7 · yes · the canvas background fallback to body is the background the re-measured stand contrast rows and painted pixels read; render.rs is unreached but its value is measured
color-palette#1.8 · no · the WPT runner's white fill is WPT conformance output, which the task's Not needed list excludes
color-palette#2.1 · no · the browser chrome focus value is in apps/browser/assets/browser.css, which the Not needed list excludes
color-palette#2.2 · no · the about:history link colour is browser app CSS, which the Not needed list excludes
color-palette#2.3 · yes · cells.rs:551-554 holds the Cells current-cell focus outline, a conflict surface in the seven_guis fixture whose recorded values must stay accurate
color-palette#2.4 · yes · default.css:43-46, 98-101 and 111 are a conflict surface, and the default link, input focus outline and button colours are read against the resolved sheet
color-palette#2.5 · yes · default.css:161-199 is a conflict surface, and the multiple select's chosen pair is the highlight pair the Needs name, with its 12.56:1 measure re-read
color-palette#2.6 · yes · SelectListView::HIGHLIGHT_BACKGROUND and HIGHLIGHT_TEXT are SelectListView engine-constant colours the Needs name
color-palette#2.7 · yes · SelectListView::DISABLED_TEXT is one of the SelectListView engine-constant colours the Needs name
color-palette#2.8 · yes · SelectListView::BORDER is one of the SelectListView engine-constant colours the Needs name
color-palette#2.9 · yes · the open list's plain pair is measured by select_popup_painted, which the font-embolden gate runs, and the list is re-read row by row
color-palette#2.10 · no · the default text-selection highlight is in blitz-paint lib.rs, which is not reached, and the scope's text.rs changes do not name selection
color-palette#2.11 · no · the counter and transparent button colours are fixture values of other examples, which the Not needed list excludes
color-palette#3.1 · no · the browser tabstrip colours are browser app CSS, which the Not needed list excludes
color-palette#3.2 · no · the browser tooltip, FPS overlay and status bar colours are browser app CSS, which the Not needed list excludes
color-palette#3.3 · no · the browser about-page backgrounds are browser app CSS, which the Not needed list excludes
color-palette#3.4 · no · the readme page background is apps/readme CSS, which the Not needed list excludes
color-palette#3.5 · no · the todomvc page background is a fixture value of another example, which the Not needed list excludes
color-palette#3.6 · no · the wasm_hello palette is a fixture value of another example, and the scope names no wasm surface
