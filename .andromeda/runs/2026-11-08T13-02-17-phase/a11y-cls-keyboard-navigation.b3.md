@answer n=1 entity=keyboard-navigation#3.22 v=yes
why: the covered cause runs through action_point, which must test the node's centre where a real call's synthetic click lands

@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion is not reached, as the read dispatches no key

@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key dispatch after shell shortcuts is not reached, as the read dispatches no key

@answer n=4 entity=keyboard-navigation#3.25 v=no
why: a key press after a dropped anonymous block is a key-press behaviour the read never performs, and its fixture presses no key

@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings are key forwarding the read does not reach

@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is not reached by a read of actionable nodes

@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus() and activeElement belong to the script runtime, which the Dioxus stand and the read do not call

@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection in the script runtime is not reached by the read's node list or verbs

@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors are the script runtime's crossing, which the Dioxus stand does not use, and the read's hidden exclusion is the tree's own rule

@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields belong to the script runtime, and the read dispatches no key

@answer n=11 entity=keyboard-navigation#4.5 v=no
why: KeyDown, KeyUp and KeyPress delivery to Dioxus is key handling the read does not perform

@answer n=12 entity=keyboard-navigation#4.6 v=no
why: the signal registry's write resolution and declared-write flags are not the read's node, owner, verb or cause, and graph.rs lines 294-320 are a re-check

@answer n=13 entity=keyboard-navigation#4.7 v=no
why: focus, blur, focusin and focusout event forwarding is not reached, as the read dispatches no event

@answer n=14 entity=keyboard-navigation#4.8 v=no
why: queued-less set_focus through MountedData is not the read's focusability rule, and the read sets no focus

@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the MountedData spawned-task hazard is a focus-setting path the read never calls, and its focus assertion is not a check this chunk adds

@answer n=16 entity=keyboard-navigation#4.10 v=no
why: the same-render MountedData limit concerns set_focus_to, which the read never calls

@answer n=17 entity=keyboard-navigation#4.11 v=no
why: forwarding of the autofocus feature is a Cargo feature the read does not depend on

@answer n=18 entity=keyboard-navigation#4.12 v=no
why: IME handling in dioxus-native-dom is not reached by a read of actionable nodes

@answer n=19 entity=keyboard-navigation#4.13 v=yes
why: a falsy bridge boolean such as disabled, hidden or inert removes the attribute, and the read's disabled cause and hidden-node exclusion key on that presence

@answer n=20 entity=keyboard-navigation#5.1 v=no
why: browser urlbar key handling lives in apps/browser, which the chunk does not reach

@answer n=21 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts live in apps/browser, which the chunk does not reach

@answer n=22 entity=keyboard-navigation#5.3 v=no
why: browser new-tab autofocus lives in apps/browser, which the chunk does not reach

@answer n=23 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts live in apps/readme, which the chunk does not reach

@answer n=24 entity=keyboard-navigation#5.5 v=no
why: use_back_button handles BrowserBack presses the read never dispatches

@answer n=25 entity=keyboard-navigation#5.6 v=no
why: todomvc mount focus and edit keys live in an example the chunk does not reach
