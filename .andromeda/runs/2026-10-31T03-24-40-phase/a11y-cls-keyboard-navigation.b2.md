# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.8 · yes · events/ is reached whole, and a click on a select's label focusing the select is a focus route into the select interaction that select_interaction re-reads
keyboard-navigation#3.9 · yes · node.rs is reached whole, and this click at the node's center is the path the Enter and Space activation of load-fetch and the native checkbox or radio Space route depend on
keyboard-navigation#3.10 · no · winit key conversion in blitz-shell; the keyboard rows press through the harness, and the redraw check reads the focus redraw, not key identity
keyboard-navigation#3.11 · yes · the event-loop key dispatch on the windowed shell's route that focus_redraw reads: a key reaches the document as KeyDown only after shell shortcuts, so the redraw check's key must avoid one
keyboard-navigation#3.12 · no · macOS standard key bindings forwarded as AppleStandardKeybinding UI events, a macOS-only path the chunk's scope and Needs do not name
keyboard-navigation#3.13 · no · IME events forwarded to the document; no debt touches IME
keyboard-navigation#4.1 · no · script-side focus(), blur() and document.activeElement in blitz-vibey-script; the chunk's focus checks read the Rust tree and snapshot
keyboard-navigation#4.2 · no · the script-side autofocus reflection in blitz-vibey-script; the chunk names no autofocus row or initial-focus need
keyboard-navigation#4.3 · no · script-side CSSStyleDeclaration accessors in blitz-vibey-script, pinned by its own dom tests; the chunk names no script write of display or visibility
keyboard-navigation#4.4 · no · JS keyboard event fields in blitz-vibey-script; the chunk's keyboard rows drive the Rust harness, not script events
keyboard-navigation#4.5 · yes · the keyboard data by which KeyDown and KeyUp reach the served pages' Dioxus handlers carries the Enter, Space and arrow keys the activation and CRUD rows press
keyboard-navigation#4.6 · no · Dioxus focus, blur, focusin and focusout data; the chunk's focus rows read the focused node, and no Need names a focus event
keyboard-navigation#4.7 · no · the Dioxus set_focus call and its unqueued focus events; the focus fixup is engine-side and no Need names a set_focus route
keyboard-navigation#4.8 · no · the autofocus Cargo feature forwarding; the chunk changes no manifest and bumps no dependency
keyboard-navigation#4.9 · no · IME events in dioxus-native-dom; no debt touches IME
keyboard-navigation#4.10 · yes · a truthy inert takes focusability, hits and the tree away from its subtree, and a falsy inert or open is removed, which inert_subtree.rs and stand_keyboard.rs assert and the fixup and the :open check must keep
keyboard-navigation#5.1 · no · the browser urlbar's arrow, Escape and Enter handling in apps/browser; no served page or reached file
keyboard-navigation#5.2 · no · browser tab shortcuts in apps/browser; no served page or reached file
keyboard-navigation#5.3 · no · browser new-tab search autofocus in apps/browser; no served page or reached file
keyboard-navigation#5.4 · no · rdme's Ctrl and Cmd shortcuts in apps/readme; no served page or reached file
keyboard-navigation#5.5 · no · use_back_button's BrowserBack handling in dioxus-native's hooks; the chunk names no back key
keyboard-navigation#5.6 · no · TodoMVC's Dioxus focus and Enter, Escape and Tab editing; no served row, reached file or Need names TodoMVC
keyboard-navigation#5.7 · yes · Cells commits an edit on Enter or blur, the activation a Cells keyboard row would assert, and Cells' keyboard rows are in this entry
keyboard-navigation#5.8 · yes · the CRUD list's third Tab stop, its ArrowDown, ArrowUp, Home and End keys, and the tree's focus on the list, witnessed at stand_keyboard.rs:720-780 under SC 2.1.1
keyboard-navigation#5.9 · no · the Preact TodoMVC input's autofocus and Enter in examples/preact; no served row, reached file or Need names it
