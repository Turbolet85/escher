# Classified — a11y-plan · keyboard-navigation.b3

## Answers
keyboard-navigation#3.24 · yes · add-to · F4 and F5 belong beside this keyboard-handling dispatch line: a key press with focus on a control whose anonymous block was dropped no longer panics (F4), and that focused control keeps focus and takes keys (F5)
keyboard-navigation#3.25 · no · macOS binding forwarding lives in blitz-shell and blitz-traits events.rs, none reached by the session
keyboard-navigation#3.26 · no · IME forwarding lives in blitz-shell and blitz-traits events.rs and shell.rs, none reached by the session
keyboard-navigation#4.1 · no · focus stays on the control after its block drops, so document.activeElement still returns it, and the binding files are unreached
keyboard-navigation#4.2 · yes · stale · blitz-dom's autofocus rule sits in the reached blitz-dom tree, reached whole, and the report shows no reading of its any-present-value rule holding
keyboard-navigation#4.3 · yes · stale · display and visibility reach the hidden-content exclusion in reached blitz-dom, the restyle narrowing covers attribute-driven hidden states, and no report line shows display's reach holding
keyboard-navigation#4.4 · no · JS keyboard event fields live in blitz-vibey-script event.rs, unreached by the session
keyboard-navigation#4.5 · yes · stale · dioxus_document.rs:391-395 sits past the +5 added at 235-239, so the citation moves to 396-400
keyboard-navigation#4.6 · yes · stale · dioxus_document.rs:386-389 sits past the +5 added at 235-239, so the citation moves to 391-394
keyboard-navigation#4.7 · no · set_focus lives in dioxus-native-dom events.rs, unreached by the session
keyboard-navigation#4.8 · no · the RefCell panic cites events.rs:162, unreached, and the synchronous-handle reading holds through stand_keyboard, which passes unedited
keyboard-navigation#4.9 · no · the autofocus feature forwards through two manifests, both byte-identical
keyboard-navigation#4.10 · yes · stale · dioxus_document.rs:404-405 sits past the +5 added at 235-239, so the citation moves to 409-410
keyboard-navigation#4.11 · no · the falsy-removal path is in mutation_writer.rs, unreached, and inert_subtree and stand_keyboard hold green with rows unedited, so inert and hidden taking effect still holds
keyboard-navigation#5.1 · no · the browser urlbar lives in apps/browser, unreached by the session
keyboard-navigation#5.2 · no · browser tab shortcuts live in apps/browser, unreached by the session
keyboard-navigation#5.3 · no · the browser new-tab search lives in apps/browser, unreached by the session
keyboard-navigation#5.4 · no · rdme shortcuts live in apps/readme, unreached by the session
keyboard-navigation#5.5 · no · use_back_button lives in packages/dioxus-native hooks.rs, unreached by the session
keyboard-navigation#5.6 · no · the todomvc example lives in examples/todomvc, unreached by the session
keyboard-navigation#5.7 · no · the Cells commit lives in seven_guis cells.rs, which has no diff
keyboard-navigation#5.8 · no · the Cells grid walks in stand_keyboard.rs stay green, and that file holds no edit
keyboard-navigation#5.9 · no · the Cells editor walks in stand_keyboard.rs:202-249 stay green, and that file holds no edit
keyboard-navigation#5.10 · no · seven_guis crud.rs has no diff and stand_keyboard.rs:932-992 holds green
keyboard-navigation#5.11 · no · the Preact index.html lies outside the session's reach
