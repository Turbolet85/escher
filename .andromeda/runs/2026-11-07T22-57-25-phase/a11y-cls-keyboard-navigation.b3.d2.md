@answer n=1 entity=keyboard-navigation#3.22 v=no
why: synthetic click synthesis is a dispatch fact the graph neither calls nor reads
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key conversion is shell input the chunk does not touch
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key dispatch after shell shortcuts is input routing, and the chunk changes no key behaviour
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: a key press not panicking on a dropped anonymous block is key handling, and the chunk changes no key behaviour
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key binding forwarding is shell key routing the chunk does not touch
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is input routing, and the graph reads no composition state
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus() and activeElement are focus calls the chunk neither makes nor changes
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: the autofocus reflection value is a focus rule, and the chunk changes no focus rule
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSS property accessors on element.style are a script surface the graph does not read
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields are script event data the graph does not carry
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: the keyboard payload reaching Dioxus handlers is event data, and the graph reads only listener names
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus and blur event forwarding is focus data outside the graph's reads
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus on mounted elements is a focus call the chunk does not make
@answer n=14 entity=keyboard-navigation#4.8 v=no
why: MountedData calls from key-handler tasks belong to the buffered-render chunk, and the graph calls no handle and changes no render path
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the same-render focus limit is a set_focus_to limit that the graph's read does not call
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: autofocus feature forwarding is a build feature the chunk does not change
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: IME handling is absent in dioxus-native-dom and outside the graph's reads
@answer n=18 entity=keyboard-navigation#4.12 v=yes
why: the graph's dynamic-attribute walk must read a falsy bool value (Bool false, "false", 0) as absent, as the bridge writes it, for hidden, disabled and inert
@answer n=19 entity=keyboard-navigation#5.1 v=no
why: browser urlbar key handling lives in apps/browser, which the chunk does not reach
@answer n=20 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts live in apps/browser, which the chunk does not reach
@answer n=21 entity=keyboard-navigation#5.3 v=no
why: browser about-page autofocus and Enter submit live in apps/browser, which the chunk does not reach
@answer n=22 entity=keyboard-navigation#5.4 v=no
why: rdme shortcuts live in apps/readme, which the chunk does not reach
@answer n=23 entity=keyboard-navigation#5.5 v=no
why: use_back_button is a dioxus-native hook the chunk does not change
@answer n=24 entity=keyboard-navigation#5.6 v=no
why: todomvc key handling lives in examples, which the chunk does not reach
@answer n=25 entity=keyboard-navigation#5.7 v=no
why: Cells' Enter and blur commit is the editor's key behaviour, which the task excludes as not needed
