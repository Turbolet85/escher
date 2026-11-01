# Classified — a11y · keyboard-navigation.b3

## Answers
keyboard-navigation#3.24 · no · shell-side key dispatch in blitz-shell window.rs, outside the reached files and the scope names no shell path
keyboard-navigation#3.25 · no · macOS AppleStandardKeybinding forwarding in blitz-shell, outside the reached files and the scope
keyboard-navigation#3.26 · no · IME forwarding in blitz-shell, outside the reached files and the scope names no IME path
keyboard-navigation#4.1 · yes · script exposure of focus(), blur() and activeElement, the keyboard exposure the bootstrap.rs and events.rs resolution keeps
keyboard-navigation#4.2 · yes · autofocus reflection writing "true" and blitz-dom's any-present-value handling, script exposure of keyboard focus the resolution keeps
keyboard-navigation#4.3 · no · CSS property accessors in style.js, not keyboard or event exposure; the bootstrap.rs clause covers keyboard and events only
keyboard-navigation#4.4 · yes · JS keyboard event fields in event.rs, the script keyboard exposure the events.rs and bootstrap.rs resolution keeps
keyboard-navigation#4.5 · yes · KeyDown, KeyUp and KeyPress keyboard data to Dioxus at dioxus_document.rs, a conflict surface
keyboard-navigation#4.6 · yes · Focus, Blur, FocusIn and FocusOut data at dioxus_document.rs, a conflict surface, the focus order the stand checks read
keyboard-navigation#4.7 · yes · set_focus with unqueued focus events, the call Cells' editor focus return makes in cells.rs
keyboard-navigation#4.8 · yes · MountedData call from a key-handler task panics, the synchronous-handle rule Cells' cells.rs code must honour under the key-press scope
keyboard-navigation#4.9 · no · autofocus Cargo feature forwarding in package Cargo.toml files, outside the reached files and the scope
keyboard-navigation#4.10 · yes · IME TODO at dioxus_document.rs:400-401, a conflict surface whose rule binds under the merge
keyboard-navigation#4.11 · yes · falsy boolean attributes (is_bool_attr, disabled, hidden, inert) named by the bridge's boolean attribute scope; stand_keyboard and inert_subtree read them
keyboard-navigation#5.1 · no · browser urlbar keys in apps/browser, outside the reached files and the scope
keyboard-navigation#5.2 · no · browser tab shortcuts in apps/browser, outside the reached files and the scope
keyboard-navigation#5.3 · no · browser new-tab search autofocus in apps/browser, outside the reached files and the scope
keyboard-navigation#5.4 · no · rdme shortcuts in apps/readme, outside the reached files and the scope
keyboard-navigation#5.5 · no · use_back_button in dioxus-native hooks, not named by the scope
keyboard-navigation#5.6 · no · todomvc example focus and keys, outside the reached files and the scope
keyboard-navigation#5.7 · yes · Cells commit on Enter or blur at cells.rs, the editor's commit the scope names
keyboard-navigation#5.8 · yes · Cells' cells-grid single Tab stop, arrows and synchronous scroll_to, the grid the cells.rs scope names and stand_keyboard re-reads
keyboard-navigation#5.9 · yes · Cells' editor open, commit, propagation stop and click-inside early return, the editor's commit and focus return in cells.rs
keyboard-navigation#5.10 · yes · CRUD list walks (ArrowDown, ArrowUp, Home, End) that stand_keyboard's list walks state, and the accessibility_active_descendant_unread witness
keyboard-navigation#5.11 · no · TodoMVC Preact autofocus in examples/preact, outside the reached files and the scope
