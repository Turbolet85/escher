@answer n=1 entity=surface#1.1 v=no
why: the session adds a registry dispatch around handle_event in dioxus_document.rs, and the UA sheet add in DioxusDocument::new and DocumentConfig::default is unchanged, so the claim still holds
@answer n=2 entity=surface#1.2 v=no
why: cites blitz-dom default.css, which the session did not reach, and no select default moved
@answer n=3 entity=surface#1.3 v=no
why: cites blitz-dom default.css multiple-select rules, which the session did not reach
@answer n=4 entity=surface#1.4 v=no
why: cites blitz-dom default.css chosen-row rule, which the session did not reach, and no colour value moved
@answer n=5 entity=surface#1.5 v=no
why: cites blitz-dom select_list.rs constants, which the session did not reach, and no colour value moved
@answer n=6 entity=surface#1.6 v=no
why: cites blitz-dom default.css optgroup rules, which the session did not reach
@answer n=7 entity=surface#1.7 v=yes mode=stale
why: docs/capabilities.md is reached and the session retired a capability line there, so the cited lines 157-162 may have moved
@answer n=8 entity=surface#1.8 v=no
why: cites blitz-paint render files the session did not reach
@answer n=9 entity=surface#1.9 v=no
why: cites blitz-dom keyboard and pointer files and select_popup.rs, none reached, and the dispatch is inert where no state is registered
@answer n=10 entity=surface#1.10 v=no
why: cites blitz-dom event files and select_popup.rs, none reached, and no choice rule moved
@answer n=11 entity=surface#1.11 v=no
why: cites blitz-dom document.rs and pointer.rs, which the session did not reach
@answer n=12 entity=surface#1.12 v=no
why: cites select_popup.rs tests the session did not reach, and no select open rule moved
@answer n=13 entity=surface#1.13 v=no
why: cites blitz-dom document.rs and a key_press test the session did not reach
@answer n=14 entity=surface#1.14 v=no
why: cites select_popup.rs tests the session did not reach, and the list header rule did not move
@answer n=15 entity=surface#1.15 v=no
why: cites blitz-dom accessibility.rs and select_popup.rs, none reached, and the select tree reading did not move
@answer n=16 entity=surface#1.16 v=yes mode=stale
why: packages/dioxus-native-dom/src/snapshot.rs is reached by the new NodeState.clickable field, so the cited lines 163-166 may have moved
@answer n=17 entity=surface#1.17 v=no
why: cites blitz-shell window.rs, which the session did not reach, and the default theme did not move
@answer n=18 entity=surface#1.18 v=no
why: cites blitz-traits shell.rs, which the session did not reach
@answer n=19 entity=surface#1.19 v=no
why: cites blitz-vibey-script files, which the session did not reach
@answer n=20 entity=surface#2.1 v=no
why: cites apps/browser files the session did not reach, and no app UI changed
@answer n=21 entity=surface#2.2 v=no
why: cites blitz-shell files the session did not reach, and no shell window control changed
@answer n=22 entity=surface#3.1 v=no
why: cites examples/custom_widget.rs, which the session did not reach
@answer n=23 entity=surface#4.1 v=no
why: cites blitz-traits devtools.rs, which the session did not reach
@answer n=24 entity=surface#5.1 v=no
why: cites apps/browser and blitz-shell window.rs files the session did not reach, and the window title did not move
@answer n=25 entity=surface#5.2 v=no
why: cites apps/readme files the session did not reach, and no window title or theme shortcut moved
