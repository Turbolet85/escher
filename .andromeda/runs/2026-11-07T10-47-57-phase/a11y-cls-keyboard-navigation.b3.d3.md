@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic_click_event's primary mouse click is not reached by the chunk's render path or the Tab press.
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion is the windowed shell's input path, and the headless harness press does not go through it.
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: window.rs shell dispatch is the windowed path, read not run, and the scope names only the harness's press.
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: anonymous-block drop and layout_parent on hidden controls is engine behaviour the chunk neither changes nor names.
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard keybinding forwarding is a shell path the chunk does not touch.
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME forwarding is not touched and IME is outside the chunk's scope.
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus(), blur() and activeElement belong to the vibey-script JS runtime, outside the chunk.
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection is a JS-side write in vibey-script, not the Dioxus bridge's attribute path.
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors belong to the vibey-script runtime, not the chunk.
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields are vibey-script behaviour, outside the chunk.
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: keyboard-data forwarding at dioxus_document.rs:391-395 is another unit of an edited file that only must keep holding, and the pins' key precondition is an existing check.
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus-event forwarding at dioxus_document.rs:386-389 is another unit of an edited file, and the chunk asserts engine focus, not focus events.
@answer n=13 entity=keyboard-navigation#4.7 v=yes
why: it states set_focus on a Mounted handle, the call the scope names for the render-time focus move and the inverted pins.
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: it states the RefCell hazard this chunk fixes and the dioxus_runtime_probe pins the chunk inverts.
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: forwarding of the autofocus Cargo feature is manifest wiring the chunk does not edit or name.
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the bridge does not handle IME events and the chunk does not touch IME.
@answer n=17 entity=keyboard-navigation#4.11 v=yes
why: it states falsy-value attribute removal on both MutationWriter paths, which the scope names as the replay's routing need.
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: browser urlbar keys belong to an app outside the chunk's scope.
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts belong to an app outside the chunk's scope.
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: browser new-tab search autofocus belongs to an app outside the chunk's scope.
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: rdme's Ctrl/Cmd shortcuts belong to an app outside the chunk's scope.
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: use_back_button handles BrowserBack presses, a hook the chunk does not touch.
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: todomvc is an example app outside the re-asserted stand_* checks, and the chunk changes no app.
@answer n=24 entity=keyboard-navigation#5.7 v=yes
why: Cells' commit on Enter or blur is what the re-asserted stand_* Cells keyboard checks witness, and a render-time focus move can reach blur.
@answer n=25 entity=keyboard-navigation#5.8 v=yes
why: it names MountedData::scroll_to, one of the MountedData calls the scope names and the chunk must keep working.
