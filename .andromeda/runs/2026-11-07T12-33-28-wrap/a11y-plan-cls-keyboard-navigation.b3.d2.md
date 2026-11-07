@answer n=1 entity=keyboard-navigation#3.22 v=no
why: the synthetic click is built in blitz-dom node.rs, which the session does not reach
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion lives in blitz-shell convert_events.rs, which the session does not reach
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: shell key dispatch lives in blitz-shell window.rs, which the session does not reach
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the anonymous-block claim lives in blitz-dom document.rs and a blitz-tests key-press test, neither reached, and the engine key rules are unchanged
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS keybinding forwarding lives in blitz-shell and blitz-traits, which the session does not reach
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME forwarding lives in blitz-shell and blitz-traits, which the session does not reach
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus, blur and activeElement live in blitz-vibey-script, which the session does not reach
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection and its Cargo.toml feature live in blitz-vibey-script, which the session does not reach
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: the CSS property accessors live in blitz-vibey-script, which the session does not reach
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: the JS keyboard event fields live in blitz-vibey-script, which the session does not reach
@answer n=11 entity=keyboard-navigation#4.5 v=yes mode=stale
why: the buffered render reaches dioxus_document.rs whole, so the cited key-event forwarding lines 401-405 may have moved and nothing shows the claim holds on the changed file
@answer n=12 entity=keyboard-navigation#4.6 v=yes mode=stale
why: the buffered render reaches dioxus_document.rs whole, so the cited focus and blur forwarding lines 396-399 may have moved and nothing shows the claim holds on the changed file
@answer n=13 entity=keyboard-navigation#4.7 v=yes mode=add-to
why: F1 adds that a task polled during render acts on the document as the previous render left it, so set_focus on a node created in the same render does not focus it
@answer n=14 entity=keyboard-navigation#4.8 v=yes mode=contradict
why: the buffered render replays the mutation buffer under one borrow, so the spawned task's set_focus no longer panics with RefCell already borrowed, closing the hazard this entity states as standing
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the autofocus feature forwarding lives in Cargo.toml files, which the session does not change
@answer n=16 entity=keyboard-navigation#4.10 v=yes mode=stale
why: the buffered render reaches dioxus_document.rs whole, so the cited IME TODO at lines 414-415 may have moved and nothing shows the claim holds on the changed file
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the falsy boolean-attribute rule and its 28-name list are unchanged and the replay applies the same mutations in the same order
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar lives in apps/browser, which the session does not reach
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: the browser tab shortcuts live in apps/browser, which the session does not reach
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search autofocus lives in apps/browser, which the session does not reach
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts live in apps/readme, which the session does not reach
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: use_back_button lives in packages/dioxus-native hooks.rs, which the session does not reach
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: the todomvc focus and key handling lives in examples/todomvc, which the session does not reach
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: the Cells edit commit lives in examples/seven_guis, which the session does not reach
@answer n=25 entity=keyboard-navigation#5.8 v=no
why: the plain synchronous scroll_to statement stays true because the calls are kept, and stand_keyboard.rs is not reached
