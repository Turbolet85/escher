# Classified — a11y-plan · keyboard-navigation.b3

## Answers
keyboard-navigation#3.24 · no · the key dispatch sits in packages/blitz-shell/src/window.rs, which the session does not reach
keyboard-navigation#3.25 · no · the macOS bindings sit in blitz-shell application.rs and window.rs and blitz-traits events.rs, none reached
keyboard-navigation#3.26 · no · the IME forwarding sits in blitz-shell convert_events.rs and window.rs and blitz-traits events.rs and shell.rs, none reached
keyboard-navigation#4.1 · no · focus() and blur() sit in blitz-vibey-script element.rs, form_controls.rs and document.rs, none reached
keyboard-navigation#4.2 · no · the autofocus reflection sits in blitz-vibey-script form_controls.rs and Cargo.toml, none reached
keyboard-navigation#4.3 · no · the style accessors sit in blitz-vibey-script css.rs, bootstrap/style.js and tests/dom.rs, none reached; the merge-surface bootstrap.rs is not cited by the entity
keyboard-navigation#4.4 · no · the JS keyboard event fields sit in blitz-vibey-script dom/event.rs, not reached
keyboard-navigation#4.5 · yes · stale · dioxus_document.rs is a merge surface and its +5 at 235-239 moves the cited lines 391-395 down
keyboard-navigation#4.6 · yes · stale · dioxus_document.rs is a merge surface and its +5 at 235-239 moves the cited lines 386-389 down
keyboard-navigation#4.7 · no · set_focus sits in dioxus-native-dom events.rs, which is not reached
keyboard-navigation#4.8 · no · the RefCell panic sits in events.rs:162, not reached; the session retires only the anonymous-block key-press panic
keyboard-navigation#4.9 · no · the autofocus forward sits in two Cargo.toml manifests, which are byte-identical
keyboard-navigation#4.10 · yes · stale · dioxus_document.rs is a merge surface and its +5 at 235-239 moves the cited IME TODO at lines 404-405 down
keyboard-navigation#4.11 · no · the falsy-attribute removal holds on the narrowed restyle path: inert_subtree, stand_keyboard and the falsy tests stay green with rows unedited, and the entity does not state the restyle mechanism
keyboard-navigation#5.1 · no · the browser urlbar sits in apps/browser toolbar.rs, not reached
keyboard-navigation#5.2 · no · the browser tab shortcuts sit in apps/browser tab_strip.rs, not reached
keyboard-navigation#5.3 · no · the new-tab autofocus sits in apps/browser about_pages.rs, not reached
keyboard-navigation#5.4 · no · the rdme shortcuts sit in apps/readme readme_application.rs, not reached
keyboard-navigation#5.5 · no · use_back_button sits in packages/dioxus-native hooks.rs, not reached
keyboard-navigation#5.6 · no · the todomvc focus and key rows sit in examples/todomvc app.rs, not reached
keyboard-navigation#5.7 · no · the Cells commit sits in examples/seven_guis cells.rs, not reached
keyboard-navigation#5.8 · no · the Cells grid tests sit in stand_keyboard.rs, not reached, and it holds green with its rows unedited
keyboard-navigation#5.9 · no · the Cells editor rows sit in cells.rs and stand_keyboard.rs, neither reached
keyboard-navigation#5.10 · no · the CRUD list rows sit in crud.rs, stand_keyboard.rs and accessibility_active_descendant_unread.rs, none reached
keyboard-navigation#5.11 · no · the TodoMVC Preact autofocus sits in examples/preact index.html, not reached
