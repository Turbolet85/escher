# Classified — a11y-plan · keyboard-navigation.b3

## Answers
keyboard-navigation#4.9 · no · autofocus forwards through the dioxus-native-dom and dioxus-native manifests, which no change moved, and no reached file holds that wiring
keyboard-navigation#4.10 · no · the IME TODO sits in dioxus_document.rs:400-401, a file the session does not reach
keyboard-navigation#4.11 · no · the falsy-value removal of disabled, hidden, inert and open is unchanged (mutation_writer.rs and the dioxus_falsy tests unreached), and stand_keyboard.rs:1027-1054 sits above the append at 1055 so it holds
keyboard-navigation#5.1 · no · the browser urlbar lives in apps/browser, which the session does not reach
keyboard-navigation#5.2 · no · browser tab shortcuts in apps/browser/src/tab_strip.rs are not reached
keyboard-navigation#5.3 · no · browser new-tab search input in apps/browser/src/about_pages.rs is not reached
keyboard-navigation#5.4 · no · rdme shortcuts in apps/readme/src/readme_application.rs are not reached
keyboard-navigation#5.5 · no · use_back_button in packages/dioxus-native/src/hooks.rs is outside the change
keyboard-navigation#5.6 · no · todomvc in examples/todomvc/src/app.rs is not reached
keyboard-navigation#5.7 · no · Cells' commit on Enter or blur in examples/seven_guis/src/tasks/cells.rs is not reached, and no change touches a seven_guis task's Enter or blur path
keyboard-navigation#5.8 · no · the cells-grid citations at stand_keyboard.rs:193-200 and 293-335 sit above the append at 1055 so they hold, and the grid is not a select, so its arrows and clamping are unchanged
keyboard-navigation#5.9 · no · Cells' editor citations (cells.rs unreached, stand_keyboard.rs:202-249 above 1055) hold, and an input's Enter rule is unmoved
keyboard-navigation#5.10 · no · crud.rs and accessibility_active_descendant_unread are unreached, and the builder still reads no aria-activedescendant because the highlight is in neither the tree nor the snapshot
keyboard-navigation#5.11 · no · the Preact TodoMVC input in examples/preact/index.html is not reached
keyboard-navigation#5.12 · no · the Android activity files are not reached
keyboard-navigation#6.1 · no · the docs.rs fixture examples/assets/docsrs_header.html is not reached
keyboard-navigation#6.2 · no · the google fixture examples/assets/google.html is not reached
keyboard-navigation#6.3 · no · the google fixture's jsaction handlers in examples/assets/google.html are not reached
keyboard-navigation#6.4 · no · the google fixture's focus-visible outline in examples/assets/google.html is not reached
keyboard-navigation#6.5 · no · the graphite fixture examples/assets/graphite.html is not reached
keyboard-navigation#6.6 · no · servo.css focus ring in examples/assets/servo.css is not reached
keyboard-navigation#6.7 · no · servo.html heading tabindex in examples/assets/servo.html is not reached
keyboard-navigation#6.8 · no · the for/id and nesting association in examples/form.rs is not reached, and the session changes what a click on a bound single select's label does, not the association
keyboard-navigation#7.1 · yes · stale · the session's insert after stand_snapshot_state.rs:121 moves the cited lines 331-367, and the focus claim itself holds
keyboard-navigation#7.2 · yes · add-to · F7 (focus stays on the select and the Tab walk holds with its list open and after each close) extends the stand walk, and the walk's select Enter and Space rows now open and choose while stand_keyboard's closed-arrow test passes untouched
