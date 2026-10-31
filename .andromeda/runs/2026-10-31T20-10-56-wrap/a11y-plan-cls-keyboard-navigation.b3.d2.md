# Classified — a11y-plan · keyboard-navigation.b3

## Answers
keyboard-navigation#4.9 · no · the autofocus forward sits in the dioxus-native-dom and dioxus-native manifests, neither reached, and no manifest moved
keyboard-navigation#4.10 · no · the IME TODO sits in dioxus_document.rs, which the change does not reach
keyboard-navigation#4.11 · no · the falsy-attribute rule lives in mutation_writer.rs and the falsy-attrs tests, none reached; its stand_keyboard.rs:1027-1054 citation sits above the append at 1055 and holds
keyboard-navigation#5.1 · no · the browser urlbar keys sit in apps/browser, not reached
keyboard-navigation#5.2 · no · the browser tab shortcuts sit in apps/browser, not reached
keyboard-navigation#5.3 · no · the browser new-tab input sits in apps/browser, not reached, and the input Enter rule is unmoved
keyboard-navigation#5.4 · no · the rdme shortcuts sit in apps/readme, not reached
keyboard-navigation#5.5 · no · use_back_button sits in packages/dioxus-native/src/hooks.rs, not reached
keyboard-navigation#5.6 · no · the todomvc input Enter sits in examples/todomvc, not reached, and the input rule is unmoved
keyboard-navigation#5.7 · no · the Cells commit sits in examples/seven_guis/src/tasks/cells.rs, not reached, and the input and button Enter rules are unmoved
keyboard-navigation#5.8 · no · the Cells arrows and scroll_to sit in cells.rs, not reached; the cited stand_keyboard lines sit above 1055 and hold, and a list's own scroll_y moves no scroll chain
keyboard-navigation#5.9 · no · the Cells editor's Enter is on a cell, not a select; cells.rs is not reached and the cited stand_keyboard lines above 1055 hold
keyboard-navigation#5.10 · no · the CRUD list sits in examples/seven_guis/src/tasks/crud.rs, not reached; the builder change is aria-expanded on a single select only, so the activedescendant reading stays unread and accessibility_active_descendant_unread is unedited
keyboard-navigation#5.11 · no · the TodoMVC Preact autofocus sits in examples/preact, not reached
keyboard-navigation#5.12 · no · the Android activities sit in apps/browser and examples/todomvc, not reached
keyboard-navigation#6.1 · no · the docs.rs fixture sits in examples/assets, which no edit reached
keyboard-navigation#6.2 · no · the google role=button fixture sits in examples/assets, which no edit reached, and the button-role activation rule is unmoved
keyboard-navigation#6.3 · no · the google jsaction handlers sit in examples/assets, which no edit reached
keyboard-navigation#6.4 · no · the google focus-visible outline sits in examples/assets, which no edit reached
keyboard-navigation#6.5 · no · the graphite fixture sits in examples/assets, which no edit reached
keyboard-navigation#6.6 · no · servo.css sits in examples/assets, which no edit reached
keyboard-navigation#6.7 · no · servo.html sits in examples/assets, which no edit reached
keyboard-navigation#6.8 · no · labels still bind by for/id and nesting, and the label change keeps the binding (a click on a bound select still focuses it), dropping only the re-dispatch; examples/form.rs is not reached
keyboard-navigation#7.1 · yes · stale · stand_snapshot_state.rs gained lines after 121, so the citation at 331-367 moves
keyboard-navigation#7.2 · yes · add-to · F7 (the Tab walk holds with the list open and after each close) and F4 (Tab chooses with a list open, in either direction) join the stand Tab walk, and its select Enter and Space rows now open and choose
