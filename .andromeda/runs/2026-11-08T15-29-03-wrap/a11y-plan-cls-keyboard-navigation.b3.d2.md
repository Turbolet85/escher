@answer n=1 entity=keyboard-navigation#3.22 v=no
why: the synthetic click builder lives in blitz-dom node.rs, which is not among the session's reached files and blitz-dom was not changed
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: the winit key conversion lives in blitz-shell convert_events.rs, outside the reached files, and the session changed no key conversion
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key dispatch in blitz-shell window.rs is not reached and the actions verb changes no key event path
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: deallocate_anonymous_block and its key-press test in blitz-dom are not reached, and the session did not touch layout parents
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS keybinding forwarding in blitz-shell and blitz-traits is not reached by the session
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding in blitz-shell and blitz-traits is not reached, and the session changed no IME path
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: focus and blur bindings in blitz-vibey-script are not reached by the session
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection in blitz-vibey-script form_controls is not reached, and the autofocus feature is unchanged
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: the CSS property accessor runtime in blitz-vibey-script is not reached by the session
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields in blitz-vibey-script event.rs are not reached by the session
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: keyboard data routing in dioxus_document.rs and events.rs is not in the reached files, and the session only adds a read-only method
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: the registry dispatch in signal_registry.rs and graph.rs is not reached, and the actions read does not use the signal registry
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: focus and blur forwarding in dioxus_document.rs is not reached and the session changed no focus event data
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: set_focus and its queueing in dioxus-native-dom events.rs are not reached, and the actions verb changes no focus
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the buffered render path and the probe pins in dioxus_document.rs, events.rs and dioxus_runtime_probe.rs are not reached by the session
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the render-time focus limit is owned by the buffered render path, which the session does not reach
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the autofocus feature forwarding in the dioxus-native-dom and dioxus-native Cargo manifests is not reached, and no feature changed
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: the IME TODO in dioxus_document.rs is not reached, and the session leaves IME events unhandled as before
@answer n=19 entity=keyboard-navigation#4.13 v=no
why: the falsy boolean attribute path in mutation_writer.rs and its tests is not reached, and the bridge's boolean attribute handling is unchanged
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar keys in apps/browser toolbar.rs are not reached by the session
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: the browser tab shortcuts in apps/browser tab_strip.rs are not reached by the session
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search autofocus in apps/browser about_pages.rs is not reached by the session
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts in apps/readme are not reached by the session
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: the back-button hook in dioxus-native hooks.rs is not reached by the session
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: the todomvc focus handling in examples/todomvc is not reached by the session
