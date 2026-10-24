# Classified — a11y · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · yes · the primary click built at a node's centre is the Click that Enter and Space dispatch on a focused stand button, which the chunk's activation re-read asserts (task Needs, Enter and Space activation)
keyboard-navigation#3.10 · no · winit key conversion is shell-only, and the stand's presses reach the document through the harness's press, not blitz-shell
keyboard-navigation#3.11 · no · the shell's KeyDown and KeyUp dispatch order is a window path the headless stand checks never cross
keyboard-navigation#3.12 · no · macOS AppleStandardKeybinding forwarding is a shell path the headless stand checks never cross
keyboard-navigation#3.13 · no · IME forwarding is input-method handling, which the task's Not needed excludes
keyboard-navigation#4.1 · no · JS focus(), blur() and document.activeElement are a script surface no chunk Need names, and the chunk reads focus from the engine and the snapshot's focused node
keyboard-navigation#4.2 · no · the autofocus reflection is a script binding no chunk Need names, and the chunk changes no feature
keyboard-navigation#4.3 · no · JS keyboard event fields are a script surface no chunk Need names
keyboard-navigation#4.4 · yes · the key data the CRUD list's onkeydown reads for ArrowDown, ArrowUp, Home and End, delivered to Dioxus from the key presses stand_keyboard drives (task Needs, CRUD list model)
keyboard-navigation#4.5 · no · Dioxus focus-event forwarding is not what the chunk asserts, which reads the engine's focus and the snapshot's focused node
keyboard-navigation#4.6 · no · set_focus is a Dioxus API no chunk Need names, and the chunk's focus moves by Tab and clicks
keyboard-navigation#4.7 · no · the autofocus feature forward is a manifest feature, and the chunk changes no manifest or feature
keyboard-navigation#4.8 · no · IME handling in dioxus-native-dom is input-method handling, which the task's Not needed excludes
keyboard-navigation#4.9 · yes · a falsy disabled, hidden or inert removes the attribute, which the enabled-state Tab orders rest on (task Needs, Dioxus bridge; its test stand_keyboard.rs:752-779 sits in a reached file)
keyboard-navigation#5.1 · no · the browser urlbar is an app outside the stand and the a11y leg
keyboard-navigation#5.2 · no · browser tab shortcuts are an app outside the stand and the a11y leg
keyboard-navigation#5.3 · no · the browser's new-tab autofocus and Enter submit are an app outside the stand and the a11y leg
keyboard-navigation#5.4 · no · rdme's key shortcuts are an app outside the stand and the a11y leg
keyboard-navigation#5.5 · no · use_back_button handles the browser-back key, which no chunk Need names
keyboard-navigation#5.6 · no · the todomvc example is outside the seven_guis stand the chunk checks
keyboard-navigation#5.7 · no · Cells is not a lean stand task that the chunk's Tab and activation checks cover, and its Enter commit is app-level handling
keyboard-navigation#5.8 · yes · the CRUD list's keyboard model the chunk re-asserts: crud-list is the third Tab stop, ArrowDown, ArrowUp, Home and End move its selection, and aria-activedescendant stays unread (task Needs; stand_keyboard.rs:657-717)
keyboard-navigation#5.9 · no · the Preact TodoMVC example is outside the seven_guis stand the chunk checks
keyboard-navigation#5.10 · no · the Android activity wrappers are outside the stand and the chunk's reached files
keyboard-navigation#6.1 · no · the docs.rs example asset's tabindex markup is a fixture fact, not an engine rule, and no chunk Need names it
