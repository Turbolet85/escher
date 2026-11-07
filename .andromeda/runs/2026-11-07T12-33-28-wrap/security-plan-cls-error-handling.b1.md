@answer n=1 entity=error-handling#1.1 v=no
why: the refusal, cause and error types live in escher-driver's refusal.rs, schema.rs and error.rs, none of which the session changes
@answer n=2 entity=error-handling#2.1 v=no
why: the twelve event-data unimplemented!() conversions sit in events.rs, which the session does not change and whose citations are unmoved, and the new MutationBuffer adds no panic path
@answer n=3 entity=error-handling#3.1 v=no
why: its setup unwraps sit in dioxus-native/src/lib.rs, blitz-shell and blitz-vibey-script, none of which the session changes, and the session touches only the render path in dioxus-native-dom
@answer n=4 entity=error-handling#4.1 v=no
why: the parallel traversal lock lives in blitz-dom's stylo.rs, a crate the session does not touch
@answer n=5 entity=error-handling#5.1 v=no
why: the select list readers live in blitz-dom's document.rs, which the session does not touch
@answer n=6 entity=error-handling#6.1 v=yes mode=contradict
why: the buffered render takes its one borrow_mut after rendering, so a task polled during render reaches the document, the RefCell already borrowed panic is gone and the probe pins are now spawned_acts; F2 names the limit such a task meets
@answer n=7 entity=error-handling#7.1 v=yes mode=stale
why: the session added 8 lines above dioxus_document.rs line 242, so the cited flush_queued_mounted_events range 242-246 moves to 250-254, while the skipped upstream test and its 1-versus-0 reading still hold
@answer n=8 entity=error-handling#8.1 v=no
why: the anonymous-block cure lives in blitz-dom's document.rs and its tests, none of which the session reaches
@answer n=9 entity=error-handling#9.1 v=no
why: the select list closing paths live in blitz-dom's document.rs and select_popup.rs, which the session does not change
@answer n=10 entity=error-handling#10.1 v=no
why: the NetHandler and ResourceHandler failure paths live in blitz-traits and blitz-dom net.rs, outside the files the session changes
