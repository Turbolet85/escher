# Classified — a11y-plan · keyboard-navigation.b2

## Answers
keyboard-navigation#3.9 · no · node.rs is not among the reached files and no engine crate moved, so the synthetic center-click claim holds
keyboard-navigation#3.10 · no · the winit key conversion in blitz-shell's convert_events.rs is not among the reached files and no engine crate moved
keyboard-navigation#3.11 · no · the shell's KeyDown and KeyUp dispatch in window.rs is not among the reached files and no engine crate moved
keyboard-navigation#3.12 · no · the macOS key-binding forwarding in blitz-shell and blitz-traits is not among the reached files and no engine crate moved
keyboard-navigation#3.13 · no · the IME forwarding in blitz-shell and blitz-traits is not among the reached files and no engine crate moved
keyboard-navigation#4.1 · no · focus() and blur() in blitz-vibey-script are not among the reached files and no engine crate moved
keyboard-navigation#4.2 · no · the autofocus reflection and its Cargo.toml feature in blitz-vibey-script are not among the reached files and no engine crate moved
keyboard-navigation#4.3 · no · the JS keyboard event fields in blitz-vibey-script are not among the reached files and no engine crate moved
keyboard-navigation#4.4 · no · the Dioxus keyboard forwarding in dioxus-native-dom is not among the reached files and no engine crate moved
keyboard-navigation#4.5 · no · the Dioxus focus-event forwarding in dioxus_document.rs is not among the reached files and no engine crate moved
keyboard-navigation#4.6 · no · set_focus and its queued-focus TODO in dioxus-native-dom's events.rs are not among the reached files and no engine crate moved
keyboard-navigation#4.7 · no · the autofocus feature forwarding in two Cargo.toml files is not among the reached files and no dependency pin moved
keyboard-navigation#4.8 · no · the IME TODO in dioxus_document.rs is not among the reached files and no engine crate moved
keyboard-navigation#4.9 · yes · stale · the cited tests/blitz-tests/tests/stand_keyboard.rs:815-842 sits in a reached file whose activation table went 20 to 16 rows and whose tests went 14 to 15, so its cited lines and button set may have moved; the bridge's falsy removal in mutation_writer.rs is unchanged
keyboard-navigation#5.1 · no · the browser urlbar's keys in apps/browser/src/toolbar.rs are not among the reached files, and no browser file is reached
keyboard-navigation#5.2 · no · the browser tab shortcuts in apps/browser/src/tab_strip.rs are not among the reached files, and no browser file is reached
keyboard-navigation#5.3 · no · the browser new-tab search input in apps/browser/src/about_pages.rs is not among the reached files, and no browser file is reached
keyboard-navigation#5.4 · no · rdme's shortcuts in apps/readme/src/readme_application.rs are not among the reached files
keyboard-navigation#5.5 · no · use_back_button in packages/dioxus-native/src/hooks.rs is not among the reached files and no engine crate moved
keyboard-navigation#5.6 · no · the todomvc focus and key handling in examples/todomvc/src/app.rs are not among the reached files
keyboard-navigation#5.7 · no · Cells' commit on Enter or blur is in examples/seven_guis/src/tasks/cells.rs, not among the reached files, so the claim holds though the stand now serves Cells
keyboard-navigation#5.8 · yes · add-to · F1, the flight booker's select moving its choice by its arrows (pinned by tests/blitz-tests/tests/stand_keyboard.rs in both layout modes), joins the seven_guis arrow keys this entry lists beside the CRUD list's; its cited stand_keyboard.rs:720-780 sits in a reached file and may have moved
keyboard-navigation#5.9 · no · the TodoMVC (Preact) Enter and autofocus in examples/preact/index.html are not among the reached files
keyboard-navigation#5.10 · no · the Android activities in apps/browser/MainActivity.kt and examples/todomvc/MainActivity.kt are not among the reached files
keyboard-navigation#6.1 · no · the docs.rs fixture examples/assets/docsrs_header.html is not among the reached files, and the session's new served pages are Cells and the load page
