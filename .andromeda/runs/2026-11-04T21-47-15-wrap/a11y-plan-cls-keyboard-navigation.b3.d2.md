@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic_click_event lives in blitz-dom node.rs, which the session did not change
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key-event conversion lives in blitz-shell, which the session did not reach
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key dispatch in blitz-shell window.rs is not in the session's changed files
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the anonymous-block key press lives in blitz-dom and its own test, neither reached by the session
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings live in blitz-shell and blitz-traits, not reached by the session
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding lives in blitz-shell and blitz-traits, not reached by the session
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus and blur live in blitz-vibey-script, not reached by the session
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection lives in blitz-vibey-script and no manifest feature changed
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors live in blitz-vibey-script, not reached by the session
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields live in blitz-vibey-script, not reached by the session
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: dioxus_document.rs key forwarding is only read by the session at :295 and :297, not changed, and the file is not a changed file
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur forwarding at dioxus_document.rs:386-389 is not changed by the session
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus at events.rs:295 sits inside the cited 294-306 and the mounted focus claim still holds at HEAD, with events.rs not a changed file
@answer n=14 entity=keyboard-navigation#4.8 v=yes mode=add-to
why: the session pins the MountedData hazard as two should_panic tests (F4) and re-measures its polling path at HEAD through handle_task_wakeup with moved coordinates at dioxus_document.rs:295 and :297
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the autofocus feature forward in the Cargo.toml features is unchanged, and the Cargo.lock change moves only pulldown-cmark
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: IME handling in dioxus_document.rs is not changed by the session
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the session leaves the boolean-attribute rule and mutation_writer.rs unchanged
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar keys live in apps/browser, not reached by the session
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: the browser tab shortcuts live in apps/browser, not reached by the session
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search input lives in apps/browser, not reached by the session
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts live in apps/readme, not reached by the session
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: use_back_button lives in dioxus-native hooks, not reached by the session
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: the todomvc focus and key handling lives in examples/todomvc, which the session did not change
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: the Cells edit commit lives in examples/seven_guis, which the session did not change
@answer n=25 entity=keyboard-navigation#5.8 v=yes mode=add-to
why: the session measures cells-grid as the one Cells element reacting to keydown and the 676 cells to click, a reading this entry must carry beside its arrow and scroll_to claims
