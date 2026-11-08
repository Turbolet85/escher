@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the covered cause must test the point a click lands on, the node's centre that the shared action_point path carries, and the read builds no click event itself
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key-event conversion; the read dispatches no key
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key press and release dispatch after shell shortcuts; the read dispatches no key
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: no-panic guarantee for a key press on a control whose anonymous block was dropped; the read presses no key and lists no node without a box
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings forwarded as UI events; the read dispatches no key
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding; the read types nothing and the type cause is disabled, covered or off-screen only
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: script-side focus() and blur() and document.activeElement; the read calls no focus and runs no script
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection in the script layer; the read's focusability rule does not take autofocus and the read focuses nothing
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: script CSSStyleDeclaration accessor path for display and visibility; the read runs no script and reads the tree's exclusion as built
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields; the read dispatches no key
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: keyboard data reaching Dioxus; the read dispatches no key
@answer n=12 entity=keyboard-navigation#4.6 v=yes
why: the listed node's owner is where its handler sits on another element, and this graph.rs rule resolves a dispatch chain to the first element carrying the listener, which the owner must agree with
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: focus, blur and focusin/focusout events forwarded as focus data; the read dispatches no event
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: set_focus and unqueued focus events; the read sets no focus and runs no step
@answer n=15 entity=keyboard-navigation#4.9 v=no standing=S5
why: the convention that a focus check reads the engine focus, the snapshot's focused node and the tree's focus agreeing, as the probe pins assert it
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: a task polled during render acting on the previous render; the read's stand proof calls no focus setter
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: autofocus cargo feature forwarding to blitz-dom; the read focuses nothing
@answer n=18 entity=keyboard-navigation#4.12 v=no
why: IME events not handled in dioxus-native-dom; the read types nothing
@answer n=19 entity=keyboard-navigation#4.13 v=yes
why: a falsy value of disabled, hidden or inert removes the attribute, so the disabled cause is keyed by presence and the visibility exclusion follows the same keying
@answer n=20 entity=keyboard-navigation#5.1 v=no
why: browser urlbar key handling; outside the driver read
@answer n=21 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts; outside the driver read
@answer n=22 entity=keyboard-navigation#5.3 v=no
why: browser new-tab search input autofocus and Enter; outside the driver read
@answer n=23 entity=keyboard-navigation#5.4 v=no
why: rdme application shortcuts; outside the driver read
@answer n=24 entity=keyboard-navigation#5.5 v=no
why: use_back_button key handling; outside the driver read
@answer n=25 entity=keyboard-navigation#5.6 v=no
why: todomvc focus and Enter handling; the read dispatches no key and the example is not reached
