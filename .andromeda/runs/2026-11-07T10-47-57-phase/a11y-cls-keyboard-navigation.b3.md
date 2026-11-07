@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic click synthesis in blitz-dom is untouched; the chunk drives focus by Tab and set_focus, never by click
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion in blitz-shell is untouched; the scope's Tab press is the harness press, not the windowed conversion
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: the shell window's key dispatch is not the harness press the scope names, and the chunk changes no key dispatch
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: a key press after an anonymous block is dropped is not named by the scope or the task's existing checks, and the chunk's focus targets are not hidden or display:none controls
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key binding forwarding is in blitz-shell and blitz-traits, which the chunk does not touch
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is untouched, and the chunk adds no text input path
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus(), blur() and document.activeElement are JS-side in blitz-vibey-script, which the chunk does not reach
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection and blitz-dom's autofocus value rule are JS-side and feature-gated, and the chunk's tests name no autofocus control
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSS property accessors are JS-side in blitz-vibey-script, not reached by the render path
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields are in blitz-vibey-script; the chunk's keys travel the Dioxus path
@answer n=11 entity=keyboard-navigation#4.5 v=yes
why: the new key-handler-spawned task test depends on KeyDown reaching the Dioxus key handler as keyboard data through the forwarding this entity cites
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur data forwarding in handle_ui_event is an untouched unit of dioxus_document.rs, and the scope names no focus-event witness
@answer n=13 entity=keyboard-navigation#4.7 v=yes
why: set_focus from a MountedData handle is the call the chunk's render-time and task-time focus tests make, and focus events from it are not queued
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: the render-time RefCell panic this chunk removes, with its two should_panic pins inverted one per layout mode
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: autofocus Cargo feature forwarding is in manifests the chunk does not edit
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: IME handling in dioxus_document.rs is an untouched unit, and the chunk adds no text input path
@answer n=17 entity=keyboard-navigation#4.11 v=yes
why: the replay must route set-attribute calls through the same falsy boolean-attribute paths of MutationWriter, which this entity states
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: browser urlbar key handling is in apps/browser, which the chunk does not touch
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts are in apps/browser, which the chunk does not touch
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search input is in apps/browser, which the chunk does not touch
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts are in apps/readme, which the chunk does not touch
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: the use_back_button hook is untouched, and the chunk's tests press no BrowserBack key
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: todomvc's mount focus and key handling are in an example app the chunk does not touch
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: Cells' edit commit on Enter or blur is stand app behaviour the chunk does not change
@answer n=25 entity=keyboard-navigation#5.8 v=yes
why: Cells' synchronous MountedData scroll_to is a MountedData call the buffer must keep working, witnessed by stand_keyboard.rs, a stand_* check the chunk re-asserts
