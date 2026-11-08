@answer n=1 entity=surface#1.1 v=no
why: the UA sheet attachment lives in blitz-dom config.rs, not reached, and the session's dioxus_document.rs change is only the registry dispatch around handle_event
@answer n=2 entity=surface#1.2 v=no
why: the select rules sit in blitz-dom default.css, which the session did not reach
@answer n=3 entity=surface#1.3 v=no
why: the multiple-select rows sit in blitz-dom default.css, which the session did not reach
@answer n=4 entity=surface#1.4 v=no
why: the multiple-select chosen-row colour sits in blitz-dom default.css, which the session did not reach
@answer n=5 entity=surface#1.5 v=no
why: the open select list constants sit in blitz-dom select_list.rs, which the session did not reach and no colour moved
@answer n=6 entity=surface#1.6 v=no
why: the single-select group display rules sit in blitz-dom default.css, which the session did not reach
@answer n=7 entity=surface#1.7 v=yes mode=stale
why: docs/capabilities.md is reached whole and the retired capability line can have shifted its cited lines 157-162
@answer n=8 entity=surface#1.8 v=no
why: the paint_scene list overlay lives in blitz-paint, which the session did not reach
@answer n=9 entity=surface#1.9 v=no
why: the select keyboard and pointer handling lives in blitz-dom events, which the session did not reach, and the Dioxus dispatch is inert where no state is registered
@answer n=10 entity=surface#1.10 v=no
why: the select choice events live in blitz-dom events, which the session did not reach, and the Dioxus dispatch is inert where no state is registered
@answer n=11 entity=surface#1.11 v=no
why: the select close rules live in blitz-dom events and document.rs, which the session did not reach, and the Dioxus dispatch is inert where no state is registered
@answer n=12 entity=surface#1.12 v=no
why: the select open rules live in blitz-dom and the cited select_popup tests, none reached by the session
@answer n=13 entity=surface#1.13 v=no
why: the focus-after-lost-box behaviour and its cited test sit in blitz-dom, which the session did not reach
@answer n=14 entity=surface#1.14 v=no
why: the group header reading sits in the cited select_popup tests, which the session did not reach
@answer n=15 entity=surface#1.15 v=no
why: the highlight is engine overlay state with no DOM click listener, so clickable does not reach it and the no-snapshot claim holds on the changed snapshot.rs
@answer n=16 entity=surface#1.16 v=yes mode=stale
why: snapshot.rs is reached whole and cited at lines 163-166, and the session added the clickable field to the snapshot node, so that citation can have moved
@answer n=17 entity=surface#1.17 v=no
why: the initial theme sits in blitz-shell window.rs, which the session did not reach
@answer n=18 entity=surface#1.18 v=no
why: the default viewport sits in blitz-traits shell.rs, which the session did not reach and no layout moved
@answer n=19 entity=surface#1.19 v=no
why: the JS window metrics sit in blitz-vibey-script, which the session did not reach
@answer n=20 entity=surface#2.1 v=no
why: the browser tabstrip sits in apps/browser, which the session did not reach
@answer n=21 entity=surface#2.2 v=no
why: the shell titlebar provider sits in blitz-shell, which the session did not reach
@answer n=22 entity=surface#3.1 v=no
why: the custom widget composite sits in examples/custom_widget.rs, which the session did not reach
@answer n=23 entity=surface#4.1 v=no
why: the devtools overlay sits in blitz-traits devtools.rs, which the session did not reach
@answer n=24 entity=surface#5.1 v=no
why: the browser window title sits in apps/browser and blitz-shell, which the session did not reach
@answer n=25 entity=surface#5.2 v=no
why: the rdme window title and theme toggle sit in apps/readme, which the session did not reach
