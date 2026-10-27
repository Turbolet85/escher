# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · yes · its rule lives in node.rs, a predicted conflict surface, so it binds whatever the hunks show
keyboard-navigation#3.10 · yes · convert_events.rs is unreached, but the Key and Modifiers it converts from winit carry the keyboard copy and paste in a focused field that the scope names (527394724)
keyboard-navigation#3.11 · yes · window.rs, a merge surface, dispatches every KeyDown and KeyUp after the shell shortcuts, the path the keyboard activation, typeahead and range keys take
keyboard-navigation#3.12 · yes · its rule lives in window.rs and blitz-traits events.rs, both merge surfaces
keyboard-navigation#3.13 · yes · its IME event forwarding and shell enable hooks live in window.rs and blitz-traits events.rs, both merge surfaces, the surface 7fc9ee3af reaches
keyboard-navigation#4.1 · no · its JS focus, blur and activeElement rules live in blitz-vibey-script dom/element.rs and dom/document.rs, neither reached, and the scope names no JS focus
keyboard-navigation#4.2 · no · its autofocus reflection lives in blitz-vibey-script element.rs and Cargo.toml, neither reached, and its blitz-dom autofocus clause cites the Focus order section, outside this batch, the scope naming no autofocus
keyboard-navigation#4.3 · no · the JS keyboard event fields live in blitz-vibey-script dom/event.rs, unreached, and the scope names no JS keyboard event
keyboard-navigation#4.4 · yes · its rule lives partly in dioxus_document.rs:371-375, a merge surface, and the CRUD list's keydown handler reads the keyboard data it states
keyboard-navigation#4.5 · yes · its rule lives in dioxus_document.rs:366-369, a merge surface
keyboard-navigation#4.6 · no · set_focus and the unqueued focus events live in dioxus-native-dom events.rs, unreached, and the scope names no mounted set_focus call
keyboard-navigation#4.7 · no · its autofocus forward sits in the dioxus-native Cargo.toml, not the clipboard-defaults edit the scope names there, and the scope names no autofocus
keyboard-navigation#4.8 · yes · its TODO lives in dioxus_document.rs:384-385, a merge surface that 7fc9ee3af edits for IME
keyboard-navigation#4.9 · yes · the scope names it: a falsy bridge boolean attribute (disabled, hidden, inert) removes the attribute, the Dioxus 0.8 change 286874cc1 reaches that bridge, and stand_keyboard and inert_subtree read it
keyboard-navigation#5.1 · no · the urlbar keys are apps/browser's own handlers, an app the chunk neither reaches nor proves, and the scope names no browser shortcut
keyboard-navigation#5.2 · no · the tab shortcuts are apps/browser's own handlers, unreached and unproven by the chunk
keyboard-navigation#5.3 · no · the browser's new-tab autofocus and Enter submit are apps/browser's, unreached and outside the stand
keyboard-navigation#5.4 · no · the rdme shortcuts are apps/readme's own handlers, unreached and outside the stand
keyboard-navigation#5.5 · no · use_back_button is dioxus-native's hooks.rs, unreached, and the scope names no back key
keyboard-navigation#5.6 · no · the todomvc example is outside the stand and the chunk's proofs, and the scope names no todomvc behaviour
keyboard-navigation#5.7 · yes · the stand's Cells task, inside the stand the chunk re-runs in both layout modes, and its Enter and blur commit ride the key forwarding and focus moves the merge surfaces carry
keyboard-navigation#5.8 · yes · the scope names it: the CRUD list's keyboard model and unread aria-activedescendant, re-asserted by stand_keyboard and accessibility_active_descendant_unread
keyboard-navigation#5.9 · no · the Preact example page is outside the stand and the chunk's proofs
keyboard-navigation#5.10 · no · Android Activity focus is platform code the chunk neither reaches nor proves
keyboard-navigation#6.1 · no · the docs.rs fixture's markup is content the chunk neither edits (no fixture is edited) nor reads, and the tabindex rule it exercises binds in node.rs and mutator.rs on its own
