@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic_click_event lives in blitz-dom node.rs, which the session does not reach
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion lives in blitz-shell convert_events.rs, which the session does not reach
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key press and release dispatch lives in blitz-shell window.rs, which the session does not reach
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the anonymous-block fix and its test live in blitz-dom document.rs and key_press_after_container_lost_its_box.rs, neither reached by the session
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings live in blitz-shell and blitz-traits, which the session does not reach
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME forwarding lives in blitz-shell and blitz-traits, which the session does not reach
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: focus and blur bindings live in blitz-vibey-script, which the session does not reach
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection lives in blitz-vibey-script, which the session does not reach
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: the CSSStyleDeclaration accessors live in blitz-vibey-script, which the session does not reach
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields live in blitz-vibey-script event.rs, which the session does not reach
@answer n=11 entity=keyboard-navigation#4.5 v=yes mode=stale
why: dioxus_document.rs is reached whole by the new reached record on DioxusDocument, so the cited lines 415-419 can have moved and the report does not show the claim holds there
@answer n=12 entity=keyboard-navigation#4.6 v=yes mode=add-to
why: the bridge's handle_event gains the ungated reached record (F1), written before runtime.handle_event beside the SignalRegistry dispatch this row describes
@answer n=13 entity=keyboard-navigation#4.7 v=yes mode=stale
why: dioxus_document.rs is reached whole by the new reached record on DioxusDocument, so the cited lines 410-413 can have moved and the report does not show the claim holds there
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: set_focus and the unqueued focus events live in events.rs, which the session does not reach
@answer n=15 entity=keyboard-navigation#4.9 v=yes mode=stale
why: the buffered render in initial_build and poll (dioxus_document.rs:309-311) sits in the reached file, so its cited lines can have moved and the report does not show the claim holds there
@answer n=16 entity=keyboard-navigation#4.10 v=yes mode=stale
why: the render-buffer limit's rule lives in the reached file dioxus_document.rs, and the report does not show it still holds on the changed file
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the autofocus forwarding lives in Cargo.toml files the session does not reach
@answer n=18 entity=keyboard-navigation#4.12 v=yes mode=stale
why: the IME TODO cited at dioxus_document.rs:428-429 sits in the reached file, so its cited lines can have moved and the report does not show the claim holds there
@answer n=19 entity=keyboard-navigation#4.13 v=no
why: the falsy boolean attribute rule lives in mutation_writer.rs and its tests, none reached by the session
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar keys live in apps/browser toolbar.rs, which the session does not reach
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: the browser tab shortcuts live in apps/browser tab_strip.rs, which the session does not reach
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search autofocus lives in apps/browser about_pages.rs, which the session does not reach
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts live in apps/readme readme_application.rs, which the session does not reach
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: use_back_button lives in packages/dioxus-native hooks.rs, which the session does not reach
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: the todomvc key handling lives in examples/todomvc app.rs, which the session does not reach
