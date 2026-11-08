@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic click synthesis is a Blitz mechanism the graph does not drive and the chunk does not change
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key event conversion is outside the graph's reading and the chunk changes no key path
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: shell dispatch of key presses and releases is not named by the chunk
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the no-panic guarantee for a focused control whose anonymous block was dropped is a layout rule the chunk does not touch
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: forwarding of macOS standard key bindings is not named and the chunk changes no key rule
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is not named and the chunk changes no IME rule
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus, blur and activeElement are script-side and the chunk reads no focus
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection on script elements is outside the chunk's reading
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSS style accessor installation is a script-side crossing the chunk does not use
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: the fields of JS keyboard events are a script-side shape the graph does not read
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: keyboard event delivery to Dioxus is event routing the graph's handler listing does not depend on
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur event forwarding is not named and the handler listing reads no focus data
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus and the unqueued focus events are a focus verb the chunk does not change
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: the stand's moved signals are written by key-handler tasks that call MountedData, so the chunk's vdom walk and registry must keep the buffered render path free of a document borrow
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the same-render focus limit is an open residual the chunk does not fix and the graph reads no focus result
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: forwarding of the autofocus feature to blitz-dom is a build feature the chunk does not touch
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: IME handling in dioxus-native-dom is an open TODO outside the graph and the chunk changes no IME rule
@answer n=18 entity=keyboard-navigation#4.12 v=yes
why: a falsy boolean attribute such as hidden or disabled is absent in the DOM but present as false in the vdom that the graph walks exactly
@answer n=19 entity=keyboard-navigation#5.1 v=no
why: browser urlbar keys belong to an app outside the stand and the graph
@answer n=20 entity=keyboard-navigation#5.2 v=no
why: browser tab open and close shortcuts belong to an app outside the stand and the graph
@answer n=21 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search input belongs to an app outside the stand and the graph
@answer n=22 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts belong to an app outside the stand and the graph
@answer n=23 entity=keyboard-navigation#5.5 v=no
why: the back-button hook is a dioxus-native hook the chunk does not use
@answer n=24 entity=keyboard-navigation#5.6 v=no
why: todomvc focus and edit keys belong to an example outside the stand and the graph
@answer n=25 entity=keyboard-navigation#5.7 v=no
why: Cells commit on Enter or blur is editor key behaviour the task lists as not needed and the graph reads only handler presence
