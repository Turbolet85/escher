@answer n=1 entity=keyboard-navigation#3.22 v=no
why: the session touches no blitz-dom click synthesis, and the reached files are the probe test, Cargo.lock and deny.toml only
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion lives in blitz-shell, which the session does not reach
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: KeyDown and KeyUp dispatch in blitz-shell window.rs is unchanged by the session
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the anonymous-block drop and its key-press test are blitz-dom code and a test the session did not touch
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS keybinding forwarding in blitz-shell and blitz-traits is untouched by the session
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME forwarding in blitz-shell and blitz-traits is untouched by the session
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: focus, blur and activeElement bindings in blitz-vibey-script are outside the session's reach
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection in blitz-vibey-script and its feature wiring are unchanged, and no manifest moved
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: the CSS property accessors in blitz-vibey-script are untouched by the session
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: the JS keyboard event fields in blitz-vibey-script are untouched by the session
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: KeyDown, KeyUp and KeyPress forwarding in dioxus_document.rs and events.rs is unchanged, and the session reaches neither file
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur event forwarding at dioxus_document.rs:386-389 is unchanged by the session
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus at events.rs:295 still sits inside the cited 294-306, and no reached file moves it
@answer n=14 entity=keyboard-navigation#4.8 v=yes mode=add-to
why: the session pins this MountedData hazard as two should_panic tests in both layout modes, the F4 fact beside it
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the autofocus feature forwarding at dioxus-native-dom/Cargo.toml:23 is unchanged, and the Cargo.lock move touches only pulldown-cmark
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the IME TODO at dioxus_document.rs:404-405 is untouched by the session
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: mutation_writer.rs is unchanged and the boolean-attribute path is untouched by the session
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: the browser urlbar keys in apps/browser are untouched by the session
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: the browser tab shortcuts in apps/browser are untouched by the session
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search input in apps/browser is untouched by the session
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: the rdme shortcuts in apps/readme are untouched by the session
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: use_back_button in dioxus-native hooks.rs is untouched by the session
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: the todomvc example is untouched by the session
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: the Cells edit commit on Enter or blur at cells.rs is an example file the session does not reach, and the click and keydown reading leaves it true
@answer n=25 entity=keyboard-navigation#5.8 v=no
why: the session re-read the synchronous scroll_to and NodeHandle claims and they hold unmoved, and stand_keyboard.rs is untouched
