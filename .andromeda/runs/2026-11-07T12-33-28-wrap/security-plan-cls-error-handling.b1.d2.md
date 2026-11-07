@answer n=1 entity=error-handling#1.1 v=no
why: the session changes no escher-driver refusal, cause or command code, and the change does not reach that crate
@answer n=2 entity=error-handling#2.1 v=no
why: its unimplemented!() sites sit in dioxus events.rs, convert_events.rs and accesskit_xplat, none reached by the change, and events.rs citations are unmoved
@answer n=3 entity=error-handling#3.1 v=no
why: its setup unwraps and expects sit in blitz-shell, blitz-vibey-script and packages/dioxus-native, none reached by the change, which touches only dioxus-native-dom files
@answer n=4 entity=error-handling#4.1 v=no
why: the parallel style traversal lock sits in blitz-dom stylo.rs, which the change does not reach
@answer n=5 entity=error-handling#5.1 v=no
why: the select list readers and close_select_list sit in blitz-dom document.rs, which the change does not reach
@answer n=6 entity=error-handling#6.1 v=yes mode=contradict
why: the buffered render in dioxus_document.rs no longer holds the borrow across render_immediate, so a spawned task's set_focus and the other MountedData calls no longer panic, and the two should_panic pins became spawned_acts tests
@answer n=7 entity=error-handling#7.1 v=yes mode=stale
why: the eight lines added above it in dioxus_document.rs (7, 90-91, 155, 166-168, 171) move the try_element_to_node_id citation at lines 242-246, while the skip itself and the left-out upstream test still hold
@answer n=8 entity=error-handling#8.1 v=no
why: the anonymous-block drop and its key-press witnesses sit in blitz-dom document.rs and the select and key tests, none reached by the change
@answer n=9 entity=error-handling#9.1 v=no
why: select list closing and its tests sit in blitz-dom document.rs and select_popup.rs, none reached by the change
@answer n=10 entity=error-handling#10.1 v=no
why: the NetHandler and ResourceHandler failure paths sit in blitz-traits and blitz-dom net.rs, none reached by the change
