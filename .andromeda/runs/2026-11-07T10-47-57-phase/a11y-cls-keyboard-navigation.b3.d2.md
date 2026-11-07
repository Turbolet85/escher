@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic click construction is not touched and no chunk test presses a pointer click
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion belongs to the windowed shell, and the headless Tab press does not run through it
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: KeyDown dispatch after shell shortcuts is window.rs's path, read not run for the windowed launch
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the dropped-anonymous-block no-panic case is not built by the chunk's pins, whose key target is a visible control
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings are shell forwarding the chunk does not touch
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is outside the chunk's scope and its tests
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus() and blur() are the script runtime's surface, which the chunk does not change
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus value acceptance is blitz-dom's rule for script reflection; the bridge's falsy path is carried by 4.11
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors are the script runtime, not the Dioxus bridge
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields belong to the script runtime and are not reached by the chunk
@answer n=11 entity=keyboard-navigation#4.5 v=yes
why: the inverted pins' key-handler spawn is triggered by keyboard data reaching Dioxus as this entity states, and the Tab press reaches it the same way
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur event forwarding is not named by the scope, whose pins assert engine and snapshot focus
@answer n=13 entity=keyboard-navigation#4.7 v=yes
why: set_focus from a mounted handle is the call the buffered render must let user code make during render, and the inverted pins depend on it
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: this is the RefCell hazard in DioxusDocument::poll that the chunk fixes and the inverted pins replace
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the autofocus Cargo feature forwarding is a manifest fact, and the chunk edits no manifest
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: IME handling is a TODO outside the chunk's scope
@answer n=17 entity=keyboard-navigation#4.11 v=yes
why: the replay must route set-attribute calls through the same two paths, so falsy boolean attributes still remove the attribute
@answer n=18 entity=keyboard-navigation#5.1 v=no
why: browser urlbar keys belong to the browser app, which the chunk does not change
@answer n=19 entity=keyboard-navigation#5.2 v=no
why: browser Cmd/Ctrl+T and W tab shortcuts are the browser app's, not reached
@answer n=20 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search autofocus is the browser app's, not reached
@answer n=21 entity=keyboard-navigation#5.4 v=no
why: rdme's Ctrl/Cmd shortcuts are the readme app's, not reached
@answer n=22 entity=keyboard-navigation#5.5 v=no
why: use_back_button is a dioxus-native hook the chunk does not touch
@answer n=23 entity=keyboard-navigation#5.6 v=no
why: todomvc's focus on mount and Enter handling are an example app's behaviour, not named by the scope
@answer n=24 entity=keyboard-navigation#5.7 v=no
why: Cells' commit on Enter or blur is an app behaviour the chunk does not change, and no stand app changes
@answer n=25 entity=keyboard-navigation#5.8 v=yes
why: Cells' synchronous MountedData scroll_to is a call the scope names among render-time handle calls, and stand_keyboard re-asserts it
