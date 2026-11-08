@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the driver's disabled refusal must dispatch nothing, so the primary click that synthetic_click_event builds is one the chunk's code must avoid on a disabled node
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion sits in the windowed shell, which the driver's replay never runs
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: shell-side key dispatch after shortcut checks, not reached by the driver's replayed actions
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: a key press on a control with a dropped anonymous block; the chunk adds no key press and changes no layout
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings are a shell forwarding path the chunk does not reach
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is shell and dispatch plumbing the chunk neither reads nor changes
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus() and blur() are script-side, and the chunk runs no script
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection is a script attribute path, and the chunk changes no focus order
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors are script-side style writes the chunk does not use
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields are script-side, and the chunk's coverage and replay read no JS event
@answer n=11 entity=keyboard-navigation#4.5 v=yes
why: KeyDown, KeyUp and KeyPress reach Dioxus as keyboard data in dioxus_document.rs, which sets the event names by which a check's dispatch exercises a handler
@answer n=12 entity=keyboard-navigation#4.6 v=yes
why: the registry dispatch around DioxusEventHandler::handle_event is the record of exercised handlers that the coverage reads, and its chain and event rule lives in signal_registry.rs
@answer n=13 entity=keyboard-navigation#4.7 v=yes
why: Focus, Blur, FocusIn and FocusOut forwarded as focus data in dioxus_document.rs decide which focus-named listeners a check can exercise
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: set_focus does not queue focus or blur events, so a focus handler reached only through set_focus is not counted as exercised, a fork the coverage result must state
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the buffered render in initial_build and poll changes no render path the chunk touches, and the spawned-task MountedData pins are their own
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the same-render node limit concerns a task polled during render, which the chunk's read after checks never reaches
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: the autofocus Cargo feature forwarding is a build flag the chunk does not change
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: IME events are unhandled in dioxus-native-dom, and the chunk's actions and coverage involve no IME input
@answer n=19 entity=keyboard-navigation#4.13 v=yes
why: a falsy disabled removes the attribute, so the driver's refusal reads enabled as not Some(false) and its disabled-refusal test depends on that
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: browser urlbar keyboard handling is app code the chunk does not touch
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts are app code the chunk does not touch
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: browser new-tab search autofocus and Enter submit are app code the chunk does not touch
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts are app code the chunk does not touch
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: use_back_button is a dioxus-native hook the chunk does not use
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: todomvc focus and key handling is an example app the chunk does not use
