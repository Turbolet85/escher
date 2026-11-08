@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the covered cause hit-tests the node centre that a real click is synthesised at, so the read's covered answer must use that same point
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key-event conversion, and the read dispatches no key
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key press and release dispatch, and the read dispatches nothing
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: a key press on a control with a dropped anonymous block, and the read presses no key
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings forwarded as UI events, and the read dispatches no key
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding, and the read takes no text input
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: script focus() and blur() and document.activeElement, and the read focuses nothing
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection writing "true", and the read's causes do not key on autofocus
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: script CSS accessors for display and visibility, and the chunk's fixture is Dioxus-rendered rather than script-written
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields, and the read builds no key event
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: keyboard data reaching Dioxus, and the read dispatches no key
@answer n=12 entity=keyboard-navigation#4.6 v=yes
why: the owner of a graph node resolves to the first chain element carrying a listener, the chain rule graph.rs:294-320 states for write resolution, which the owner must agree with for select-served rows
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: focus and blur event forwarding, and the read raises no focus event
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: set_focus and unqueued focus events, and the read sets no focus
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: MountedData calls from a key-handler task, and the read runs no step and no task
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: set_focus_to on a node created in the same render, and the read sets no focus
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: autofocus feature forwarding to blitz-dom, and no read or fixture uses autofocus
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: IME events unhandled in dioxus-native-dom, and the read takes no input
@answer n=19 entity=keyboard-navigation#4.13 v=yes
why: a falsy bool attribute removes disabled, hidden or inert, the keying the read's disabled cause and listed visibility follow
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: browser urlbar key handling in apps/browser, outside the driver read
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts in apps/browser, outside the driver read
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: browser new-tab search autofocus and Enter, outside the driver read
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: rdme application shortcuts, outside the driver read
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: use_back_button handling of BrowserBack, outside the driver read
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: todomvc focus on mount and its key handling, outside the driver read
