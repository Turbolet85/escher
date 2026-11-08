@answer n=1 entity=keyboard-navigation#3.22 v=no
why: the synthetic click builder lives in blitz-dom node.rs, which the session does not reach
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion lives in blitz-shell, which the session does not reach
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: the shell's KeyDown and KeyUp dispatch lives in blitz-shell window.rs, which the session does not reach
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the anonymous-block fix and its test live in blitz-dom and blitz-tests files the session does not reach
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS key bindings are forwarded in blitz-shell and blitz-traits, neither reached by the session
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME forwarding lives in blitz-shell and blitz-traits, neither reached by the session
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: focus() and blur() live in blitz-vibey-script, which the session does not reach
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection lives in blitz-vibey-script, which the session does not reach
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: the CSSStyleDeclaration accessors live in blitz-vibey-script, which the session does not reach
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: the JS keyboard event fields live in blitz-vibey-script, which the session does not reach
@answer n=11 entity=keyboard-navigation#4.5 v=yes mode=add-to
why: the bridge's event path, where the session's handle_event dispatch wrap sits in dioxus_document.rs, now carries F6, a write resolved along its event's dispatch chain to the first listener element
@answer n=12 entity=keyboard-navigation#4.6 v=yes mode=stale
why: the session reaches dioxus_document.rs whole, and the focus-forwarding lines 409-412 may have moved with the handle_event change
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus and the unqueued focus events live in events.rs, which the session does not reach
@answer n=14 entity=keyboard-navigation#4.8 v=yes mode=stale
why: the session reaches dioxus_document.rs and cells.rs whole, so the buffered-render lines 308-310 and Cells' synchronous handle calls may have moved
@answer n=15 entity=keyboard-navigation#4.9 v=yes mode=stale
why: the render limit lives in the poll path of dioxus_document.rs, which the session reaches whole
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the autofocus feature lives in the dioxus-native-dom and dioxus-native manifests, which the session does not reach and no dependency moved
@answer n=17 entity=keyboard-navigation#4.11 v=yes mode=stale
why: the IME TODO cited at dioxus_document.rs:427-428 sits in a file the session reaches whole
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: the falsy-attribute path lives in mutation_writer.rs and stand_keyboard.rs, neither reached by the session
@answer n=19 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar keys live in apps/browser, which the session does not reach
@answer n=20 entity=keyboard-navigation#5.2 v=no
why: the browser tab shortcuts live in apps/browser, which the session does not reach
@answer n=21 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab autofocus lives in apps/browser, which the session does not reach
@answer n=22 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts live in apps/readme, which the session does not reach
@answer n=23 entity=keyboard-navigation#5.5 v=no
why: use_back_button lives in packages/dioxus-native hooks.rs, which the session does not reach
@answer n=24 entity=keyboard-navigation#5.6 v=no
why: the todomvc key handling lives in examples/todomvc, which the session does not reach
@answer n=25 entity=keyboard-navigation#5.7 v=yes mode=stale
why: examples/seven_guis/src/tasks/cells.rs is reached whole, and its cited Enter-or-blur commit lines 500-520 may have moved
