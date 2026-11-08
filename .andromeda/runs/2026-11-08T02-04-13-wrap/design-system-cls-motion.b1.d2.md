@answer n=1 entity=motion#1.1 v=yes mode=stale
why: the session registers the seven_guis task signals through use_graph_signal in examples/seven_guis/src/tasks, which is the tree this absence search covers
@answer n=2 entity=motion#1.2 v=no
why: the smooth-scroll rules live in blitz-dom scrolling.rs and the session reaches no blitz-dom file
@answer n=3 entity=motion#1.3 v=no
why: the open select list's wheel rules live in blitz-dom document.rs and scrolling.rs, which the session does not reach
@answer n=4 entity=motion#1.4 v=no
why: the scroll behaviour parsing sits in blitz-vibey-script and the events.rs lines cited are not among the reached files
@answer n=5 entity=motion#1.5 v=yes mode=stale
why: the Cells grid arrow-key handler that calls scroll_to with Instant lives in examples/seven_guis/src/tasks/cells.rs, which the session reaches through its signal registration
@answer n=6 entity=motion#1.6 v=no
why: the instant into-view and driver scroll behaviour lives in blitz-dom scrolling.rs and blitz-test-harness input.rs, and the reached driver files only dispatch the verb
@answer n=7 entity=motion#1.7 v=no
why: the touch fling deceleration lives in blitz-dom scrolling.rs, which the session does not reach
@answer n=8 entity=motion#1.8 v=no
why: the overlay scrollbar fade constants live in blitz-dom node/scrollbar.rs, which the session does not reach
@answer n=9 entity=motion#1.9 v=yes mode=stale
why: the Cells grid cell position that a click lands on under the scrollbar band is defined in examples/seven_guis/src/tasks/cells.rs, which the session reaches
@answer n=10 entity=motion#2.1 v=no
why: CSS animation and transition timing lives in blitz-dom stylo.rs, which the session does not reach
@answer n=11 entity=motion#2.2 v=no
why: the settle membership rules live in blitz-test-harness settle.rs, which the session does not reach, and a graph read runs no settle
@answer n=12 entity=motion#3.1 v=yes mode=stale
why: the stand's whole-Timer tick that the driver's advance drives lives in examples/seven_guis/src/tasks/timer.rs, which the session reaches through its signal registration
@answer n=13 entity=motion#3.2 v=no
why: the select list no-motion rule lives in blitz-dom and stand_act_popup.rs, and the session's clickable snapshot field does not change read freshness
@answer n=14 entity=motion#4.1 v=no
why: the load wait lives in escher-driver session.rs, which the session does not reach, and a graph read runs no acting step
@answer n=15 entity=motion#4.2 v=no
why: the requestAnimationFrame timer approximation lives in blitz-vibey-script timers.rs, which the session does not reach
@answer n=16 entity=motion#4.3 v=no
why: the unimplemented animation and transition event data lives in dioxus-native-dom events.rs, which the session does not reach, and the reached dioxus_document.rs change adds only a registry dispatch
@answer n=17 entity=motion#5.1 v=no
why: the BBC reduced-motion fixture is an example asset that the session does not reach
@answer n=18 entity=motion#6.1 v=no
why: the session adds no prefers-reduced-motion or motion-preference rule to any searched slice
@answer n=19 entity=motion#6.2 v=no
why: the session adds no transition or animation rule to any searched slice
@answer n=20 entity=motion#7.1 v=no
why: the keyframes and transitions live in the examples/assets html files, which the session does not reach
@answer n=21 entity=motion#7.2 v=no
why: the google.html keyframes and transitions live in an example asset that the session does not reach
@answer n=22 entity=motion#7.3 v=no
why: the graphite, gosub and inline-flex-transform transitions live in example assets that the session does not reach
@answer n=23 entity=motion#7.4 v=no
why: the servo.css spinner and transitions live in an example asset that the session does not reach
@answer n=24 entity=motion#7.5 v=no
why: the svg_native and transforms example transitions live in example binaries that the session does not reach
