@answer n=1 entity=keyboard-navigation#3.22 v=no
why: the synthetic click builder sits in blitz-dom node.rs, which the session does not reach
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: the winit key conversion sits in blitz-shell convert_events.rs, which the session does not reach
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: the shell's key dispatch in blitz-shell window.rs is outside the session's files
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the anonymous-block guard and its test live in blitz-dom and blitz-tests, neither reached by the session
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: the macOS keybinding forwarding lives in blitz-shell and blitz-traits, which the session does not reach
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: the IME forwarding lives in blitz-shell and blitz-traits, which the session does not reach
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: focus and blur in blitz-vibey-script are outside the session's files
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection lives in blitz-vibey-script, which the session does not reach
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: the CSS property accessors live in blitz-vibey-script, which the session does not reach
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: the JS keyboard event fields live in blitz-vibey-script, which the session does not reach
@answer n=11 entity=keyboard-navigation#4.5 v=yes mode=add-to
why: the bridge's handle_event in dioxus_document.rs now sets a SignalRegistry dispatch around the runtime call for the events it forwards, and the list of keyboard events reaching Dioxus gains F6, a write resolving along its dispatch chain to the first listener of its event
@answer n=12 entity=keyboard-navigation#4.6 v=yes mode=stale
why: the bridge's handle_event in dioxus_document.rs, reached whole by the session, may have moved the cited focus-data lines 409-412
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus and the unqueued focus events live in events.rs, which the session does not reach
@answer n=14 entity=keyboard-navigation#4.8 v=yes mode=stale
why: the bridge's handle_event in dioxus_document.rs, reached whole by the session, may have moved the cited lines 308-310, and cells.rs, also reached, carries the synchronous handle calls it describes
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the render-time limit is about render buffering and set_focus_to, which the session's event-dispatch change does not touch
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the autofocus feature forward is in Cargo manifests, which no session change moved
@answer n=17 entity=keyboard-navigation#4.11 v=yes mode=stale
why: the bridge's handle_event in dioxus_document.rs, reached whole by the session, may have moved the cited IME TODO lines 427-428
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: the falsy boolean attribute rule lives in mutation_writer.rs and its tests, neither reached by the session
@answer n=19 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar lives in apps/browser, which the session does not reach
@answer n=20 entity=keyboard-navigation#5.2 v=no
why: the browser tab shortcuts live in apps/browser, which the session does not reach
@answer n=21 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab input lives in apps/browser, which the session does not reach
@answer n=22 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts live in apps/readme, which the session does not reach
@answer n=23 entity=keyboard-navigation#5.5 v=no
why: use_back_button lives in packages/dioxus-native hooks.rs, which the session does not reach
@answer n=24 entity=keyboard-navigation#5.6 v=no
why: the todomvc input handling lives in examples/todomvc, which the session does not reach
@answer n=25 entity=keyboard-navigation#5.7 v=yes mode=stale
why: examples/seven_guis/src/tasks/cells.rs, reached whole by the session, may have moved the cited edit-commit lines 500-520
