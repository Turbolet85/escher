@answer n=1 entity=surface#1.1 v=yes mode=stale
why: dioxus_document.rs is reached whole and its constructor changed, so the claim that DioxusDocument::new no longer adds the UA sheet is not shown still holding
@answer n=2 entity=surface#1.2 v=no
why: select defaults in blitz-dom default.css, and the session changed no stylesheet or CSS default
@answer n=3 entity=surface#1.3 v=no
why: multiple-select row rules in blitz-dom default.css, and the session changed no stylesheet or CSS default
@answer n=4 entity=surface#1.4 v=no
why: multiple-select chosen-row colour in default.css, and the session changed no colour or stylesheet
@answer n=5 entity=surface#1.5 v=no
why: SelectListView colour constants in blitz-dom select_list.rs, which the session does not reach
@answer n=6 entity=surface#1.6 v=no
why: single-select optgroup layout rules in default.css, and the session changed no stylesheet or CSS default
@answer n=7 entity=surface#1.7 v=no
why: select list painting and accessibility in blitz-dom and docs/capabilities.md, which the session does not reach
@answer n=8 entity=surface#1.8 v=no
why: list overlay reached through paint_scene in blitz-paint, and the session changed paint content not at all, only mutation order
@answer n=9 entity=surface#1.9 v=no
why: keyboard and pointer highlight handling in blitz-dom events, which the session does not reach
@answer n=10 entity=surface#1.10 v=no
why: choose-row input and change events in blitz-dom events and select_popup tests, which the session does not reach
@answer n=11 entity=surface#1.11 v=no
why: list close triggers in blitz-dom document.rs and pointer events, which the session does not reach
@answer n=12 entity=surface#1.12 v=no
why: select opening gates tested in select_popup.rs, which the session does not reach
@answer n=13 entity=surface#1.13 v=no
why: focus kept and key handled after the container loses its box is key handling in blitz-dom document.rs, not MountedData set_focus, which the session does not change
@answer n=14 entity=surface#1.14 v=no
why: option group header reading in the select list and its tests, which the session does not reach
@answer n=15 entity=surface#1.15 v=no
why: select ComboBox and expanded roles in blitz-dom accessibility.rs, which the session does not reach
@answer n=16 entity=surface#1.16 v=no
why: row bounds and screenshot crop in dioxus-native-dom snapshot.rs, which is not a changed file and whose content the session did not move
@answer n=17 entity=surface#1.17 v=no
why: initial window theme in blitz-shell window.rs, which the session does not reach
@answer n=18 entity=surface#1.18 v=no
why: default viewport values in blitz-traits shell.rs, which the session does not reach
@answer n=19 entity=surface#1.19 v=no
why: JS window metrics in blitz-vibey-script, which the session does not reach
@answer n=20 entity=surface#2.1 v=no
why: macOS browser tabstrip styling in apps/browser, which the session does not reach
@answer n=21 entity=surface#2.2 v=no
why: shell provider window controls in blitz-shell lib.rs and event.rs, which the session does not reach
@answer n=22 entity=surface#3.1 v=no
why: custom widget opacity layers in examples/custom_widget.rs, which the session does not reach
@answer n=23 entity=surface#4.1 v=no
why: devtools overlay drawing in blitz-traits devtools.rs, which the session does not reach
@answer n=24 entity=surface#5.1 v=no
why: browser window title and shell title-node path in apps/browser and blitz-shell window.rs, which the session does not reach
@answer n=25 entity=surface#5.2 v=no
why: rdme window title and theme toggle in apps/readme, which the session does not reach
