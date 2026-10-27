# Classified — a11y-plan · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · no · the click builder sits in blitz-dom's node.rs, and nothing under packages/blitz-dom/ changed
keyboard-navigation#3.10 · no · the winit key conversion sits in blitz-shell's convert_events.rs, which the session did not reach
keyboard-navigation#3.11 · no · the KeyDown and KeyUp dispatch sits in blitz-shell's window.rs, which the session did not reach
keyboard-navigation#3.12 · no · the AppleStandardKeybinding forwarding sits in blitz-shell and blitz-traits, neither of which the session reached
keyboard-navigation#3.13 · no · the IME forwarding sits in blitz-shell and blitz-traits, neither of which the session reached
keyboard-navigation#4.1 · no · focus, blur and activeElement sit in blitz-vibey-script, which the session did not reach
keyboard-navigation#4.2 · no · the autofocus reflection sits in blitz-vibey-script, which the session did not reach
keyboard-navigation#4.3 · no · the JS keyboard event fields sit in blitz-vibey-script's event.rs, which the session did not reach
keyboard-navigation#4.4 · no · the Dioxus keyboard bridge sits in dioxus-native-dom, which the session did not reach
keyboard-navigation#4.5 · no · the focus-data forwarding sits in dioxus-native-dom's dioxus_document.rs, which the session did not reach
keyboard-navigation#4.6 · no · set_focus sits in dioxus-native-dom's events.rs, which the session did not reach
keyboard-navigation#4.7 · no · the autofocus forwarding sits in two Cargo.toml files, and the session changed no manifest or lockfile
keyboard-navigation#4.8 · no · the IME TODO sits in dioxus-native-dom's dioxus_document.rs, which the session did not reach
keyboard-navigation#4.9 · no · the falsy-attribute rule and its 28-name count sit in dioxus-native-dom's mutation_writer.rs, which the session did not reach
keyboard-navigation#5.1 · no · the urlbar arrow keys sit in apps/browser's toolbar.rs, which the session did not reach
keyboard-navigation#5.2 · no · the tab shortcuts sit in apps/browser's tab_strip.rs, which the session did not reach
keyboard-navigation#5.3 · no · the new-tab autofocus sits in apps/browser's about_pages.rs, which the session did not reach
keyboard-navigation#5.4 · no · the rdme shortcuts sit in apps/readme's readme_application.rs, which the session did not reach
keyboard-navigation#5.5 · no · use_back_button sits in dioxus-native's hooks.rs, which the session did not reach
keyboard-navigation#5.6 · no · the todomvc input focus sits in examples/todomvc's app.rs, which the session did not reach
keyboard-navigation#5.7 · no · the Cells commit keys sit in examples/seven_guis/src/tasks/cells.rs, which the session did not reach
keyboard-navigation#5.8 · no · the CRUD list keys sit in crud.rs and its checks, none reached, and the task's Untouched list holds them unedited
keyboard-navigation#5.9 · no · the Preact todomvc autofocus sits in examples/preact/index.html, which the session did not reach
keyboard-navigation#5.10 · no · the Android focus calls sit in two MainActivity.kt files, which the session did not reach
keyboard-navigation#6.1 · no · the docs.rs fixture's tabindex sits in examples/assets/docsrs_header.html, which the session did not reach
