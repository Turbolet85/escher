@answer n=1 entity=surface#1.1 v=yes mode=stale
why: the reached file dioxus_document.rs, which this entry names as where DioxusDocument::new once added the sheet, is changed whole by the session
@answer n=2 entity=surface#1.2 v=no
why: its select sheet rules live in blitz-dom default.css, which no changed file touches
@answer n=3 entity=surface#1.3 v=no
why: its multiple-select row rules live in blitz-dom default.css, which no changed file touches
@answer n=4 entity=surface#1.4 v=no
why: its chosen-row colour rule lives in blitz-dom default.css and the session adds no colour
@answer n=5 entity=surface#1.5 v=no
why: the open-list colour constants live in blitz-dom select_list.rs, which no changed file touches and the session colours nothing
@answer n=6 entity=surface#1.6 v=no
why: its optgroup display rules live in blitz-dom default.css, which no changed file touches
@answer n=7 entity=surface#1.7 v=yes mode=stale
why: the reached file docs/capabilities.md gains three handler-coverage limits, so the cited line range 186-191 may have moved
@answer n=8 entity=surface#1.8 v=no
why: its paint rules live in blitz-paint render.rs and select_list.rs, which no changed file touches
@answer n=9 entity=surface#1.9 v=no
why: its key and pointer rules live in blitz-dom keyboard.rs and pointer.rs, which no changed file touches
@answer n=10 entity=surface#1.10 v=no
why: its choose-a-row rules live in blitz-dom keyboard.rs and pointer.rs, which no changed file touches
@answer n=11 entity=surface#1.11 v=no
why: its close rules live in blitz-dom document.rs and pointer.rs, which no changed file touches
@answer n=12 entity=surface#1.12 v=no
why: its no-open rules live in blitz-dom and the select_popup tests, none of which the session changes
@answer n=13 entity=surface#1.13 v=no
why: its focus and lost-box rules live in blitz-dom document.rs and a test file, none of which the session changes
@answer n=14 entity=surface#1.14 v=no
why: its list header rules live in blitz-dom and the select_popup tests, none of which the session changes
@answer n=15 entity=surface#1.15 v=no
why: its accessibility rules live in blitz-dom accessibility.rs, which no changed file touches
@answer n=16 entity=surface#1.16 v=no
why: its bounds claim cites dioxus-native-dom snapshot.rs, which the change does not reach, and stand_act_popup.rs is not reached either
@answer n=17 entity=surface#1.17 v=no
why: its theme rule lives in blitz-shell window.rs, which no changed file touches
@answer n=18 entity=surface#1.18 v=no
why: its viewport rule lives in blitz-traits shell.rs, which no changed file touches
@answer n=19 entity=surface#1.19 v=no
why: its JS window size rules live in blitz-vibey-script, which no changed file touches
@answer n=20 entity=surface#2.1 v=no
why: its titlebar rules live in apps/browser tab_strip.rs and browser.css, which no changed file touches
@answer n=21 entity=surface#2.2 v=no
why: it cites blitz-shell lib.rs and event.rs, which are not the reached dioxus-native-dom lib.rs and no changed file touches them
@answer n=22 entity=surface#3.1 v=no
why: its custom widget opacity rule lives in examples/custom_widget.rs, which the change does not reach
@answer n=23 entity=surface#4.1 v=no
why: its devtools overlay rule lives in blitz-traits devtools.rs, which no changed file touches
@answer n=24 entity=surface#5.1 v=no
why: its window title rules live in apps/browser main.rs and blitz-shell window.rs, which no changed file touches
@answer n=25 entity=surface#5.2 v=no
why: its window title and theme rules live in apps/readme files, which no changed file touches
