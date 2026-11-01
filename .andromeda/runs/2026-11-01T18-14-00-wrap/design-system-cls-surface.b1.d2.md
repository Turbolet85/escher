# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the sheet still loads once through DocumentConfig::default() in config.rs, which the session did not reach
surface#1.2 · yes · stale · default.css is a merge surface that lost lines to the upstream picture rule, so the cited lines 77-78, 84-86 and 155-159 may shift; the select rules stay
surface#1.3 · yes · stale · default.css is a merge surface that lost lines, so the cited lines 161-199 may shift; the multiple-select rows stay
surface#1.4 · yes · stale · default.css is a merge surface that lost lines, so the cited lines 161-199 may shift; the chosen-row rule stays
surface#1.5 · no · select_list.rs is not reached, and the report re-reads the SelectListView constants as unmoved
surface#1.6 · yes · stale · default.css is a merge surface that lost lines, so the cited lines 161-168 may shift; the select rules stay
surface#1.7 · yes · stale · stylo.rs is a merge surface with lines added, so the cited lines 1132-1147 may move; the list's painting and behaviour stand
surface#1.8 · yes · stale · render.rs is a merge surface with lines added, so the cited lines 248-249 may move; the list still paints last through paint_scene
surface#1.9 · yes · stale · cites select_popup.rs, where the row-3 skip is removed and the press at line 513 now covers every row; the citation 747-777 sits past that change and may move
surface#1.10 · yes · stale · cites select_popup.rs, where the row-3 skip is removed and the press at line 513 now covers every row
surface#1.11 · yes · stale · document.rs is a merge surface with lines added, so the cited 892-918 may move; the cited 478-516 also spans the line-513 change in select_popup.rs
surface#1.12 · yes · stale · cites select_popup.rs, where the row-3 skip is removed and the press at line 513 now covers every row
surface#1.13 · yes · stale · cites select_popup.rs lines 801-851, past the line-513 change where the row-3 skip is removed, so the citation may move
surface#1.14 · yes · stale · cites select_popup.rs lines 561-647, past the line-513 change where the row-3 skip is removed, so the citations may move
surface#1.15 · yes · stale · document.rs is a merge surface with lines added, so the cited lines 1006-1023 may move
surface#1.16 · no · window.rs is not reached and the initial theme is not touched by the session
surface#1.17 · no · blitz-traits shell.rs is not reached and the default viewport stands
surface#1.18 · yes · stale · bootstrap.rs is a merge surface where URLSearchParams landed, so the cited lines 135-140 may have moved
surface#2.1 · no · apps/browser is untouched by the session
surface#2.2 · no · blitz-shell lib.rs and event.rs are not reached
surface#3.1 · no · the row states the opacity composite and not when the set_custom_widget replay fires, and the example is not reached
surface#4.1 · no · devtools.rs is not reached
surface#5.1 · no · apps/browser and blitz-shell window.rs are not reached
surface#5.2 · no · apps/readme is not reached
surface#6.1 · no · apps/browser main.rs is not reached
