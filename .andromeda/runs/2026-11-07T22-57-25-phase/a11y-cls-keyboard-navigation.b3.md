@answer n=1 entity=keyboard-navigation#3.22 v=no
why: blitz-dom's synthetic click for keyboard activation is a path the chunk neither changes nor reads into the graph's click reading
@answer n=2 entity=keyboard-navigation#3.23 v=no
why: winit key-event conversion in blitz-shell is input plumbing the chunk does not touch
@answer n=3 entity=keyboard-navigation#3.24 v=no
why: key dispatch after shell shortcuts is not a rule over the graph, the snapshot or the element ids
@answer n=4 entity=keyboard-navigation#3.25 v=no
why: the no-panic rule for a dropped anonymous block concerns key presses, not the graph's reads or bounds
@answer n=5 entity=keyboard-navigation#3.26 v=no
why: macOS standard key bindings forwarded as UI events are shell input the chunk does not change
@answer n=6 entity=keyboard-navigation#3.27 v=no
why: IME event forwarding is input plumbing outside the graph, the snapshot and the masking rule
@answer n=7 entity=keyboard-navigation#4.1 v=no
why: element focus() and blur() in blitz-vibey-script are a script-side API the chunk does not use
@answer n=8 entity=keyboard-navigation#4.2 v=no
why: autofocus reflection in blitz-vibey-script is a script-side write the chunk does not touch
@answer n=9 entity=keyboard-navigation#4.3 v=no
why: CSSStyleDeclaration accessors in blitz-vibey-script are script-to-DOM style writes the graph does not read
@answer n=10 entity=keyboard-navigation#4.4 v=no
why: JS keyboard event fields in blitz-vibey-script are script-side and carry no graph value
@answer n=11 entity=keyboard-navigation#4.5 v=no
why: keyboard data delivered to Dioxus is event payload the graph neither reads nor carries, its handler listing being registration only
@answer n=12 entity=keyboard-navigation#4.6 v=no
why: focus, blur and focusin/focusout data forwarding is event delivery the chunk does not read or change
@answer n=13 entity=keyboard-navigation#4.7 v=no
why: set_focus on mounted elements and the unqueued focus events are a focus call the chunk neither makes nor reads
@answer n=14 entity=keyboard-navigation#4.8 v=yes
why: the escher signal registry carries the stand's signal writes from key-handler tasks, and this buffered render path is the one a spawned task's MountedData call reaches the document through
@answer n=15 entity=keyboard-navigation#4.9 v=no
why: the same-render focus limit concerns set_focus_to on freshly created nodes, which the chunk does not call
@answer n=16 entity=keyboard-navigation#4.10 v=no
why: forwarding the autofocus Cargo feature to blitz-dom is a dependency wiring the chunk does not change
@answer n=17 entity=keyboard-navigation#4.11 v=no
why: unhandled IME events in dioxus-native-dom are a TODO outside the graph and the snapshot
@answer n=18 entity=keyboard-navigation#4.12 v=yes
why: link 1 reads dynamic attributes exactly, so a falsy boolean such as hidden, disabled or inert must read as absent, as the writer removes it, for the walk and the actionable-key rule
@answer n=19 entity=keyboard-navigation#5.1 v=no
why: browser urlbar key handling in apps/browser is an app the chunk does not touch
@answer n=20 entity=keyboard-navigation#5.2 v=no
why: browser tab shortcuts in apps/browser are an app the chunk does not touch
@answer n=21 entity=keyboard-navigation#5.3 v=no
why: the browser new-tab search input's autofocus and Enter submit are in apps/browser, which the chunk does not touch
@answer n=22 entity=keyboard-navigation#5.4 v=no
why: rdme's Ctrl/Cmd shortcuts in apps/readme are an app the chunk does not touch
@answer n=23 entity=keyboard-navigation#5.5 v=no
why: use_back_button's BrowserBack handling is a hook in dioxus-native the chunk does not change or read
@answer n=24 entity=keyboard-navigation#5.6 v=no
why: todomvc's focus and Enter handling is an example app outside the chunk's stand and graph
@answer n=25 entity=keyboard-navigation#5.7 v=no
why: Cells' Enter and blur commit is Cells' editor key behaviour, which the scope names as not needed and the signal move keeps
