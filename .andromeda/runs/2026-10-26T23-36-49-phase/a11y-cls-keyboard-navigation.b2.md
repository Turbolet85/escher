# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · no · the click that Enter and Space dispatch for keyboard activation, which the task's Not-needed list excludes; the chunk changes no click path
keyboard-navigation#3.10 · no · winit key-event conversion in the windowed shell, a path the headless checks and the chunk do not reach
keyboard-navigation#3.11 · no · shell key dispatch in the windowed shell, which the chunk does not reach
keyboard-navigation#3.12 · no · macOS standard key bindings forwarded by the windowed shell, which the chunk does not reach
keyboard-navigation#3.13 · no · IME events forwarded to the document, the IME section the Not-needed list excludes
keyboard-navigation#4.1 · no · the JS focus() · blur() · activeElement API in blitz-vibey-script, which the chunk does not reach
keyboard-navigation#4.2 · no · autofocus reflection in blitz-vibey-script, which the chunk does not reach; the lint reads no focus
keyboard-navigation#4.3 · no · JS keyboard event fields in blitz-vibey-script, which the chunk does not reach
keyboard-navigation#4.4 · no · key data delivered to Dioxus handlers; the chunk presses no key and no handler sits on the lint's path
keyboard-navigation#4.5 · no · focus and blur event forwarding in dioxus-native-dom; focus is outside the chunk and the lint reads none of it
keyboard-navigation#4.6 · no · set_focus and its unqueued focus events, outside the chunk's focus scope
keyboard-navigation#4.7 · no · a Cargo feature forward; the chunk edits no manifest and reads no autofocus path
keyboard-navigation#4.8 · no · IME events in dioxus-native-dom, the IME section the Not-needed list excludes
keyboard-navigation#4.9 · yes · hidden: false and inert: false elements stay in the snapshot the lint walks, disabled: false sets the enabled reading the contrast exemption reads, and it names inert_subtree.rs, a re-run a11y file
keyboard-navigation#5.1 · no · the browser urlbar's keys in apps/browser, which the chunk does not reach
keyboard-navigation#5.2 · no · browser tab shortcuts in apps/browser, which the chunk does not reach
keyboard-navigation#5.3 · no · browser new-tab autofocus in apps/browser, which the chunk does not reach
keyboard-navigation#5.4 · no · rdme's shortcuts in apps/readme, which the chunk does not reach
keyboard-navigation#5.5 · no · dioxus-native's back-button hook, which the chunk does not reach
keyboard-navigation#5.6 · no · the todomvc example's focus and Enter handling, which the chunk does not reach
keyboard-navigation#5.7 · no · the Cells task's Enter and blur commit; Cells is outside the screens stand_lint reads and the entity names no check
keyboard-navigation#5.8 · yes · crud-list, the list the lint reads after twelve Creates, and it names accessibility_active_descendant_unread, a re-run a11y file
keyboard-navigation#5.9 · no · the Preact TodoMVC input in examples/preact, which the chunk does not reach
keyboard-navigation#5.10 · no · the Android activities' focus in touch mode, which the chunk does not reach
keyboard-navigation#6.1 · no · the docs.rs fixture's tabindex on its toolbar; no check of the chunk loads that fixture
