@answer n=1 entity=visual-design-verification#1.1 v=no
why: the WCAG contrast helper for border bevels and scrollbar thumbs, the probe reads no colour pair and paints no border (not_needed 1)
@answer n=2 entity=visual-design-verification#1.2 v=no
why: default scrollbar thumb contrast stroke, the chunk paints no scrollbar and reads no colour pair (not_needed 1)
@answer n=3 entity=visual-design-verification#1.3 v=no
why: near-white author scrollbar thumb on hover and drag, the probe drives no scrollbar drag and reads no colour
@answer n=4 entity=visual-design-verification#1.4 v=no
why: Harness::text_contrast, stand_contrast's 17 rows and the accent colour, the probe reads no colour pair and changes no stand CSS (not_needed 1)
@answer n=5 entity=visual-design-verification#1.5 v=no
why: text_contrast's None reading past a mix-blend-mode ancestor, no stand row moves and the probe reads no colour pair (not_needed 1)
@answer n=6 entity=visual-design-verification#1.6 v=no
why: mix-blend-mode and isolation painting on every document, the chunk changes no engine path for every document and reads no blend (scope line 6, not_needed 1)
@answer n=7 entity=visual-design-verification#1.7 v=no
why: stand_contrast's load page and Cells tests, the chunk runs neither page and re-measures no colour (not_needed 1)
@answer n=8 entity=visual-design-verification#1.8 v=no
why: the driver's visual lint contrast finding, its at-boot reading and the a11y leg's test count, the chunk runs no lint and reads no contrast (not_needed 1), the leg's count is ci-integration's rule and not this visual one
@answer n=9 entity=visual-design-verification#1.9 v=no
why: Session::lint's contrast reading past a blend, the chunk runs no lint and reads no colour pair (not_needed 1)
@answer n=10 entity=visual-design-verification#1.10 v=no
why: the lint's overlap reading of closed-select options and open-list rows, including CRUD's role=option rows, the probe runs no lint and reads CRUD rows only through the snapshot (not_needed 2)
@answer n=11 entity=visual-design-verification#1.11 v=no
why: the lint's overlap on an open list's row and the plain pair of a highlighted row, the probe opens no list and runs no lint (not_needed 1 and 2)
@answer n=12 entity=visual-design-verification#1.12 v=no
why: the user-agent single select layout and select#flight-type author CSS, the single select's list is not_needed 2 and the flight booker is outside the chunk's Counter or CRUD scope
@answer n=13 entity=visual-design-verification#1.13 v=no
why: select_popup_painted's highlight and disabled-row reading, the probe reads no select list (not_needed 2)
@answer n=14 entity=visual-design-verification#1.14 v=no
why: the engine default chosen-row pair on select[multiple], the probe reads no colour pair and no multiple select (not_needed 1 and 2)
@answer n=15 entity=visual-design-verification#1.15 v=no
why: SelectListView's engine colour constants, the probe reads no select list colour (not_needed 2)
@answer n=16 entity=visual-design-verification#1.16 v=no
why: an open list's row height and clipping of a larger authored font, the chunk opens no list (not_needed 2)
@answer n=17 entity=visual-design-verification#2.1 v=no
why: the absence of contrast checks over the s05 files, the chunk adds no contrast check and reads no colour pair (not_needed 1)
@answer n=18 entity=visual-design-verification#2.2 v=no
why: the absence of contrast or user-preference media handling, user-preference tokens are not_needed 12 and the chunk reads no media query
@answer n=19 entity=visual-design-verification#3.1 v=no
why: the browser urlbar's focus border and outline, the focus ring is not_needed 12 and the browser app is not reached by the stand
@answer n=20 entity=visual-design-verification#3.2 v=no
why: the new-tab search input's outline removal on focus, the focus ring is not_needed 12 and the about page is not reached
@answer n=21 entity=visual-design-verification#3.3 v=no
why: todomvc's zero focus outline, the focus ring is not_needed 12 and todomvc is not reached
@answer n=22 entity=visual-design-verification#3.4 v=no
why: seven_guis inputs replacing the focus outline with a border or box-shadow, the focus ring is not_needed 12 and the probe reads the snapshot's focus, not the outline
@answer n=23 entity=visual-design-verification#3.5 v=no
why: Cells' grid focus outline, Cells is outside the chunk's scope and the focus ring is not_needed 12
@answer n=24 entity=visual-design-verification#3.6 v=no
why: the 4px focus outline on examples/counter and transparent buttons, the focus ring is not_needed 12 and the probe reads focus from the tree, never an outline value
@answer n=25 entity=visual-design-verification#3.7 v=no
why: the rdme stylesheet's focus-visible outline, the focus ring is not_needed 12 and the rdme app is not reached
