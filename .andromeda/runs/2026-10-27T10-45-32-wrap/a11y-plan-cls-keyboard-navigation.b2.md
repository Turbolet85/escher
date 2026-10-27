# Classified — a11y-plan · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · no · node/node.rs, where synthetic_click_event is built, is not a reached file and the session changed no click construction
keyboard-navigation#3.10 · no · convert_events.rs, where winit key conversion sits, is not a reached file and the session changed no key conversion
keyboard-navigation#3.11 · yes · stale · window.rs is a merge surface that gained two lines at 193-194, so the cited dispatch lines 690-698 move to 692-700
keyboard-navigation#3.12 · yes · stale · window.rs is a merge surface that gained two lines at 193-194, so the cited binding lines 578-583 move to 580-585
keyboard-navigation#3.13 · yes · stale · window.rs is a merge surface that gained two lines at 193-194, so the cited hook lines 638-641 move to 640-643 while the plumbing itself is re-read and unmoved
keyboard-navigation#4.1 · yes · stale · blitz-vibey-script dom/element.rs is a merge surface whose focus and blur bindings moved to dom/element/attributes.rs and form_controls.rs, so the cited lines 323-324 and 1007-1019 are stale
keyboard-navigation#4.2 · yes · stale · blitz-vibey-script dom/element.rs is a merge surface whose autofocus binding moved to dom/element/attributes.rs, so the cited lines 530-547 are stale
keyboard-navigation#4.3 · no · dom/event.rs, where the JS keyboard-event fields are built, is not a reached file and the session moved no event-object code
keyboard-navigation#4.4 · yes · stale · dioxus_document.rs is a conflict surface and the report re-reads only its accessibility override and its one added test, not the keyboard-data path at 371-375
keyboard-navigation#4.5 · yes · stale · dioxus_document.rs is a conflict surface and the report does not re-read the focus-data path at 366-369
keyboard-navigation#4.6 · no · dioxus-native-dom's events.rs, where set_focus and the unqueued-focus TODO sit, is not a reached file and the report names no change to it
keyboard-navigation#4.7 · no · both Cargo.toml files carrying the autofocus feature are not reached and the session changed no dependency or feature
keyboard-navigation#4.8 · no · the report re-reads this IME-event plumbing line and finds it unmoved, and the file's one added test pins IME enabling, not an IME-event path
keyboard-navigation#4.9 · yes · add-to · F1: an `open` removal through DocumentMutator runs flush_open_state, so a details or dialog that a falsy `open` removes stops matching `:open`, as a falsy `disabled` stops matching `:disabled`
keyboard-navigation#5.1 · no · apps/browser/src/toolbar.rs, where the urlbar keys sit, is not a reached file
keyboard-navigation#5.2 · no · apps/browser/src/tab_strip.rs, where the Cmd and Ctrl T/W bindings sit, is not a reached file
keyboard-navigation#5.3 · no · apps/browser/src/about_pages.rs is not a reached file and the session changed no autofocus or Enter handling
keyboard-navigation#5.4 · no · apps/readme/src/readme_application.rs is not a reached file and the session changed no key handling
keyboard-navigation#5.5 · no · packages/dioxus-native/src/hooks.rs is not a reached file
keyboard-navigation#5.6 · no · examples/todomvc/src/app.rs is not a reached file and the session changed no focus or Enter behaviour
keyboard-navigation#5.7 · no · examples/seven_guis/src/tasks/cells.rs is not a reached file
keyboard-navigation#5.8 · no · crud.rs, stand_keyboard.rs and accessibility_active_descendant_unread.rs are not reached, and the report keeps the CRUD list keys and the unread aria-activedescendant unchanged
keyboard-navigation#5.9 · no · examples/preact/index.html is not a reached file
keyboard-navigation#5.10 · no · the two MainActivity.kt files are not reached files
keyboard-navigation#6.1 · no · examples/assets/docsrs_header.html is not a reached file
