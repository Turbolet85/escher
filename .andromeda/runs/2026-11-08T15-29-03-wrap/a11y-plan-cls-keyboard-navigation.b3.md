@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic_click_event lives in blitz-dom node.rs, a file the session did not reach.
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion lives in blitz-shell convert_events.rs, which the session did not reach.
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: shell key dispatch lives in blitz-shell window.rs, which the session did not reach.
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: anonymous-block drop handling lives in blitz-dom document.rs and its pinned test, neither in the session's file list.
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS keybinding forwarding lives in blitz-shell and blitz-traits, which the session did not reach.
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding lives in blitz-shell and blitz-traits, which the session did not reach.
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: focus, blur and activeElement live in blitz-vibey-script, a crate the session did not touch.
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection lives in blitz-vibey-script and the autofocus feature forward, neither reached by the session.
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors live in blitz-vibey-script, which the session did not reach.
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields live in blitz-vibey-script, which the session did not reach.
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: KeyDown, KeyUp and KeyPress forwarding lives in dioxus_document.rs and events.rs, neither in the session's file list.
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: use_graph_state and the SignalRegistry live in signal_registry.rs, graph.rs and the seven_guis tasks, none reached by the session.
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: Focus and Blur event forwarding lives in dioxus_document.rs, which the session did not reach.
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: set_focus on mounted elements lives in events.rs, which the session did not reach, and the new actions read changes no focus.
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the MountedData buffered render lives in dioxus_document.rs and the runtime probe, neither reached, and the actions read changes no focus.
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the render-time task limit and set_focus_to live in events.rs, which the session did not reach.
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the autofocus Cargo feature forward lives in the dioxus-native-dom and dioxus-native Cargo.toml files, neither reached by the session.
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: the IME TODO lives in dioxus_document.rs, which the session did not reach.
@answer n=19 entity=keyboard-navigation#4.13 v=no
why: falsy boolean attribute removal lives in mutation_writer.rs and its tests, neither reached, and check_target reads disabled without changing the bridge's attribute handling.
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: browser urlbar keys live in apps/browser toolbar.rs, which the session did not touch.
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts live in apps/browser tab_strip.rs, which the session did not touch.
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: browser new-tab autofocus lives in apps/browser about_pages.rs, which the session did not touch.
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts live in apps/readme, which the session did not touch.
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: use_back_button lives in packages/dioxus-native hooks.rs, which the session did not reach.
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: todomvc focus and keys live in examples/todomvc app.rs, which the session did not touch.
