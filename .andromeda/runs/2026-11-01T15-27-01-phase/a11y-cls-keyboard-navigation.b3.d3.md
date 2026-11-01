# Classified — a11y · keyboard-navigation.b3

## Answers
keyboard-navigation#3.24 · yes · key presses are dispatched as KeyDown and KeyUp, the verb the scope's key-press panic run and its Enter and Space checks send through
keyboard-navigation#3.25 · no · macOS AppleStandardKeybinding forwarding in blitz-shell and blitz-traits; the scope names no such binding and no reached file carries it
keyboard-navigation#3.26 · no · IME forwarding in blitz-shell and blitz-traits; the scope names no IME path
keyboard-navigation#4.1 · yes · script focus(), blur() and document.activeElement, the script exposure of keyboard focus that the events.rs and bootstrap.rs add/add resolution keeps
keyboard-navigation#4.2 · no · autofocus reflection and blitz-dom's autofocus start focus; the scope names Tab orders and no initial-focus path
keyboard-navigation#4.3 · no · script CSSStyleDeclaration accessors; the scope names no script style write, and the hidden-content values it names are default-sheet values
keyboard-navigation#4.4 · yes · JS keyboard event fields, the script exposure of keyboard events that the events.rs and bootstrap.rs resolution keeps
keyboard-navigation#4.5 · yes · KeyDown, KeyUp and KeyPress keyboard data into Dioxus at dioxus_document.rs, a conflict surface, read by the key-press and keyboard checks
keyboard-navigation#4.6 · yes · Focus, Blur, FocusIn and FocusOut focus data at dioxus_document.rs, a conflict surface, read by the focus-return and focus-order checks
keyboard-navigation#4.7 · yes · set_focus on mounted elements and unqueued focus and blur events, the call Cells' focus return rests on
keyboard-navigation#4.8 · yes · a MountedData call from a key-handler task panicking on RefCell, the synchronous handle calls cells.rs must keep and not defer
keyboard-navigation#4.9 · no · autofocus feature forwarding in the dioxus-native-dom and dioxus-native manifests; the scope names no autofocus feature
keyboard-navigation#4.10 · no · IME not handled in dioxus-native-dom; the scope names no IME path
keyboard-navigation#4.11 · yes · falsy boolean attributes removing disabled, hidden and inert through the bridge's is_bool_attr, which a26c6ddeb and 286874cc1 reach
keyboard-navigation#5.1 · no · browser urlbar keys in apps/browser, outside the reached files and the scope
keyboard-navigation#5.2 · no · browser tab shortcuts in apps/browser, outside the reached files and the scope
keyboard-navigation#5.3 · no · browser new-tab search input in apps/browser, outside the reached files and the scope
keyboard-navigation#5.4 · no · rdme shortcuts in apps/readme, outside the reached files and the scope
keyboard-navigation#5.5 · no · use_back_button in dioxus-native hooks, outside the reached files and the scope
keyboard-navigation#5.6 · no · todomvc keys in examples/todomvc, outside the reached files and the scope
keyboard-navigation#5.7 · yes · Cells commits an edit on Enter or blur at cells.rs, a conflict surface, the editor commit the scope names
keyboard-navigation#5.8 · yes · cells-grid as one Tab stop, its arrows and the synchronous scroll_to into view, the scope's cells-grid arrows and Enter
keyboard-navigation#5.9 · yes · Cells' editor Enter, commit, propagation stop and focus return to the grid, the scope's editor commit and focus return
keyboard-navigation#5.10 · yes · the seven_guis CRUD list walk keys and the unread aria-activedescendant, which stand_keyboard's list walks and accessibility_active_descendant_unread carry
keyboard-navigation#5.11 · no · TodoMVC Preact input in examples/preact, outside the reached files and the scope
