# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the sheet is still given once through DocumentConfig::default() in config.rs, which the session did not reach, and the report names no change to that delivery
surface#1.2 · yes · stale · default.css is a merge surface whose picture rule left, so the cited lines 77-78, 84-86 and 155-159 may have moved; the select rules themselves stand
surface#1.3 · yes · stale · default.css is a merge surface and its cited lines 161-199 may have moved with the picture rule's removal; every select rule stays
surface#1.4 · yes · stale · default.css is a merge surface and its cited lines 161-199 may have moved; the chosen-row rule stays
surface#1.5 · no · select_list.rs is unreached by the session and the SelectListView constants were re-read and unmoved
surface#1.6 · yes · stale · default.css is a merge surface and its cited lines 161-164 and 166-168 may have moved; the select rules stay
surface#1.7 · yes · stale · stylo.rs is a merge surface with lines added, so the cited lines 1132-1147 may have moved, and the report does not show the size reading holds there
surface#1.8 · yes · stale · render.rs is a merge surface with lines added, so the cited lines 248-249 may have moved; the overlay still paints through paint_scene
surface#1.9 · yes · stale · select_popup.rs lost the row-3 skip and its citations past line 513 may move (747-777 here)
surface#1.10 · yes · stale · select_popup.rs lost the row-3 skip, so these tests now press every row; the cited lines sit before 513 and hold
surface#1.11 · yes · stale · select_popup.rs cited range 478-516 spans line 513, where the row-3 skip's removal left one press for every row; document.rs is a merge surface whose cited lines 892-918 may have moved
surface#1.12 · yes · stale · select_popup.rs lost the row-3 skip, so these tests now press every row; the cited lines sit before 513 and hold
surface#1.13 · yes · stale · select_popup.rs citations past line 513 (801-851) may move after the row-3 skip's removal
surface#1.14 · yes · stale · select_popup.rs citations past line 513 (561-647) may move after the row-3 skip's removal
surface#1.15 · yes · stale · document.rs is a merge surface with lines added, so the cited lines 1006-1023 may have moved; the bounds and crop claims are not shown to change
surface#1.16 · no · blitz-shell window.rs is unreached by the session and the default theme is not named by it
surface#1.17 · no · blitz-traits shell.rs is unreached by the session and the viewport defaults are not named by it
surface#1.18 · yes · stale · bootstrap.rs is a merge surface where URLSearchParams landed, so the cited lines 135-140 may have moved and the report does not show the reading holds
surface#2.1 · no · apps/browser was not edited by the session
surface#2.2 · no · blitz-shell lib.rs and event.rs are unreached by the session
surface#3.1 · no · the row names no firing point for the set_custom_widget attribute replay, and examples/custom_widget.rs is unreached
surface#4.1 · no · blitz-traits devtools.rs is unreached by the session
surface#5.1 · no · window title code in apps/browser and blitz-shell window.rs is unreached by the session
surface#5.2 · no · apps/readme is unreached by the session
surface#6.1 · no · apps/browser main.rs is unreached by the session
