@answer n=1 entity=surface#1.1 v=no
why: the default user-agent sheet in packages/blitz-dom/src/config.rs is unchanged, and the session's reached files are only the new probe test, Cargo.lock and deny.toml
@answer n=2 entity=surface#1.2 v=no
why: the select rules in packages/blitz-dom/assets/default.css are unchanged by the session
@answer n=3 entity=surface#1.3 v=no
why: the multiple-select row rules in packages/blitz-dom/assets/default.css are unchanged by the session
@answer n=4 entity=surface#1.4 v=no
why: the chosen-row colour rule in packages/blitz-dom/assets/default.css is unchanged, and no colour or token changed
@answer n=5 entity=surface#1.5 v=no
why: the SelectListView colour constants in packages/blitz-dom/src/node/select_list.rs are unchanged by the session
@answer n=6 entity=surface#1.6 v=no
why: the optgroup display rules in packages/blitz-dom/assets/default.css are unchanged by the session
@answer n=7 entity=surface#1.7 v=no
why: the list paint and label behaviour in packages/blitz-dom/src/node/select_list.rs and docs/capabilities.md are unchanged, and neither file is reached
@answer n=8 entity=surface#1.8 v=no
why: the overlay paint path in packages/blitz-paint is unchanged by the session, which moved no paint code
@answer n=9 entity=surface#1.9 v=no
why: the highlight keyboard and pointer handling in packages/blitz-dom/src/events is unchanged by the session
@answer n=10 entity=surface#1.10 v=no
why: the choose-with-input-and-change handling in packages/blitz-dom/src/events is unchanged by the session
@answer n=11 entity=surface#1.11 v=no
why: the close-list handling in packages/blitz-dom/src/document.rs and events/pointer.rs is unchanged by the session
@answer n=12 entity=surface#1.12 v=no
why: the disabled, inert and hidden select rules in tests/blitz-tests/tests/select_popup.rs are unchanged, and that file is not reached
@answer n=13 entity=surface#1.13 v=no
why: the claim rests on the engine key path through packages/blitz-dom/src/document.rs and key_press_after_container_lost_its_box.rs, neither reached, and the session's MountedData set_focus hazard is a different path
@answer n=14 entity=surface#1.14 v=no
why: the group header reading in tests/blitz-tests/tests/select_popup.rs is unchanged, and that file is not reached
@answer n=15 entity=surface#1.15 v=no
why: the snapshot and tree exclusion of the highlighted row is unchanged, since the session added no snapshot field
@answer n=16 entity=surface#1.16 v=no
why: the row bounds in packages/dioxus-native-dom/src/snapshot.rs are unchanged, since the session added no snapshot field
@answer n=17 entity=surface#1.17 v=no
why: the initial theme in packages/blitz-shell/src/window.rs is unchanged by the session
@answer n=18 entity=surface#1.18 v=no
why: the default viewport in packages/blitz-traits/src/shell.rs is unchanged by the session
@answer n=19 entity=surface#1.19 v=no
why: the JS window size aliases in packages/blitz-vibey-script/src/runtime are unchanged by the session
@answer n=20 entity=surface#2.1 v=no
why: the macOS merged-titlebar tabstrip lives in apps/browser, and no apps/browser file changed
@answer n=21 entity=surface#2.2 v=no
why: the shell titlebar provider in packages/blitz-shell is unchanged by the session
@answer n=22 entity=surface#3.1 v=no
why: the custom widget opacity rule in examples/custom_widget.rs is unchanged, and no example changed
@answer n=23 entity=surface#4.1 v=no
why: the devtools overlay types in packages/blitz-traits/src/devtools.rs are unchanged by the session
@answer n=24 entity=surface#5.1 v=no
why: the browser window title logic in apps/browser/src/main.rs is unchanged, since no apps/browser file changed
@answer n=25 entity=surface#5.2 v=no
why: the rdme title and theme toggle live in apps/readme sources that the session left unchanged, and the pulldown-cmark bump in Cargo.lock reaches neither
