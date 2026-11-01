# Classified — design · surface.b1

## Answers
surface#1.1 · yes · the once-per-document UA sheet add must survive a resolution of default.css and of dioxus_document.rs, both conflict surfaces
surface#1.2 · yes · the select border, padding, background and inline-block rules are engine defaults in default.css, a conflict surface read against the resolved sheet
surface#1.3 · yes · the multiple-select row display and nowrap rules in default.css, a conflict surface the select list re-read covers
surface#1.4 · yes · the multiple-select chosen-row rule is the pair the highlight shares with SelectListView, named in scope
surface#1.5 · yes · SelectListView's engine-constant colours are named in scope as the select model and open list
surface#1.6 · yes · the single-select group and option:checked display rules sit in default.css, a conflict surface read row by row
surface#1.7 · yes · the open gestures, the group label in the tree and the size hint in stylo.rs are named in scope, and select_popup.rs edits its key-press row
surface#1.8 · yes · the open list's paint through paint_scene is what the font-embolden gate runs via select_popup_painted, and the scope adds no window
surface#1.9 · yes · highlight movement over enabled rows and choosing nothing is the open list's key and pointer behaviour the select model names, checked by select_popup.rs
surface#1.10 · yes · choosing a row with input then change, and a disabled row changing nothing, is the select model named in scope, witnessed by the select_popup.rs key-press row
surface#1.11 · yes · closing the list by Escape, click, wheel, page scroll or removal is the open-list behaviour named in scope, and page scroll is the into-view scrolling the chunk re-reads
surface#1.12 · yes · a disabled, inert or hidden select opening nothing is the select model named in scope, with select_popup.rs re-read row by row
surface#1.13 · yes · a group header reading as a header and choosing nothing is the open list named in scope, re-read row by row
surface#1.14 · yes · the open select's ComboBox and expanded reading in the tree is the accessibility identity re-proven by stand_keyboard and the a11y leg
surface#1.15 · yes · a row's own bounds in the snapshot and its cropped screenshot are the dioxus_document snapshot surface and the stand_snapshot re-reads
surface#1.16 · yes · the Light default theme is the scheme the re-measured screenshot pixels are painted under, and the chunk re-measures them
surface#1.17 · yes · the default viewport size and scale are the size the re-measured bounds and screenshot pixels are taken at
surface#1.18 · yes · innerWidth, innerHeight and devicePixelRatio are read in blitz-vibey-script bootstrap.rs, a conflict surface
surface#2.1 · no · the macOS browser tabstrip is app chrome; the scope names no titlebar change and the desktop-native titlebar is not needed
surface#2.2 · no · the shell provider's titlebar controls sit in blitz-shell, which the scope does not reach; the desktop-native titlebar is not needed
surface#3.1 · yes · the custom widget pattern's HTML layer opacity is what a26c6ddeb's CustomWidget attribute replay reaches, and the scope names that pattern
surface#4.1 · no · the devtools overlay is named as not needed; the scope changes no devtools behaviour
surface#5.1 · no · window titling from the active tab is browser and blitz-shell window behaviour the scope neither reaches nor names
surface#5.2 · no · rdme's window title and theme toggle are app behaviour in apps/readme, which the scope does not reach
surface#6.1 · no · the browser's macOS titlebar is app chrome the scope does not name; the desktop-native titlebar is not needed
