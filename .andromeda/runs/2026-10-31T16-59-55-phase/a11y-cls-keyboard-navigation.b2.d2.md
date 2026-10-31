# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.5 · yes · a multiple select's Space and typeahead keys in keyboard.rs, a file the change edits, and the scope keeps a multiple select's keys as built
keyboard-navigation#3.6 · yes · a multiple select's disabled rows are never chosen and Ctrl+A is not modelled, in keyboard.rs and select.rs (reached whole), and the scope keeps a multiple select's keys as built
keyboard-navigation#3.7 · yes · a multiple select's Shift span from its last chosen row in select.rs (reached whole), which the scope keeps as built
keyboard-navigation#3.8 · yes · an optgroup's options are the select's own in tree order and read disabled through option_disabled, the group label the chunk paints and reads
keyboard-navigation#3.9 · no · the multiple-write record and set_option_selectedness sit in mutator.rs, which the chunk does not reach, and the scope names no call carrying them since the chunk's choice writes only a closed single select
keyboard-navigation#3.10 · yes · the driver's closed sets hold no verb, argument, key name or result for a multiple select's type, which the chunk's no-verb-added rule over the driver must keep
keyboard-navigation#3.11 · yes · press holds no Ctrl and the schema help states that limit at schema.rs, a help unit the chunk edits
keyboard-navigation#3.12 · yes · the Enter and Space activation lists and the disabled gate, which the scope names as the surface a fork over those keys must act on
keyboard-navigation#3.13 · no · macOS Backspace is left to Apple keybindings in text.rs, which the chunk does not reach, and the scope names no select key over it
keyboard-navigation#3.14 · no · action plus C copies selected text outside the select keys unit the chunk edits
keyboard-navigation#3.15 · no · implicit form submission of text inputs, not a select unit the chunk edits
keyboard-navigation#3.16 · yes · a label click runs the select's default click and focuses the select, the click-on-select rule the chunk changes in pointer.rs
keyboard-navigation#3.17 · yes · synthetic_click_event centres the click, which the driver's act on an open-list option uses, and the scope names that act
keyboard-navigation#3.18 · no · winit key conversion in blitz-shell, not reached and not named by the scope
keyboard-navigation#3.19 · no · shell shortcut ordering before KeyDown dispatch, a path the chunk uses without changing
keyboard-navigation#3.20 · no · macOS standard key bindings forwarded by blitz-shell, not reached and not named by the scope
keyboard-navigation#3.21 · no · IME events, which the task's Not needed list excludes since the list adds no composition surface
keyboard-navigation#4.1 · no · JS focus, blur and activeElement in blitz-vibey-script, not reached and not named by the scope
keyboard-navigation#4.2 · no · autofocus reflection in blitz-vibey-script, not reached and not named by the scope
keyboard-navigation#4.3 · no · JS style accessors reaching the hidden exclusion, a script route the chunk does not change or name
keyboard-navigation#4.4 · no · JS keyboard event fields in blitz-vibey-script, not reached and not named by the scope
keyboard-navigation#4.5 · no · KeyDown and KeyUp keyboard data reaching Dioxus, an event path in dioxus-native-dom the chunk does not reach
keyboard-navigation#4.6 · no · focus and blur forwarded to Dioxus as focus data, not the snapshot's focused or the tree's focus the scope names
keyboard-navigation#4.7 · no · Mounted set_focus and unqueued focus events in dioxus-native-dom, not reached and not named by the scope
keyboard-navigation#4.8 · no · MountedData call from a key handler task in dioxus-native-dom, not reached and not named by the scope
