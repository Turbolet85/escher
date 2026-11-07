@answer n=1 entity=surface#1.1 v=no
why: the default sheet is added by DocumentConfig in blitz-dom config.rs, and the DioxusDocument::new mention is a dated history the render-path change leaves intact
@answer n=2 entity=surface#1.2 v=no
why: engine select defaults in default.css, a stylesheet the session did not change
@answer n=3 entity=surface#1.3 v=no
why: engine multiple-select row defaults in default.css, untouched by the buffered render
@answer n=4 entity=surface#1.4 v=no
why: multiple-select chosen-row colour rule in default.css, a colour the session did not change
@answer n=5 entity=surface#1.5 v=no
why: open select list colour constants in select_list.rs, colours the session did not change
@answer n=6 entity=surface#1.6 v=no
why: single-select group display rules in default.css, outside the mutation buffer path
@answer n=7 entity=surface#1.7 v=no
why: select list paint and attribute reading, which the session does not reach
@answer n=8 entity=surface#1.8 v=no
why: the overlay list reaches the screen through paint_scene, which the session does not change
@answer n=9 entity=surface#1.9 v=no
why: keyboard and pointer highlight behaviour of the open list, untouched by the render-order change
@answer n=10 entity=surface#1.10 v=no
why: choice events on the open list, untouched by the render-order change
@answer n=11 entity=surface#1.11 v=no
why: list close behaviour in document.rs and pointer.rs, not reached by the session
@answer n=12 entity=surface#1.12 v=no
why: open-gating rules for disabled, inert and multiple selects, not reached by the session
@answer n=13 entity=surface#1.13 v=no
why: key handling after a container loses its box is a document.rs behaviour the session does not change
@answer n=14 entity=surface#1.14 v=no
why: group header reading in the list, not reached by the session
@answer n=15 entity=surface#1.15 v=no
why: highlight absence and select ComboBox tree reading, not MountedData focus and not reached by the session
@answer n=16 entity=surface#1.16 v=no
why: row bounds in snapshot.rs and list screenshot crop, a file the session does not reach
@answer n=17 entity=surface#1.17 v=no
why: initial theme from the window in blitz-shell, not reached by the session
@answer n=18 entity=surface#1.18 v=no
why: default viewport values in blitz-traits, not reached by the session
@answer n=19 entity=surface#1.19 v=no
why: JS window size bindings in blitz-vibey-script, not reached by the session
@answer n=20 entity=surface#2.1 v=no
why: macOS tabstrip CSS and padding in apps/browser, not on the Dioxus render path
@answer n=21 entity=surface#2.2 v=no
why: shell provider window controls in blitz-shell, which the render-order change does not reach
@answer n=22 entity=surface#3.1 v=no
why: GPU compositing opacity in the custom widget example, which is not focus and not reached by the session
@answer n=23 entity=surface#4.1 v=no
why: devtools overlay drawing in blitz-traits, not reached by the session
@answer n=24 entity=surface#5.1 v=no
why: window title source in blitz-shell, and replay order leaves the title node's timing unchanged
@answer n=25 entity=surface#5.2 v=no
why: rdme window title and theme shortcut, which are neither the render path nor MountedData
